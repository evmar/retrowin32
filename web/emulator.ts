import { Breakpoints } from './debugger/break';
import * as wasm from './glue/pkg/glue';
import { FileSet, JsHost } from './host';

/** Functions the emulator may need to call. */
export interface EmulatorHost {
  exit(code: number): void;
  onWindowChanged(): void;
  showTab(name: string): void;
  onError(msg: string): void;
  onStdOut(stdout: string): void;
  onStopped(): void;
}

/** Wraps wasm.Emulator, able to run in a loop while still yielding to browser events. */
export class Emulator extends JsHost {
  readonly emu: wasm.Emulator;
  /** True when the emulator is actively trying to loop and executing instructions, false when stopped or blocked. */
  running = false;
  breakpoints: Breakpoints;
  channel = new MessageChannel();

  constructor(
    host: EmulatorHost,
    files: FileSet,
    readonly exePath: string,
    cmdLine: string,
    bytes: Uint8Array,
    relocate: boolean,
  ) {
    super(host, files);
    this.emu = wasm.new_emulator(this, cmdLine);
    this.emu.load_exe(exePath, bytes, relocate);
    this.breakpoints = new Breakpoints(exePath);

    this.channel.port2.onmessage = () => this.loop();
  }

  step() {
    this.emu.unblock(); // Attempt to resume any blocked threads.
    this.emu.run(1);
  }

  /** Number of instructions to execute per stepMany, adjusted dynamically. */
  stepSize = 5000;
  /** Moving average of instructions executed per millisecond. */
  instrPerMs = 0;

  private runBatch() {
    const startTime = performance.now();
    const startSteps = this.emu.instr_count;
    const cpuState = this.emu.run(this.stepSize) as wasm.Status;
    const endTime = performance.now();
    const endSteps = this.emu.instr_count;

    const steps = endSteps - startSteps;
    if (steps > 1000) { // only update if we ran enough instructions to get a good measurement
      const deltaTime = endTime - startTime;

      const instrPerMs = steps / deltaTime;
      const alpha = 0.5; // smoothing factor
      this.instrPerMs = alpha * instrPerMs + (1 - alpha) * this.instrPerMs;

      if (deltaTime < 8) {
        this.stepSize = this.instrPerMs * 8;
        // console.log(`${steps} instructions in ${deltaTime.toFixed(0)}ms; adjusted step rate: ${this.stepSize}`);
      }
    }

    return cpuState;
  }

  /** Runs a batch of instructions.  Returns false if we should stop. */
  private stepMany(): boolean {
    this.breakpoints.install(this.emu);
    const cpuState = this.runBatch();
    this.breakpoints.uninstall(this.emu);

    switch (cpuState) {
      case wasm.Status.Running:
        return true;
      case wasm.Status.DebugBreak: {
        const bp = this.breakpoints.isAtBreakpoint(this.emu.eip);
        if (bp) {
          if (!bp.oneShot) {
            this.emuHost.showTab('breakpoints');
          }
          this.emuHost.onStopped();
        }
        return false;
      }
      case wasm.Status.Blocked:
      case wasm.Status.Error:
        this.emuHost.onStopped();
        return false;
      case wasm.Status.Exit:
        this.emuHost.onStopped();
        this.emuHost.exit(this.emu.exit_code);
        return false;
    }
  }

  start() {
    if (this.running) return;
    this.emu.unblock(); // Attempt to resume any blocked threads.
    // Advance past the current breakpoint, if any.
    if (this.breakpoints.isAtBreakpoint(this.emu.eip)) {
      this.step();
    }
    this.running = true;
    // Don't loop() immediately, to allow the message loop one tick before emulation starts.
    this.channel.port1.postMessage(null);
  }

  /** Runs a batch of instructions; called in a loop. */
  private loop() {
    if (!this.running) return;
    if (!this.stepMany()) {
      this.stop();
      return;
    }
    this.channel.port1.postMessage(null);
  }

  stop() {
    this.running = false;
  }

  mappings(): wasm.Mapping[] {
    return JSON.parse(this.emu.mappings_json()) as wasm.Mapping[];
  }

  labels(): Array<[number, string]> {
    const obj = JSON.parse(this.emu.labels()) as Record<number, string>;
    return Object.entries(obj).map(([addr, label]) => [parseInt(addr, 10), label]);
  }

  disassemble(addr: number): wasm.Instruction[] {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    return JSON.parse(this.emu.disassemble_json(addr, 20)) as wasm.Instruction[];
  }
}
