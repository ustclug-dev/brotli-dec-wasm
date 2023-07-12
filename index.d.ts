import type * as BrotliDecWasm from './pkg/index'

export type * from './pkg/index'

export type BrotliDecWasmType = typeof BrotliDecWasm & {
  DecompressStream: typeof BrotliDecWasm.BrotliDecStream
}

declare const BrotliDecWasmTypePromise: Promise<BrotliDecWasmType>
export default BrotliDecWasmTypePromise
