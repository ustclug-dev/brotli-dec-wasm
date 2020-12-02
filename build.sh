#!/bin/bash
set -euo pipefail

rm -rf pkg
wasm-pack build --target web --release --out-name brotli-dec-wasm
rm pkg/package.json pkg/README.md pkg/LICENSE pkg/.gitignore
