async function main() {
    const wasmBuffer = await fetch('t.wasm');

    const memory = new WebAssembly.Memory({ initial: 64 }); // 4mb
    const importObject = {
        host: {
            mem: memory,
            icall() {
                console.log('icall');
            }
        },
    };
    const mod = await WebAssembly.instantiateStreaming(wasmBuffer, importObject);
    mod.instance.exports.run();
}
main();