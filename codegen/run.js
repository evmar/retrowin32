const fs = require('fs');

function hex32(n) {
    return n.toString(16).padStart(8, '0');
}

function dump_state(wasm) {
    const regs = ['EAX', 'EBX'];
    //console.log(`EAX ${}`)
}

async function main() {
    const wasmBuffer = fs.readFileSync('t.wasm');

    const memory = new WebAssembly.Memory({ initial: 64 }); // 4mb
    const importObject = {
        host: {
            mem: memory,
            icall() {
                console.log('icall');
            }
        },
    };
    const mod = await WebAssembly.instantiate(wasmBuffer, importObject);
    console.log('exports are', mod.instance.exports);

    mod.instance.exports.run();
}
main();