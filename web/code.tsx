import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';
import { Instruction } from './wasm/wasm';

namespace Code {
  export interface Props {
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
            return <u>{text}</u>;
          case 'Number':
            return (
              <a
                href='#'
                onMouseOver={() => {
                  this.props.highlightMemory(parseInt(text, 16));
                }}
                onClick={(event) => {
                  this.props.showMemory(parseInt(text, 16) & ~0xF);
                  event.preventDefault();
                }}
              >
                {text}
              </a>
            );
          default:
            return text;
        }
      });
      return (
        <div>
          <a
            href='#'
            class='stealth'
            onClick={(event) => {
              event.preventDefault();
              this.props.runTo(instr.addr);
            }}
          >
            {hex(instr.addr, 8)}
          </a>{' '}
          {instr.bytes.padEnd(16, ' ')} {code} ({instr.ops.join(',')})
        </div>
      );
    });
    return (
      <section>
        <code>{instrs}</code>
      </section>
    );
  }
}
