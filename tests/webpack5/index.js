const input = new Uint8Array([
  27, 23, 0, 248, 141, 148, 169, 61, 94, 181, 142, 183, 224, 96, 98, 170, 98, 161, 150, 56, 160, 232, 44, 0,
])
const output = new Uint8Array([
  227, 130, 173, 227, 131, 169, 227, 130, 173, 227, 131, 169, 227, 131, 137, 227, 130, 173, 227, 131, 137, 227, 130,
  173,
])

import('brotli-dec-wasm').then(({ brotliDec, BrotliDecStream, BrotliDecStreamResult }) => {
  const res1 = brotliDec(input)
  console.log(res1)
  console.assert(res1.length === output.length)
  res1.forEach((v, i) => console.assert(v === output[i]))

  const dec = new BrotliDecStream()
  const res2 = dec.dec(input, 2048)
  const code = dec.result()
  console.log(res2)
  console.log(code)
  console.assert(code === BrotliDecStreamResult.ResultSuccess)
  res2.forEach((v, i) => console.assert(v === output[i]))
})
