import * as preact from 'preact';
import * as wasm from '../glue/pkg/glue';
import { MemoryView, Number } from './memory';

namespace DirectDraw {
  export interface Props extends MemoryView {
    surfaces: wasm.SurfaceDebug[];
  }
  export interface State {
    hover?: wasm.SurfaceDebug;
  }
}
export class DirectDraw extends preact.Component<DirectDraw.Props, DirectDraw.State> {
  canvasContainer = (parent: HTMLElement | null) => {
    if (!parent) return;
    parent.appendChild(this.state.hover!.canvas);
  };

  render() {
    const rows = this.props.surfaces.map((surface) => {
      return (
        <tr
          onMouseEnter={() => this.setState({ hover: surface })}
          onMouseLeave={() => this.setState({ hover: undefined })}
        >
          <td style={{ textAlign: 'right' }}>
            <Number digits={8} {...this.props}>{surface.ptr}</Number>
          </td>
          <td style={{ padding: '0 1ex', textAlign: 'right' }}>
            {surface.width}x{surface.height}x{surface.bytes_per_pixel}
          </td>
          <td style={{ padding: '0 1ex', textAlign: 'right' }}>{surface.primary ? 'yes' : 'no'}</td>
          <td style={{ padding: '0 1ex', textAlign: 'right' }}>
            {surface.palette && <Number digits={8} {...this.props}>{surface.palette}</Number>}
          </td>
        </tr>
      );
    });
    return (
      <div style={{ flex: 1, minHeight: 0 }}>
        <table>
          <tr>
            <th>ptr</th>
            <th>format</th>
            <th>primary</th>
            <th>palette</th>
          </tr>
          {rows}
        </table>
        {this.state.hover && <div ref={this.canvasContainer} />}
      </div>
    );
  }
}
