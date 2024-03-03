/**
 * Interfaces matching the types defined in glue/*.rs.
 * In principle these could be codegenned by wasm-bindgen but they aren't for whatever reason.
 */

import { SurfaceOptions } from './glue/pkg';
export { SurfaceOptions };

// Matches 'pub type JsSurface' in glue/host.rs.
export interface JsSurface {
  write_pixels(pixels: Uint8Array): void;
  show(): void;
  bit_blt(dx: number, dy: number, other: JsSurface, sx: number, sy: number, w: number, h: number): void;
}

// Matches 'pub type JsWindow' in glue/host.rs.
export interface JsWindow {
  title: string;
  set_size(width: number, height: number): void;
}

// Matches 'pub type JsFile' in glue/host.rs.
export interface JsFile {
  seek(ofs: number): boolean;
  read(buf: Uint8Array): number;
}

// Matches 'pub type JsLogger' in glue/log.rs.
export interface JsLogger {
  log(level: number, msg: string): void;
}

// Matches 'pub type JsHost' in glue/host.rs.
export interface JsHost {
  exit(code: number): void;

  open(path: string): JsFile;
  write(buf: Uint8Array): number;

  create_window(hwnd: number): JsWindow;
  create_surface(opts: SurfaceOptions): JsSurface;
}
