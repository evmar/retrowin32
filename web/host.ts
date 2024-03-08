import * as emulator from './emulator';
import { EmulatorHost } from './emulator';
import * as glue from './glue';
import { Debugger } from './web';

async function fetchBytes(path: string): Promise<Uint8Array> {
  const resp = await fetch(path);
  if (!resp.ok) throw new Error(`failed to load ${path}`);
  return new Uint8Array(await resp.arrayBuffer());
}

class Surface implements glue.JsSurface {
  /** Where to render output on show() */
  screen?: CanvasRenderingContext2D;

  /** Backing pixels. */
  canvas: HTMLCanvasElement;
  ctx: CanvasRenderingContext2D;

  constructor(width: number, height: number, primary: boolean) {
    this.canvas = document.createElement('canvas');
    this.canvas.width = width;
    this.canvas.height = height;
    this.ctx = this.canvas.getContext('2d')!;
    this.ctx.fillStyle = 'black';
    this.ctx.fillRect(0, 0, 640, 480);
    this.ctx.fill();
  }

  write_pixels(pixels: Uint8Array): void {
    const data = new ImageData(this.canvas.width, this.canvas.height);
    // XXX Ew copy.  Docs suggest the ImageData ctor accepts pixel data as a param
    // but I couldn't see it working.
    data.data.set(pixels);
    this.ctx.putImageData(data, 0, 0);
  }

  show() {
    this.screen!.drawImage(this.canvas, 0, 0);
  }

  bit_blt(dx: number, dy: number, other: glue.JsSurface, sx: number, sy: number, w: number, h: number): void {
    this.ctx.drawImage((other as unknown as Surface).canvas, sx, sy, w, h, dx, dy, w, h);
  }
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
    this.canvas.oncontextmenu = (ev) => {
      return false;
    };
  }

  title: string = '';
  canvas: HTMLCanvasElement = document.createElement('canvas');

  set_size(w: number, h: number) {
    this.canvas.width = w;
    this.canvas.height = h;
    this.jsHost.emuHost.onWindowChanged();
  }
}

class File implements glue.JsFile {
  ofs = 0;

  constructor(readonly path: string, readonly bytes: Uint8Array) {
  }
  seek(ofs: number): boolean {
    this.ofs = ofs;
    return true;
  }
  read(buf: Uint8Array): number {
    const n = Math.min(buf.length, this.bytes.length - this.ofs);
    buf.set(this.bytes.subarray(this.ofs, this.ofs + n));
    this.ofs += n;
    return n;
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

  stdout = '';
  decoder = new TextDecoder();

  constructor(public emuHost: EmulatorHost, readonly files: FileSet) {}

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
        throw new Error(`unexpected log level #{level}`);
    }
  }

  exit(code: number) {
    this.emuHost.exit(code);
  }

  abstract start(): void;

  enqueueEvent(event: Event) {
    this.events.push(event);
    this.start();
  }

  get_event(): Event | undefined {
    return this.events.shift();
  }

  open(path: string): glue.JsFile {
    // TODO: async file loading.
    let bytes = this.files.get(path);
    if (!bytes) {
      console.error(`unknown file ${path}, returning empty file`);
      bytes = new Uint8Array();
    }
    return new File(path, bytes);
  }
  write(buf: Uint8Array): number {
    const text = this.decoder.decode(buf);
    this.stdout += text;
    this.emuHost.onStdOut(this.stdout);
    return buf.length;
  }

  windows: Window[] = [];
  create_window(hwnd: number): glue.JsWindow {
    let window = new Window(this, hwnd);
    this.windows.push(window);
    this.emuHost.onWindowChanged();
    return window;
  }

  create_surface(opts: glue.SurfaceOptions): glue.JsSurface {
    const { width, height, primary } = opts;
    opts.free();
    const surface = new Surface(width, height, primary);
    // XXX how to tie surface and window together?
    // The DirectDraw calls SetCooperativeLevel() on the hwnd, and then CreateSurface with primary,
    // but how to plumb that info across JS boundary?
    console.warn('hack: attached surface to window');
    const win = this.windows[this.windows.length - 1];
    surface.screen = win.canvas.getContext('2d')!;
    return surface;
  }
}
