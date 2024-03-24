import * as preact from 'preact';
import { h } from 'preact';
import { Emulator } from './glue/pkg/glue';
import { Labels } from './labels';
import { Number } from './memory';

namespace Stack {
  export interface Props extends Number.Interactions {
    emu: Emulator;
    labels: Labels;
  }
}
export class Stack extends preact.Component<Stack.Props> {
  render() {
    const { emu } = this.props;
    const esp = emu.esp;
    const memory = emu.memory();
    const rows = [];
    for (let addr = esp - 0x10; addr < esp + 0x20; addr += 4) {
      const value = memory.getUint32(addr, true);
      let label = this.props.labels.get(value);
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
      <section class='panel'>
        <code>{rows}</code>
      </section>
    );
  }
}
