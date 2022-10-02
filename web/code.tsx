import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';
import { Instruction } from './wasm/wasm';

namespace Code {
  export interface Props {
    labels: Map<number, string>;
    showMemory: (addr: number) => void;
    highlightMemory: (addr: number) => void;
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
            const label = this.props.labels.get(addr);
            if (label) {
              text += ` ${label}`;
            }
            return (
              <span
                class='clicky'
                title='show in memory dump'
                onMouseOver={() => {
                  this.props.highlightMemory(addr);
                }}
                onClick={(event) => {
                  this.props.showMemory(addr & ~0xF);
                }}
              >
                {text}
              </span>
            );
          }
          default:
            return text;
        }
      });
      return (
        <tr>
          <td>
            <span
              class='clicky'
              title='run to this address'
              onClick={(event) => {
                this.props.runTo(instr.addr);
              }}
            >
              {hex(instr.addr, 8)}
            </span>
          </td>
          <td>&nbsp;&nbsp;</td>
          <td>{instr.bytes}</td>
          <td>&nbsp;&nbsp;</td>
          <td title={instr.ops.join(',')}>{code}</td>
        </tr>
      );
    });
    return (
      <section>
        <code class='disassembly'>
          <table>
            {instrs}
          </table>
        </code>
      </section>
    );
  }
}
