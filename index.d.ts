export { brotliDec, BrotliDecStreamResultCode } from './pkg'
import { BrotliDecStream as _BrotliDecStream } from './pkg'

export class BrotliDecStream extends _BrotliDecStream {
  dec(input: Uint8Array, output_size: number): { code: number; output: Uint8Array }
}
