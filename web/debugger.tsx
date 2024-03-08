import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { BreakpointsComponent } from './break';
import { Code } from './code';
import { Emulator, EmulatorHost } from './emulator';
import { Mappings } from './mappings';
import { Memory } from './memory';
import { RegistersComponent } from './registers';
import { SnapshotsComponent } from './snapshots';
import { Stack } from './stack';
import { Tabs } from './tabs';
import { EmulatorComponent, loadEmulator } from './web';

namespace Debugger {
  export interface Props {
    emulator: Emulator;
  }
  export interface State {
    stdout: string;
    error: string;
    memBase: number;
    memHighlight?: number;
    /** When running, the setInterval id that is updating the UI. */
    running?: number;
    selectedTab: string;
  }
}
export class Debugger extends preact.Component<Debugger.Props, Debugger.State> implements EmulatorHost {
  stdout = ''; // XXX hack for making exit() not clobber state
  state: Debugger.State = { stdout: '', error: '', memBase: 0x40_1000, selectedTab: 'output' };

  constructor(props: Debugger.Props) {
    super(props);
    this.props.emulator.emuHost = this;
  }

  exit(code: number): void {
    this.setState({ stdout: this.stdout + `\nexited with code ${code}` });
    this.stop();
  }
  onWindowChanged(): void {
    this.forceUpdate();
  }
  showTab(name: string): void {
    this.setState({ selectedTab: name });
  }
  onError(msg: string): void {
    this.setState({ error: msg });
  }

  onStdOut(msg: string): void {
    this.stdout = msg;
    this.setState({ stdout: msg });
  }

  step() {
    try {
      this.props.emulator.step();
    } finally {
      this.forceUpdate();
    }
  }

  start() {
    if (this.state.running) return;
    this.setState({
      running: setInterval(() => {
        this.forceUpdate();
      }, 500),
    });
    this.props.emulator.start();
  }

  stop() {
    if (!this.state.running) return;
    this.props.emulator.stop();
    clearInterval(this.state.running);
    this.setState({ running: undefined });
  }

  runTo(addr: number) {
    this.props.emulator.addBreak({ addr, oneShot: true });
    this.start();
  }

  highlightMemory = (addr: number) => this.setState({ memHighlight: addr });
  showMemory = (memBase: number) => {
    this.setState({ selectedTab: 'memory', memBase });
  };

  render() {
    // Note: disassemble_json() may cause allocations, invalidating any existing .memory()!
    const instrs = this.props.emulator.disassemble(this.props.emulator.emu.eip);
    return (
      <>
        <EmulatorComponent emulator={this.props.emulator} />
        <section class='panel' style={{ display: 'flex', alignItems: 'baseline' }}>
          <button
            onClick={() => this.state.running ? this.stop() : this.start()}
          >
            {this.state.running ? 'stop' : 'run'}
          </button>
          &nbsp;
          <button
            onClick={() => this.step()}
          >
            step
          </button>
          &nbsp;
          <button
            onClick={() => this.runTo(instrs[1].addr)}
          >
            step over
          </button>
          &nbsp;
          <div>
            {this.props.emulator.emu.instr_count} instrs executed | {Math.floor(this.props.emulator.instrPerMs)}/ms
          </div>
        </section>
        <div style={{ display: 'flex', margin: '1ex' }}>
          <Code
            instrs={instrs}
            labels={this.props.emulator.labels}
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            runTo={(addr: number) => this.runTo(addr)}
          />
          <div style={{ width: '12ex' }} />
          <RegistersComponent
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            regs={this.props.emulator.emu.regs()}
          />
        </div>
        <div style={{ display: 'flex' }}>
          <Tabs
            style={{ width: '80ex' }}
            tabs={{
              output: (
                <div>
                  <code>
                    {this.state.stdout}
                    {this.state.error ? <div class='error'>ERROR: {this.state.error}</div> : null}
                  </code>
                </div>
              ),

              memory: (
                <Memory
                  mem={this.props.emulator.emu.memory()}
                  base={this.state.memBase}
                  highlight={this.state.memHighlight}
                  jumpTo={(addr) => this.setState({ memBase: addr })}
                />
              ),
              mappings: <Mappings mappings={this.props.emulator.mappings()} highlight={this.state.memHighlight} />,

              imports: (
                <div>
                  <code>
                    {this.props.emulator.imports.map(imp => <div>{imp}</div>)}
                  </code>
                </div>
              ),

              breakpoints: (
                <BreakpointsComponent
                  breakpoints={Array.from(this.props.emulator.breakpoints.values())}
                  labels={this.props.emulator.labels}
                  highlight={this.props.emulator.emu.eip}
                  highlightMemory={this.highlightMemory}
                  showMemory={this.showMemory}
                  toggle={(addr) => {
                    this.props.emulator.toggleBreak(addr);
                    this.forceUpdate();
                  }}
                  add={(text) => {
                    const ret = this.props.emulator.addBreakByName(text);
                    this.forceUpdate();
                    return ret;
                  }}
                  remove={(addr) => {
                    this.props.emulator.delBreak(addr);
                    this.forceUpdate();
                  }}
                />
              ),

              snapshots: (
                <SnapshotsComponent
                  take={() => this.props.emulator.emu.snapshot()}
                  load={(snap) => {
                    this.props.emulator.emu.load_snapshot(snap);
                    this.forceUpdate();
                  }}
                />
              ),
            }}
            selected={this.state.selectedTab}
            switchTab={(selectedTab) => this.setState({ selectedTab })}
          />
          <Stack
            highlightMemory={this.highlightMemory}
            showMemory={this.showMemory}
            labels={this.props.emulator.labels}
            emu={this.props.emulator.emu}
          />
        </div>
      </>
    );
  }
}

export async function main() {
  const emulator = await loadEmulator();
  preact.render(<Debugger emulator={emulator} />, document.body);
}
