import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';
import * as wasm from './wasm/wasm';

namespace Mappings {
  export interface Props {
    mappings: wasm.Mapping[];
  }
}
export class Mappings extends preact.Component<Mappings.Props> {
  render() {
    const rows = this.props.mappings.map(mapping => {
      return (
        <tr>
          <td style={{ width: '10ch' }}>{hex(mapping.addr, 8)}</td>
          <td style={{ width: '8ch' }}>{hex(mapping.size)}</td>
          <td>{mapping.desc}</td>
        </tr>
      );
    });
    return (
      <section>
        <code>
          <table>
            <thead>
              <tr>
                <td>addr</td>
                <td>size</td>
                <td>desc</td>
              </tr>
            </thead>
            {rows}
          </table>
        </code>
      </section>
    );
  }
}
