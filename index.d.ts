import type * as BrotliDecWasm from './pkg/index.d.ts'
import type init from './pkg/index.d.ts'

export type * from './pkg/index.d.ts'
export { init }

declare const BrotliDecWasmTypePromise: Promise<typeof BrotliDecWasm>
export default BrotliDecWasmTypePromise
