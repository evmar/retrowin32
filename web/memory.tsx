import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';

export namespace Number {
  /**
   * Props used for any place in the UI where we show hex numbers, to add 'jump to address'
   * shortcuts.
   */
  export interface Interactions {
    showMemory: (addr: number) => void;
    highlightMemory: (addr: number) => void;
  }
  export interface Props extends Interactions {
    digits?: number;
    text?: string;
    children: number;
  }
}
export class Number extends preact.Component<Number.Props> {
  render() {
    return (
      <span
        class='clicky'
        title='show in memory dump'
        onMouseOver={() => {
          this.props.highlightMemory(this.props.children);
        }}
        onClick={(event) => {
          this.props.showMemory(this.props.children);
        }}
      >
        {this.props.text ? this.props.text : hex(this.props.children, this.props.digits)}
      </span>
    );
  }
}

namespace Memory {
  export interface Props {
    mem: DataView;
    base: number;
    highlight?: number;
    jumpTo: (addr: number) => void;
  }
}
export class Memory extends preact.Component<Memory.Props> {
  onSubmit = (e: Event) => {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const addr = (form.elements.namedItem('addr') as HTMLInputElement).value;
    this.props.jumpTo(parseInt(addr, 16));
  };

  jump(e: PointerEvent, direction: number) {
    let step = 0x100;
    if (e.shiftKey) step *= 0x10;
    if (e.altKey) step *= 0x100;
    step *= direction;
    this.props.jumpTo(this.props.base + step);
  }
  onJumpForward = (e: Event) => {
    this.jump(e as PointerEvent, 1);
  };
  onJumpBack = (e: Event) => {
    this.jump(e as PointerEvent, -1);
  };

  render() {
    let rows = [];
    const base = this.props.base & ~0xf;
    // Somehow the above can go negative on overflow(?).
    if (base >= 0) {
      for (let rowAddr = 0; rowAddr < 0x100; rowAddr += 0x10) {
        if (base + rowAddr >= this.props.mem.byteLength) break;
        const row = [];
        row.push(hex(base + rowAddr, 8));
        for (let offset = 0; offset < 0x10; offset++) {
          const addr = base + rowAddr + offset;
          if (addr >= this.props.mem.byteLength) break;
          if (offset % 4 === 0) row.push('  ');
          else row.push(' ');
          let value: preact.ComponentChild = hex(this.props.mem.getUint8(addr));
          if (addr === this.props.highlight) {
            value = <span class='highlight'>{value}</span>;
          }
          row.push(value);
        }
        rows.push(<div>{row}</div>);
      }
    }
    return (
      <section>
        <form style={{ display: 'flex', justifyContent: 'center' }} onSubmit={this.onSubmit}>
          <button type='button' onClick={this.onJumpBack}>&lt;</button>
          <input name='addr' size={8} value={hex(this.props.base, 8)} />
          <button type='button' onClick={this.onJumpForward}>&gt;</button>
        </form>
        <code>{rows}</code>
      </section>
    );
  }
}
