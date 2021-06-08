mod utils;

use brotli_decompressor::BrotliDecompress;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = brotliDec)]
pub fn brotli_dec(buf: Box<[u8]>) -> Result<Box<[u8]>, JsValue> {
  let mut out = Vec::<u8>::new();
  match BrotliDecompress(&mut buf.as_ref(), &mut out) {
    Ok(_) => (),
    Err(_) => return Err(JsValue::from_str("brotli dec failed")),
  }
  Ok(out.into_boxed_slice())
}
