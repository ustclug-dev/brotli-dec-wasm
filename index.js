import { BrotliDecStream } from './pkg'
import * as brotliDecWasm from './pkg'

export * from './pkg/index.js'

export const DecompressStream = BrotliDecStream

// The following bootstrapping code is to match `brotli-wasm`
export default async () => brotliDecWasm
