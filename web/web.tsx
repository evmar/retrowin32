import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { BreakpointsComponent } from './break';
import { Code } from './code';
import { Emulator } from './emulator';
import * as wasm from './glue/pkg';
import { Host } from './host';
import { parseCSV } from './labels';
import { Mappings } from './mappings';
import { Memory } from './memory';
import { RegistersComponent } from './registers';
import { SnapshotsComponent } from './snapshots';
import { Stack } from './stack';
import { Tabs } from './tabs';

namespace WindowComponent {
  export interface Props {
    title: string;
    size: [number | undefined, number | undefined];
    canvas: HTMLCanvasElement;
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
    host: Host;
    emulator: Emulator;
  }
  export interface State {
    stdout: string;
    error: string;
    memBase: number;
    memHighlight?: number;
    /** When running, the setInterval id that is updating the UI. */
    running?: number;
    selectedTab: string;
  }
}
export class Page extends preact.Component<Page.Props, Page.State> {
  state: Page.State = { stdout: '', error: '', memBase: 0x40_1000, selectedTab: 'output' };

  constructor(props: Page.Props) {
    super(props);
    this.props.host.page = this;
  }

  step() {
    try {
      this.props.emulator.step();
    } finally {
      this.forceUpdate();
    }
  }

  start() {
    if (this.state.running) return;
    // Advance past the current breakpoint, if any.
    if (this.props.emulator.isAtBreakpoint()) {
      this.step();
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
      stop = !this.props.emulator.stepMany();
    } catch (e) {
      const err = e as Error;
      console.error(err);
      this.setState({ error: err.message });
      stop = true;
    }
    if (stop) {
      this.stop();
      return;
    }
    requestAnimationFrame(() => this.runFrame());
  }

  runTo(addr: number) {
    this.props.emulator.addBreak({ addr, oneShot: true });
    this.start();
  }

  highlightMemory = (addr: number) => this.setState({ memHighlight: addr });
  showMemory = (memBase: number) => {
    this.setState({ selectedTab: 'memory', memBase });
  };

  render() {
    let windows = this.props.host.windows.map((window) => {
      return (
        <WindowComponent
          key={window.hwnd}
          title={window.title}
          size={[window.width, window.height]}
          canvas={window.canvas}
        />
      );
    });
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = this.props.emulator.disassemble(this.props.emulator.emu.eip);
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
            {this.props.emulator.emu.instr_count} instrs executed | {Math.floor(this.props.emulator.instrPerMs)}/ms
          </div>
        </div>
        <div style={{ display: 'flex' }}>
          <Code
            instrs={instrs}
            labels={this.props.emulator.labels}
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            runTo={(addr: number) => this.runTo(addr)}
          />
          <div style={{ width: '12ex' }} />
          <RegistersComponent
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            regs={this.props.emulator.emu.regs()}
          />
        </div>
        <div style={{ display: 'flex' }}>
          <Tabs
            style={{ width: '80ex' }}
            tabs={{
              output: (
                <section>
                  <code>
                    {this.state.stdout}
                    {this.state.error ? <div class='error'>ERROR: {this.state.error}</div> : null}
                  </code>
                </section>
              ),

              memory: (
                <Memory
                  mem={this.props.emulator.emu.memory()}
                  base={this.state.memBase}
                  highlight={this.state.memHighlight}
                  jumpTo={(addr) => this.setState({ memBase: addr })}
                />
              ),
              mappings: <Mappings mappings={this.props.emulator.mappings()} highlight={this.state.memHighlight} />,

              imports: (
                <section>
                  <code>
                    {this.props.emulator.imports.map(imp => <div>{imp}</div>)}
                  </code>
                </section>
              ),

              breakpoints: (
                <BreakpointsComponent
                  breakpoints={Array.from(this.props.emulator.breakpoints.values())}
                  labels={this.props.emulator.labels}
                  highlight={this.props.emulator.emu.eip}
                  highlightMemory={this.highlightMemory}
                  showMemory={this.showMemory}
                  toggle={(addr) => {
                    this.props.emulator.toggleBreak(addr);
                    this.forceUpdate();
                  }}
                  add={(text) => {
                    const ret = this.props.emulator.addBreakByName(text);
                    this.forceUpdate();
                    return ret;
                  }}
                  remove={(addr) => {
                    this.props.emulator.delBreak(addr);
                    this.forceUpdate();
                  }}
                />
              ),

              snapshots: (
                <SnapshotsComponent
                  take={() => this.props.emulator.emu.snapshot()}
                  load={(snap) => {
                    this.props.emulator.emu.load_snapshot(snap);
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
            labels={this.props.emulator.labels}
            emu={this.props.emulator.emu}
          />
        </div>
      </>
    );
  }
}

interface URLParams {
  /** URL directory that all other paths are resolved relative to. */
  dir?: string;
  /** Executable to run. */
  exe: string;
  /** Other data files to load.  TODO: we should fetch these dynamically instead. */
  files: string[];
  /** If true, relocate the exe on load. */
  relocate?: boolean;
}

function parseURL(): URLParams | undefined {
  const query = new URLSearchParams(document.location.search);
  const exe = query.get('exe');
  if (!exe) return undefined;
  const dir = query.get('dir') || undefined;
  const files = query.getAll('file');
  const relocate = query.has('relocate');
  const params: URLParams = { dir, exe, files, relocate };
  return params;
}

async function debuggerPage() {
  const params = parseURL();
  if (!params) {
    return <p>invalid URL params</p>;
  }

  const host = new Host();
  await host.fetch([params.exe, ...params.files], params.dir);

  await wasm.default(new URL('wasm.wasm', document.location.href));

  const csvLabels = new Map<number, string>();
  const resp = await fetch(params.exe + '.csv');
  if (resp.ok) {
    for (const [addr, name] of parseCSV(await resp.text())) {
      csvLabels.set(addr, name);
    }
  }

  const storageKey = (params.dir ?? '') + params.exe;
  const emulator = new Emulator(host, storageKey, host.files.get(params.exe)!, csvLabels, params.relocate ?? false);

  return <Page host={host} emulator={emulator} />;
}

async function main() {
  preact.render(await debuggerPage(), document.body);
}

main();
