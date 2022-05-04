use brotli_decompressor::{BrotliDecompressStream, BrotliResult, BrotliState, StandardAlloc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BrotliDecStream {
    state: BrotliState<StandardAlloc, StandardAlloc, StandardAlloc>,
    #[wasm_bindgen(js_name = totalOut)]
    pub total_out: usize,
}

#[wasm_bindgen]
#[repr(u8)]
pub enum BrotliDecStreamResultCode {
    ResultFailure = 0,
    ResultSuccess = 1,
    NeedsMoreInput = 2,
    NeedsMoreOutput = 3,
}

#[wasm_bindgen(module = "/src/utils.js")]
extern "C" {
    pub type BrotliDecStreamResult;
    #[wasm_bindgen(constructor)]
    fn new(code: u8, output: Box<[u8]>) -> BrotliDecStreamResult;
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

    pub fn dec(
        &mut self,
        input: Box<[u8]>,
        output_size: usize,
    ) -> Result<BrotliDecStreamResult, JsValue> {
        let mut output = vec![0; output_size];
        let mut available_in = input.len();
        let mut input_offset = 0;
        let mut available_out = output_size;
        let mut output_offset = 0;
        match BrotliDecompressStream(
            &mut available_in,
            &mut input_offset,
            &input,
            &mut available_out,
            &mut output_offset,
            &mut output,
            &mut self.total_out,
            &mut self.state,
        ) {
            BrotliResult::ResultFailure => {
                Err(JsValue::from_str("Brotli streaming decompressing failed"))
            }
            BrotliResult::NeedsMoreOutput => Ok(BrotliDecStreamResult::new(
                BrotliDecStreamResultCode::NeedsMoreOutput as u8,
                output.into_boxed_slice(),
            )),
            BrotliResult::ResultSuccess => {
                output.truncate(output_offset);
                Ok(BrotliDecStreamResult::new(
                    BrotliDecStreamResultCode::ResultSuccess as u8,
                    output.into_boxed_slice(),
                ))
            }
            BrotliResult::NeedsMoreInput => {
                output.truncate(output_offset);
                Ok(BrotliDecStreamResult::new(
                    BrotliDecStreamResultCode::NeedsMoreInput as u8,
                    output.into_boxed_slice(),
                ))
            }
        }
    }
}
