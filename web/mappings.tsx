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
      return <div>{hex(mapping.addr, 8)}+{hex(mapping.size)} {mapping.desc}</div>;
    });
    return (
      <section>
        <code>{rows}</code>
      </section>
    );
  }
}
