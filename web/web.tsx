import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { BreakpointsComponent } from './break';
import { Code } from './code';
import { Emulator, EmulatorHost } from './emulator';
import * as wasm from './glue/pkg';
import { fetchFileSet, JsHost } from './host';
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
    return (
      <div class='window' style={{ left: `${this.state.pos[0]}px`, top: `${this.state.pos[1]}px` }}>
        <div class='titlebar' onPointerDown={this.beginDrag} onPointerUp={this.endDrag} onPointerMove={this.onDrag}>
          {this.props.title}
        </div>
        <div ref={this.ref} />
      </div>
    );
  }
}

namespace EmulatorComponent {
  export interface Props {
    emulator: Emulator;
  }
}
class EmulatorComponent extends preact.Component<EmulatorComponent.Props> {
  render() {
    return this.props.emulator.windows.map((window) => {
      return (
        <WindowComponent
          key={window.hwnd}
          title={window.title}
          canvas={window.canvas}
        />
      );
    });
  }
}

namespace Debugger {
  export interface Props {
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
export class Debugger extends preact.Component<Debugger.Props, Debugger.State> implements EmulatorHost {
  stdout = ''; // XXX hack for making exit() not clobber state
  state: Debugger.State = { stdout: '', error: '', memBase: 0x40_1000, selectedTab: 'output' };

  constructor(props: Debugger.Props) {
    super(props);
    this.props.emulator.emuHost = this;
  }

  exit(code: number): void {
    this.setState({ stdout: this.stdout + `\nexited with code ${code}` });
    this.stop();
  }
  onWindowChanged(): void {
    this.forceUpdate();
  }
  showTab(name: string): void {
    this.setState({ selectedTab: name });
  }
  onError(msg: string): void {
    this.setState({ error: msg });
  }

  onStdOut(msg: string): void {
    this.stdout = msg;
    this.setState({ stdout: msg });
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
    this.setState({
      running: setInterval(() => {
        this.forceUpdate();
      }, 500),
    });
    this.props.emulator.start();
  }

  stop() {
    if (!this.state.running) return;
    this.props.emulator.stop();
    clearInterval(this.state.running);
    this.setState({ running: undefined });
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
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = this.props.emulator.disassemble(this.props.emulator.emu.eip);
    return (
      <>
        <EmulatorComponent emulator={this.props.emulator} />
        <section class='panel' style={{ display: 'flex', alignItems: 'baseline' }}>
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
        </section>
        <div style={{ display: 'flex', margin: '1ex' }}>
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
                <div>
                  <code>
                    {this.state.stdout}
                    {this.state.error ? <div class='error'>ERROR: {this.state.error}</div> : null}
                  </code>
                </div>
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
                <div>
                  <code>
                    {this.props.emulator.imports.map(imp => <div>{imp}</div>)}
                  </code>
                </div>
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

  const fileset = await fetchFileSet([params.exe, ...params.files], params.dir);

  await wasm.default(new URL('wasm.wasm', document.location.href));

  const csvLabels = new Map<number, string>();
  const resp = await fetch(params.exe + '.csv');
  if (resp.ok) {
    for (const [addr, name] of parseCSV(await resp.text())) {
      csvLabels.set(addr, name);
    }
  }

  const storageKey = (params.dir ?? '') + params.exe;
  const emulator = new Emulator(
    null!,
    fileset,
    storageKey,
    fileset.get(params.exe)!,
    csvLabels,
    params.relocate ?? false,
  );

  return <Debugger emulator={emulator} />;
}

async function main() {
  preact.render(await debuggerPage(), document.body);
}

main();
