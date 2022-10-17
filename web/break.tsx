import * as preact from 'preact';
import { h } from 'preact';
import { Number } from './memory';

export interface Breakpoint {
  disabled?: boolean;
  temporary: boolean;
}

export type Breakpoints = Map<number, Breakpoint>;

namespace BreakpointsComponent {
  export interface Props extends Number.Interactions {
    breakpoints: Breakpoints;
    highlight: number;
    toggle: (addr: number) => void;
  }
}

export class BreakpointsComponent extends preact.Component<BreakpointsComponent.Props> {
  render() {
    const rows = [];
    for (const [addr, bp] of this.props.breakpoints) {
      const className = addr === this.props.highlight ? 'highlight' : undefined;
      rows.push(
        <div className={className} style={{ display: 'flex', alignItems: 'center' }}>
          <input type='checkbox' checked={!bp.disabled} onChange={() => this.props.toggle(addr)} />
          <div>
            <Number digits={8} {...this.props}>{addr}</Number>
          </div>
        </div>,
      );
    }
    return (
      <section>
        {rows}
      </section>
    );
  }
}
