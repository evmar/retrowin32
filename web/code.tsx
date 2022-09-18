import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';

export interface Instruction {
  addr: number;
  bytes: string;
  code: string;
}

namespace Code {
  export interface Props {
    instrs: Instruction[];
  }
}
export class Code extends preact.Component<Code.Props> {
  render() {
    return <pre>
        {this.props.instrs.map(instr => {
            return <div>{hex(instr.addr, 8)} {instr.bytes.padEnd(16, ' ')} {instr.code}</div>
    })}</pre>;
  }
}
