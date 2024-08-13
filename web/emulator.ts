import { Breakpoints } from './break';
import * as wasm from './glue/pkg/glue';
import { FileSet, JsHost } from './host';
import { Labels } from './labels';
import { hex } from './util';

/** Functions the emulator may need to call. */
export interface EmulatorHost {
  exit(code: number): void;
  onWindowChanged(): void;
  showTab(name: string): void;
  onError(msg: string): void;
  onStdOut(stdout: string): void;
  onStopped(): void;
}

/** Wraps wasm.Emulator, able to run in a loop while still yielding to browser events. */
export class Emulator extends JsHost {
  readonly emu: wasm.Emulator;
  imports: string[] = [];
  labels: Labels;
  /** True when the emulator is actively trying to loop and executing instructions, false when stopped or blocked. */
  running = false;
  breakpoints: Breakpoints;
  channel = new MessageChannel();

  constructor(
    host: EmulatorHost,
    files: FileSet,
    exePath: string,
    cmdLine: string,
    bytes: Uint8Array,
    labels: Map<number, string>,
    relocate: boolean,
  ) {
    super(host, files);
    this.emu = wasm.new_emulator(this, cmdLine);
    this.emu.load_exe(exePath, bytes, relocate);
    this.breakpoints = new Breakpoints(exePath);

    const importsJSON = JSON.parse(this.emu.labels());
    for (const [jsAddr, jsName] of Object.entries(importsJSON)) {
      const addr = parseInt(jsAddr);
      const name = jsName as string;
      this.imports.push(`${hex(addr, 8)}: ${name}`);
      labels.set(addr, name);
    }
    this.labels = new Labels(labels);

    // // Hack: twiddle msvcrt output mode to use console.
    // this.x86.poke(0x004095a4, 1);

    this.channel.port2.onmessage = () => this.loop();
  }

  step() {
    this.emu.unblock(); // Attempt to resume any blocked threads.
    this.emu.run(1);
  }

  /** Number of instructions to execute per stepMany, adjusted dynamically. */
  stepSize = 5000;
  /** Moving average of instructions executed per millisecond. */
  instrPerMs = 0;

  private runBatch() {
    const startTime = performance.now();
    const startSteps = this.emu.instr_count;
    const cpuState = this.emu.run(this.stepSize) as wasm.CPUState;
    const endTime = performance.now();
    const endSteps = this.emu.instr_count;

    const steps = endSteps - startSteps;
    const deltaTime = endTime - startTime;
    if (steps > 1000 && deltaTime >= 1) { // only update if we ran enough instructions to get a good measurement
      const instrPerMs = steps / deltaTime;
      const alpha = 0.5; // smoothing factor
      this.instrPerMs = alpha * instrPerMs + (1 - alpha) * this.instrPerMs;

      if (deltaTime < 8) {
        this.stepSize = Math.min(Math.max(this.instrPerMs * 8, 1000), 100000);
        // console.log(`${steps} instructions in ${deltaTime.toFixed(0)}ms; adjusted step rate: ${this.stepSize}`);
      }
    }

    return cpuState;
  }

  /** Runs a batch of instructions.  Returns false if we should stop. */
  private stepMany(): boolean {
    this.breakpoints.install(this.emu);
    const cpuState = this.runBatch();
    this.breakpoints.uninstall(this.emu);

    switch (cpuState) {
      case wasm.CPUState.Running:
        return true;
      case wasm.CPUState.DebugBreak: {
        const bp = this.breakpoints.isAtBreakpoint(this.emu.eip);
        if (bp) {
          if (!bp.oneShot) {
            this.emuHost.showTab('breakpoints');
          }
          this.emuHost.onStopped();
        }
        return false;
      }
      case wasm.CPUState.Blocked:
      case wasm.CPUState.Error:
      case wasm.CPUState.Exit:
        this.emuHost.onStopped();
        return false;
    }
  }

  start() {
    if (this.running) return;
    this.emu.unblock(); // Attempt to resume any blocked threads.
    // Advance past the current breakpoint, if any.
    if (this.breakpoints.isAtBreakpoint(this.emu.eip)) {
      this.step();
    }
    this.running = true;
    this.loop();
  }

  /** Runs a batch of instructions; called in a loop. */
  private loop() {
    if (!this.running) return;
    if (!this.stepMany()) {
      this.stop();
      return;
    }
    this.channel.port1.postMessage(null);
  }

  stop() {
    this.running = false;
  }

  mappings(): wasm.Mapping[] {
    return JSON.parse(this.emu.mappings_json()) as wasm.Mapping[];
  }

  disassemble(addr: number): wasm.Instruction[] {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    return JSON.parse(this.emu.disassemble_json(addr, 20)) as wasm.Instruction[];
  }
}
