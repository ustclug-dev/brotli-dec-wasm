import init, * as brotliDecWasm from './pkg/index.js'

export * from './pkg/index.js'

export default init().then(() => brotliDecWasm)
