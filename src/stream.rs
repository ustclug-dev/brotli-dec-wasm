use brotli_decompressor::{BrotliDecompressStream, BrotliResult, BrotliState, StandardAlloc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BrotliDecStream {
    state: BrotliState<StandardAlloc, StandardAlloc, StandardAlloc>,
    #[wasm_bindgen(js_name = totalOut)]
    pub total_out: usize,
}

#[wasm_bindgen]
pub enum BrotliDecStreamResultCode {
    ResultFailure = 0,
    ResultSuccess = 1,
    NeedsMoreInput = 2,
    NeedsMoreOutput = 3,
}

#[wasm_bindgen]
impl BrotliDecStream {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let alloc = StandardAlloc::default();
        Self {
            state: BrotliState::new(alloc, alloc, alloc),
            total_out: 0,
        }
    }

    pub fn dec(&mut self, input: Box<[u8]>, output_size: usize) -> Result<Box<[u8]>, JsValue> {
        // 1 more byte to store the result code,
        // because returning a JS object including Vec is (a little) hard with wasm-bindgen
        let mut output = vec![0; output_size + 1];
        let mut available_in = input.len();
        let mut input_offset = 0;
        let mut available_out = output_size;
        let mut output_offset = 0;
        let res = BrotliDecompressStream(
            &mut available_in,
            &mut input_offset,
            &input,
            &mut available_out,
            &mut output_offset,
            &mut output,
            &mut self.total_out,
            &mut self.state,
        );
        return match res {
            BrotliResult::ResultFailure => {
                Err(JsValue::from_str("Brotli streaming decompressing failed"))
            }
            BrotliResult::NeedsMoreOutput => {
                output[output_size + 1] = BrotliDecStreamResultCode::NeedsMoreOutput as u8;
                Ok(output.into_boxed_slice())
            }
            BrotliResult::NeedsMoreInput | BrotliResult::ResultSuccess => {
                output.resize(output_offset + 1, 0);
                output[output_offset] = res as u8;
                Ok(output.into_boxed_slice())
            }
        };
    }
}
