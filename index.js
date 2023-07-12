import init, * as brotliDecWasm from './pkg'

export * from './pkg'

export default init().then(() => brotliDecWasm)
