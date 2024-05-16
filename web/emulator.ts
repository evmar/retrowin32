import { messageProxy, setOnMessage } from './worker/proxy';
import * as glue from './worker/glue';

/** Functions the emulator may need to call. */
export interface EmulatorHost {
  exit(code: number): void;
  onWindowChanged(): void;
  onError(msg: string): void;
  onStdOut(stdout: string): void;
}

class Window {
  constructor(readonly hwnd: number) {
    //     const stashEvent = (ev: Event) => {
    //       (ev as any).hwnd = hwnd;
    //       emulator.enqueueEvent(ev);
    //       return false;
    //     };
    //     this.canvas.onmousedown = stashEvent;
    //     this.canvas.onmouseup = stashEvent;
    //     this.canvas.oncontextmenu = (ev) => {
    //       return false;
    //     };
  }

  title: string = '';
  //   canvas: HTMLCanvasElement = document.createElement('canvas');

}

// class File implements glue.JsFile {
//   ofs = 0;

//   constructor(readonly path: string, readonly bytes: Uint8Array) {
//   }

//   info(): number {
//     return this.bytes.length;
//   }

//   seek(ofs: number): boolean {
//     this.ofs = ofs;
//     return true;
//   }

//   read(buf: Uint8Array): number {
//     const n = Math.min(buf.length, this.bytes.length - this.ofs);
//     buf.set(this.bytes.subarray(this.ofs, this.ofs + n));
//     this.ofs += n;
//     return n;
//   }
// }

/** Emulator host, providing the emulation worker=>web API. */
export class Emulator implements glue.JsHost, glue.HostLogger {
  private events: Event[] = [];
  private timer?: number;
  private glueProxy: glue.Emulator;
  windows = new Map<number, Window>();
  private decoder = new TextDecoder();

  static async initWorker(): Promise<Worker> {
    const worker = new Worker('worker-main.js');
    return new Promise((res) => {
      worker.onmessage = (e) => {
        // TODO send init here to worker
        worker.onmessage = null;
        res(worker);
      };
    });
  }

  constructor(readonly worker: Worker, private emuHost: EmulatorHost) {
    this.glueProxy = messageProxy(worker) as glue.Emulator;
    setOnMessage(worker, this);
  }

  log(level: number, msg: string) {
    // TODO: surface this in the UI.
    switch (level) {
      case 5:
        console.error(msg);
        this.emuHost.onError(msg);
        break;
      case 4:
        console.warn(msg);
        break;
      case 3:
        console.info(msg);
        break;
      case 2:
        console.log(msg);
        break;
      case 1:
        console.debug(msg);
        break;
      default:
        throw new Error(`unexpected log level ${level}`);
    }
  }

  exit(code: number): void {
    this.emuHost.exit(code);
  }

  write(buf: Uint8Array) {
    const text = this.decoder.decode(buf);
    this.emuHost.onStdOut(text);
  }

  window_create(hwnd: number) {
    const window = new Window(hwnd);
    this.windows.set(hwnd, window);
    this.emuHost.onWindowChanged();
    return window;
  }

  window_set_title(hwnd: number, title: string) {
    const window = this.windows.get(hwnd)!;
    window.title = title;
    this.emuHost.onWindowChanged();
  }

  window_set_size(w: number, h: number) {
    //     // Note: the canvas must be sized to the size of physical pixels,
    //     // or else it will be scaled up and pixels will be blurry.
    //     this.canvas.width = w * window.devicePixelRatio;
    //     this.canvas.height = h * window.devicePixelRatio;
    //     this.canvas.style.width = `${w}px`;
    //     this.canvas.style.height = `${h}px`;

    //     // The context scale seems preserved across calls to getContext, but then also
    //     // lost when the canvas is resized.  Rather than relying on this, always reset
    //     // and scale the context immediately on resize.
    //     const ctx = this.canvas.getContext('2d')!;
    //     ctx.reset();
    //     ctx.imageSmoothingEnabled = false;
    //     ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

    this.emuHost.onWindowChanged();
  }

  start() {
    this.glueProxy.run(10000);
  }
}
