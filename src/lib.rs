mod utils;

use brotli_decompressor::BrotliDecompress;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = brotliDec)]
pub fn brotli_dec(input: Box<[u8]>) -> Result<Box<[u8]>, JsValue> {
    let mut output = Vec::new();
    match BrotliDecompress(&mut input.as_ref(), &mut out) {
        Ok(_) => Ok(output.into_boxed_slice()),
        Err(_) => Err(JsValue::from_str("brotli dec failed")),
    }
}
