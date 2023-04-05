import { Breakpoint } from './break';
import * as wasm from './glue/pkg';
import { Labels, Loader as LabelsLoader } from './labels';
import { hex } from './util';

export interface Host {
  emulator: Emulator;
  showTab(name: string): void;
}

export class Emulator {
  emu: wasm.Emulator;
  breakpoints = new Map<number, Breakpoint>();
  imports: string[] = [];
  labels: Labels;
  exitCode: number | undefined = undefined;

  constructor(readonly host: Host, readonly storageKey: string, bytes: Uint8Array, labelsLoader: LabelsLoader) {
    host.emulator = this;
    this.emu = wasm.new_emulator(host);
    this.emu.load_exe(bytes);

    const importsJSON = JSON.parse(this.emu.labels());
    for (const [jsAddr, jsName] of Object.entries(importsJSON)) {
      const addr = parseInt(jsAddr);
      const name = jsName as string;
      this.imports.push(`${hex(addr, 8)}: ${name}`);
      labelsLoader.add(addr, name);
    }
    this.labels = new Labels(labelsLoader);

    // // Hack: twiddle msvcrt output mode to use console.
    // this.x86.poke(0x004095a4, 1);

    this.loadBreakpoints();
  }

  private loadBreakpoints() {
    const json = window.localStorage.getItem(this.storageKey);
    if (!json) return;
    const bps = JSON.parse(json) as Breakpoint[];
    for (const bp of bps) {
      this.addBreak(bp, /* save= */ false);
    }
  }

  private saveBreakpoints() {
    window.localStorage.setItem(this.storageKey, JSON.stringify(Array.from(this.breakpoints.values())));
  }

  addBreak(bp: Breakpoint, save = true) {
    this.breakpoints.set(bp.addr, bp);
    if (!bp.disabled) {
      this.emu.breakpoint_add(bp.addr);
    }
    if (save) this.saveBreakpoints();
  }

  addBreakByName(name: string): boolean {
    for (const [addr, label] of this.labels.byAddr) {
      if (label === name) {
        this.addBreak({ addr });
        return true;
      }
    }
    if (name.match(/^[0-9a-fA-F]+$/)) {
      const addr = parseInt(name, 16);
      this.addBreak({ addr });
      return true;
    }
    return false;
  }

  delBreak(addr: number) {
    this.breakpoints.delete(addr);
    this.emu.breakpoint_clear(addr);
    this.saveBreakpoints();
  }

  toggleBreak(addr: number) {
    const bp = this.breakpoints.get(addr)!;
    bp.disabled = !bp.disabled;
    if (bp.disabled) {
      this.emu.breakpoint_clear(addr);
    } else {
      this.emu.breakpoint_add(addr);
    }
    this.saveBreakpoints();
  }

  /** Check if the current address is a break/exit point, returning true if so. */
  checkBreak(): boolean {
    if (this.exitCode !== undefined) return true;
    const ip = this.emu.eip;
    const bp = this.breakpoints.get(ip);
    if (bp && !bp.disabled) {
      if (bp.oneShot) {
        this.delBreak(bp.addr);
      } else {
        this.host.showTab('breakpoints');
      }
      return true;
    }
    return false;
  }

  /** Returns true if we should keep running after this (no breakpoint). */
  stepPastBreak(): boolean {
    const ip = this.emu.eip;
    const bp = this.breakpoints.get(ip);
    if (bp && !bp.disabled) {
      this.emu.breakpoint_clear(ip);
      const ret = this.step();
      this.emu.breakpoint_add(ip);
      return ret;
    } else {
      return this.step();
    }
  }

  /** Returns true if we should keep running after this (no breakpoint). */
  step(): boolean {
    this.emu.single_step();
    return !this.checkBreak();
  }

  /** Number of instructions to execute per stepMany, adjusted dynamically. */
  stepSize = 5000;
  /** Moving average of instructions executed per millisecond. */
  instrPerMs = 0;

  /** Runs a batch of instructions.  Returns false if we should stop. */
  stepMany(): boolean {
    const start = performance.now();
    const steps = this.emu.execute_many(this.stepSize);
    const end = performance.now();

    if (this.checkBreak()) {
      return false;
    }

    const delta = end - start;
    const instrPerMs = steps / delta;
    const alpha = 0.5; // smoothing factor
    this.instrPerMs = alpha * (instrPerMs) + (alpha - 1) * this.instrPerMs;

    if (delta < 10) {
      this.stepSize *= 2;
      console.log('adjusted step rate', this.stepSize);
    }

    return true;
  }

  mappings(): wasm.Mapping[] {
    return JSON.parse(this.emu.mappings_json()) as wasm.Mapping[];
  }
  disassemble(addr: number): wasm.Instruction[] {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    return JSON.parse(this.emu.disassemble_json(addr)) as wasm.Instruction[];
  }
}
