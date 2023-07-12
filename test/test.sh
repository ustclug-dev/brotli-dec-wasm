#!/usr/bin/env bash

cd test
case "$1" in
  "preinstall")
    sed -i '/"wasm-pack": "/d' brotli-wasm/package.json
    sed -i '/"devDependencies": {/a\    "brotli-dec-wasm": "file:../..",' brotli-wasm/package.json
    ;;
  "pretest")
    sed -i "/import brotliPromise, { type BrotliWasmType } from '..'/a import brotliDecPromise, { type BrotliDecWasmType } from 'brotli-dec-wasm';" brotli-wasm/test/brotli.spec.ts
    sed -i '/let brotli: BrotliWasmType/a\    let brotliDec: BrotliDecWasmType;' brotli-wasm/test/brotli.spec.ts
    sed -i "/brotli = await brotliPromise/a\        brotliDec = await brotliDecPromise; brotliDec['DecompressStream'] = brotliDec.BrotliDecStream;" brotli-wasm/test/brotli.spec.ts
    sed -i 's/brotli.decompress/brotliDec.decompress/g' brotli-wasm/test/brotli.spec.ts
    sed -i 's/brotli.DecompressStream/brotliDec.DecompressStream/g' brotli-wasm/test/brotli.spec.ts
    ;;
  "test")
    cd brotli-wasm
    npm run test:webpack
    ;;
  "reset")
    cd brotli-wasm
    git checkout package.json test/brotli.spec.ts
    ;;
  *)
    echo "Unknown command: $1"
    ;;
esac
