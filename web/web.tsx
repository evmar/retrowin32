import * as preact from 'preact';
import { h } from 'preact';
import { RegistersView } from './registers.js';
import { hex } from './util.js';
import * as wasm from './wasm/wasm.js';

async function loadExe(): Promise<ArrayBuffer> {
  return await (await fetch('tiny.exe')).arrayBuffer();
}

namespace Memory {
  export interface Props {
    base: number;
    buf: Uint8Array;
  }
}
class Memory extends preact.Component<Memory.Props> {
  render() {
    let text = '';
    for (let row = 0; row < 0x100; row += 0x10) {
      text += hex(this.props.base + row, 8) + ' ';
      for (let col = 0; col < 0x10; col++) {
        text += ' ' + hex(this.props.buf[row + col]);
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
    const base = 0x0040_1000;
    const buf = this.props.x86.mem(base, 0x1000);
    return (
      <main>
        <Memory base={base} buf={buf} />
        <RegistersView regs={JSON.parse(this.props.x86.regs_json())} />
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
