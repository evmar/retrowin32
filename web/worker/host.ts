import * as glue from '../glue/pkg/glue';

// export type WorkerHost = glue.Host;

// export class GlueHostX implements glue.Host, glue.HostLogger {
//   private emu: glue.Emulator;
//   private decoder = new TextDecoder();

//   constructor(private workerHost: WorkerHost, cmdline: string, buf: Uint8Array) {
//     let glueHost: glue.Host = {
//       ...workerHost,

//       ensure_timer: undefined!,
//       get_event: undefined!,

//       open: undefined!,
//       write: undefined!,
//     };
//     this.emu = glue.new_emulator(glueHost, cmdline, buf);
//   }

//   log(level: number, msg: string) {
//     // TODO: surface this in the UI.
//     switch (level) {
//       case 5:
//         console.error(msg);
//         this.workerHost.onError(msg);
//         break;
//       case 4:
//         console.warn(msg);
//         break;
//       case 3:
//         console.info(msg);
//         break;
//       case 2:
//         console.log(msg);
//         break;
//       case 1:
//         console.debug(msg);
//         break;
//       default:
//         throw new Error(`unexpected log level ${level}`);
//     }
//   }

//   exit(code: number) {
//     this.workerHost.exit(code);
//   }

//   start(): void {
//     todo!();
//     //    this.emu.run(10000);
//   }

//   // enqueueEvent(event: Event) {
//   //   this.events.push(event);
//   //   this.start();
//   // }

//   ensure_timer(when: number): void {
//     // if (this.timer) {
//     //   clearTimeout(this.timer);
//     // }
//     // const now = performance.now();
//     // console.log('timer for', when - now);
//     // const id = setTimeout(() => {
//     //   if (this.timer !== id) {
//     //     return;
//     //   }
//     //   this.timer = undefined;
//     //   this.start();
//     // }, when - now);
//     // this.timer = id;
//   }

//   get_event(): Event | undefined {
//     // return this.events.shift();
//   }

//   open(path: string): glue.JsFile {
//     // // TODO: async file loading.
//     // let bytes = this.files.get(path);
//     // if (!bytes) {
//     //   console.error(`unknown file ${path}, returning empty file`);
//     //   bytes = new Uint8Array();
//     // }
//     // return new File(path, bytes);
//   }

//   write(buf: Uint8Array) {
//     const text = this.decoder.decode(buf);
//     this.workerHost.onStdOut(text);
//   }

//   screen() {
//     // XXX how to tie surface and window together?
//     // The DirectDraw calls SetCooperativeLevel() on the hwnd, and then CreateSurface with primary,
//     // but how to plumb that info across JS boundary?
//     return this.windows[this.windows.length - 1].canvas.getContext('2d')!;
//   }
// }
