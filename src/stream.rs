use crate::utils;
use brotli_decompressor::{BrotliDecompressStream, BrotliResult, BrotliState, StandardAlloc};
use wasm_bindgen::prelude::*;

/// Returned by every successful (de)compression.
//
// Use `getter_with_clone` because `buf` does not implement `Copy`.
#[wasm_bindgen(getter_with_clone)]
pub struct BrotliStreamResult {
    /// Result code.
    ///
    /// See [`BrotliStreamResultCode`] for available values.
    ///
    /// When error, the error code is not passed here but rather goes to `Err`.
    pub code: BrotliStreamResultCode,
    /// Output buffer
    pub buf: Box<[u8]>,
    /// Consumed bytes of the input buffer
    pub input_offset: usize,
}

#[wasm_bindgen]
/// Same as [`brotli_decompressor::BrotliResult`] except [`brotli_decompressor::BrotliResult::ResultFailure`].
///
/// Always `> 0`.
///
/// `ResultFailure` is removed
/// because we will convert the failure to an actual negative error code (if available) and pass it elsewhere.
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum BrotliStreamResultCode {
    ResultSuccess = 1,
    NeedsMoreInput = 2,
    NeedsMoreOutput = 3,
}

#[wasm_bindgen]
pub struct BrotliDecStream {
    state: BrotliState<StandardAlloc, StandardAlloc, StandardAlloc>,
    total_out: usize,
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
            total_out: 0,
        }
    }

    pub fn dec(
        &mut self,
        input: Box<[u8]>,
        output_size: usize,
    ) -> Result<BrotliStreamResult, JsError> {
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
                // It should be a negative error code
                let err_code = self.state.error_code as i32;
                Err(JsError::new(&format!(
                    "Brotli streaming decompress failed: Error code {err_code}"
                )))
            }
            BrotliResult::NeedsMoreOutput => Ok(BrotliStreamResult {
                code: BrotliStreamResultCode::NeedsMoreOutput,
                buf: output.into_boxed_slice(),
                input_offset,
            }),
            BrotliResult::ResultSuccess => {
                output.truncate(output_offset);
                Ok(BrotliStreamResult {
                    code: BrotliStreamResultCode::ResultSuccess,
                    buf: output.into_boxed_slice(),
                    input_offset,
                })
            }
            BrotliResult::NeedsMoreInput => {
                output.truncate(output_offset);
                Ok(BrotliStreamResult {
                    code: BrotliStreamResultCode::NeedsMoreInput,
                    buf: output.into_boxed_slice(),
                    input_offset,
                })
            }
        }
    }

    /// See [`Self::dec()`].
    ///
    /// For drop-in replacement of `brotli-wasm`.
    #[wasm_bindgen(js_name = decompress)]
    pub fn dec_alt(
        &mut self,
        input: Box<[u8]>,
        output_size: usize,
    ) -> Result<BrotliStreamResult, JsError> {
        self.dec(input, output_size)
    }

    pub fn total_out(&self) -> usize {
        self.total_out
    }
}

/// See [`BrotliDecStream`].
///
/// For drop-in replacement of `brotli-wasm`.
#[wasm_bindgen(js_name = DecompressStream)]
pub struct BrotliDecStreamAlt {
    stream: BrotliDecStream,
}

#[wasm_bindgen(js_class = DecompressStream)]
impl BrotliDecStreamAlt {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            stream: BrotliDecStream::new(),
        }
    }

    pub fn decompress(
        &mut self,
        input: Box<[u8]>,
        output_size: usize,
    ) -> Result<BrotliStreamResult, JsError> {
        self.stream.dec(input, output_size)
    }

    pub fn total_out(&self) -> usize {
        self.stream.total_out()
    }
}
