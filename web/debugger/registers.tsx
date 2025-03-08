import * as preact from 'preact';
import { Emulator, Register, Registers } from '../glue/pkg/glue';
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

function getNamedRegister(emu: Emulator, reg: NamedRegister): number {
  switch (reg) {
    case 'eax':
      return emu.reg(Register.EAX);
    case 'ecx':
      return emu.reg(Register.ECX);
    case 'edx':
      return emu.reg(Register.EDX);
    case 'ebx':
      return emu.reg(Register.EBX);
    case 'esp':
      return emu.reg(Register.ESP);
    case 'ebp':
      return emu.reg(Register.EBP);
    case 'esi':
      return emu.reg(Register.ESI);
    case 'edi':
      return emu.reg(Register.EDI);
    case 'eip':
      return emu.eip;
  }
}

function setNamedRegister(emu: Emulator, reg: NamedRegister, value: number) {
  switch (reg) {
    case 'eax':
      return emu.set_reg(Register.EAX, value);
    case 'ecx':
      return emu.set_reg(Register.ECX, value);
    case 'edx':
      return emu.set_reg(Register.EDX, value);
    case 'ebx':
      return emu.set_reg(Register.EBX, value);
    case 'esp':
      return emu.set_reg(Register.ESP, value);
    case 'ebp':
      return emu.set_reg(Register.EBP, value);
    case 'esi':
      return emu.set_reg(Register.ESI, value);
    case 'edi':
      return emu.set_reg(Register.EDI, value);
    case 'eip':
      emu.eip = value;
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
    emu: Emulator;
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
    setNamedRegister(this.props.emu, this.props.reg, value);
    this.forceUpdate();
  };

  render() {
    const value = getNamedRegister(this.props.emu, this.props.reg);
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
    emu: Emulator;
  }
}
export class RegistersComponent extends preact.Component<RegistersComponent.Props> {
  render() {
    const regs = this.props.emu.regs();
    const st = regs.st;
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

        cs&nbsp;<Number digits={4} {...this.props}>{regs.cs}</Number>{' '}
        fs&nbsp;<Number digits={4} {...this.props}>{regs.fs}</Number>
        <br />
        ds&nbsp;<Number digits={4} {...this.props}>{regs.ds}</Number>{' '}
        gs&nbsp;<Number digits={4} {...this.props}>{regs.gs}</Number>
        <br />
        es&nbsp;<Number digits={4} {...this.props}>{regs.es}</Number>{' '}
        ss&nbsp;<Number digits={4} {...this.props}>{regs.ss}</Number>
        <br />
        <br />

        flags&nbsp;{hex(regs.flags)} {regs.flags_str}
        <br />
        {st.length > 0
          ? (
            <div>
              fpu<br />
              {Array.from(regs.st).map(n => (
                <span>
                  {n.toFixed(6)}
                  <br />
                </span>
              ))}
            </div>
          )
          : null}
      </div>
    );
  }
}
