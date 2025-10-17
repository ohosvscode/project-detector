export interface Signal<T = unknown> {
  /** Get current signal value. */
  (): T
  /** Set signal value. */
  (value: T): void
}

export interface DisposableSignal<T = unknown, R = any> extends Signal<T> {
  dispose(): R
}

export namespace DisposableSignal {
  export function fromSignal<T, R>(signal: Signal<T>, disposable?: () => R): DisposableSignal<T, R> {
    // eslint-disable-next-line ts/ban-ts-comment
    // @ts-expect-error
    signal.dispose = disposable ?? (() => {})
    return signal as DisposableSignal<T>
  }
}
