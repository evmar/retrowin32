import * as preact from 'preact';
import * as emulator from '../emulator';
import { Instruction } from '../glue/pkg/glue';
import { EmulatorComponent } from '../web';
import { BreakpointsComponent } from './break';
import { Code } from './code';
import { CPUs } from './cpus';
import { DirectDraw } from './ddraw';
import { Labels, parseCSV } from './labels';
import { Mappings } from './mappings';
import { Memory, MemoryView, Number } from './memory';
import { RegistersComponent } from './registers';
import { Stack } from './stack';
import { Tabs } from './tabs';
import { TraceComponent } from './trace';

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
  }
  export interface State {
    emulator?: emulator.Emulator;
    stdout?: string;
    error: string;
    /** Initial address to show in memory pane. */
    memBase: number;
    /** Address to highlight in memory pane. */
    memHighlight?: number;
    /** When running, the setInterval id that is updating the UI.  Note that Emulator.running has slightly different semantics. */
    running?: number;
    selectedTab: string;
    labels: Labels;
  }
}
export class Debugger extends preact.Component<Debugger.Props, Debugger.State> {
  state: Debugger.State = { error: '', memBase: 0, labels: new Labels(), selectedTab: 'output' };

  private async load() {
    this.print('Loading...\n');
    const host: emulator.Host = {
      onWindowChanged: () => {
        this.forceUpdate();
      },
      onError: (msg: string) => {
        this.print(msg + '\n');
        // Note: if this was a Rust panic, then the error will never display because
        // rendering will fail due to the debugger UI accessing the Rust object after a panic.
        //
        // Even rendering synchronously as in the following won't work due to rendering
        // accessing the object in a callback:
        //   preact.options.debounceRendering = (cb) => { cb(); };
        //   this.forceUpdate();
      },
      onStdOut: (stdout: string) => {
        this.print(stdout);
      },
      onStopped: (status) => {
        switch (status) {
          case emulator.Status.Blocked:
            return; // don't stop the UI on this
          case emulator.Status.DebugBreak:
            const bp = emu.breakpoints.isAtBreakpoint(emu.emu.cpu().eip);
            if (bp) {
              if (!bp.oneShot) {
                this.setState({ selectedTab: 'breakpoints' });
              }
            }
            break;
          case emulator.Status.Exit:
            this.print(`\nexited with code ${emu.emu.exit_code}`);
            break;
        }
        this.stop(); // update the run/stop button etc.
      },
      onEvent: (event) => {
        if (!this.state.running) {
          console.warn('dropping event while not running');
          return;
        }
        emu.enqueueEvent(event);
      },
    };
    const emu = await emulator.load(host);
    emu.emu.set_tracing_scheme('*');

    const labels = emu.labels();
    this.state.labels.load(labels);

    this.setState({ stdout: undefined, emulator: emu });

    this.loadLabelsCSV(emu).catch((e) => this.print(e.stack ?? e.toString()));
  }

  /** Load ghidra CSV, if any exists, into the label list. */
  async loadLabelsCSV(emulator: emulator.Emulator) {
    const resp = await fetch(emulator.exePath + '.csv');
    if (!resp.ok) {
      return;
    }
    const labels = parseCSV(await resp.text());
    this.state.labels.load(labels);
    this.setState({ labels: this.state.labels });
  }

  componentDidMount(): void {
    this.load().catch((e) => this.print(e.stack ?? e.toString()));
  }

  private print(text: string) {
    this.setState((state) => ({ stdout: (state.stdout ?? '') + text }));
  }

  private step = () => {
    try {
      this.state.emulator!.step();
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
    this.state.emulator!.start();
  };

  private stop = () => {
    if (!this.state.running) return;
    this.state.emulator!.stop();
    clearInterval(this.state.running);
    this.setState({ running: undefined });
  };

  private runTo = (addr: number) => {
    this.state.emulator!.breakpoints.addBreak({ addr, oneShot: true });
    this.start();
  };

  private memoryView: MemoryView = {
    highlightMemory: (addr: number) => this.setState({ memHighlight: addr }),
    showMemory: (memBase: number) => this.setState({ selectedTab: 'memory', memBase }),
  };

  render() {
    const { emulator, labels } = this.state;

    const output = (
      <code>
        {this.state.stdout}
        {this.state.error ? <div class='error'>ERROR: {this.state.error}</div> : null}
      </code>
    );

    if (!emulator) {
      return output;
    }

    // Note: disassemble() may cause allocations, invalidating any existing .memory()!
    let instrs: Instruction[] = [];
    const eip = emulator.emu.cpu().eip;
    if (eip == 0xffff_fff0) {
      instrs = [];
    } else {
      instrs = emulator.disassemble(eip);
    }
    return (
      <>
        <EmulatorComponent emulator={emulator} />
        <section class='panel' style={{ display: 'flex', alignItems: 'baseline' }}>
          <StartStop
            running={this.state.running != null}
            start={this.start}
            stop={this.stop}
            step={this.step}
            stepOver={() => instrs ? this.runTo(instrs[1].addr) : this.step()}
          />
          &nbsp;
          <div>
            {emulator.emu.instr_count} instrs executed | {Math.floor(emulator.looper.stepsPerMs)}/ms
          </div>
        </section>
        <div style={{ display: 'flex' }}>
          <section class='code'>
            <Code
              instrs={instrs}
              labels={labels}
              {...this.memoryView}
              runTo={this.runTo}
            />
          </section>
          <section>
            <RegistersComponent
              cpu={emulator.emu.cpu()}
              {...this.memoryView}
            />
          </section>
          <section style={{ flex: 1, minWidth: 0, overflow: 'auto' }}>
            <Stack
              {...this.memoryView}
              labels={labels}
              emu={emulator.emu}
            />
          </section>
        </div>
        <section style={{ flex: 1, minHeight: 0, display: 'flex' }}>
          <Tabs
            style={{ flex: 1 }}
            tabs={{
              output: () => output,

              trace: () => <TraceComponent trace={emulator.trace} />,

              cpus: () => <CPUs cpus={emulator.emu.cpus()} />,

              memory: () => (
                <Memory
                  mem={emulator.emu.memory()}
                  base={this.state.memBase}
                  highlight={this.state.memHighlight}
                  jumpTo={(addr) => this.setState({ memBase: Math.max(0, addr) })}
                />
              ),
              mappings: () => (
                <Mappings
                  mappings={emulator.mappings()}
                  highlight={this.state.memHighlight}
                  {...this.memoryView}
                />
              ),

              imports: () => {
                const labels = emulator.labels();
                return (
                  <div style={{ minHeight: 0, flex: 1, overflow: 'auto' }}>
                    {labels.map(([addr, name]) => (
                      <div>
                        <code>
                          <Number digits={8} {...this.memoryView}>{addr}</Number>: {name}
                        </code>
                      </div>
                    ))}
                  </div>
                );
              },

              breakpoints: () => (
                <BreakpointsComponent
                  breakpoints={emulator.breakpoints}
                  labels={labels}
                  highlight={eip}
                  {...this.memoryView}
                />
              ),

              directdraw: () => <DirectDraw surfaces={emulator.emu.direct_draw_surfaces()} {...this.memoryView} />,
            }}
            selected={this.state.selectedTab}
            switchTab={(selectedTab) => this.setState({ selectedTab })}
          />
        </section>
      </>
    );
  }
}

export function main() {
  preact.render(<Debugger />, document.body);
}
