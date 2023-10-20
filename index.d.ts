import type * as BrotliDecWasm from './pkg/index.d.ts'

export type * from './pkg/index.d.ts'

export type BrotliDecWasmType = typeof BrotliDecWasm

declare const BrotliDecWasmTypePromise: Promise<BrotliDecWasmType>
export default BrotliDecWasmTypePromise
