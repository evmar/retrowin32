import * as preact from 'preact';
import { Fragment, h } from 'preact';

namespace Tabs {
  export interface Props {
    style: preact.JSX.CSSProperties;
    tabs: { [name: string]: preact.ComponentChild };
  }
  export interface State {
    cur: string;
  }
}
export class Tabs extends preact.Component<Tabs.Props, Tabs.State> {
  state: Tabs.State = { cur: '' };
  render() {
    const tabs = this.props.tabs;
    return (
      <div style={this.props.style}>
        <div class='tabs-strip'>
          {Object.keys(tabs).map((name) => {
            let button = <span class='clicky' onClick={() => this.setState({ cur: name })}>{name}</span>;
            if (name === this.state.cur) {
              button = <b>{button}</b>;
            }
            return <>{button}&nbsp;</>;
          })}
        </div>
        {tabs[this.state.cur]}
      </div>
    );
  }
}
