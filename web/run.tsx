/**
 * The main page for running the emulator.
 * The top panel, console, and EmulatorComponent.
 */

import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Emulator, EmulatorHost, EmulatorStatus, loadEmulator } from './emulator';
import { EmulatorComponent } from './web';

interface Status {
  instrCount: number;
  instrPerMs: number;
}

class Panel extends preact.Component<{ emulator?: Emulator }, { status?: Status }> {
  private debugger() {
    window.location.pathname = window.location.pathname.replace('/run.html', '/debugger.html');
  }

  private updateStatus = () => {
    if (!this.props.emulator) return;

    this.setState({
      status: {
        instrCount: this.props.emulator.emu.instr_count,
        instrPerMs: Math.floor(this.props.emulator.looper.stepsPerMs),
      },
    });
  };
  interval?: number;
  componentDidUpdate(): void {
    if (this.props.emulator && !this.interval) {
      this.updateStatus();
      this.interval = setInterval(this.updateStatus, 500);
    } else if (!this.props.emulator && this.interval) {
      clearInterval(this.interval);
      this.interval = undefined;
    }
  }

  render() {
    let status;
    if (this.state.status) {
      status = (
        <div>
          {this.state.status.instrCount} instrs executed, {Math.floor(this.state.status.instrPerMs)}/ms
        </div>
      );
    }

    return (
      <header class='panel'>
        <a style='font-weight: bold; color: inherit' href='./'>retrowin32</a>: a windows emulator
        <div style='width: 2ex'></div>
        <button onClick={this.debugger}>
          view in debugger
        </button>
        <div style={{ flex: '1' }} />
        {status}
      </header>
    );
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
    const host: EmulatorHost = {
      onWindowChanged: () => {
        this.forceUpdate();
      },
      onError: (msg) => {
        this.print(msg + '\n');
        this.setState({ emulator: undefined });
      },
      onStdOut: (stdout) => {
        this.print(stdout);
      },
      onStopped: (status) => {
        switch (status) {
          case EmulatorStatus.Exit:
            this.print(`exited with code ${emulator.emu.exit_code}\n`);
        }
      },
      onEvent: (event) => {
        emulator.enqueueEvent(event);
      },
    };
    const emulator = await loadEmulator(host);
    emulator.emu.set_tracing_scheme('-');
    this.setState({ emulator });
    emulator.start();
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
        <Panel emulator={this.state.emulator} />
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
