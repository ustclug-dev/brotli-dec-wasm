import brotliDecWasmPromise from 'brotli-dec-wasm/asset'
// import wasm from 'brotli-dec-wasm/pkg/index_bg.wasm'
// import { init } from 'brotli-dec-wasm'

async function main() {
  const brotliDecWasm = await brotliDecWasmPromise
  // const brotliDecWasm = await init(wasm)
  window.brotliDecWasm = brotliDecWasm
}

main().then(() => {
  console.log('init ok')
  document.getElementById('status').innerText = 'init ok'
})
