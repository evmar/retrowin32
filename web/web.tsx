import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Breakpoint, BreakpointsComponent } from './break';
import { Code } from './code';
import * as wasm from './glue/pkg';
import { Labels, Loader as LabelsLoader } from './labels';
import { Mappings } from './mappings';
import { Memory } from './memory';
import { RegistersComponent } from './registers';
import { SnapshotsComponent } from './snapshots';
import { Stack } from './stack';
import { Tabs } from './tabs';
import { hex } from './util';

async function loadExe(path: string): Promise<ArrayBuffer> {
  return await (await fetch(path)).arrayBuffer();
}

// Matches 'pub type JsSurface' in lib.rs.
interface JsSurface {
  write_pixels(pixels: Uint8Array): void;
  get_attached(): JsSurface;
  flip(): void;
  bit_blt(dx: number, dy: number, other: JsSurface, sx: number, sy: number, w: number, h: number): void;
}

// Matches 'pub type JsWindow' in lib.rs.
interface JsWindow {
  title: string;
  set_size(width: number, height: number): void;
}

// Matches 'pub type JsHost' in lib.rs.
interface JsHost {
  exit(code: number): void;
  write(buf: Uint8Array): number;
  time(): number;
  create_window(): JsWindow;
  create_surface(opts: wasm.SurfaceOptions): JsSurface;
}

class Surface implements JsSurface {
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D;
  back?: Surface;

  constructor(width: number, height: number, primary: boolean) {
    this.canvas = document.createElement('canvas');
    if (primary) {
      this.back = new Surface(width, height, false);
    }
    this.canvas.width = width;
    this.canvas.height = height;
    this.ctx = this.canvas.getContext('2d')!;
    this.ctx.fillStyle = 'black';
    this.ctx.fillRect(0, 0, 640, 480);
    this.ctx.fill();
  }

  write_pixels(pixels: Uint8Array): void {
    const data = new ImageData(this.canvas.width, this.canvas.height);
    // XXX Ew copy.  Docs suggest the ImageData ctor accepts pixel data as a param
    // but I couldn't see it working.
    data.data.set(pixels);
    this.ctx.putImageData(data, 0, 0);
  }

  get_attached(): JsSurface {
    if (!this.back) throw new Error('no back for attached');
    return this.back!;
  }

  flip() {
    if (!this.back) throw new Error('no back for flip');
    this.ctx.drawImage(this.back.canvas, 0, 0);
    // TODO: do we need to swap canvases or something?
  }

  bit_blt(dx: number, dy: number, other: JsSurface, sx: number, sy: number, w: number, h: number): void {
    this.ctx.drawImage((other as unknown as Surface).canvas, sx, sy, w, h, dx, dy, w, h);
  }
}

class Window implements JsWindow {
  constructor(
    /** Unique ID for React purposes. */
    readonly key: number,
  ) {}
  title: string = '';
  width: number | undefined;
  height: number | undefined;
  surface?: Surface;
  set_size(w: number, h: number) {
    this.width = w;
    this.height = h;
  }
}

class VM implements JsHost {
  emu: wasm.Emulator = wasm.new_emulator(this);
  decoder = new TextDecoder();
  breakpoints = new Map<number, Breakpoint>();
  imports: string[] = [];
  labels: Labels;
  exitCode: number | undefined = undefined;
  stdout = '';
  page!: Page;

  constructor(private readonly path: string, exe: ArrayBuffer, labelsLoader: LabelsLoader) {
    // new Uint8Array(exe: TypedArray) creates a uint8 view onto the buffer, no copies.
    // But then passing the buffer to Rust must copy the array into the WASM heap...
    const importsJSON = JSON.parse(this.emu.load_exe(new Uint8Array(exe)));
    for (const [jsAddr, jsName] of Object.entries(importsJSON)) {
      const addr = parseInt(jsAddr);
      const name = jsName as string;
      this.imports.push(`${hex(addr, 8)}: ${name}`);
      labelsLoader.add(addr, name);
    }
    this.labels = new Labels(labelsLoader);

    // // Hack: twiddle msvcrt output mode to use console.
    // this.x86.poke(0x004095a4, 1);

    this.loadBreakpoints();
  }

