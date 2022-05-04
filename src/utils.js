export class BrotliDecStreamResult {
  /**
   * @param {number} code
   * @param {Uint8Array} output
   */
  constructor(code, output) {
    this._code = code
    this._output = output
  }

  get code() {
    return this._code
  }

  get output() {
    return this._output
  }
}
