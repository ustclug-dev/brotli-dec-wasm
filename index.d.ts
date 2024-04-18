import * as BrotliDecWasm from './pkg/brotli_dec_wasm.d.ts'

declare const promisedValue: Promise<typeof BrotliDecWasm>
export default promisedValue

export type BrotliDecWasmType = typeof BrotliDecWasm
