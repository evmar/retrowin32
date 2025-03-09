import * as preact from 'preact';
import { CPU, Register } from '../glue/pkg/glue';
import { MemoryView, Number } from './memory';
import { hex } from './util';

function selectAllOnFocus(e: Event) {
  (e.target as HTMLInputElement).select();
}

/** Contents of the popup shown when you click on a register. */
namespace RegisterPanel {
  export interface Props {
    value: number;
    showMemory(addr: number): void;
    set(value: number): void;
  }
  export interface State {
    /** Current text in input field, possibly invalid value. */
    text: string;
  }
}
class RegisterPanel extends preact.Component<RegisterPanel.Props, RegisterPanel.State> {
  constructor(props: RegisterPanel.Props) {
    super(props);
    this.state = { text: hex(this.props.value, 8) };
  }

  getValue(): number {
    return parseInt(this.state.text, 16) & 0xFFFF_FFFF;
  }

  commit = (e: Event) => {
    e.preventDefault();
    this.props.set(this.getValue());
  };

  showMemory = () => {
    this.props.showMemory(this.getValue());
  };

  onInput = (e: Event) => {
    const text = (e.target as HTMLInputElement).value;
    this.setState({ text });
  };

  render() {
    return (
      <div class='panel' style={{ padding: '1ex', display: 'flex', flexDirection: 'column', gap: '1ex' }}>
        <form onSubmit={this.commit}>
          <input
            type='text'
            size={8}
            defaultValue={this.state.text}
            onInput={this.onInput}
            onFocusIn={selectAllOnFocus}
            onFocusOut={this.commit}
          />
          <sub>16</sub>
        </form>
        <div>
          {this.getValue()}
          <sub>10</sub>
        </div>
        <button onClick={this.showMemory}>show memory</button>
      </div>
    );
  }
}

declare global {
  interface HTMLElement {
    showPopover(options?: { source: HTMLElement }): void;
  }
}

/** The names of registers that we can display/set/get via RegisterComponent. */
type NamedRegister = 'eax' | 'ecx' | 'edx' | 'ebx' | 'esp' | 'ebp' | 'esi' | 'edi' | 'eip';

function getNamedRegister(cpu: CPU, reg: NamedRegister): number {
  switch (reg) {
    case 'eax':
      return cpu.get(Register.EAX);
    case 'ecx':
      return cpu.get(Register.ECX);
    case 'edx':
      return cpu.get(Register.EDX);
    case 'ebx':
      return cpu.get(Register.EBX);
    case 'esp':
      return cpu.get(Register.ESP);
    case 'ebp':
      return cpu.get(Register.EBP);
    case 'esi':
      return cpu.get(Register.ESI);
    case 'edi':
      return cpu.get(Register.EDI);
    case 'eip':
      return cpu.eip;
  }
}

function setNamedRegister(cpu: CPU, reg: NamedRegister, value: number) {
  switch (reg) {
    case 'eax':
      return cpu.set(Register.EAX, value);
    case 'ecx':
      return cpu.set(Register.ECX, value);
    case 'edx':
      return cpu.set(Register.EDX, value);
    case 'ebx':
      return cpu.set(Register.EBX, value);
    case 'esp':
      return cpu.set(Register.ESP, value);
    case 'ebp':
      return cpu.set(Register.EBP, value);
    case 'esi':
      return cpu.set(Register.ESI, value);
    case 'edi':
      return cpu.set(Register.EDI, value);
    case 'eip':
      cpu.jmp(value);
      break;
  }
}

/**
 * Displays one
 *    eax 00123456
 * row, with a clickable value that pops up a RegisterPanel.
 */
namespace RegisterComponent {
  export interface Props extends MemoryView {
    cpu: CPU;
    reg: NamedRegister;
  }
  export interface State {
    pop: boolean;
  }
}
class RegisterComponent extends preact.Component<RegisterComponent.Props, RegisterComponent.State> {
  ref = preact.createRef<HTMLDivElement>();

  pop = (e: Event) => {
    this.setState({ pop: true }, () => {
      // Wait for the setState before showing the popover to allow the DOM to
      // get created.
      // This callback can fire after the popup was already immediately closed.
      if (!this.state.pop) return;
      this.ref.current!.showPopover({ source: e.currentTarget as HTMLElement });
    });
  };

  unPop = (event: Event) => {
    const e = event as ToggleEvent;
    if (e.newState == 'closed') {
      this.setState({ pop: false });
    }
  };

  setValue = (value: number) => {
    setNamedRegister(this.props.cpu, this.props.reg, value);
    this.forceUpdate();
  };

  render() {
    const value = getNamedRegister(this.props.cpu, this.props.reg);
    return (
      <div style={{ display: 'flex', alignItems: 'baseline', gap: '1ex' }}>
        <div style={{ flex: 1 }}>{this.props.reg}</div>
        <button onClick={this.pop}>
          <code>{hex(value, 8)}</code>
        </button>
        {this.state.pop
          && (
            <div ref={this.ref} popover='auto' onToggle={this.unPop}>
              <RegisterPanel value={value} showMemory={this.props.showMemory} set={this.setValue} />
            </div>
          )}
      </div>
    );
  }
}

namespace RegistersComponent {
  export interface Props extends MemoryView {
    cpu: CPU;
  }
}
export class RegistersComponent extends preact.Component<RegistersComponent.Props> {
  render() {
    const { cpu } = this.props;

    const st = cpu.st();
    let fpu;
    if (st.length > 0) {
      fpu = (
        <div>
          fpu<br />
          {Array.from(st).map(n => (
            <span>
              {n.toFixed(6)}
              <br />
            </span>
          ))}
        </div>
      );
    }

    return (
      <div>
        <RegisterComponent reg='eax' {...this.props} />
        <RegisterComponent reg='ecx' {...this.props} />
        <RegisterComponent reg='edx' {...this.props} />
        <RegisterComponent reg='ebx' {...this.props} />
        <br />

        <RegisterComponent reg='esp' {...this.props} />
        <RegisterComponent reg='ebp' {...this.props} />
        <RegisterComponent reg='esi' {...this.props} />
        <RegisterComponent reg='edi' {...this.props} />
        <br />

        <RegisterComponent reg='eip' {...this.props} />
        <br />

        cs&nbsp;<Number digits={4} {...this.props}>{cpu.get(Register.CS)}</Number>{' '}
        fs&nbsp;<Number digits={4} {...this.props}>{cpu.get(Register.FS)}</Number>
        <br />
        ds&nbsp;<Number digits={4} {...this.props}>{cpu.get(Register.DS)}</Number>{' '}
        gs&nbsp;<Number digits={4} {...this.props}>{cpu.get(Register.GS)}</Number>
        <br />
        es&nbsp;<Number digits={4} {...this.props}>{cpu.get(Register.ES)}</Number>{' '}
        ss&nbsp;<Number digits={4} {...this.props}>{cpu.get(Register.SS)}</Number>
        <br />
        <br />

        flags&nbsp;{hex(cpu.flags())} {cpu.flags_str()}
        <br />
        {fpu}
      </div>
    );
  }
}
