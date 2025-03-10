import * as preact from 'preact';
import * as emulator from '../emulator';
import * as wasm from '../glue/pkg/glue';
import { hex } from './util';

const SHOW_ROWS = 10;

namespace TraceComponent {
  export interface Props {
    trace: emulator.Trace[];
  }
  export interface State {
    row: number;
    /** When true, always show most recent row. */
    followTail: boolean;
  }
}
export class TraceComponent extends preact.Component<TraceComponent.Props, TraceComponent.State> {
  constructor() {
    super();
    this.state = { row: 0, followTail: true };
  }

  static getDerivedStateFromProps(props: TraceComponent.Props, state: TraceComponent.State) {
    if (state.followTail) {
      return { row: Math.max(state.row, props.trace.length - SHOW_ROWS) };
    }
    return null;
  }

  onWheel = (ev: WheelEvent) => {
    const ofs = Math.round(ev.deltaY);
    const row = Math.max(0, Math.min(this.props.trace.length - SHOW_ROWS, this.state.row + ofs));
    this.setState({ row, followTail: row + SHOW_ROWS >= this.props.trace.length });
  };

  jumpToEnd = () => {
    this.setState({ row: Math.max(0, this.props.trace.length - SHOW_ROWS) });
  };

  render() {
    const { trace } = this.props;
    const rows = [];
    let i;
    for (i = this.state.row; i < Math.min(trace.length, this.state.row + SHOW_ROWS); i++) {
      const { context, msg } = trace[i];
      rows.push(
        <tr>
          <td style={{ textAlign: 'right' }}>{context}</td>
          <td>{msg}</td>
        </tr>,
      );
    }
    if (i < trace.length) {
      rows.push(
        <tr>
          <td />
          <td>
            <button onClick={this.jumpToEnd}>jump to end</button>
          </td>
        </tr>,
      );
    }

    return (
      <div>
        <table style={{ width: '100%' }}>
          <tr>
            <th style={{ width: '15ex', textAlign: 'right' }}>context</th>
            <th align='left'>call</th>
          </tr>
          <tbody onWheel={this.onWheel}>
            {rows}
          </tbody>
        </table>
      </div>
    );
  }
}