  private loadBreakpoints() {
    const json = window.localStorage.getItem(this.path);
    if (!json) return;
    const bps = JSON.parse(json) as Breakpoint[];
    for (const bp of bps) {
      this.addBreak(bp, /* save= */ false);
    }
  }

  private saveBreakpoints() {
    window.localStorage.setItem(this.path, JSON.stringify(Array.from(this.breakpoints.values())));
  }

  addBreak(bp: Breakpoint, save = true) {
    this.breakpoints.set(bp.addr, bp);
    if (!bp.disabled) {
      this.emu.breakpoint_add(bp.addr);
    }
    if (save) this.saveBreakpoints();
  }

  addBreakByName(name: string): boolean {
    for (const [addr, label] of this.labels.byAddr) {
      if (label === name) {
        this.addBreak({ addr });
        return true;
      }
    }
    if (name.match(/^[0-9a-fA-F]+$/)) {
      const addr = parseInt(name, 16);
      this.addBreak({ addr });
      return true;
    }
    return false;
  }

  delBreak(addr: number) {
    this.breakpoints.delete(addr);
    this.emu.breakpoint_clear(addr);
    this.saveBreakpoints();
  }

  toggleBreak(addr: number) {
    const bp = this.breakpoints.get(addr)!;
    bp.disabled = !bp.disabled;
    if (bp.disabled) {
      this.emu.breakpoint_clear(addr);
    } else {
      this.emu.breakpoint_add(addr);
    }
    this.saveBreakpoints();
  }

  /** Check if the current address is a break/exit point, returning true if so. */
  checkBreak(): boolean {
    if (this.exitCode !== undefined) return true;
    const ip = this.emu.eip;
    const bp = this.breakpoints.get(ip);
    if (bp && !bp.disabled) {
      if (bp.oneShot) {
        this.delBreak(bp.addr);
      } else {
        this.page.setState({ selectedTab: 'breakpoints' });
      }
      return true;
    }
    return false;
  }

  /** Returns true if we should keep running after this (no breakpoint). */
  stepPastBreak(): boolean {
    const ip = this.emu.eip;
    const bp = this.breakpoints.get(ip);
    if (bp && !bp.disabled) {
      this.emu.breakpoint_clear(ip);
      const ret = this.step();
      this.emu.breakpoint_add(ip);
      return ret;
    } else {
      return this.step();
    }
  }

  /** Returns true if we should keep running after this (no breakpoint). */
  step(): boolean {
    this.emu.step();
    return !this.checkBreak();
  }

  /** Number of instructions to execute per stepMany, adjusted dynamically. */
  stepSize = 5000;
  /** Moving average of instructions executed per millisecond. */
  instrPerMs = 0;

  /** Runs a batch of instructions.  Returns false if we should stop. */
  stepMany(): boolean {
    const start = performance.now();
    const steps = this.emu.step_many(this.stepSize);
    const end = performance.now();

    if (this.checkBreak()) {
      return false;
    }

    const delta = end - start;
    const instrPerMs = this.stepSize / delta;
    const alpha = 0.5; // smoothing factor
    this.instrPerMs = alpha * (instrPerMs) + (alpha - 1) * this.instrPerMs;

    if (delta < 10) {
      this.stepSize *= 2;
      console.log('adjusted step rate', this.stepSize);
    }

    return true;
  }

  mappings(): wasm.Mapping[] {
    return JSON.parse(this.emu.mappings_json()) as wasm.Mapping[];
  }
  disassemble(addr: number): wasm.Instruction[] {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    return JSON.parse(this.emu.disassemble_json(addr)) as wasm.Instruction[];
  }

  exit(code: number) {
    console.warn('exited with code', code);
    this.exitCode = code;
  }
  write(buf: Uint8Array): number {
    this.stdout += this.decoder.decode(buf);
    this.page.setState({ stdout: this.stdout });
    return buf.length;
  }
  time(): number {
    return Math.floor(performance.now());
  }

