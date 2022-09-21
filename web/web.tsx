import * as preact from 'preact';
import { h } from 'preact';
import { Code } from './code';
import { Memory } from './memory';
import { Registers } from './registers';
import { Stack } from './stack';
import { hex } from './util';
import * as wasm from './wasm/wasm';

async function loadExe(path: string): Promise<ArrayBuffer> {
  return await (await fetch(path)).arrayBuffer();
}

namespace Page {
  export interface Props {
    x86: wasm.X86;
  }
  export interface State {
    memBase: number;
    memHighlight?: number;
  }
}
class Page extends preact.Component<Page.Props, Page.State> {
  state: Page.State = { memBase: 0x40_1000 };

  render() {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = JSON.parse(this.props.x86.disassemble_json(this.props.x86.eip)) as wasm.Instruction[];
    return (
      <main>
        <button
          onClick={() => {
            this.props.x86.step();
            this.forceUpdate();
          }}
        >
          step
        </button>
        <div style={{ display: 'flex' }}>
          <Code
            instrs={instrs}
            highlightMemory={(addr) => this.setState({ memHighlight: addr })}
            showMemory={(memBase) => this.setState({ memBase })}
            runTo={(addr: number) => {
              for (let i = 0; i < 0x20; i++) {
                this.props.x86.step();
                if (this.props.x86.eip === addr) break;
              }
              this.forceUpdate();
            }}
          />
          <div style={{ width: '12ex' }} />
          <Registers regs={this.props.x86} />
        </div>
        <div style={{ display: 'flex' }}>
          <Memory mem={this.props.x86.memory()} base={this.state.memBase} highlight={this.state.memHighlight} />
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

  preact.render(<Page x86={x86} />, document.body);
}

main();
