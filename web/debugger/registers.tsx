import * as preact from 'preact';
import { Registers } from '../glue/pkg/glue';
import { MemoryView, Number } from './memory';
import { hex } from './util';

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
    this.props.set(this.getValue());
    return false;
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
          <input type='text' size={8} defaultValue={this.state.text} onInput={this.onInput} />
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

/// Displays one
///    eax 00123456
/// row, with a clickable value that pops up an editing panel.
namespace RegisterComponent {
  export interface Props extends MemoryView {
    regs: Registers;
    reg: string;
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

  set = (value: number) => {
    const regs = this.props.regs as unknown as Record<string, number>;
    // TODO: register setting
    // regs[this.props.reg] = value;
    // this.forceUpdate();
  };

  render() {
    const value = (this.props.regs as unknown as Record<string, number>)[this.props.reg];
    return (
      <div style={{ display: 'flex', alignItems: 'baseline', gap: '1ex' }}>
        <div style={{ flex: 1 }}>{this.props.reg}</div>
        <button onClick={this.pop}>
          <code>{hex(value, 8)}</code>
        </button>
        {this.state.pop
          && (
            <div ref={this.ref} popover='auto' onToggle={this.unPop}>
              <RegisterPanel value={value} showMemory={this.props.showMemory} set={() => {}} />
            </div>
          )}
      </div>
    );
  }
}

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
