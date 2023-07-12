import init, * as brotliDecWasm from './pkg'

export * from './pkg'

export default init().then(() => {
  let mod = brotliDecWasm
  brotliDecWasm['DecompressStream'] = brotliDecWasm.BrotliDecStream
  return mod
})
