import init, * as brotliDecWasm from './pkg/index.js'

export * from './pkg/index.js'
export { init }

export default init().then(() => brotliDecWasm)
