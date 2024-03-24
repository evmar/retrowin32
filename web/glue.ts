/**
 * Interfaces matching the types defined in glue/*.rs.
 * In principle these could be codegenned by wasm-bindgen but they aren't for whatever reason.
 */

import { SurfaceOptions } from './glue/pkg/glue';
export { SurfaceOptions };

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

  get_event(): Event | undefined;

  open(path: string): JsFile;
  write(buf: Uint8Array): number;

  create_window(hwnd: number): JsWindow;
}
