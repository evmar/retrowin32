import * as preact from 'preact';
import { h } from 'preact';
import { Number } from './memory';

export interface Breakpoint {
  addr: number;
  temporary: boolean;
}

export type Breakpoints = Map<number, Breakpoint>;

namespace BreakpointsComponent {
  export interface Props extends Number.Interactions {
    breakpoints: Breakpoints;
    highlight: number;
  }
}

export class BreakpointsComponent extends preact.Component<BreakpointsComponent.Props> {
  render() {
    const rows = [];
    for (const bp of this.props.breakpoints.values()) {
      const className = bp.addr === this.props.highlight ? 'highlight' : undefined;
      rows.push(
        <div className={className}>
          <Number digits={8} {...this.props}>{bp.addr}</Number>
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
