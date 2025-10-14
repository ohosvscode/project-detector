export interface Signal<T> {
  /** Get current signal value. */
  (): T
  /** Set signal value. */
  (value: T): void
}
