import type * as BrotliDecWasm from './pkg/index.d.ts'

export type * from './pkg/index.d.ts'

declare const BrotliDecWasmTypePromise: Promise<typeof BrotliDecWasm>
export default BrotliDecWasmTypePromise
