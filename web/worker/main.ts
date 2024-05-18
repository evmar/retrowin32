import * as glue from '../glue/pkg/glue';
import { messageProxy, setOnMessage } from './proxy';

async function fetchBytes(path: string): Promise<Uint8Array> {
  const resp = await fetch(path);
  if (!resp.ok) throw new Error(`failed to load ${path}`);
  return new Uint8Array(await resp.arrayBuffer());
}

/** A set of (pre)loaded files; a temporary hack until the emulator can load files itself. */
export type FileSet = Map<string, Uint8Array>;

export async function fetchFileSet(files: string[], dir: string = ''): Promise<FileSet> {
  const fileset: FileSet = new Map();
  for (const file of files) {
    const path = dir + file;
    fileset.set(file, await fetchBytes(path));
  }
  return fileset;
}

export async function main() {
  const params = {
    exe: 'exe/rust-out/gdi.exe',
    files: [],
    dir: '',
  };

  const fileset = await fetchFileSet([params.exe, ...params.files], params.dir);

  await glue.default('wasm.wasm');
  self.onmessage = (e) => {
    console.error('BUG: dropped message', e.data);
  };
  self.postMessage('ready');
  const workerHost = messageProxy(self) as glue.JsHost;
  const emu = glue.new_emulator(workerHost, params.exe, params.exe, fileset.get(params.exe)!);
  setOnMessage(self, emu);
}
