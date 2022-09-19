import * as preact from 'preact';
import { h } from 'preact';
import { Code } from './code';
import { Registers } from './registers';
import { Stack } from './stack';
import { hex } from './util';
import * as wasm from './wasm/wasm';

async function loadExe(): Promise<ArrayBuffer> {
  return await (await fetch('unpacked.exe')).arrayBuffer();
}

namespace Memory {
  export interface Props {
    base: number;
    mem: DataView;
  }
}
class Memory extends preact.Component<Memory.Props> {
  render() {
    let text = '';
    for (let row = 0; row < 0x100; row += 0x10) {
      text += hex(this.props.base + row, 8) + ' ';
      for (let col = 0; col < 0x10; col++) {
        text += ' ' + hex(this.props.mem.getUint8(this.props.base + row + col));
      }
      text += '\n';
    }
    return <pre>{text}</pre>;
  }
}

namespace Page {
  export interface Props {
    x86: wasm.X86;
  }
}
class Page extends preact.Component<Page.Props> {
  render() {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const base = 0x0040_1000;
    const instrs = JSON.parse(this.props.x86.disassemble_json(this.props.x86.eip)) as wasm.Instruction[];
    return (
      <main>
        <div style={{ display: 'flex' }}>
          <Code instrs={instrs} />
          <div style={{ width: '12ex' }} />
          <Registers regs={this.props.x86} />
        </div>
        <div style={{ display: 'flex' }}>
          <Memory base={base} mem={this.props.x86.memory()} />
          <div style={{ width: '12ex' }} />
          <Stack x86={this.props.x86} />
        </div>
        <button
          onClick={() => {
            this.props.x86.step();
            this.forceUpdate();
          }}
        >
          step
        </button>
      </main>
    );
  }
}

async function main() {
  const exe = await loadExe();
  await wasm.default(new URL('wasm/wasm_bg.wasm', document.location.href));
  // ick copies
  const x86 = wasm.load_exe(new Uint8Array(exe));

  preact.render(<Page x86={x86} />, document.body);
}

main();