  windows: Window[] = [];
  create_window(): JsWindow {
    let id = this.windows.length + 1;
    this.windows.push(new Window(id));
    this.page.forceUpdate();
    return this.windows[id - 1];
  }

  create_surface(opts: wasm.SurfaceOptions): JsSurface {
    const { width, height, primary } = opts;
    opts.free();
    const surface = new Surface(width, height, primary);
    // XXX how to tie surface and window together?
    // The DirectDraw calls SetCooperativeLevel() on the hwnd, and then CreateSurface with primary,
    // but how to plumb that info across JS boundary?
    if (primary) {
      this.windows[this.windows.length - 1].surface = surface;
      console.warn('hack: attached surface to window');
      this.page.forceUpdate();
    }
    return surface;
  }
}

namespace WindowComponent {
  export interface Props {
    title: string;
    size: [number | undefined, number | undefined];
    canvas?: HTMLCanvasElement;
  }
  export interface State {
    drag?: [number, number];
    pos: [number, number];
  }
}
class WindowComponent extends preact.Component<WindowComponent.Props, WindowComponent.State> {
  state: WindowComponent.State = {
    pos: [200, 200],
  };
  ref = preact.createRef();

  beginDrag = (e: PointerEvent) => {
    const node = e.currentTarget as HTMLElement;
    this.setState({ drag: [e.offsetX, e.offsetY] });
    node.setPointerCapture(e.pointerId);
    e.preventDefault();
  };
  onDrag = (e: PointerEvent) => {
    if (!this.state.drag) return;
    this.setState({ pos: [e.clientX - this.state.drag[0], e.clientY - this.state.drag[1]] });
    e.preventDefault();
  };
  endDrag = (e: PointerEvent) => {
    const node = e.currentTarget as HTMLElement;
    this.setState({ drag: undefined });
    node.releasePointerCapture(e.pointerId);
    e.preventDefault();
  };

  ensureCanvas() {
    // XXX: how to ensure the canvas appears as a child of this widget?
    if (this.props.canvas && this.ref.current && !this.ref.current.firstChild) {
      this.ref.current.appendChild(this.props.canvas);
    }
  }

  componentDidMount(): void {
    this.ensureCanvas();
  }

  render() {
    this.ensureCanvas();
    function pxOrUndefined(x: number | undefined): string | undefined {
      if (x !== undefined) return `${x}px`;
      return undefined;
    }
    return (
      <div class='window' style={{ left: `${this.state.pos[0]}px`, top: `${this.state.pos[1]}px` }}>
        <div class='titlebar' onPointerDown={this.beginDrag} onPointerUp={this.endDrag} onPointerMove={this.onDrag}>
          {this.props.title}
        </div>
        <div
          ref={this.ref}
          style={{ width: pxOrUndefined(this.props.size[0]), height: pxOrUndefined(this.props.size[1]) }}
        >
        </div>
      </div>
    );
  }
}

namespace Page {
  export interface Props {
    vm: VM;
  }
  export interface State {
    stdout: string;
    memBase: number;
    memHighlight?: number;
    /** When running, the setInterval id that is updating the UI. */
    running?: number;
    selectedTab: string;
  }
}
class Page extends preact.Component<Page.Props, Page.State> {
  state: Page.State = { stdout: '', memBase: 0x40_1000, selectedTab: 'output' };

  constructor(props: Page.Props) {
    super(props);
    this.props.vm.page = this;
  }

  step(): boolean {
    try {
      return this.props.vm.stepPastBreak();
    } finally {
      this.forceUpdate();
    }
  }

  start() {
    if (this.state.running) return;
    if (!this.step()) { // Advance past the current breakpoint, if any.
      return;
    }
    const interval = setInterval(() => {
      this.forceUpdate();
    }, 500);
    this.setState({ running: interval }, () => this.runFrame());
  }

  stop() {
    if (!this.state.running) return;
    clearInterval(this.state.running);
    this.setState({ running: undefined });
  }

