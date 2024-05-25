import * as preact from 'preact';
import { h } from 'preact';
import { Labels } from './labels';
import { MemoryView, Number } from './memory';
import * as wasm from './glue/pkg/glue';

export interface Breakpoint {
  addr: number;
  disabled?: boolean;
  oneShot?: boolean;
}

/** Manages a set of UI-surfaced breakpoints, including e.g. disabled state. */
export class Breakpoints {
  breakpoints = new Map<number, Breakpoint>;
  constructor(private storageKey: string) {
    const json = window.localStorage.getItem(storageKey);
    if (!json) return;
    const list = JSON.parse(json) as Breakpoint[];
    this.breakpoints = new Map(list.map(bp => [bp.addr, bp]));
  }

  private save() {
    window.localStorage.setItem(this.storageKey, JSON.stringify(Array.from(this.breakpoints.values())));
  }

  addBreak(bp: Breakpoint) {
    this.breakpoints.set(bp.addr, bp);
    this.save();
  }

  addBreakByName(labels: Labels, name: string): boolean {
    for (const [addr, label] of labels.byAddr) {
      if (label === name) {
        this.addBreak({ addr });
        return true;
      }
    }
    if (name.match(/^[0-9a-fA-F]+$/)) {
      const addr = parseInt(name, 16);
      this.addBreak({ addr });
      return true;
    }
    return false;
  }

  delBreak(addr: number) {
    const bp = this.breakpoints.get(addr);
    if (!bp) return;
    this.breakpoints.delete(addr);
    this.save();
  }

  toggleBreak(addr: number) {
    const bp = this.breakpoints.get(addr)!;
    bp.disabled = !bp.disabled;
    this.save();
  }

  /** Check if the current address is a break/exit point, returning it if so. */
  isAtBreakpoint(ip: number): Breakpoint | undefined {
    const bp = this.breakpoints.get(ip);
    if (bp && !bp.disabled) {
      if (bp.oneShot) {
        this.delBreak(bp.addr);
      }
      return bp;
    }
    return undefined;
  }

  install(emu: wasm.Emulator) {
    for (const bp of this.breakpoints.values()) {
      if (!bp.disabled) {
        emu.breakpoint_add(bp.addr);
      }
    }
  }

  uninstall(emu: wasm.Emulator) {
    for (const bp of this.breakpoints.values()) {
      if (!bp.disabled) {
        emu.breakpoint_clear(bp.addr);
      }
    }
  }
}

namespace BreakpointsComponent {
  export interface Props extends MemoryView {
    breakpoints: Breakpoint[];
    labels: Labels;
    highlight: number;
    toggle: (addr: number) => void;
    add: (text: string) => boolean;
    remove: (addr: number) => void;
  }
}

export class BreakpointsComponent extends preact.Component<BreakpointsComponent.Props> {
  render() {
    const rows = [];
    for (const bp of this.props.breakpoints) {
      const className = bp.addr === this.props.highlight ? 'highlight' : undefined;
      const label = this.props.labels.get(bp.addr);
      rows.push(
        <div className={className} style={{ display: 'flex', alignItems: 'center', gap: '0.5ex' }}>
          <input type='checkbox' checked={!bp.disabled} onChange={() => this.props.toggle(bp.addr)} />
          <div>
            <code>
              <Number digits={8} {...this.props}>{bp.addr}</Number>
            </code>
          </div>
          {bp.oneShot ? '[once]' : null}
          {label
            ? (
              <div>
                (<code>{label}</code>)
              </div>
            )
            : null}
          <button class='x' onClick={() => this.props.remove(bp.addr)}>x</button>
        </div>,
      );
    }
    return (
      <section>
        {rows}
        <AddComponent onAccept={(text) => this.props.add(text)} />
      </section>
    );
  }
}

namespace AddComponent {
  export interface Props {
    onAccept(text: string): boolean;
  }
  export interface State {
    text: string;
  }
}
class AddComponent extends preact.Component<AddComponent.Props, AddComponent.State> {
  onInput = (ev: Event) => {
    const text = (ev.target! as HTMLInputElement).value;
    this.setState({ text });
  };
  onSubmit = (ev: Event) => {
    ev.preventDefault();
    if (this.props.onAccept(this.state.text)) {
      this.setState({ text: '' });
    }
  };
  render() {
    return (
      <form onSubmit={this.onSubmit}>
        add: <input value={this.state.text} onInput={this.onInput} />
      </form>
    );
  }
}
