import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Emulator, EmulatorHost } from './emulator';
import { EmulatorComponent, loadEmulator } from './web';

class Runner extends preact.Component<{ emulator: Emulator }> implements EmulatorHost {
    constructor(props: { emulator: Emulator }) {
        super(props);
        this.props.emulator.emuHost = this;
    }

    componentDidMount(): void {
        this.props.emulator.start();
    }

    exit(code: number): void {
        throw new Error('Method not implemented.');
    }

    onWindowChanged(): void {
        this.forceUpdate();
    }

    showTab(name: string): void {
        throw new Error('Method not implemented.');
    }

    onError(msg: string): void {
    }

    onStdOut(stdout: string): void {
        throw new Error('Method not implemented.');
    }

    render() {
        return <EmulatorComponent emulator={this.props.emulator} />;
    }
}

export async function main() {
    const emulator = await loadEmulator();
    preact.render(<Runner emulator={emulator} />, document.getElementById('main')!);
}
