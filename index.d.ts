import type * as BrotliDecWasm from './pkg/index'

export type * from './pkg/index'

export type BrotliDecWasmType = typeof BrotliDecWasm

declare const BrotliDecWasmTypePromise: Promise<BrotliDecWasmType>
export default BrotliDecWasmTypePromise
