import * as preact from 'preact';
import * as wasm from '../glue/pkg/glue';
import { hex } from './util';

namespace CPUs {
  export interface Props {
    cpus: wasm.CPU[];
  }
  export interface State {
  }
}
export class CPUs extends preact.Component<CPUs.Props, CPUs.State> {
  render() {
    const rows = this.props.cpus.map((cpu) => {
      return (
        <tr>
          <td>{hex(cpu.eip, 8)}</td>
          <td>{cpu.state()}</td>
        </tr>
      );
    });
    return (
      <div>
        <table>
          <tr>
            <th>ip</th>
            <th>state</th>
          </tr>
          {rows}
        </table>
      </div>
    );
  }
}
