import * as preact from 'preact';
import { h } from 'preact';
import { Number } from './memory';

export interface Breakpoint {
  addr: number;
  disabled?: boolean;
  oneShot?: boolean;
}

namespace BreakpointsComponent {
  export interface Props extends Number.Interactions {
    breakpoints: Breakpoint[];
    highlight: number;
    toggle: (addr: number) => void;
  }
}

export class BreakpointsComponent extends preact.Component<BreakpointsComponent.Props> {
  render() {
    const rows = [];
    for (const bp of this.props.breakpoints) {
      const className = bp.addr === this.props.highlight ? 'highlight' : undefined;
      rows.push(
        <div className={className} style={{ display: 'flex', alignItems: 'center' }}>
          <input type='checkbox' checked={!bp.disabled} onChange={() => this.props.toggle(bp.addr)} />
          <div>
            <Number digits={8} {...this.props}>{bp.addr}</Number>
          </div>
          {bp.oneShot ? ' [once]' : null}
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
