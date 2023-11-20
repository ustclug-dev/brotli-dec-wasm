import init from './pkg/index.js'
import wasm from './pkg/index_bg.wasm'

export * from './pkg/index.js'

export default init(wasm)
