import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';

export interface Registers {
  eax: number;
  ebx: number;
  ecx: number;
  edx: number;
  esp: number;
  ebp: number;
  esi: number;
  edi: number;
  eip: number;
  cs: number;
  ds: number;
  es: number;
  fs: number;
  gs: number;
  ss: number;
}

class Register extends preact.Component<{ value: number }> {
  render() {
    return <code>{hex(this.props.value, 8)}</code>;
  }
}

namespace RegistersView {
  export interface Props {
    regs: Registers;
  }
}
export class RegistersView extends preact.Component<RegistersView.Props> {
  render() {
    const { regs } = this.props;
    return (
      <code>
        <div>
          eax <Register value={regs.eax} />
        </div>
        <div>
          ebx <Register value={regs.ebx} />
        </div>
        <div>
          ecx <Register value={regs.ecx} />
        </div>
        <div>
          edx <Register value={regs.edx} />
        </div>
        <div>&nbsp;</div>
        <div>
          esp <Register value={regs.esp} />
        </div>
        <div>
          ebp <Register value={regs.ebp} />
        </div>
        <div>
          esi <Register value={regs.esi} />
        </div>
        <div>
          edi <Register value={regs.edi} />
        </div>
        <div>
          eip <Register value={regs.eip} />
        </div>
        <div>
          cs <Register value={regs.cs} />
        </div>
        <div>
          ds <Register value={regs.ds} />
        </div>
        <div>
          es <Register value={regs.es} />
        </div>
        <div>
          fs <Register value={regs.fs} />
        </div>
        <div>
          gs <Register value={regs.gs} />
        </div>
        <div>
          ss <Register value={regs.ss} />
        </div>
      </code>
    );
  }
}
