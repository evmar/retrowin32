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

export interface Params {
  /** URL directory that all other paths are resolved relative to. */
  dir?: string;
  /** Executable to run. */
  exe: string;
  /** Other data files to load.  TODO: we should fetch these dynamically instead. */
  files: string[];
  /** If true, relocate the exe on load. */
  relocate?: boolean;
}

export async function main() {
  // It appears if we register onmessage before any 'await' we will receive any initial
  // messages sent from the host.  StackOverflow etc claim this but I can't figure out where
  // the spec guarantees this...
  const params = await new Promise<Params>((res) => {
    self.onmessage = (e) => {
      self.onmessage = (e) => {
        // catch any onmessage sent before ready
        console.error('BUG: dropped message', e.data);
      };
      res(e.data);
    };
  });

  const files: Array<[string, Uint8Array]> = await Promise.all([params.exe, ...params.files].map(async f => {
    return [f, await fetchBytes(params.dir + f)];
  }));
  await glue.default('wasm.wasm');

  const channel = new MessageChannel();

  self.postMessage('ready');
  const workerHost = messageProxy(self) as glue.JsHost;
  const emu = glue.new_emulator(workerHost, params.exe, params.exe, files, channel.port1);
  emu.start = () => {
    channel.port2.onmessage = () => {
      const state = emu.run(100_000);
      // XXX adjust dynamically
      // XXX check return code
      if (state === glue.CPUState.Running) {
        channel.port1.postMessage(null);
      }
    };
    channel.port1.postMessage(null);
  };
  setOnMessage(self, emu);
}
