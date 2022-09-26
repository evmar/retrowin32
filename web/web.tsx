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

namespace Page {
  export interface Props {
    x86: wasm.X86;
  }
  export interface State {
    breakpoints: Map<number, Breakpoint>;
    memBase: number;
    memHighlight?: number;
  }
}
class Page extends preact.Component<Page.Props, Page.State> {
  state: Page.State = { memBase: 0x40_1000, breakpoints: new Map() };

  updateAfter(f: () => void) {
    try {
      f();
    } finally {
      this.forceUpdate();
    }
  }

  step() {
    this.updateAfter(() => this.step());
  }

  run() {
    this.updateAfter(() => {
      for (;;) {
        this.props.x86.step();
        const bp = this.state.breakpoints.get(this.props.x86.eip);
        if (bp) {
          if (bp.temporary) {
            this.state.breakpoints.delete(bp.addr);
          }
          break;
        }
      }
    });
  }

  runTo(addr: number) {
    this.state.breakpoints.set(addr, { addr, temporary: true });
    this.run();
  }

  render() {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = JSON.parse(this.props.x86.disassemble_json(this.props.x86.eip)) as wasm.Instruction[];
    const mappings = JSON.parse(this.props.x86.mappings_json()) as wasm.Mapping[];
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
          <Registers regs={this.props.x86} />
        </div>
        <div style={{ display: 'flex' }}>
          <div>
            <Memory
              mem={this.props.x86.memory()}
              base={this.state.memBase}
              highlight={this.state.memHighlight}
              jumpTo={(addr) => this.setState({ memBase: addr })}
            />
            <Mappings mappings={mappings} />
          </div>
          <div style={{ width: '12ex' }} />
          <Stack x86={this.props.x86} />
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
  wasm.init_logging();
  // ick copies
  const x86 = wasm.load_exe(new Uint8Array(exe));

  const mappings: wasm.Mapping[] = JSON.parse(x86.mappings_json()) as wasm.Mapping[];
  console.log(mappings);

  preact.render(<Page x86={x86} />, document.body);
}

main();
