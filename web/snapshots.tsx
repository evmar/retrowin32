import * as preact from 'preact';
import { h } from 'preact';

/** Entry in the 'snap' object store, metadata about a snapshot. */
interface Snapshot {
  /** Key for this entry as well as the key in the 'image' object store. */
  idbKey: IDBValidKey;
  /** Byte size of this entry's image. */
  size: number;
}

/** Run an IDB request to completion, waiting for its result. */
function idbRequest<T>(req: IDBRequest<T>): Promise<T> {
  return new Promise((res, rej) => {
    req.onerror = (err: any) => {
      rej(err);
    };
    req.onsuccess = function() {
      res(this.result);
    };
  });
}

/** Run an IDB transation to completion. */
function finishTransaction(t: IDBTransaction): Promise<void> {
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
  interface StateLoading {
    state: undefined;
    data: undefined;
  }
  interface StateLoaded {
    state: 'ok';
    data: Snapshots.DBProps;
  }
  interface StateError {
    state: 'error';
    data: string;
  }
  export type State = StateLoading | StateLoaded | StateError;
}
export class SnapshotsComponent extends preact.Component<SnapshotsComponent.Props, SnapshotsComponent.State> {
  private async load(): Promise<Snapshots.DBProps> {
    const req = indexedDB.open('retrowin32');
    req.onupgradeneeded = () => {
      const db = req.result;
      db.createObjectStore('image', { autoIncrement: true });
      db.createObjectStore('snap', { keyPath: 'idbKey' });
    };

    const db = await idbRequest(req);
    const snaps: Snapshot[] = await idbRequest(db.transaction('snap').objectStore('snap').getAll());

    const st: Snapshots.DBProps = { db, snaps };
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
    snaps: Snapshot[];
  }
}
class Snapshots extends preact.Component<Snapshots.Props> {
  private async save() {
    const image = this.props.take();
    const t = this.props.db.transaction(['snap', 'image'], 'readwrite');
    const idbKey = await idbRequest(t.objectStore('image').add(image));
    const snap: Snapshot = { idbKey, size: image.length };
    await idbRequest(t.objectStore('snap').add(snap));
    await finishTransaction(t);
    this.props.reload();
  }
  private async load(key: IDBValidKey) {
    const t = this.props.db.transaction(['image'], 'readonly');
    const image = await idbRequest(t.objectStore('image').get(key));
    await finishTransaction(t);
    this.props.load(image);
  }
  private async clear() {
    const t = this.props.db.transaction(['snap', 'image'], 'readwrite');
    t.objectStore('snap').clear();
    t.objectStore('image').clear();
    await finishTransaction(t);
    this.props.reload();
  }

  render() {
    let snaps = [];
    if (this.props.snaps.length > 0) {
      for (const snap of this.props.snaps) {
        snaps.push(
          <div>
            {snap.size} bytes <button onClick={() => this.load(snap.idbKey)}>load</button>
          </div>,
        );
      }
      snaps.push(
        <p>
          <button onClick={() => this.clear()}>clear snapshots</button>
        </p>,
      );
    }
    return (
      <section>
        <p>
          <button onClick={() => this.save()}>save snapshot</button>
        </p>
        {snaps}
      </section>
    );
  }
}
