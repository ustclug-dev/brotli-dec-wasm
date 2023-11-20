# Asset helper

This package exports an entry `brotli-dec-wasm/asset` to help you bundle it with bundlers like webpack

## Usage

An example for webpack is available in [example/webpack-asset]

[example/webpack-asset]: example/webpack-asset

Briefly speaking, your code should look like:

```js
import brotliDecWasmPromise from 'brotli-dec-wasm/asset'

const brotliDecWasm = await brotliDecWasmPromise
brotliDecWasm.decompress(uint8Array)
```

Additionally you need to configure the bundler so that `import wasm from './pkg/index_bg.wasm'` can be handled by importing the WASM file as bytes, a URL string, or any other one that can be passed to `WebAssembly.instantiate`.
For webpack, you can use [Asset Modules] with `asset/resource` (imported as a URL string).

[Asset Modules]: https://webpack.js.org/guides/asset-modules/

If you are not satisfied with the import and want to use, e.g., `resourceQuery` condition, since the WASM binary file is exported, you can:

```js
import { init } from 'brotli-dec-wasm'
import wasm from 'brotli-dec-wasm/pkg/index_bg.wasm?url'

const brotliDecWasm = await init(wasm)
```

## Alternatives

wasm-pack ships a `bundler` target which emits code with `import * as wasm from './pkg/index_bg.wasm'`.
However, while looks similar, wasm-pack expects the import result to be a `WebAssembly.Module`, which then requires `syncWebAssembly: true` for webpack 5.
But webpack 5 already suggests users migrating to `asyncWebAssembly: true`, which requires `import('./pkg/index_bg.wasm')`.
Not to say some other bundlers require plugins to support WASM and have various import methods.
This asset helper is a bypass for WASM over the module resolution system in the web / web worker environment (URLs available) and should makes things a little easier.
