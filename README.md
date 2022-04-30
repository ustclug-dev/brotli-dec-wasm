# brotli-dec-wasm

[Brotli](https://github.com/google/brotli) decompressor for browsers and web workers with WASM, but still having a small size (~300K)

If you are looking for a compressor, see [Alternatives](#alternatives)

## Problems

### Broken in webpack 5

> BREAKING CHANGE: Since webpack 5 WebAssembly is not enabled by default and flagged as experimental feature.
> You need to enable one of the WebAssembly experiments via 'experiments.asyncWebAssembly: true' (based on async modules) or 'experiments.syncWebAssembly: true' (like webpack 4, deprecated).

Set `experiments.syncWebAssembly: true`

## Implementation

The code is quite simple, which is just a wrapper of Cloudflare [`brotli-decompressor`](https://crates.io/crates/brotli-decompressor) crate (other than the well-known [`brotli`](https://crates.io/crates/brotli) crate)

Build configuration such as `opt-level = "s"` and no `lto`, are fine-tuned with manual tests, to make the bundle as small as possible

## Maintenance

The package is at least used by myself in my blog [`mylmoe`](https://github.com/myl7/mylmoe), which provides [a page to (de)compress Brotli online](https://myl.moe/utils/brotli)

## Alternatives

- [brotli-wasm](https://github.com/httptoolkit/brotli-wasm): A reliable compressor and decompressor for Brotli, supporting node & browsers via wasm. If you need a compressor, use it. Actively maintained by an organization.

More alternatives are available in [brotli-wasm](https://github.com/httptoolkit/brotli-wasm) *Alternatives* section

One surprising thing is, in `js` folder of the offical [google/brotli](https://github.com/google/brotli) repository, there is a pure JavaScript decompressor implementation, which is even a little smaller than this package in size. However, it is not published on NPM. I can not imagine the reason and since that, I do not suggest you to use it.

## License

MIT
