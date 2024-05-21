import * as glue from '../glue/pkg/glue';
import { FileHost } from './files';
import { MethodChannel } from './proxy';

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

  const fileHost = await FileHost.fetch([params.exe, ...params.files], params.dir);
  await glue.default('wasm.wasm');

  const channel = new MessageChannel();

  self.postMessage('ready');
  const workerHost = new MethodChannel<glue.JsHost>(self);
  const emu = glue.new_emulator(workerHost.asProxy(), fileHost, params.exe, params.exe, channel.port1);
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
  workerHost.setLocal(emu);
}
