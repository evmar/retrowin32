import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Emulator } from './emulator';
import { parseCSV } from './labels';

namespace WindowComponent {
  export interface Props {
    title: string;
    canvas: HTMLCanvasElement;
  }
  export interface State {
    drag?: [number, number];
    pos: [number, number];
  }
}
class WindowComponent extends preact.Component<WindowComponent.Props, WindowComponent.State> {
  state: WindowComponent.State = {
    pos: [200, 200],
  };
  ref = preact.createRef();

  beginDrag = (e: PointerEvent) => {
    const node = e.currentTarget as HTMLElement;
    this.setState({ drag: [e.offsetX, e.offsetY] });
    node.setPointerCapture(e.pointerId);
    e.preventDefault();
  };
  onDrag = (e: PointerEvent) => {
    if (!this.state.drag) return;
    this.setState({ pos: [e.clientX - this.state.drag[0], e.clientY - this.state.drag[1]] });
    e.preventDefault();
  };
  endDrag = (e: PointerEvent) => {
    const node = e.currentTarget as HTMLElement;
    this.setState({ drag: undefined });
    node.releasePointerCapture(e.pointerId);
    e.preventDefault();
  };

  ensureCanvas() {
    // XXX: how to ensure the canvas appears as a child of this widget?
    if (this.props.canvas && this.ref.current && !this.ref.current.firstChild) {
      this.ref.current.appendChild(this.props.canvas);
    }
  }

  componentDidMount(): void {
    this.ensureCanvas();
  }

  render() {
    this.ensureCanvas();
    return (
      <div class='window' style={{ left: `${this.state.pos[0]}px`, top: `${this.state.pos[1]}px` }}>
        <div class='titlebar' onPointerDown={this.beginDrag} onPointerUp={this.endDrag} onPointerMove={this.onDrag}>
          {this.props.title}
        </div>
        <div ref={this.ref} />
      </div>
    );
  }
}

namespace EmulatorComponent {
  export interface Props {
    emulator: Emulator;
  }
}
export class EmulatorComponent extends preact.Component<EmulatorComponent.Props> {
  render() {
    return Array.from(this.props.emulator.windows.values()).map((window) => {
      return (
        <WindowComponent
          key={window.hwnd}
          title={window.title}
          canvas={window.canvas}
        />
      );
    });
  }
}

// export async function loadEmulator() {
//   const params = parseURL();
//   if (!params) {
//     throw new Error('invalid URL params');
//   }

//   //await wasm.default(new URL('wasm.wasm', document.location.href));

//   const csvLabels = new Map<number, string>();
//   const resp = await fetch(params.exe + '.csv');
//   if (resp.ok) {
//     for (const [addr, name] of parseCSV(await resp.text())) {
//       csvLabels.set(addr, name);
//     }
//   }

//   const storageKey = (params.dir ?? '') + params.exe;
//   return new Emulator(
//     null!,
//     fileset,
//     storageKey,
//     fileset.get(params.exe)!,
//     csvLabels,
//     params.relocate ?? false,
//   );
// }
