mod err;
mod stream;
mod utils;

use brotli_decompressor::BrotliDecompress;
pub use err::*;
pub use stream::*;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// No error reporting included. To get the detailed error code, use `BrotliDecStream`.
#[wasm_bindgen(js_name = brotliDec)]
pub fn brotli_dec(input: Box<[u8]>) -> Result<Box<[u8]>, JsValue> {
    let mut output = Vec::new();
    match BrotliDecompress(&mut input.as_ref(), &mut output) {
        Ok(_) => Ok(output.into_boxed_slice()),
        Err(_) => Err(JsValue::from_str("Brotli decompressing failed")),
    }
}
