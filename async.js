import init from './pkg/index.js'

export default import('./pkg/index_bg.wasm').then(init)
