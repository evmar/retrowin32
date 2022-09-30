import * as preact from 'preact';
import { h } from 'preact';
import { Breakpoint } from './break';
import { Code } from './code';
import { Mappings } from './mappings';
import { Memory } from './memory';
import { Registers } from './registers';
import { Stack } from './stack';
import * as wasm from './wasm/wasm';

async function loadExe(path: string): Promise<ArrayBuffer> {
  return await (await fetch(path)).arrayBuffer();
}

// Matches 'pub type JsHost' in lib.rs.
interface JsHost {
  exit(code: number): void;
  write(buf: Uint8Array): number;
}

class X86 implements JsHost {
  x86: wasm.X86;
  decoder = new TextDecoder();
  breakpoints = new Map<number, Breakpoint>();
  exitCode: number | undefined = undefined;

  constructor(exe: ArrayBuffer) {
    // new Uint8Array(exe: TypedArray) creates a uint8 view onto the buffer, no copies.
    // But then passing the buffer to Rust must copy the array into the WASM heap...
    this.x86 = wasm.load_exe(this, new Uint8Array(exe));
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
    console.log(this.decoder.decode(buf));
    return buf.length;
  }
}

namespace Page {
  export interface Props {
    x86: X86;
  }
  export interface State {
    memBase: number;
    memHighlight?: number;
  }
}
class Page extends preact.Component<Page.Props, Page.State> {
  state: Page.State = { memBase: 0x40_1000 };

  updateAfter(f: () => void) {
    try {
      f();
    } finally {
      this.forceUpdate();
    }
  }

  step() {
    this.updateAfter(() => this.props.x86.step());
  }

  run() {
    this.updateAfter(() => {
      for (;;) {
        if (!this.props.x86.step()) break;
      }
    });
  }

  runTo(addr: number) {
    this.props.x86.breakpoints.set(addr, { addr, temporary: true });
    this.run();
  }

  render() {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = this.props.x86.disassemble(this.props.x86.x86.eip);
    return (
      <main>
        <button
          onClick={() => this.run()}
        >
          run
        </button>
        <button
          onClick={() => {
            this.props.x86.step();
            this.forceUpdate();
          }}
        >
          step
        </button>
        <button
          onClick={() => this.runTo(instrs[1].addr)}
        >
          step over
        </button>
        <div style={{ display: 'flex' }}>
          <Code
            instrs={instrs}
            highlightMemory={(addr) => this.setState({ memHighlight: addr })}
            showMemory={(memBase) => this.setState({ memBase })}
            runTo={(addr: number) => this.runTo(addr)}
          />
          <div style={{ width: '12ex' }} />
          <Registers regs={this.props.x86.x86} />
        </div>
        <div style={{ display: 'flex' }}>
          <div>
            <Memory
              mem={this.props.x86.x86.memory()}
              base={this.state.memBase}
              highlight={this.state.memHighlight}
              jumpTo={(addr) => this.setState({ memBase: addr })}
            />
            <Mappings mappings={this.props.x86.mappings()} />
          </div>
          <div style={{ width: '12ex' }} />
          <Stack x86={this.props.x86.x86} />
        </div>
      </main>
    );
  }
}

async function main() {
  const path = document.location.search.substring(1);
  if (!path) throw new Error('expected ?path in URL');
  const exe = await loadExe(path);
  await wasm.default(new URL('wasm/wasm_bg.wasm', document.location.href));

  const x86 = new X86(exe);
  console.log(x86.mappings());

  preact.render(<Page x86={x86} />, document.body);
}

main();
