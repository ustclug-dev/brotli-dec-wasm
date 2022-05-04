# brotli-dec-wasm

[Brotli](https://github.com/google/brotli) decompressor for browsers and web workers with WASM, but still having a small size (about 300KB)

If you are looking for a compressor, see [Alternatives](#alternatives)

## Features

- [Streaming](https://brotli.org/decode.html#a234) support

## Problems

### Broken in webpack 5

> BREAKING CHANGE: Since webpack 5 WebAssembly is not enabled by default and flagged as experimental feature.
> You need to enable one of the WebAssembly experiments via 'experiments.asyncWebAssembly: true' (based on async modules) or 'experiments.syncWebAssembly: true' (like webpack 4, deprecated).

Set `experiments.syncWebAssembly: true` or `experiments.asyncWebAssembly: true`.
An example of `experiments.asyncWebAssembly: true` is available as a test in [`tests/webpack5`](tests/webpack5).

## Implementation

The code is quite simple, which is just a wrapper of DropBox [`brotli-decompressor`](https://crates.io/crates/brotli-decompressor) crate (other than Cloudflare [`brotli`](https://crates.io/crates/brotli) crate, though `brotli` depends on `brotli-decompressor`)

Build configuration such as `opt-level = "s"` and no `lto`, are fine-tuned with manual tests, to make the bundle as small as possible

## Maintenance

The package is at least used by myself in my blog [`mylmoe`](https://github.com/myl7/mylmoe), which provides [a page to (de)compress Brotli online](https://myl.moe/utils/brotli)

## Alternatives

- [brotli-wasm](https://github.com/httptoolkit/brotli-wasm): A reliable compressor and decompressor for Brotli, supporting node & browsers via wasm. If you need a compressor, use it. Actively maintained by an organization.

More alternatives are available in [brotli-wasm](https://github.com/httptoolkit/brotli-wasm) _Alternatives_ section

One surprising thing is, in `js` folder of the offical [google/brotli](https://github.com/google/brotli) repository, there is a pure JavaScript decompressor implementation, which is even a little smaller than this package in size. However, it is not published on NPM. I can not imagine the reason and since that, I do not suggest you to use it.

## License

SPDX-License-Identifier: MIT

Unless otherwise explicitly stated
