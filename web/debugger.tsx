import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { BreakpointsComponent } from './break';
import { Code } from './code';
import { Emulator, EmulatorHost } from './emulator';
import { Instruction, Registers } from './glue/pkg/glue';
import { Mappings } from './mappings';
import { Memory, MemoryView } from './memory';
import { RegistersComponent } from './registers';
import { SnapshotsComponent } from './snapshots';
import { Stack } from './stack';
import { Tabs } from './tabs';
import { EmulatorComponent } from './web';

namespace StartStop {
  export interface Props {
    running: boolean;
    start(): void;
    stop(): void;
    step(): void;
    stepOver(): void;
  }
}
class StartStop extends preact.Component<StartStop.Props> {
  render() {
    const { running, start, stop, step, stepOver } = this.props;
    return (
      <span>
        <button
          onClick={() => running ? stop() : start()}
        >
          {running ? 'stop' : 'run'}
        </button>
        &nbsp;
        <button
          onClick={() => step()}
        >
          step
        </button>
        &nbsp;
        <button
          onClick={() => stepOver()}
        >
          step over
        </button>
      </span>
    );
  }
}

namespace Debugger {
  export interface Props {
    worker: Worker;
  }
  export interface State {
    stdout?: string;
    error: string;
    /** Initial address to show in memory pane. */
    memBase: number;
    /** Address to highlight in memory pane. */
    memHighlight?: number;
    /** When running, the setInterval id that is updating the UI. */
    running?: number;
    selectedTab: string;
  }
}
export class Debugger extends preact.Component<Debugger.Props, Debugger.State> implements EmulatorHost {
  emulator: Emulator;
  state: Debugger.State = { error: '', memBase: 0x40_1000, selectedTab: 'output' };

  constructor(props: Debugger.Props) {
    super(props);
    this.emulator = new Emulator(this.props.worker, this);
  }

  private print(text: string) {
    this.setState((state) => ({ stdout: (state.stdout ?? '') + text }));
  }

  exit(code: number): void {
    this.print(`\nexited with code ${code}`);
    this.stop();
  }
  onWindowChanged(): void {
    this.forceUpdate();
  }
  showTab(name: string): void {
    this.setState({ selectedTab: name });
  }
  onError(msg: string): void {
    this.print(msg + '\n');
    // Note: if this was a Rust panic, then the error will never display because
    // rendering will fail due to the debugger UI accessing the Rust object after a panic.
    //
    // Even rendering synchronously as in the following won't work due to rendering
    // accessing the object in a callback:
    //   preact.options.debounceRendering = (cb) => { cb(); };
    //   this.forceUpdate();
  }

  onStdOut(msg: string): void {
    this.print(msg);
  }

  private step = () => {
    try {
      this.emulator.step();
    } finally {
      this.forceUpdate();
    }
  };

  private start = () => {
    if (this.state.running) return;
    this.setState({
      running: setInterval(() => {
        this.forceUpdate();
      }, 500),
    });
    this.emulator.start();
  }

  private stop = () => {
    if (!this.state.running) return;
    this.emulator.stop();
    clearInterval(this.state.running);
    this.setState({ running: undefined });
  };

  private runTo = (addr: number) => {
    this.emulator.addBreak({ addr, oneShot: true });
    this.start();
  };

  private memoryView: MemoryView = {
    highlightMemory: (addr: number) => this.setState({ memHighlight: addr }),
    showMemory: (memBase: number) => this.setState({ selectedTab: 'memory', memBase }),
  };

  render() {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    let instrs: Instruction[] = [];
    let code;
    const regs = this.emulator.regs() as Registers;
    if (regs.eip >= 0xf1a7_0000) {
      const label = regs.eip == 0xffff_fff0 ? 'async' : this.emulator.labels.get(regs.eip) ?? 'shim';
      code = <section class='code'>(in {label})</section>;
    } else if (false) {
      instrs = this.emulator.disassemble(regs.eip);
      code = (
        <Code
          instrs={instrs}
          labels={this.emulator.labels}
          {...this.memoryView}
          runTo={this.runTo}
        />
      );
    }
    return (
      <>
        <EmulatorComponent emulator={this.emulator} />
        <section class='panel' style={{ display: 'flex', alignItems: 'baseline' }}>
          <StartStop
            running={this.state.running != null}
            start={this.start}
            stop={this.stop}
            step={this.step}
            stepOver={() => instrs ? this.runTo(instrs[1].addr) : this.step()}
          />
          &nbsp;
          {
            /* <div>
            {this.emulator.emu.instr_count} instrs executed | {Math.floor(this.emulator.instrPerMs)}/ms
          </div> */
          }
        </section>
        <div style={{ display: 'flex', margin: '1ex' }}>
          {code}
          <div style={{ width: '12ex' }} />
          <RegistersComponent
            {...this.memoryView}
            regs={regs}
          />
        </div>
        <div style={{ display: 'flex' }}>
          <Tabs
            style={{ width: '80ex' }}
            tabs={{
              output: () => (
                <div>
                  <code>
                    {this.state.stdout}
                    {this.state.error ? <div class='error'>ERROR: {this.state.error}</div> : null}
                  </code>
                </div>
              ),

              memory: () => (
                <Memory
                  mem={this.emulator.emu.memory()}
                  base={this.state.memBase}
                  highlight={this.state.memHighlight}
                  jumpTo={(addr) => this.setState({ memBase: addr })}
                />
              ),
              mappings: <Mappings mappings={this.emulator.mappings()} highlight={this.state.memHighlight} />,

              imports: () => (
                <div>
                  <code>
                    {this.emulator.imports.map(imp => <div>{imp}</div>)}
                  </code>
                </div>
              ),

              breakpoints: () => (
                <BreakpointsComponent
                  breakpoints={Array.from(this.emulator.breakpoints.values())}
                  labels={this.emulator.labels}
                  highlight={regs.eip}
                  {...this.memoryView}
                  toggle={(addr) => {
                    this.emulator.toggleBreak(addr);
                    this.forceUpdate();
                  }}
                  add={(text) => {
                    const ret = this.emulator.addBreakByName(text);
                    this.forceUpdate();
                    return ret;
                  }}
                  remove={(addr) => {
                    this.emulator.delBreak(addr);
                    this.forceUpdate();
                  }}
                />
              ),

              snapshots: () => (
                <SnapshotsComponent
                  take={() => this.emulator.emu.snapshot()}
                  load={(snap) => {
                    this.emulator.emu.load_snapshot(snap);
                    this.forceUpdate();
                  }}
                />
              ),
            }}
            selected={this.state.selectedTab}
            switchTab={(selectedTab) => this.setState({ selectedTab })}
          />
          <Stack
            {...this.memoryView}
            labels={this.emulator.labels}
            emu={this.emulator.emu}
          />
        </div>
      </>
    );
  }
}

export async function main() {
  const worker = await Emulator.initWorker();
  // emulator.emu.set_tracing_scheme('*');
  preact.render(<Debugger worker={worker} />, document.body);
}
