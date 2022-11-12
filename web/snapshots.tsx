import * as preact from 'preact';
import { Fragment, h } from 'preact';

function wrap<T>(req: IDBRequest<T>): Promise<T> {
  return new Promise((res, rej) => {
    req.onerror = (err: any) => {
      rej(err);
    };
    req.onsuccess = function() {
      res(this.result);
    };
  });
}

function runTransaction<T>(t: IDBTransaction): Promise<void> {
  return new Promise((res, rej) => {
    t.onerror = (err: any) => {
      rej(err);
    };
    t.oncomplete = function() {
      res();
    };
  });
}

namespace SnapshotsComponent {
  export interface Props {
    take: () => Uint8Array;
    load: (snap: Uint8Array) => void;
  }
  export type State = { state: undefined; data: undefined } | { state: 'ok'; data: Snapshots.DBProps } | {
    state: 'error';
    data: string;
  };
}

export class SnapshotsComponent extends preact.Component<SnapshotsComponent.Props, SnapshotsComponent.State> {
  private async load(): Promise<Snapshots.DBProps> {
    const req = indexedDB.open('retrowin32');
    req.onupgradeneeded = () => {
      console.log('upg');
      req.result.createObjectStore('snap', { autoIncrement: true });
    };

    const db = await wrap(req);
    const snaps = await wrap(db.transaction('snap', 'readonly').objectStore('snap').getAll());

    const st = { db, snaps };
    this.setState({ state: 'ok', data: st });
    db.onerror = (error) => {
      this.setState({
        state: 'error',
        data: error.toString(),
      });
    };

    return st;
  }

  componentDidMount() {
    this.load();
  }

  render() {
    if (this.state.state === 'ok') {
      return <Snapshots {...this.props} {...this.state.data} reload={() => this.load()} />;
    } else if (this.state.state === 'error') {
      return <section>error: {this.state.data}</section>;
    } else {
      return <section>loading</section>;
    }
  }
}

namespace Snapshots {
  export type Props = SnapshotsComponent.Props & DBProps & { reload: () => void };
  export interface DBProps {
    db: IDBDatabase;
    snaps: Uint8Array[];
  }
}
class Snapshots extends preact.Component<Snapshots.Props> {
  private async trans(f: (t: IDBTransaction) => Promise<void>): Promise<void> {
    const t = this.props.db.transaction(['snap'], 'readwrite');
    return new Promise((res, rej) => {
      f(t);
      t.oncomplete = () => res();
      t.onerror = (err) => rej(err);
    });
  }

  private async save() {
    const snap = this.props.take();
    await this.trans(async (t) => {
      t.objectStore('snap').add(snap);
    });
    this.props.reload();
  }
  private load() {
    this.props.load(this.props.snaps[0]);
  }
  private async clear() {
    await this.trans(async (t) => {
      t.objectStore('snap').clear();
    });
    this.props.reload();
  }

  render() {
    return (
      <section>
        <p>
          <button onClick={() => this.save()}>save snapshot</button>
        </p>
        {this.props.snaps.length > 0
          ? (
            <>
              <div>{this.props.snaps.map(snap => <span>{snap.length} bytes</span>)}</div>
              <p>
                <button onClick={() => this.load()}>load snapshot</button>
              </p>
              <p>
                <button onClick={() => this.clear()}>clear snapshot</button>
              </p>
            </>
          )
          : null}
      </section>
    );
  }
}
