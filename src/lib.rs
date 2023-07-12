pub mod err;
pub mod stream;
mod utils;

use brotli_decompressor::BrotliDecompress;
use wasm_bindgen::prelude::*;

/// No error reporting included.
/// To get the detailed error code, use [`stream::BrotliDecStream`].
#[wasm_bindgen]
pub fn brotli_dec(input: Box<[u8]>) -> Result<Box<[u8]>, JsError> {
    utils::set_panic_hook();
    let mut output = Vec::new();
    match BrotliDecompress(&mut input.as_ref(), &mut output) {
        Ok(_) => Ok(output.into_boxed_slice()),
        Err(_) => Err(JsError::new("Brotli decompress failed")),
    }
}

/// See [`brotli_dec`].
///
/// For drop-in replacement of `brotli-wasm`.
#[wasm_bindgen(js_name = decompress)]
pub fn brotli_dec_alt(buf: Box<[u8]>) -> Result<Box<[u8]>, JsError> {
    brotli_dec(buf)
}
