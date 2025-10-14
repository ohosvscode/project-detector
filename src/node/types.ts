export interface Signal<T> {
  /** Get current signal value. */
  (): T
  /** Set signal value. */
  (value: T): void
}

export interface DisposableSignal<T> extends Signal<T> {
  dispose(): any
}

export namespace DisposableSignal {
  export function create<T>(signal: Signal<T>, disposable?: () => any): DisposableSignal<T> {
    // eslint-disable-next-line ts/ban-ts-comment
    // @ts-expect-error
    signal.dispose = disposable ?? (() => {})
    return signal as DisposableSignal<T>
  }
}
