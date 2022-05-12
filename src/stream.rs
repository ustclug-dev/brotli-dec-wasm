use crate::utils;
use brotli_decompressor::{BrotliDecompressStream, BrotliResult, BrotliState, StandardAlloc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(i32)]
pub enum BrotliDecStreamResult {
    /// The stream is just inited and have not been runned.
    /// `BrotliResult` uses `ResultFailure = 0`, but as we will convert `ResultFailure` to a negative actual error code,
    /// 0 is reused as no input currently.
    Init = 0,
    ResultSuccess = 1,
    NeedsMoreInput = 2,
    NeedsMoreOutput = 3,
}

#[wasm_bindgen]
pub struct BrotliDecStream {
    state: BrotliState<StandardAlloc, StandardAlloc, StandardAlloc>,
    result: i32,
    total_out: usize,
    last_input_offset: usize,
}

#[wasm_bindgen]
impl BrotliDecStream {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        utils::set_panic_hook();
        let alloc = StandardAlloc::default();
        Self {
            state: BrotliState::new(alloc, alloc, alloc),
            result: BrotliDecStreamResult::Init as i32,
            total_out: 0,
            last_input_offset: 0,
        }
    }

    pub fn dec(&mut self, input: Box<[u8]>, output_size: usize) -> Result<Box<[u8]>, JsValue> {
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
                // It should be a negative error code, otherwise brotli-decompressor goes wrong
                self.result = self.state.error_code as i32;
                self.last_input_offset = 0;
                Err(JsValue::from_str("Brotli streaming decompressing failed"))
            }
            BrotliResult::NeedsMoreOutput => {
                self.result = BrotliDecStreamResult::NeedsMoreOutput as i32;
                self.last_input_offset = input_offset;
                Ok(output.into_boxed_slice())
            }
            BrotliResult::ResultSuccess => {
                output.truncate(output_offset);
                self.result = BrotliDecStreamResult::ResultSuccess as i32;
                self.last_input_offset = input.len();
                Ok(output.into_boxed_slice())
            }
            BrotliResult::NeedsMoreInput => {
                output.truncate(output_offset);
                self.result = BrotliDecStreamResult::NeedsMoreInput as i32;
                self.last_input_offset = input.len();
                Ok(output.into_boxed_slice())
            }
        }
    }

    pub fn total_out(&self) -> usize {
        self.total_out
    }

    pub fn result(&self) -> i32 {
        self.result
    }

    pub fn last_input_offset(&self) -> usize {
        self.last_input_offset
    }
}
