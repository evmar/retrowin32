function hex32(n) {
  return n.toString(16).padStart(8, '0');
}

function dump_state(wasm) {
  for (const row of [['EAX', 'ECX', 'EDX', 'EBX'], ['ESP', 'EBP', 'ESI', 'EDI']]) {
    console.log(
      row.map(r => {
        const value = wasm.exports[r].value;
        return `${r} ${hex32(value)}`;
      }).join(' '),
    );
  }
}

async function run(wasmBuffer) {
  const memory = new WebAssembly.Memory({ initial: 128 }); // 8mb
  const view = new DataView(memory.buffer);
  const importObject = {
    host: {
      mem: memory,
      icall(fn) {
        console.log(`icall ${hex32(fn)}`);
        dump_state(wasm);
        const esp = wasm.exports.ESP.value;
        for (const ofs of [0, 4, 8, 0xc, 0x10]) {
          const addr = esp + ofs;
          console.log(`${hex32(addr)} ${hex32(view.getUint32(addr, true))}`);
        }
      },
    },
  };

  const mod = wasmBuffer instanceof Response
    ? await WebAssembly.instantiateStreaming(wasmBuffer, importObject)
    : await WebAssembly.instantiate(wasmBuffer, importObject);
  const wasm = mod.instance;
  wasm.exports.ESP.value = 0x0010_0000;
  wasm.exports.run();
  return [memory, wasm];
}

globalThis.run = run;
