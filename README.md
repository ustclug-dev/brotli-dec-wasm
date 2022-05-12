# brotli-dec-wasm

[Brotli](https://github.com/google/brotli) decompressor for browsers and web workers with WASM, but still having a small size (about 300KB)

If you are looking for a compressor, see [Alternatives](#alternatives)

## Features

- [Streaming](https://brotli.org/decode.html#a234) support

## Quick start

```ts
// One-shot decompressing
const brotli = await import('brotli-dec-wasm')
const output = brotli.brotliDec(input)

// Streaming decompressing
const brotli = await import('brotli-dec-wasm')
const stream = new brotli.BrotliDecStream()
// Set max output buffer size as 1024
const output1 = stream.dec(input1, 1024)
const result = stream.result()
// If result = NeedsMoreInput = 1, put more input into the stream to get next output
if (result == brotli.BrotliDecStreamResult.NeedsMoreInput) {
  const output2 = stream.dec(input2, 1024)
}
// If result = NeedsMoreOutput = 2, slice the input and take another output buffer out
if (result == brotli.BrotliDecStreamResult.NeedsMoreOutput) {
  const input1r = input1.slice(steam.lastInputOffset())
  const output2 = stream.dec(input1r, 1024)
}
// If result = ResultSuccess = 3, decompressing succeeded and finished. No more input is required.
if (result == brotli.BrotliDecStreamResult.ResultSuccess) {
  console.log(output1)
}
// If result < 0, an error occurs. You may refer BrotliDecStreamErrCode to lookup the error code.
if result < 0 {
  console.error('Brotli decompressing failed')
}
```

## Problems

### Broken in webpack 5

> BREAKING CHANGE: Since webpack 5 WebAssembly is not enabled by default and flagged as experimental feature.
> You need to enable one of the WebAssembly experiments via 'experiments.asyncWebAssembly: true' (based on async modules) or 'experiments.syncWebAssembly: true' (like webpack 4, deprecated).

Set `experiments.syncWebAssembly: true` for old code.
As for `experiments.asyncWebAssembly: true`, an example is available as a test in [`tests/webpack5`](tests/webpack5).

## Implementation

The code is quite simple, which is just a wrapper of [`brotli-decompressor`](https://crates.io/crates/brotli-decompressor) crate (other than [`brotli`](https://crates.io/crates/brotli) crate, though `brotli` depends on `brotli-decompressor`)

Build configuration such as `opt-level = "s"`, are fine-tuned with manual tests, to make the bundle as small as possible

## Maintenance

The package is at least used by myself in my blog [`mylmoe`](https://github.com/myl7/mylmoe), which provides [a page to (de)compress Brotli online](https://myl.moe/utils/brotli)

## Alternatives

- [brotli-wasm](https://github.com/httptoolkit/brotli-wasm): A reliable compressor and decompressor for Brotli, supporting node & browsers via wasm. If you need a compressor, use it. Actively maintained by an organization.

More alternatives are available in [brotli-wasm](https://github.com/httptoolkit/brotli-wasm) _Alternatives_ section

One surprising thing is, in `js` folder of the offical [google/brotli](https://github.com/google/brotli) repository, there is a pure JavaScript decompressor implementation, which is even a little smaller than this package in size. However, it is not published on NPM. I can not imagine the reason and since that, I do not suggest you to use it.

## License

SPDX-License-Identifier: MIT

Unless otherwise explicitly stated
