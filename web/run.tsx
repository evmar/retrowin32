import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Emulator, EmulatorHost } from './emulator';
import { EmulatorComponent, loadEmulator } from './web';

interface State {
  output?: string;
}

class Runner extends preact.Component<{ emulator: Emulator }, State> implements EmulatorHost {
  constructor(props: { emulator: Emulator }) {
    super(props);
    this.props.emulator.emuHost = this;
  }

  private print(text: string) {
    this.setState((state) => ({ output: (state.output ?? '') + text }));
  }

  componentDidMount(): void {
    this.props.emulator.start();
  }

  exit(code: number): void {
    this.print(`exited with code ${code}\n`);
  }

  onWindowChanged(): void {
    this.forceUpdate();
  }

  showTab(name: string): void {
    throw new Error('Method not implemented.');
  }

  onError(msg: string): void {
    this.print(msg + '\n');
  }

  onStdOut(stdout: string): void {
    this.print(stdout);
  }

  onStopped(): void {
    // this.print(`emulator stopped`);
  }

  render() {
    return (
      <>
        {this.state.output ? <pre class='stdout'>{this.state.output}</pre> : null}
        <EmulatorComponent emulator={this.props.emulator} />
      </>
    );
  }
}

export async function main() {
  const emulator = await loadEmulator();
  emulator.emu.set_tracing_scheme('-');
  preact.render(<Runner emulator={emulator} />, document.getElementById('main')!);
}
