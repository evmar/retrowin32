import * as preact from 'preact';
import { Emulator, Register } from '../glue/pkg/glue';
import { Labels } from './labels';
import { MemoryView, Number } from './memory';

namespace Stack {
  export interface Props extends MemoryView {
    emu: Emulator;
    labels: Labels;
  }
}
export class Stack extends preact.Component<Stack.Props> {
  render() {
    const { emu } = this.props;
    const esp = emu.cpu().get(Register.ESP);
    if (esp === 0) {
      return <code></code>;
    }
    const memory = emu.memory();
    const rows = [];
    for (let addr = esp - 0x10; addr < esp + 0x20; addr += 4) {
      const value = memory.getUint32(addr, true);
      const label = this.props.labels.get(value);
      let row = (
        <div>
          <Number digits={8} {...this.props}>{addr}</Number>
          &nbsp;
          <Number digits={8} {...this.props}>{value}</Number>
          &nbsp;
          {label}
        </div>
      );
      if (addr === esp) {
        row = <b>{row}</b>;
      }
      rows.push(row);
    }
    return <code>{rows}</code>;
  }
}
