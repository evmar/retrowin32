/** Creates an object that makes any obj.foo() call into a postMessage onto the target. */
export function messageProxy(target: Worker | Window): object {
  return new Proxy({}, {
    get(_target, prop, _receiver) {
      return (...args: {}[]) => {
        const transfers = [];
        for (let i = 0; i < args.length; i++) {
          const arg = args[i];
          // Avoid transferring the entire contents of large Uint8Arrays.
          if (arg instanceof Uint8Array) {
            if (arg.buffer.byteLength > 1000 && arg.buffer.byteLength > arg.byteLength) {
              const slice = arg.slice();
              args[i] = slice;
              transfers.push(slice.buffer);
            }
          }
        }
        target.postMessage([prop, args], { transfer: transfers });
      };
    },
  });
}

/** Sets the onmessage handler to receive postMessage calls from a proxy and forward them to the handler. */
export function setOnMessage(target: Worker | Window, handler: object) {
  target.onmessage = function(e: MessageEvent<[string, {}[]]>) {
    const [fn, params] = e.data;
    (handler as any)[fn](...params);
  };
}
