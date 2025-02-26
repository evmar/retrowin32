import * as preact from 'preact';
import { h } from 'preact';
import { Registers } from '../glue/pkg/glue';
import { MemoryView, Number } from './memory';
import { hex } from './util';

namespace RegistersComponent {
  export interface Props extends MemoryView {
    regs: Registers;
  }
}
export class RegistersComponent extends preact.Component<RegistersComponent.Props> {
  render() {
    const { regs } = this.props;
    const st = regs.st;
    return (
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
          cs&nbsp;<Number digits={4} {...this.props}>{regs.cs}</Number>{' '}
          fs&nbsp;<Number digits={4} {...this.props}>{regs.fs}</Number>
          <br />
          ds&nbsp;<Number digits={4} {...this.props}>{regs.ds}</Number>{' '}
          gs&nbsp;<Number digits={4} {...this.props}>{regs.gs}</Number>
          <br />
          es&nbsp;<Number digits={4} {...this.props}>{regs.es}</Number>{' '}
          ss&nbsp;<Number digits={4} {...this.props}>{regs.ss}</Number>
          <br />
        </div>
        <br />
        <div>
          flags&nbsp;{hex(regs.flags)} {regs.flags_str}
        </div>
        <br />
        {st.length > 0
          ? (
            <div>
              fpu<br />
              {Array.from(regs.st).map(n => (
                <span>
                  {n}
                  <br />
                </span>
              ))}
            </div>
          )
          : null}
      </code>
    );
  }
}
