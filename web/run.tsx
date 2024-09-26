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

class Page extends preact.Component<{ emulator: Emulator }> {
  private debugger() {
    window.location.pathname = window.location.pathname.replace('/run.html', '/debugger.html');
  }

  render({ emulator }: { emulator: Emulator }) {
    return (
      <>
        <header class='panel'>
          <a style='font-weight: bold; color: inherit' href='https://evmar.github.io/retrowin32/'>retrowin32</a>: a
          windows emulator
          <div style='width: 2ex'></div>
          <button onClick={this.debugger}>
            view in debugger
          </button>
        </header>

        <main>
          <Runner emulator={emulator} />
        </main>
      </>
    );
  }
}

export async function main() {
  const emulator = await loadEmulator();
  emulator.emu.set_tracing_scheme('-');
  preact.render(<Page emulator={emulator} />, document.body);
}
