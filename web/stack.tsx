import * as preact from 'preact';
import { h } from 'preact';
import { Number } from './memory';
import { hex } from './util';
import { X86 } from './wasm/wasm';

namespace Stack {
  export interface Props extends Number.Interactions {
    x86: X86;
    labels: Map<number, string>;
  }
}
export class Stack extends preact.Component<Stack.Props> {
  render() {
    const { x86 } = this.props;
    const esp = x86.esp;
    const memory = x86.memory();
    const rows = [];
    for (let addr = esp - 0x10; addr < esp + 0x20; addr += 4) {
      const value = memory.getUint32(addr, true);
      let label = this.props.labels.get(addr);
      if (label) {
        label = ` ${label}`;
      }
      let row = (
        <div>
          <Number digits={8} {...this.props}>{addr}</Number>
          &nbsp;
          <Number digits={8} {...this.props}>{value}</Number>
          {label}
        </div>
      );
      if (addr === esp) {
        row = <b>{row}</b>;
      }
      rows.push(row);
    }
    return (
      <section>
        <code>{rows}</code>
      </section>
    );
  }
}
