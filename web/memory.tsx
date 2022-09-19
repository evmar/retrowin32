import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';

namespace Memory {
  export interface Props {
    mem: DataView;
    base: number;
    highlight?: number;
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
          value = <b>{value}</b>;
        }
        row.push(value);
      }
      rows.push(<div>{row}</div>);
    }
    return (
      <section>
        <div style={{ display: 'flex', justifyContent: 'center' }}>
          <button>&lt;</button>
          <input size={8} value={hex(this.props.base, 8)} />
          <button>&gt;</button>
        </div>
        <code>{rows}</code>
      </section>
    );
  }
}
