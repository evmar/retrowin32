const fs = require('fs');

require('./run');
globalThis.run(fs.readFileSync('t.wasm'));
