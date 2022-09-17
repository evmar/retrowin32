import * as wasm from './wasm/wasm.js';

async function main() {
    await wasm.default();
    wasm.greet('miro');
}

main();
