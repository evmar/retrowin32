type GenFn = (...args: any[]) => any;
type Method<T> = T extends GenFn ? T : never;

const MSG_SYNC_RETURN = ':MethodChannelSyncReturn';

interface Message {
  sync?: boolean;
  fn: string;
  params: {}[];
}

export class MethodChannel<T> {
  pendingCall?: (data: any) => void;

  private local!: {};
  constructor(private remote: Worker | Window) {
    this.remote.onmessage = (e) => this.onMessage(e);
  }
  setLocal(local: {}) {
    this.local = local;
  }

  post<K extends keyof T>(fn: K, params: Parameters<Method<T[K]>>) {
    const transfer = [];
    for (let i = 0; i < params.length; i++) {
      const arg = params[i];
      // Avoid transferring the entire contents of large Uint8Arrays.
      if (arg instanceof Uint8Array) {
        if (arg.buffer.byteLength > 1000 && arg.buffer.byteLength > arg.byteLength) {
          const slice = arg.slice();
          params[i] = slice;
          transfer.push(slice.buffer);
        }
      } else if (arg instanceof ImageBitmap) {
        transfer.push(arg);
      }
    }
    this.remote.postMessage({ fn, params }, { transfer });
  }

  async call<K extends keyof T>(fn: K, args: Parameters<Method<T[K]>>): Promise<ReturnType<Method<T[K]>>> {
    if (this.pendingCall) throw new Error('nested sync calls');
    this.post(fn, args);
    return new Promise((res) => {
      this.pendingCall = (data) => {
        this.pendingCall = undefined;
        res(data as any);
      }
    });
  }

  private onMessage(e: MessageEvent<Message>) {
    const msg = e.data;
    if (msg.fn === MSG_SYNC_RETURN) {
      if (!this.pendingCall) throw new Error('unexpected sync return');
      this.pendingCall(msg.params);
      return;
    }
    (this.local as any)[msg.fn](...msg.params);
    if (msg.sync) {
      this.remote.postMessage({ fn: MSG_SYNC_RETURN, params: msg.params });
    }
  }

  asProxy(): T {
    return new Proxy(this, {
      get(target, prop, _receiver) {
        return (...args: {}[]) => { (target as any).post(prop, args) };
      }
    }) as unknown as T;
  }
}
