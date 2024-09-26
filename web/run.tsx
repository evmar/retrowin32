import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Emulator, EmulatorHost } from './emulator';
import { EmulatorComponent, loadEmulator } from './web';

namespace Host {
  export interface Props {
    emulator: Emulator;
    print(text: string): void;
  }
}

class Host extends preact.Component<Host.Props, {}> implements EmulatorHost {
  constructor(props: Host.Props) {
    super(props);
    this.props.emulator.emuHost = this;
  }

  override componentDidMount(): void {
    this.props.emulator.start();
  }

  exit(code: number): void {
    this.props.print(`exited with code ${code}\n`);
  }

  onWindowChanged(): void {
    this.forceUpdate();
  }

  showTab(name: string): void {
    throw new Error('Method not implemented.');
  }

  onError(msg: string): void {
    this.props.print(msg + '\n');
  }

  onStdOut(stdout: string): void {
    this.props.print(stdout);
  }

  onStopped(): void {
    // this.print(`emulator stopped`);
  }

  render() {
    return <EmulatorComponent emulator={this.props.emulator} />;
  }
}

namespace Page {
  export interface State {
    emulator?: Emulator;
    output?: string;
  }
}

class Page extends preact.Component<{}, Page.State> {
  private async load() {
    const emulator = await loadEmulator();
    emulator.emu.set_tracing_scheme('-');
    this.setState({ emulator });
  }

  private debugger() {
    window.location.pathname = window.location.pathname.replace('/run.html', '/debugger.html');
  }

  private print = (text: string) => {
    this.setState((state) => ({ output: (state.output ?? '') + text }));
  };

  componentDidMount(): void {
    this.load().catch((e) => this.print(e.stack ?? e.toString()));
  }

  render() {
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
          {this.state.output ? <pre class='stdout'>{this.state.output}</pre> : null}
          {this.state.emulator ? <Host emulator={this.state.emulator} print={this.print} /> : null}
        </main>
      </>
    );
  }
}

export function main() {
  preact.render(<Page />, document.body);
}
