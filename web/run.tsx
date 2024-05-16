import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { EmulatorComponent } from './web';
import { Emulator, EmulatorHost } from './emulator';

interface State {
  output?: string;
}

class Runner extends preact.Component<{ worker: Worker }, State> implements EmulatorHost {
  emulator: Emulator;

  constructor(props: { worker: Worker }) {
    super(props);
    this.emulator = new Emulator(this.props.worker, this);
  }

  private print(text: string) {
    this.setState((state) => ({ output: (state.output ?? '') + text }));
  }

  componentDidMount(): void {
    this.emulator.start();
  }

  exit(code: number): void {
    this.print(`Exited with code ${code}`);
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

  render() {
    return (
      <>
        {this.state.output ? <pre class='stdout'>{this.state.output}</pre> : null}
        <EmulatorComponent emulator={this.emulator} />
      </>
    );
  }
}

export async function main() {
  const worker = await Emulator.initWorker();
  preact.render(<Runner worker={worker} />, document.getElementById('main')!);
}
