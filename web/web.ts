import * as wasm from './wasm/wasm.js';

async function loadExe(): Promise<ArrayBuffer> {
    return await (await fetch('tiny.exe')).arrayBuffer();
}

async function main() {
    const exe = await loadExe();
    await wasm.default();
    // ick copies
    const x86 = wasm.load_exe(new Uint8Array(exe));

    const pre = document.createElement('pre');
    const base = 0x0040_1000;
    const buf = x86.mem(base, 0x1000);

    let text = '';
    for (let i = 0; i < 0x1000; i++) {
        if (i % 16 === 0) {
            text += (base + i).toString(16) + ' ';
        }
        text += ' ' + buf[i].toString(16);
        if (i % 16 === 15) {
            text += '\n';
        }
    }
    pre.innerText = text;
    document.body.appendChild(pre);
}

main();
