import _init, * as brotliDecWasm from "./pkg/brotli_dec_wasm.js";

let initPromise: Promise<typeof brotliDecWasm> | null = null;

function init() {
    if (!initPromise) {
        initPromise = _init().then(() => brotliDecWasm);
    }
    return initPromise;
}

export default init();
