import { Breakpoints } from './debugger/break';
import * as wasm from './glue/pkg/glue';
import { Status } from './glue/pkg/glue';
export { Status };

async function fetchBytes(path: string): Promise<Uint8Array> {
  const resp = await fetch(path);
  if (!resp.ok) throw new Error(`failed to load ${path}`);
  return new Uint8Array(await resp.arrayBuffer());
}

/** A set of (pre)loaded files; a temporary hack until the emulator can load files itself. */
export type FileSet = Map<string, Uint8Array>;

export async function fetchFileSet(files: string[], dir: string = ''): Promise<FileSet> {
  const fileset: FileSet = new Map();
  for (const file of files) {
    const path = dir + file;
    fileset.set(file.toLowerCase(), await fetchBytes(path));
  }
  return fileset;
}

class Window implements wasm.JsWindow {
  constructor(readonly host: Host, readonly hwnd: number) {
    const stashEvent = (ev: Event) => {
      (ev as any).hwnd = hwnd;
      host.onEvent(ev);
      return false;
    };
    this.canvas.onmousedown = stashEvent;
    this.canvas.onmouseup = stashEvent;
    this.canvas.onmousemove = stashEvent;
    this.canvas.oncontextmenu = (ev) => {
      return false;
    };
  }

  title: string = '';
  canvas: HTMLCanvasElement = document.createElement('canvas');
  size: [number, number] = [0, 0];
  is_fullscreen = false;

  private reset_canvas() {
    const [w, h] = this.size;
    let scale = 1;
    if (this.is_fullscreen && w < 640) scale *= 2;

    // Note: the canvas must be sized to the size of physical pixels,
    // or else it will be scaled up with smoothing and pixels will be blurry.
    this.canvas.width = w * window.devicePixelRatio;
    this.canvas.height = h * window.devicePixelRatio;
    this.canvas.style.width = `${w * scale}px`;
    this.canvas.style.height = `${h * scale}px`;

    // The context scale seems preserved across calls to getContext, but then also
    // lost when the canvas is resized.  Rather than relying on this, always reset
    // and scale the context immediately on resize.
    const ctx = this.canvas.getContext('2d')!;
    ctx.reset();
    ctx.imageSmoothingEnabled = false;
    ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    this.host.onWindowChanged();
  }

  set_size(w: number, h: number) {
    this.size = [w, h];
    this.reset_canvas();
  }

  fullscreen() {
    this.is_fullscreen = true;
    this.reset_canvas();
  }
}

class File implements wasm.JsFile {
  ofs = 0;

  constructor(readonly path: string, readonly bytes: Uint8Array) {
  }

  info(): number {
    return this.bytes.length;
  }

  seek(from: number, ofs: number): number {
    switch (from) {
      case 0: // start
        break;
      case 1: // end
        ofs = this.bytes.length + ofs;
        break;
      case 2: // cur
        ofs = this.ofs + ofs;
        break;
    }
    if (ofs < 0 || ofs > this.bytes.length) {
      throw new Error(`seek out of bounds: ${from}:${ofs}`);
    }
    this.ofs = ofs;
    return ofs;
  }

  read(buf: Uint8Array): number {
    const n = Math.min(buf.length, this.bytes.length - this.ofs);
    buf.set(this.bytes.subarray(this.ofs, this.ofs + n));
    this.ofs += n;
    return n;
  }

  write(buf: Uint8Array): number {
    console.warn('ignoring write');
    return buf.length;
  }
}

/** Functions the emulator may need to call. */
export interface Host {
  onWindowChanged(): void;
  onError(msg: string): void;
  onStdOut(stdout: string): void;
  /** Notification that the emulator stopped, e.g. on blocking, error, or exit. */
  onStopped(status: Status): void;
  /** DOM event on an emulator surface; should be forwarded to emulator. */
  onEvent(event: Event): void;
}

/** Windows function calls, for debugging. */
export interface Trace {
  context: string;
  msg: string;
}

/** Wraps wasm.Emulator, providing the emulation=>web API. */
export class Emulator implements wasm.JsHost {
  readonly emu: wasm.Emulator;

  breakpoints: Breakpoints;
  looper: Looper;

  trace: Trace[] = [];
  events: Event[] = [];
  timer?: number;

  decoder = new TextDecoder();

  constructor(
    readonly host: Host,
    readonly files: FileSet,
    readonly storageKey: string,
    externalDLLs: string[],
  ) {
    this.emu = wasm.new_emulator(this);
    this.emu.set_external_dlls(externalDLLs);
    this.breakpoints = new Breakpoints(storageKey);
    this.looper = new Looper(this.runBatch);
  }

