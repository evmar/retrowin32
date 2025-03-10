import * as preact from 'preact';
import * as emulator from '../emulator';
import * as wasm from '../glue/pkg/glue';
import { hex } from './util';

namespace TraceComponent {
  export interface Props {
    trace: emulator.Trace[];
  }
  export interface State {
    row: number;
  }
}
export class TraceComponent extends preact.Component<TraceComponent.Props, TraceComponent.State> {
  constructor() {
    super();
    this.state = { row: 0 };
  }

  onWheel = (ev: WheelEvent) => {
    const ofs = Math.round(ev.deltaY);
    const row = Math.max(0, Math.min(this.props.trace.length - 20, this.state.row + ofs));
    this.setState({ row });
  };

  render() {
    const { trace } = this.props;
    const rows = [];
    for (let i = this.state.row; i < Math.min(trace.length, this.state.row + 20); i++) {
      const { context, msg } = trace[i];
      rows.push(
        <tr>
          <td style={{ textAlign: 'right' }}>{context}</td>
          <td>{msg}</td>
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
