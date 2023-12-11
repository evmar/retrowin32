import * as preact from 'preact';
import { h } from 'preact';
import { Labels } from './labels';
import { Number } from './memory';

export interface Breakpoint {
  addr: number;
  disabled?: boolean;
  oneShot?: boolean;
}

namespace BreakpointsComponent {
  export interface Props extends Number.Interactions {
    breakpoints: Breakpoint[];
    labels: Labels;
    highlight: number;
    toggle: (addr: number) => void;
    add: (text: string) => boolean;
    remove: (addr: number) => void;
  }
}

export class BreakpointsComponent extends preact.Component<BreakpointsComponent.Props> {
  render() {
    const rows = [];
    for (const bp of this.props.breakpoints) {
      const className = bp.addr === this.props.highlight ? 'highlight' : undefined;
      const label = this.props.labels.get(bp.addr);
      rows.push(
        <div className={className} style={{ display: 'flex', alignItems: 'center', gap: '0.5ex' }}>
          <input type='checkbox' checked={!bp.disabled} onChange={() => this.props.toggle(bp.addr)} />
          <div>
            <code>
              <Number digits={8} {...this.props}>{bp.addr}</Number>
            </code>
          </div>
          {bp.oneShot ? '[once]' : null}
          {label
            ? (
              <div>
                (<code>{label}</code>)
              </div>
            )
            : null}
          <button class='x' onClick={() => this.props.remove(bp.addr)}>x</button>
        </div>,
      );
    }
    return (
      <section>
        {rows}
        <AddComponent onAccept={(text) => this.props.add(text)} />
      </section>
    );
  }
}

namespace AddComponent {
  export interface Props {
    onAccept(text: string): boolean;
  }
  export interface State {
    text: string;
  }
}
class AddComponent extends preact.Component<AddComponent.Props, AddComponent.State> {
  onInput = (ev: Event) => {
    const text = (ev.target! as HTMLInputElement).value;
    this.setState({ text });
  };
  onSubmit = (ev: Event) => {
    ev.preventDefault();
    if (this.props.onAccept(this.state.text)) {
      this.setState({ text: '' });
    }
  };
  render() {
    return (
      <form onSubmit={this.onSubmit}>
        add: <input value={this.state.text} onInput={this.onInput} />
      </form>
    );
  }
}
