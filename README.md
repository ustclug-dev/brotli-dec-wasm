# brotli-dec-wasm

[![npm](https://img.shields.io/npm/v/brotli-dec-wasm)][npm:brotli-dec-wasm]
[![npm unpacked size](https://img.shields.io/endpoint?url=https%3A%2F%2Fnpm-unpackedsize-badge.myl.workers.dev%2Fbrotli-dec-wasm&color=blue)][npm:brotli-dec-wasm]

[npm:brotli-dec-wasm]: https://npmjs.com/package/brotli-dec-wasm

[Brotli][google/brotli] decompressor for browsers and web workers with WASM, which still has a small size (about
300KB)

[google/brotli]: https://github.com/google/brotli

If you are looking for a compressor, see [Alternatives](#alternatives)

## Features

- [Stream][brotli_doc_stream] support

[brotli_doc_stream]: https://brotli.org/decode.html#a234

## Usage

Starting from v2.0.0, this package can be used as an exact drop-in replacement of [brotli-wasm].
You can simply switch between `import brotli from 'brotli-wasm'` and `import brotli from 'brotli-dec-wasm'`.

More detailed usage can be found in [brotli-wasm] _Usage_ section.
Examples are also available in the unit tests in [brotli-wasm:test/brotli.spec.ts] and example projects in [brotli-wasm:example].
Especially, a [`TransformStream`] example is available in [brotli-wasm:example/web-next-transformstream/app/utils.ts].

[brotli-wasm:test/brotli.spec.ts]: https://github.com/httptoolkit/brotli-wasm/blob/main/test/brotli.spec.ts
[brotli-wasm:example]: https://github.com/httptoolkit/brotli-wasm/blob/main/example
[`TransformStream`]: https://developer.mozilla.org/en-US/docs/Web/API/TransformStream
[brotli-wasm:example/web-next-transformstream/app/utils.ts]: https://github.com/httptoolkit/brotli-wasm/blob/main/example/web-next-transformstream/app/utils.ts

To help colaborating with various bundlers (e.g., webpack, esbuild), we addtionally provide an `asset` entry and export the WASM binary file directly.
The documentation for the `asset` entry is available at [doc/asset.md], including how to either use the `asset` entry or access the WASM binary file directly.

[doc/asset.md]: doc/asset.md

## Problems

### Broken in webpack 5

> BREAKING CHANGE: Since webpack 5 WebAssembly is not enabled by default and flagged as experimental feature.
> You need to enable one of the WebAssembly experiments via 'experiments.asyncWebAssembly: true' (based on async modules) or 'experiments.syncWebAssembly: true' (like webpack 4, deprecated).

Set `experiments.syncWebAssembly: true` for old code, `experiments.asyncWebAssembly: true` for new code.

## Implementation

The code is quite simple, which is just a wrapper of the crate [brotli-decompressor] (other than crate [brotli], though brotli depends on brotli-decompressor)

[brotli-decompressor]: https://crates.io/crates/brotli-decompressor
[brotli]: https://crates.io/crates/brotli

Build configuration such as `opt-level = "s"`, are fine-tuned with manual tests, to make the bundle as small as possible

## Maintenance

The package is at least used by myself in my blog [mylmoe], which provides [a page to (de)compress Brotli online]

[mylmoe]: https://github.com/myl7/mylmoe
[a page to (de)compress Brotli online]: https://myl.moe/utils/brotli

## Alternatives

- [brotli-wasm]: A reliable compressor and decompressor for Brotli, supporting node & browsers via wasm. If you need a compressor, use it. Actively maintained by an organization.

More alternatives are available in [brotli-wasm] _Alternatives_ section

One surprising thing is, in `js` folder of the offical [google/brotli] repository, there is a pure JavaScript decompressor implementation, which is even a little smaller than this package in size. However, it is not published on NPM. I do not know the reason and since that, I do not suggest using it.

## Security

At least **>= v1.3.3**

- < v1.3.3: Rust dependency wee_alloc is unmaintained and has open serious issues. Use version >= 1.3.3 to replace it with the default Rust allocator on wasm32 targets.

## License

Copyright (C) myl7

SPDX-License-Identifier: MIT OR Apache-2.0

[brotli-wasm]: https://github.com/httptoolkit/brotli-wasm
