# brotli-dec-wasm

[Brotli](https://github.com/google/brotli) decompressor for browsers and web workers with WASM, but still having a small size (~300K)

## Problems

### Broken in webpack 5

> BREAKING CHANGE: Since webpack 5 WebAssembly is not enabled by default and flagged as experimental feature.
> You need to enable one of the WebAssembly experiments via 'experiments.asyncWebAssembly: true' (based on async modules) or 'experiments.syncWebAssembly: true' (like webpack 4, deprecated).

Set `experiments.syncWebAssembly: true`

## License

MIT
