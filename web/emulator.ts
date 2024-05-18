import * as glue from './worker/glue';
import { messageProxy, setOnMessage } from './worker/proxy';
import type { Params } from './worker/main';

export function parseURL(): Params | undefined {
  const query = new URLSearchParams(document.location.search);
  const exe = query.get('exe');
  if (!exe) return undefined;
  const dir = query.get('dir') || undefined;
  const files = query.getAll('file');
  const relocate = query.has('relocate');
  const params: Params = { dir, exe, files, relocate };
  return params;
}

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
  canvas: HTMLCanvasElement = document.createElement('canvas');
}

/** Emulator host, providing the emulation worker=>web API. */
export class Emulator implements glue.JsHost, glue.HostLogger {
  private events: Event[] = [];
  private timer?: number;
  private glueProxy: glue.Emulator;
  windows = new Map<number, Window>();
  private decoder = new TextDecoder();

  static async initWorker(): Promise<Worker> {
    const params = parseURL();
    const worker = new Worker('worker-main.js');
    worker.postMessage(params);
    return new Promise((res) => {
      worker.onmessage = (e) => {
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
    const win = new Window(hwnd);
    this.windows.set(hwnd, win);
    this.emuHost.onWindowChanged();
    return win;
  }

  window_set_title(hwnd: number, title: string) {
    const win = this.windows.get(hwnd)!;
    win.title = title;
    this.emuHost.onWindowChanged();
  }

  window_set_size(hwnd: number, w: number, h: number) {
    const win = this.windows.get(hwnd)!;

    // Note: the canvas must be sized to the size of physical pixels,
    // or else it will be scaled up and pixels will be blurry.
    const devicePixelRatio = window.devicePixelRatio;
    win.canvas.width = w * devicePixelRatio;
    win.canvas.height = h * devicePixelRatio;
    win.canvas.style.width = `${w}px`;
    win.canvas.style.height = `${h}px`;

    // The context scale seems preserved across calls to getContext, but then also
    // lost when the canvas is resized.  Rather than relying on this, always reset
    // and scale the context immediately on resize.
    const ctx = win.canvas.getContext('2d')!;
    ctx.reset();
    ctx.imageSmoothingEnabled = false;
    ctx.scale(devicePixelRatio, devicePixelRatio);

    this.emuHost.onWindowChanged();
  }

  window_show(hwnd: number, bitmap: ImageBitmap) {
    const win = this.windows.get(hwnd)!;
    const ctx = win.canvas.getContext('2d')!;
    ctx.drawImage(bitmap, 0, 0);
  }

  start() {
    this.glueProxy.start();
  }
}
