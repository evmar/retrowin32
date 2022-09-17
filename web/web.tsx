import * as wasm from './wasm/wasm.js';
import * as preact from 'preact';
import {h} from 'preact';

async function loadExe(): Promise<ArrayBuffer> {
    return await (await fetch('tiny.exe')).arrayBuffer();
}

function hex(i: number, digits = 2): string {
    return i.toString(16).padStart(digits, '0');
}

namespace Memory {
    export interface Props {
        base: number,
        buf: Uint8Array,
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


async function main() {
    const exe = await loadExe();
    await wasm.default(new URL('wasm/wasm_bg.wasm', document.location.href));
    // ick copies
    const x86 = wasm.load_exe(new Uint8Array(exe));

    const pre = document.createElement('pre');
    const base = 0x0040_1000;
    const buf = x86.mem(base, 0x1000);

    preact.render(<Memory base={base} buf={buf}/>, document.body);
}

main();
