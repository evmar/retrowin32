import * as preact from 'preact';
import { Fragment, h } from 'preact';
import * as wasm from '../glue/pkg/glue';

namespace DirectDraw {
  export interface Props {
    state: wasm.DirectDrawState;
  }
}
export class DirectDraw extends preact.Component<DirectDraw.Props> {
  render() {
    const rows = this.props.state.surfaces.map((surface) => {
      return (
        <tr>
          <td>{surface.width}</td>
          <td>{surface.height}</td>
          <td>{surface.bytes_per_pixel}</td>
          <td>{surface.primary ? 'yes' : 'no'}</td>
        </tr>
      );
    });
    return (
      <div style={{ flex: 1, minHeight: 0 }}>
        <table>
          <tr>
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
