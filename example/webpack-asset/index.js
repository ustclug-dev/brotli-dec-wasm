import brotliDecWasmPromise from 'brotli-dec-wasm/asset'

async function main() {
  const brotliDecWasm = await brotliDecWasmPromise
  window.brotliDecWasm = brotliDecWasm
}

main().then(() => {
  console.log('init ok')
  document.getElementById('status').innerText = 'init ok'
})
