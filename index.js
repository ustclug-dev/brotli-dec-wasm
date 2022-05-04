import * as wasm from './pkg/brotli-dec-wasm_bg.wasm'
export { brotliDec, BrotliDecStreamResultCode } from './pkg/brotli-dec-wasm'
import { BrotliDecStream as _BrotliDecStream } from './pkg/brotli-dec-wasm'

export class BrotliDecStream extends _BrotliDecStream {
  /**
   * @typedef {Result}
   * @property {number} code
   * @property {Uint8Array} output
   */
  /**
   * @param {Uint8Array} input
   * @param {number} output_size
   * @returns {Result}
   */
  dec(input, output_size) {
    const res = super.dec(input, output_size)
    const code = res[res.length - 1]
    return { code, output: res.slice(0, res.length - 1) }
  }
}
