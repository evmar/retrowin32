import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Breakpoint } from './break';
import { Code } from './code';
import { Mappings } from './mappings';
import { Memory } from './memory';
import { Registers } from './registers';
import { Stack } from './stack';
import { Tabs } from './tabs';
import { hex } from './util';
import * as wasm from './wasm/wasm';

async function loadExe(path: string): Promise<ArrayBuffer> {
  return await (await fetch(path)).arrayBuffer();
}

async function loadLabels(path: string): Promise<Map<number, string>> {
  const labels = new Map<number, string>();
  const resp = await fetch(path + '.csv');
  if (!resp.ok) return labels;
  const text = await resp.text();
  for (const line of text.split('\n')) {
    const [name, addr] = line.split('\t');
    labels.set(parseInt(addr, 16), name);
  }
  return labels;
}

// Matches 'pub type JsWindow' in lib.rs.
interface JsWindow {
  id: number;
  title: string;
}

// Matches 'pub type JsHost' in lib.rs.
interface JsHost {
  exit(code: number): void;
  write(buf: Uint8Array): number;
  time(): number;
  create_window(): JsWindow;
  get_window(id: number): JsWindow;
}

class Window implements JsWindow {
  constructor(readonly id: number) {}
  title: string = '';
  width: number = 0;
  height: number = 0;
}

class VM implements JsHost {
  x86: wasm.X86 = wasm.new_x86(this);
  decoder = new TextDecoder();
  breakpoints = new Map<number, Breakpoint>();
  imports: string[] = [];
  labels: Map<number, string>;
  exitCode: number | undefined = undefined;
  stdout = '';
  page!: Page;

  constructor(exe: ArrayBuffer, labels: Map<number, string>) {
    this.labels = labels;
    // new Uint8Array(exe: TypedArray) creates a uint8 view onto the buffer, no copies.
    // But then passing the buffer to Rust must copy the array into the WASM heap...
    const importsJSON = JSON.parse(this.x86.load_exe(new Uint8Array(exe)));
    for (const [jsAddr, jsName] of Object.entries(importsJSON)) {
      const addr = parseInt(jsAddr);
      const name = jsName as string;
      this.imports.push(`${hex(addr, 8)}: ${name}`);
      this.labels.set(addr, name);
    }

    // // XXX hacks for debugging basicDD.exe
    // this.labels.set(0x004023fe, 'setup_env');
    // this.labels.set(0x00402850, 'setup_heap');
    // this.labels.set(0x004019da, 'fatal');

    // // Hack: twiddle msvcrt output mode to use console.
    // this.x86.poke(0x004095a4, 1);
    this.breakpoints.set(0x4010d5, { addr: 0x4010d5, temporary: true });
  }

  step() {
    this.x86.step();
    if (this.exitCode !== undefined) return false;
    const bp = this.breakpoints.get(this.x86.eip);
    if (bp) {
      if (bp.temporary) {
        this.breakpoints.delete(bp.addr);
      }
      return false;
    }
    return true;
  }

  mappings(): wasm.Mapping[] {
    return JSON.parse(this.x86.mappings_json()) as wasm.Mapping[];
  }
  disassemble(addr: number): wasm.Instruction[] {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    return JSON.parse(this.x86.disassemble_json(addr)) as wasm.Instruction[];
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
  time() {
    return Math.floor(performance.now() * 1000);
  }

  windows: Window[] = [];
  create_window(): JsWindow {
    let id = this.windows.length + 1;
    this.windows.push(new Window(id));
    return this.windows[id - 1];
  }
  get_window(id: number): JsWindow {
    return this.windows[id - 1];
  }
}

namespace WindowComponent {
  export interface Props {
    title: string;
  }
  export interface State {
    drag?: [number, number];
    pos: [number, number];
  }
}
class WindowComponent extends preact.Component<WindowComponent.Props, WindowComponent.State> {
  state: WindowComponent.State = {
    pos: [400, 400],
  };
  ref = preact.createRef();

  beginDrag = (e: PointerEvent) => {
    console.log('begin');
    const node = e.currentTarget as HTMLElement;
    this.setState({ drag: [e.offsetX, e.offsetY] });
    node.setPointerCapture(e.pointerId);
    e.preventDefault();
  };
  onDrag = (e: PointerEvent) => {
    if (!this.state.drag) return;
    console.log('drag', e);
    this.setState({ pos: [e.clientX - this.state.drag[0], e.clientY - this.state.drag[1]] });
    e.preventDefault();
  };
  endDrag = (e: PointerEvent) => {
    console.log('end');
    const node = e.currentTarget as HTMLElement;
    this.setState({ drag: undefined });
    node.releasePointerCapture(e.pointerId);
    e.preventDefault();
  };

  render() {
    return (
      <div class='window' style={{ left: `${this.state.pos[0]}px`, top: `${this.state.pos[1]}px` }}>
        <div class='titlebar' onPointerDown={this.beginDrag} onPointerUp={this.endDrag} onPointerMove={this.onDrag}>
          {this.props.title}
        </div>
        <div ref={this.ref}>content</div>
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
  }
}
class Page extends preact.Component<Page.Props, Page.State> {
  state: Page.State = { stdout: '', memBase: 0x40_1000 };

  constructor(props: Page.Props) {
    super(props);
    this.props.vm.page = this;
  }

  updateAfter(f: () => void) {
    try {
      f();
    } finally {
      this.forceUpdate();
    }
  }

  step() {
    this.updateAfter(() => this.props.vm.step());
  }

  run() {
    this.updateAfter(() => {
      for (let i = 0; i < 10000; i++) {
        if (!this.props.vm.step()) break;
      }
    });
  }

  runTo(addr: number) {
    this.props.vm.breakpoints.set(addr, { addr, temporary: true });
    this.run();
  }

  highlightMemory = (addr: number) => this.setState({ memHighlight: addr });
  showMemory = (memBase: number) => this.setState({ memBase });

  render() {
    let windows = this.props.vm.windows.map((window) => {
      return <WindowComponent key={window.id} title={window.title} />;
    });
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = this.props.vm.disassemble(this.props.vm.x86.eip);
    return (
      <>
        {windows}
        <div style={{ margin: '1ex' }}>
          <button
            onClick={() => this.run()}
          >
            run
          </button>
          &nbsp;
          <button
            onClick={() => {
              this.props.vm.step();
              this.forceUpdate();
            }}
          >
            step
          </button>
          &nbsp;
          <button
            onClick={() => this.runTo(instrs[1].addr)}
          >
            step over
          </button>
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
          <Registers
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            regs={this.props.vm.x86}
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
                  mem={this.props.vm.x86.memory()}
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
            }}
          />
          <Stack
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            labels={this.props.vm.labels}
            x86={this.props.vm.x86}
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
  const labels = await loadLabels(path);
  await wasm.default(new URL('wasm/wasm_bg.wasm', document.location.href));

  const vm = new VM(exe, labels);
  preact.render(<Page vm={vm} />, document.body);
}

main();