  log(level: number, msg: string) {
    // TODO: surface this in the UI.
    switch (level) {
      case 1:
        console.error(msg);
        this.host.onError(msg);
        break;
      case 2:
        console.warn(msg);
        break;
      case 3:
        console.info(msg);
        break;
      case 4:
      case 5: // "trace"
        console.debug(msg);
        break;
      default:
        throw new Error(`unexpected log level #{level}`);
    }
  }

  win32_trace(context: string, msg: string) {
    this.trace.push({ context, msg });
  }

  ensure_timer(when: number): void {
    if (this.timer) {
      clearTimeout(this.timer);
    }
    const now = performance.now();
    console.log('timer for', when - now);
    const id = setTimeout(() => {
      if (this.timer !== id) {
        return;
      }
      this.timer = undefined;
      this.start();
    }, when - now);
    this.timer = id;
  }

  get_event(): Event | undefined {
    return this.events.shift();
  }

  open(path: string, options: wasm.FileOptions): wasm.JsFile | null {
    if (options.create) {
      return new File(path, new Uint8Array());
    }
    // TODO: async file loading.
    let bytes = this.files.get(path.toLowerCase());
    if (!bytes) {
      return null;
    }
    return new File(path, bytes);
  }

  stdout(buf: Uint8Array) {
    const text = this.decoder.decode(buf);
    this.host.onStdOut(text);
  }

  windows: Window[] = [];
  create_window(hwnd: number): wasm.JsWindow {
    let window = new Window(this.host, hwnd);
    this.windows.push(window);
    this.host.onWindowChanged();
    return window;
  }

  screen() {
    // XXX how to tie surface and window together?
    // The DirectDraw calls SetCooperativeLevel() on the hwnd, and then CreateSurface with primary,
    // but how to plumb that info across JS boundary?
    return this.windows[this.windows.length - 1].canvas.getContext('2d')!;
  }

  audio(buf: Int16Array) {
    console.warn('TODO: audio');
  }

  startExe(cmdLine: string, relocate: boolean) {
    this.emu.start_exe(cmdLine, relocate);
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
      this.looper.stop();
      this.host.onStopped(cpuState);
      return null;
    }

    const steps = endSteps - startSteps;
    return steps;
  };

  start() {
    if (this.looper.running) return;
    this.emu.unblock(); // Attempt to resume any blocked threads.
    // Advance past the current breakpoint, if any.
    if (this.breakpoints.isAtBreakpoint(this.emu.cpu().eip)) {
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

    const deltaTime = endTime - startTime;
    if (steps > 1000 && deltaTime > 0) { // only update if we ran enough instructions to get a good measurement
      const stepsPerMs = steps / deltaTime;
      const alpha = 0.5; // smoothing factor
      this.stepsPerMs = alpha * stepsPerMs + (1 - alpha) * this.stepsPerMs;

      if (deltaTime < TARGET_MS) {
        this.stepSize = Math.round(this.stepsPerMs * TARGET_MS);
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
  /** Command line to run. */
  cmdLine: string;
  /** DLLs to load from files instead of builtin implementations. */
  externalDLLs: string[];
  /** Files to load.  TODO: we should fetch these dynamically instead. */
  files: string[];
  /** If true, relocate the exe on load. */
  relocate?: boolean;
}

function parseURL(): URLParams | undefined {
  const query = new URLSearchParams(document.location.search);
  const files = query.getAll('file');
  const dir = query.get('dir') || undefined;
  const cmdLine = query.get('cmdline') || files[0];
  const exe = cmdLine.split(' ')[0];
  if (!files.includes(exe)) {
    files.unshift(exe);
  }
  const externalDLLs = (query.get('external') || '').split(',');
  const relocate = query.has('relocate');
  const params: URLParams = { dir, cmdLine, externalDLLs, files, relocate };
  return params;
}

export async function load(host: Host) {
  const params = parseURL();
  if (!params) {
    throw new Error('invalid URL params');
  }

  const fileset = await fetchFileSet(params.files, params.dir);

  await wasm.default(new URL('wasm.wasm', document.location.href));

  const storageKey = (params.dir ?? '') + params.cmdLine.split(' ')[0];
  const emu = new Emulator(host, fileset, storageKey, params.externalDLLs);
  emu.startExe(params.cmdLine, params.relocate ?? false);
  return emu;
}
