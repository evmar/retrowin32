import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Emulator, EmulatorHost } from './emulator';
import { EmulatorComponent, loadEmulator } from './web';

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
    const host: EmulatorHost = {
      exit: (code) => {
        this.print(`exited with code ${code}\n`);
      },
      onWindowChanged: () => {
        this.forceUpdate();
      },
      showTab: function(name: string): void {
      },
      onError: (msg) => {
        this.print(msg + '\n');
      },
      onStdOut: (stdout) => {
        this.print(stdout);
      },
      onStopped: () => {
        // TODO
      },
    };
    emulator.emuHost = host;
    this.setState({ emulator });
    emulator.start();
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
          {this.state.emulator ? <EmulatorComponent emulator={this.state.emulator} /> : null}
        </main>
      </>
    );
  }
}

export function main() {
  preact.render(<Page />, document.body);
}
