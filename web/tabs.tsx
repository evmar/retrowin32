import * as preact from 'preact';
import { Fragment, h } from 'preact';

namespace Tabs {
  export interface Props {
    style: preact.JSX.CSSProperties;
    tabs: { [name: string]: preact.ComponentChild };
    selected: string;
    switchTab: (name: string) => void;
  }
}
export class Tabs extends preact.Component<Tabs.Props> {
  constructor(props: Tabs.Props) {
    super(props);
    this.state = { cur: Object.keys(props.tabs)[0] };
  }

  render() {
    const tabs = this.props.tabs;
    return (
      <section class='panel' style={this.props.style}>
        <div class='tabs-strip'>
          {Object.keys(tabs).map((name) => {
            let button = <span class='clicky' onClick={() => this.props.switchTab(name)}>{name}</span>;
            if (name === this.props.selected) {
              button = <b>{button}</b>;
            }
            return <>&nbsp;|&nbsp;{button}</>;
          })}
        </div>
        {tabs[this.props.selected]}
      </section>
    );
  }
}
