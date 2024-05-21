import * as preact from 'preact';
import { Fragment, h } from 'preact';

namespace Tabs {
  export interface Props {
    style: preact.JSX.CSSProperties;
    tabs: { [name: string]: () => preact.ComponentChild };
    selected: string;
    switchTab: (name: string) => void;
  }
}
export class Tabs extends preact.Component<Tabs.Props> {
  render() {
    const { style, tabs, selected, switchTab } = this.props;
    const content = tabs[selected]();
    return (
      <section class='panel' style={style}>
        <div class='tabs-strip'>
          |
          {Object.keys(tabs).map((name) => {
            let button = <span class='clicky' onClick={() => switchTab(name)}>{name}</span>;
            if (name === selected) {
              button = <b>{button}</b>;
            }
            return <>&nbsp;{button}&nbsp;|</>;
          })}
        </div>
        {content}
      </section>
    );
  }
}
