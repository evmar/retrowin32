import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';
import * as wasm from './wasm/wasm';

class Register extends preact.Component<{ value: number }> {
  render() {
    return <code>{hex(this.props.value, 8)}</code>;
  }
}

namespace Registers {
  export interface Props {
    regs: wasm.Registers;
  }
}
export class Registers extends preact.Component<Registers.Props> {
  render() {
    const { regs } = this.props;
    return (
      <section>
        <code>
          <div>
            eax <Register value={regs.eax} />
            <br />
            ebx <Register value={regs.ebx} />
            <br />
            ecx <Register value={regs.ecx} />
            <br />
            edx <Register value={regs.edx} />
            <br />
          </div>
          <br />
          <div>
            eip <Register value={regs.eip} />
            <br />
            esp <Register value={regs.esp} />
            <br />
            ebp <Register value={regs.ebp} />
            <br />
            esi <Register value={regs.esi} />
            <br />
            edi <Register value={regs.edi} />
            <br />
          </div>
          <br />
          <div>
            cs <Register value={regs.cs} />
            <br />
            ds <Register value={regs.ds} />
            <br />
            es <Register value={regs.es} />
            <br />
            fs <Register value={regs.fs} />
            <br />
            gs <Register value={regs.gs} />
            <br />
            ss <Register value={regs.ss} />
            <br />
          </div>
        </code>
      </section>
    );
  }
}
