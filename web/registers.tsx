import * as preact from 'preact';
import { h } from 'preact';
import { Number } from './memory';
import { hex } from './util';
import * as wasm from './wasm/wasm';

namespace Registers {
  export interface Props extends Number.Interactions {
    regs: wasm.Registers & { flags_str(): string };
  }
}
export class Registers extends preact.Component<Registers.Props> {
  render() {
    const { regs } = this.props;
    return (
      <section>
        <code>
          <div>
            eax&nbsp;<Number digits={8} {...this.props}>{regs.eax}</Number>
            <br />
            ebx&nbsp;<Number digits={8} {...this.props}>{regs.ebx}</Number>
            <br />
            ecx&nbsp;<Number digits={8} {...this.props}>{regs.ecx}</Number>
            <br />
            edx&nbsp;<Number digits={8} {...this.props}>{regs.edx}</Number>
            <br />
          </div>
          <br />
          <div>
            eip&nbsp;<Number digits={8} {...this.props}>{regs.eip}</Number>
            <br />
            esp&nbsp;<Number digits={8} {...this.props}>{regs.esp}</Number>
            <br />
            ebp&nbsp;<Number digits={8} {...this.props}>{regs.ebp}</Number>
            <br />
            esi&nbsp;<Number digits={8} {...this.props}>{regs.esi}</Number>
            <br />
            edi&nbsp;<Number digits={8} {...this.props}>{regs.edi}</Number>
            <br />
          </div>
          <br />
          <div>
            cs&nbsp;<Number digits={4} {...this.props}>{regs.cs}</Number>
            <br />
            ds&nbsp;<Number digits={4} {...this.props}>{regs.ds}</Number>
            <br />
            es&nbsp;<Number digits={4} {...this.props}>{regs.es}</Number>
            <br />
            fs&nbsp;<Number digits={4} {...this.props}>{regs.fs}</Number>
            <br />
            gs&nbsp;<Number digits={4} {...this.props}>{regs.gs}</Number>
            <br />
            ss&nbsp;<Number digits={4} {...this.props}>{regs.ss}</Number>
            <br />
          </div>
          <br />
          <div>
            flags&nbsp;{hex(regs.flags)}
            <br />
            {regs.flags_str()}
          </div>
        </code>
      </section>
    );
  }
}
