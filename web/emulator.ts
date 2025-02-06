import { Breakpoints } from './debugger/break';
import * as wasm from './glue/pkg/glue';
import { Status as EmulatorStatus } from './glue/pkg/glue';
import { fetchFileSet, FileSet, JsHost } from './host';

export { EmulatorStatus };

/** Functions the emulator may need to call. */
export interface EmulatorHost {
  onWindowChanged(): void;
  onError(msg: string): void;
  onStdOut(stdout: string): void;
  onStopped(status: EmulatorStatus): void;
  /** DOM event on an emulator surface; should be forwarded to emulator. */
  onEvent(event: Event): void;
}

/** Wraps wasm.Emulator, able to run in a loop while still yielding to browser events. */
export class Emulator extends JsHost {
  readonly emu: wasm.Emulator;
  breakpoints: Breakpoints;
  looper: Looper;

  constructor(
    host: EmulatorHost,
    files: FileSet,
    readonly exePath: string,
    cmdLine: string,
    externalDLLs: string[],
    bytes: Uint8Array,
    relocate: boolean,
  ) {
    super(host, files);
    this.emu = wasm.new_emulator(this);
    this.emu.set_external_dlls(externalDLLs);
    this.emu.load_exe(exePath, bytes, cmdLine, relocate);
    this.breakpoints = new Breakpoints(exePath);
    this.looper = new Looper(this.runBatch);
  }

  step() {
    this.emu.unblock(); // Attempt to resume any blocked threads.
    this.emu.run(1);
  }

  private runBatch = (stepSize: number): number | null => {
    const startSteps = this.emu.instr_count;
    const cpuState = this.emu.run(stepSize) as wasm.Status;
    const endSteps = this.emu.instr_count;

    if (cpuState !== wasm.Status.Running) {
      this.breakpoints.uninstall(this.emu);
      this.emuHost.onStopped(cpuState);
      return null;
    }

    const steps = endSteps - startSteps;
    return steps;
  };

  start() {
    if (this.looper.running) return;
    this.emu.unblock(); // Attempt to resume any blocked threads.
    // Advance past the current breakpoint, if any.
    if (this.breakpoints.isAtBreakpoint(this.emu.eip)) {
      this.step();
    }
    this.breakpoints.install(this.emu);
    this.looper.start();
  }

  stop() {
    this.looper.stop();
  }

  enqueueEvent(event: Event) {
    this.events.push(event);
    this.start();
  }

  mappings(): wasm.Mapping[] {
    return JSON.parse(this.emu.mappings_json()) as wasm.Mapping[];
  }

  labels(): Array<[number, string]> {
    const obj = JSON.parse(this.emu.labels()) as Record<number, string>;
    return Object.entries(obj).map(([addr, label]) => [parseInt(addr, 10), label]);
  }

  disassemble(addr: number): wasm.Instruction[] {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    return JSON.parse(this.emu.disassemble_json(addr, 20)) as wasm.Instruction[];
  }
}

/** Target milliseconds to execute per batch. */
const TARGET_MS = 8;

/** Runs blocking code (emulation) while yielding to to the browser event loop. */
class Looper {
  channel = new MessageChannel();

  running = false;

  /** Number of steps to execute per stepMany, adjusted dynamically. */
  stepSize = 5000;
  /** Moving average of steps executed per millisecond. */
  stepsPerMs = 0;

  constructor(private loopee: (count: number) => number | null) {
    this.channel.port2.onmessage = () => this.loop();
  }

  /** Main "loop" that loops asynchronously by posting to the internal message channel. */
  private loop() {
    if (!this.running) return;
    const keepGoing = this.runBatch();
    if (!keepGoing) {
      this.stop();
    } else {
      this.channel.port1.postMessage(null);
    }
  }

  /** Runs one batch of steps, measuring and adjusting parameters to hit this.targetMs. */
  private runBatch(): boolean {
    const startTime = performance.now();
    const steps = this.loopee(this.stepSize);
    const endTime = performance.now();

    if (steps === null) {
      return false;
    }

    if (steps > 1000) { // only update if we ran enough instructions to get a good measurement
      const deltaTime = endTime - startTime;

      const stepsPerMs = steps / deltaTime;
      const alpha = 0.5; // smoothing factor
      this.stepsPerMs = alpha * stepsPerMs + (1 - alpha) * this.stepsPerMs;

      if (deltaTime < TARGET_MS) {
        this.stepSize = this.stepsPerMs * TARGET_MS;
        // console.log(`${steps} instructions in ${deltaTime.toFixed(0)}ms; adjusted step rate: ${this.stepSize}`);
      }
    }

    return true;
  }

  start() {
    if (this.running) return;
    this.running = true;
    this.channel.port1.postMessage(null);
  }

  stop() {
    this.running = false;
  }
}

interface URLParams {
  /** URL directory that all other paths are resolved relative to. */
  dir?: string;
  /** Executable to run. */
  exe: string;
  /** DLLs to load from files instead of builtin implementations. */
  externalDLLs: string[];
  /** Other data files to load.  TODO: we should fetch these dynamically instead. */
  files: string[];
  /** If true, relocate the exe on load. */
  relocate?: boolean;
  /** Command line to pass to executable. */
  cmdLine?: string;
}

function parseURL(): URLParams | undefined {
  const query = new URLSearchParams(document.location.search);
  const exe = query.get('exe');
  if (!exe) return undefined;
  const dir = query.get('dir') || undefined;
  const externalDLLs = (query.get('external') || '').split(',');
  const files = query.getAll('file');
  const relocate = query.has('relocate');
  const cmdLine = query.get('cmdline') || undefined;
  const params: URLParams = { dir, exe, externalDLLs, files, relocate, cmdLine };
  return params;
}

export async function loadEmulator(host: EmulatorHost) {
  const params = parseURL();
  if (!params) {
    throw new Error('invalid URL params');
  }

  const fileset = await fetchFileSet([params.exe, ...params.files], params.dir);

  await wasm.default(new URL('wasm.wasm', document.location.href));

  const cmdLine = params.cmdLine ?? params.exe;
  const exePath = (params.dir ?? '') + params.exe;
  return new Emulator(
    host,
    fileset,
    exePath,
    cmdLine,
    params.externalDLLs,
    fileset.get(params.exe)!,
    params.relocate ?? false,
  );
}
