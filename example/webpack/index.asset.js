import wasmUrl from 'brotli-dec-wasm/web/bg.wasm'
import { default as init, decompress } from 'brotli-dec-wasm/web'

async function main() {
  await init(wasmUrl)
  window.decompress = decompress
}

main().then(() => {
  console.log('init ok')
  document.getElementById('status').innerText = 'init ok'
})
