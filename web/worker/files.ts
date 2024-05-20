import * as glue from '../glue/pkg/glue';

async function fetchBytes(path: string): Promise<Uint8Array> {
  const resp = await fetch(path);
  if (!resp.ok) throw new Error(`failed to load ${path}`);
  return new Uint8Array(await resp.arrayBuffer());
}

/** A set of (pre)loaded files; a temporary hack until the emulator can load files itself. */
type FileSet = Map<string, Uint8Array>;

export class FileHost implements glue.FileHost {
  constructor(readonly files: FileSet) {}

  static async fetch(files: string[], dir: string = ''): Promise<FileHost> {
    const fileset = new Map(
      await Promise.all(files.map(async f => {
        return [f, await fetchBytes(dir + f)] as const;
      })),
    );
    return new FileHost(fileset);
  }

  open(path: string): glue.JsFile {
    let bytes = this.files.get(path);
    if (!bytes) {
      console.error(`unknown file ${path}, returning empty file`);
      bytes = new Uint8Array();
    }
    return new File(path, bytes);
  }
}

class File implements glue.JsFile {
  ofs = 0;

  constructor(readonly path: string, readonly bytes: Uint8Array) {
  }

  info(): number {
    return this.bytes.length;
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
