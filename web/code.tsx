import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';
import { Instruction } from './wasm/wasm';

namespace Code {
  export interface Props {
    instrs: Instruction[];
  }
}
export class Code extends preact.Component<Code.Props> {
  render() {
    const instrs = this.props.instrs.map(instr => {
      let code = instr.code.map(({ kind, text }) => {
        switch (kind) {
          case 'FunctionAddress':
          case 'LabelAddress':
            return <u>{text}</u>;
          default:
            return text;
        }
      });
      return <div>{hex(instr.addr, 8)} {instr.bytes.padEnd(16, ' ')} {code}</div>;
    });
    return <code>{instrs}</code>;
  }
}
