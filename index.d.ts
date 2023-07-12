import type * as BrotliDecWasm from './pkg'

export type * from './pkg'

export type BrotliDecWasmType = typeof BrotliDecWasm

export default Promise<BrotliDecWasmType>