  /** Runs a batch of instructions; called in RAF loop. */
  runFrame() {
    if (!this.state.running) return;
    let stop;
    try {
      stop = !this.props.vm.stepMany();
    } catch (e) {
      console.error(e);
      stop = true;
    }
    if (stop) {
      this.stop();
      return;
    }
    requestAnimationFrame(() => this.runFrame());
  }

  runTo(addr: number) {
    this.props.vm.addBreak({ addr, oneShot: true });
    this.start();
  }

  highlightMemory = (addr: number) => this.setState({ memHighlight: addr });
  showMemory = (memBase: number) => {
    this.setState({ selectedTab: 'memory', memBase });
  };

  render() {
    let windows = this.props.vm.windows.map((window) => {
      return (
        <WindowComponent
          key={window.key}
          title={window.title}
          size={[window.width, window.height]}
          canvas={window.surface?.canvas}
        />
      );
    });
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = this.props.vm.disassemble(this.props.vm.emu.eip);
    return (
      <>
        {windows}
        <div style={{ margin: '1ex', display: 'flex', alignItems: 'baseline' }}>
          <button
            onClick={() => this.state.running ? this.stop() : this.start()}
          >
            {this.state.running ? 'stop' : 'run'}
          </button>
          &nbsp;
          <button
            onClick={() => this.step()}
          >
            step
          </button>
          &nbsp;
          <button
            onClick={() => this.runTo(instrs[1].addr)}
          >
            step over
          </button>
          &nbsp;
          <div>
            {this.props.vm.emu.instr_count} instrs executed | {Math.floor(this.props.vm.instrPerMs)}/ms
          </div>
        </div>
        <div style={{ display: 'flex' }}>
          <Code
            instrs={instrs}
            labels={this.props.vm.labels}
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            runTo={(addr: number) => this.runTo(addr)}
          />
          <div style={{ width: '12ex' }} />
          <RegistersComponent
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            regs={this.props.vm.emu}
          />
        </div>
        <div style={{ display: 'flex' }}>
          <Tabs
            style={{ width: '80ex' }}
            tabs={{
              output: (
                <section>
                  <code>{this.state.stdout}</code>
                </section>
              ),

              memory: (
                <Memory
                  mem={this.props.vm.emu.memory()}
                  base={this.state.memBase}
                  highlight={this.state.memHighlight}
                  jumpTo={(addr) => this.setState({ memBase: addr })}
                />
              ),
              mappings: <Mappings mappings={this.props.vm.mappings()} highlight={this.state.memHighlight} />,

              imports: (
                <section>
                  <code>
                    {this.props.vm.imports.map(imp => <div>{imp}</div>)}
                  </code>
                </section>
              ),

              breakpoints: (
                <BreakpointsComponent
                  breakpoints={Array.from(this.props.vm.breakpoints.values())}
                  labels={this.props.vm.labels}
                  highlight={this.props.vm.emu.eip}
                  highlightMemory={this.highlightMemory}
                  showMemory={this.showMemory}
                  toggle={(addr) => {
                    this.props.vm.toggleBreak(addr);
                    this.forceUpdate();
                  }}
                  add={(text) => {
                    const ret = this.props.vm.addBreakByName(text);
                    this.forceUpdate();
                    return ret;
                  }}
                />
              ),

              snapshots: (
                <SnapshotsComponent
                  take={() => this.props.vm.emu.snapshot()}
                  load={(snap) => {
                    this.props.vm.emu.load_snapshot(snap);
                    this.forceUpdate();
                  }}
                />
              ),
            }}
            selected={this.state.selectedTab}
            switchTab={(selectedTab) => this.setState({ selectedTab })}
          />
          <Stack
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            labels={this.props.vm.labels}
            emu={this.props.vm.emu}
          />
        </div>
      </>
    );
  }
}

async function main() {
  const path = document.location.search.substring(1);
  if (!path) throw new Error('expected ?path in URL');
  const exe = await loadExe(path);
  const loader = new LabelsLoader();
  await loader.fetchCSV(path);
  await wasm.default(new URL('wasm.wasm', document.location.href));

  const vm = new VM(path, exe, loader);
  preact.render(<Page vm={vm} />, document.body);
}

main();
