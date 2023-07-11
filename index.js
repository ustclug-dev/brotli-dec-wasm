import { BrotliDecStream } from './pkg/index.js'
import * as brotliDecWasm from './pkg/index.js'

export const DecompressStream = BrotliDecStream

export * from './pkg/index.js'

// The following bootstrapping code is to match `brotli-wasm`
export default async () => brotliDecWasm
