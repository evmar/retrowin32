import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Emulator, EmulatorHost } from '../emulator';
import { Instruction } from '../glue/pkg/glue';
import { EmulatorComponent, loadEmulator } from '../web';
import { BreakpointsComponent } from './break';
import { Code } from './code';
import { Labels, parseCSV } from './labels';
import { Mappings } from './mappings';
import { Memory, MemoryView, Number } from './memory';
import { RegistersComponent } from './registers';
import { Stack } from './stack';
import { Tabs } from './tabs';
import { hex } from './util';

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
    emulator?: Emulator;
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
    const host: EmulatorHost = {
      exit: (code: number) => {
        this.print(`\nexited with code ${code}`);
        this.stop();
      },
      onWindowChanged: () => {
        this.forceUpdate();
      },
      showTab: (name: string) => {
        this.setState({ selectedTab: name });
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
      onStopped: () => {
        this.stop();
      },
    };
    const emulator = await loadEmulator(host);
    emulator.emu.set_tracing_scheme('*');

    const labels = emulator.labels();
    this.state.labels.load(labels);

    this.setState({ stdout: undefined, emulator });

    this.loadLabelsCSV(emulator).catch((e) => this.print(e.stack ?? e.toString()));
  }

  /** Load ghidra CSV, if any exists, into the label list. */
  async loadLabelsCSV(emulator: Emulator) {
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
      <div>
        <code>
          {this.state.stdout}
          {this.state.error ? <div class='error'>ERROR: {this.state.error}</div> : null}
        </code>
      </div>
    );

    if (!emulator) {
      return output;
    }

    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    let instrs: Instruction[] = [];
    let code;
    const eip = emulator.emu.eip;
    if (eip == 0xffff_fff0) {
      code = <section class='code'>(in async)</section>;
    } else {
      instrs = emulator.disassemble(eip);
      code = (
        <Code
          instrs={instrs}
          labels={labels}
          {...this.memoryView}
          runTo={this.runTo}
        />
      );
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
            {emulator.emu.instr_count} instrs executed | {Math.floor(emulator.instrPerMs)}/ms
          </div>
        </section>
        <div style={{ display: 'flex', gap: '8px' }}>
          {code}
          <RegistersComponent
            {...this.memoryView}
            regs={emulator.emu.regs()}
          />
          <Stack
            {...this.memoryView}
            labels={labels}
            emu={emulator.emu}
          />
        </div>
        <Tabs
          style={{ width: '80ex', flex: 1, minHeight: 0, display: 'flex', flexDirection: 'column' }}
          tabs={{
            output: () => output,

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
                <div style={{ flex: 1, overflow: 'auto' }}>
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
          }}
          selected={this.state.selectedTab}
          switchTab={(selectedTab) => this.setState({ selectedTab })}
        />
      </>
    );
  }
}

export function main() {
  preact.render(<Debugger />, document.body);
}
