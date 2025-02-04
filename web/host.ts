/**
 * The hosting side of the emulator, with implementations of JsWindow, JsFile, etc.
 */

import { EmulatorHost } from './emulator';
import * as glue from './glue/pkg/glue';

async function fetchBytes(path: string): Promise<Uint8Array> {
  const resp = await fetch(path);
  if (!resp.ok) throw new Error(`failed to load ${path}`);
  return new Uint8Array(await resp.arrayBuffer());
}

class Window implements glue.JsWindow {
  constructor(readonly jsHost: JsHost, readonly hwnd: number) {
    const stashEvent = (ev: Event) => {
      (ev as any).hwnd = hwnd;
      jsHost.enqueueEvent(ev);
      return false;
    };
    this.canvas.onmousedown = stashEvent;
    this.canvas.onmouseup = stashEvent;
    this.canvas.onmousemove = stashEvent;
    this.canvas.oncontextmenu = (ev) => {
      return false;
    };
  }

  title: string = '';
  canvas: HTMLCanvasElement = document.createElement('canvas');
  size: [number, number] = [0, 0];
  is_fullscreen = false;

  private reset_canvas() {
    const [w, h] = this.size;
    let scale = 1;
    if (this.is_fullscreen && w < 640) scale *= 2;

    // Note: the canvas must be sized to the size of physical pixels,
    // or else it will be scaled up with smoothing and pixels will be blurry.
    this.canvas.width = w * window.devicePixelRatio;
    this.canvas.height = h * window.devicePixelRatio;
    this.canvas.style.width = `${w * scale}px`;
    this.canvas.style.height = `${h * scale}px`;

    // The context scale seems preserved across calls to getContext, but then also
    // lost when the canvas is resized.  Rather than relying on this, always reset
    // and scale the context immediately on resize.
    const ctx = this.canvas.getContext('2d')!;
    ctx.reset();
    ctx.imageSmoothingEnabled = false;
    ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    this.jsHost.emuHost.onWindowChanged();
  }

  set_size(w: number, h: number) {
    this.size = [w, h];
    this.reset_canvas();
  }

  fullscreen() {
    this.is_fullscreen = true;
    this.reset_canvas();
  }
}

class File implements glue.JsFile {
  ofs = 0;

  constructor(readonly path: string, readonly bytes: Uint8Array) {
  }

  info(): number {
    return this.bytes.length;
  }

  seek(from: number, ofs: number): number {
    switch (from) {
      case 0: // start
        break;
      case 1: // end
        ofs = this.bytes.length + ofs;
        break;
      case 2: // cur
        ofs = this.ofs + ofs;
        break;
    }
    if (ofs < 0 || ofs > this.bytes.length) {
      throw new Error(`seek out of bounds: ${from}:${ofs}`);
    }
    this.ofs = ofs;
    return ofs;
  }

  read(buf: Uint8Array): number {
    const n = Math.min(buf.length, this.bytes.length - this.ofs);
    buf.set(this.bytes.subarray(this.ofs, this.ofs + n));
    this.ofs += n;
    return n;
  }

  write(buf: Uint8Array): number {
    console.warn('ignoring write');
    return buf.length;
  }
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

/** Emulator host, providing the emulation=>web API.  Extended by Emulator class. */
export abstract class JsHost implements glue.JsHost, glue.JsLogger {
  private events: Event[] = [];
  private timer?: number;

  decoder = new TextDecoder();

  constructor(readonly emuHost: EmulatorHost, readonly files: FileSet) { }

  log(level: number, msg: string) {
    // TODO: surface this in the UI.
    switch (level) {
      case 1:
        console.error(msg);
        this.emuHost.onError(msg);
        break;
      case 2:
        console.warn(msg);
        break;
      case 3:
        console.info(msg);
        break;
      case 4:
      case 5: // "trace"
        console.debug(msg);
        break;
      default:
        throw new Error(`unexpected log level #{level}`);
    }
  }

  abstract start(): void;

  enqueueEvent(event: Event) {
    this.events.push(event);
    this.start();
  }

  ensure_timer(when: number): void {
    if (this.timer) {
      clearTimeout(this.timer);
    }
    const now = performance.now();
    console.log('timer for', when - now);
    const id = setTimeout(() => {
      if (this.timer !== id) {
        return;
      }
      this.timer = undefined;
      this.start();
    }, when - now);
    this.timer = id;
  }

  get_event(): Event | undefined {
    return this.events.shift();
  }

  open(path: string, options: glue.FileOptions): glue.JsFile | null {
    if (options.create) {
      return new File(path, new Uint8Array());
    }
    // TODO: async file loading.
    let bytes = this.files.get(path);
    if (!bytes) {
      return null;
    }
    return new File(path, bytes);
  }

  stdout(buf: Uint8Array) {
    const text = this.decoder.decode(buf);
    this.emuHost.onStdOut(text);
  }

  windows: Window[] = [];
  create_window(hwnd: number): glue.JsWindow {
    let window = new Window(this, hwnd);
    this.windows.push(window);
    this.emuHost.onWindowChanged();
    return window;
  }

  screen() {
    // XXX how to tie surface and window together?
    // The DirectDraw calls SetCooperativeLevel() on the hwnd, and then CreateSurface with primary,
    // but how to plumb that info across JS boundary?
    return this.windows[this.windows.length - 1].canvas.getContext('2d')!;
  }

  audio(buf: Int16Array) {
    console.warn('TODO: audio');
  }
}
