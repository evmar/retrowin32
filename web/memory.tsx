import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';

export namespace Number {
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
          this.props.showMemory(this.props.children & ~0xF);
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
  render() {
    let rows = [];
    for (let rowAddr = 0; rowAddr < 0x100; rowAddr += 0x10) {
      const row = [];
      row.push(hex(this.props.base + rowAddr, 8));
      for (let offset = 0; offset < 0x10; offset++) {
        if (offset % 4 === 0) row.push('  ');
        else row.push(' ');
        const addr = this.props.base + rowAddr + offset;
        let value: preact.ComponentChild = hex(this.props.mem.getUint8(addr));
        if (addr === this.props.highlight) {
          value = <span class='highlight'>{value}</span>;
        }
        row.push(value);
      }
      rows.push(<div>{row}</div>);
    }
    return (
      <section>
        <div style={{ display: 'flex', justifyContent: 'center' }}>
          <button onClick={() => this.props.jumpTo(this.props.base - 0x100)}>&lt;</button>
          <input size={8} value={hex(this.props.base, 8)} />
          <button onClick={() => this.props.jumpTo(this.props.base + 0x100)}>&gt;</button>
        </div>
        <code>{rows}</code>
      </section>
    );
  }
}
