import * as preact from 'preact';
import * as wasm from '../glue/pkg/glue';
import { MemoryView, Number } from './memory';

namespace DirectDraw {
  export interface Props extends MemoryView {
    state: wasm.DirectDrawState;
  }
}
export class DirectDraw extends preact.Component<DirectDraw.Props> {
  render() {
    const rows = this.props.state.surfaces.map((surface) => {
      return (
        <tr>
          <td style={{ textAlign: 'right' }}>
            <Number digits={8} {...this.props}>{surface.ptr}</Number>
          </td>
          <td style={{ textAlign: 'right' }}>{surface.width}</td>
          <td style={{ textAlign: 'right' }}>{surface.height}</td>
          <td style={{ textAlign: 'right' }}>{surface.bytes_per_pixel}</td>
          <td style={{ textAlign: 'right' }}>{surface.primary ? 'yes' : 'no'}</td>
        </tr>
      );
    });
    return (
      <div style={{ flex: 1, minHeight: 0 }}>
        <table>
          <tr>
            <th>ptr</th>
            <th>width</th>
            <th>height</th>
            <th>bytes_per_pixel</th>
            <th>primary</th>
          </tr>
          {rows}
        </table>
      </div>
    );
  }
}
