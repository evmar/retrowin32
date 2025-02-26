import * as preact from 'preact';
import { Fragment, h } from 'preact';
import { Instruction } from '../glue/pkg/glue';
import { Labels } from './labels';
import { MemoryView, Number } from './memory';
import { hex } from './util';

namespace Code {
  export interface Props extends MemoryView {
    labels: Labels;
    runTo: (addr: number) => void;
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
          case 'Number': {
            const addr = parseInt(text, 16);
            let label = this.props.labels.get(addr);
            if (label) {
              label = ` ${label}`;
            }
            return (
              <>
                <Number text={text} {...this.props}>{addr}</Number>
                {label}
              </>
            );
          }
          default:
            return text;
        }
      });
      return (
        <div>
          <span
            class='clicky'
            title='run to this address'
            onClick={(event) => {
              this.props.runTo(instr.addr);
            }}
          >
            {hex(instr.addr, 8)}
          </span>
          &nbsp;&nbsp;
          <span title={`${instr.bytes} (${instr.ops.join(',')})`}>{code}</span>
        </div>
      );
    });
    return (
      <code class='disassembly'>
        {instrs}
      </code>
    );
  }
}
