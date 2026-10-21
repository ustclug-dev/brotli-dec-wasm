pub mod err;
pub mod stream;
mod utils;

use brotli_decompressor::{
    Allocator, BrotliDecompress, BrotliDecompressStream, BrotliResult, BrotliState,
    SliceWrapperMut, StandardAlloc,
};
use wasm_bindgen::prelude::*;

/// No error reporting included.
/// To get the detailed error code, use [`stream::BrotliDecStream`].
///
/// With `custom_dict`, decompress a stream encoded against that raw (LZ77) dictionary,
/// e.g. by the brotli CLI's `-D FILE`. An empty dictionary is treated as absent.
#[wasm_bindgen]
pub fn brotli_dec(input: Box<[u8]>, custom_dict: Option<Box<[u8]>>) -> Result<Box<[u8]>, JsError> {
    utils::set_panic_hook();
    let mut output = Vec::new();
    let result = match custom_dict {
        Some(dict) if !dict.is_empty() => {
            let mut alloc_u8 = StandardAlloc::default();
            let mut dict_mem = alloc_u8.alloc_cell(dict.len());
            dict_mem.slice_mut().copy_from_slice(&dict);
            let alloc = StandardAlloc::default();
            let mut state =
                BrotliState::new_with_custom_dictionary(alloc_u8, alloc, alloc, dict_mem);
            decompress_all(&input, &mut state, &mut output)
        }
        _ => BrotliDecompress(&mut input.as_ref(), &mut output).map_err(|_| ()),
    };
    match result {
        Ok(_) => Ok(output.into_boxed_slice()),
        Err(_) => Err(JsError::new("Brotli decompress failed")),
    }
}

/// One-shot decompression via the streaming state machine, so no second
/// monomorphization of the generic decode loop is linked in.
fn decompress_all(
    input: &[u8],
    state: &mut BrotliState<StandardAlloc, StandardAlloc, StandardAlloc>,
    output: &mut Vec<u8>,
) -> Result<(), ()> {
    let mut pos = 0;
    let mut total_out = 0;
    loop {
        if output.len() == total_out {
            output.resize(total_out + 4096, 0);
        }
        let mut available_in = input.len() - pos;
        let mut input_offset = 0;
        let mut available_out = output.len() - total_out;
        let mut output_offset = 0;
        let result = BrotliDecompressStream(
            &mut available_in,
            &mut input_offset,
            &input[pos..],
            &mut available_out,
            &mut output_offset,
            &mut output[total_out..],
            &mut total_out,
            state,
        );
        pos += input_offset;
        match result {
            BrotliResult::ResultSuccess => {
                output.truncate(total_out);
                return Ok(());
            }
            BrotliResult::ResultFailure => return Err(()),
            // A truncated stream: all input consumed without reaching success.
            BrotliResult::NeedsMoreInput if input_offset == 0 => return Err(()),
            BrotliResult::NeedsMoreInput | BrotliResult::NeedsMoreOutput => continue,
        }
    }
}

/// See [`brotli_dec`].
///
/// For drop-in replacement of `brotli-wasm`.
#[wasm_bindgen(js_name = decompress)]
pub fn brotli_dec_alt(
    buf: Box<[u8]>,
    custom_dict: Option<Box<[u8]>>,
) -> Result<Box<[u8]>, JsError> {
    brotli_dec(buf, custom_dict)
}
