import * as preact from 'preact';
import { Fragment, h } from 'preact';

namespace Tabs {
  export interface Props {
  }
  export interface State {
    cur: number;
  }
}
export class Tabs extends preact.Component<Tabs.Props, Tabs.State> {
  state: Tabs.State = { cur: 0 };
  render() {
    const children = this.props.children as preact.ComponentChild[];
    const tabs: Array<[string, preact.ComponentChild]> = [];
    for (let i = 0; i < children.length; i += 2) {
      tabs.push([children[i] as string, children[i + 1]]);
    }
    return (
      <div>
        <div class='tabs-strip'>
          {tabs.map(([name, _], i) => {
            let button = <span class='clicky' onClick={() => this.setState({ cur: i })}>{name}</span>;
            if (i === this.state.cur) {
              button = <b>{button}</b>;
            }
            return <>{button}&nbsp;</>;
          })}
        </div>
        {tabs[this.state.cur][1]}
      </div>
    );
  }
}
