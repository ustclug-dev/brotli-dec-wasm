/*
    Copyright (c) 2016 Dropbox, Inc.
    All rights reserved.

    Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

    1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

    2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

    3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use wasm_bindgen::prelude::*;

/// Copied and modified from enum BrotliDecoderErrorCode of dropbox/rust-brotli-decompressor.
/// It is not PascalCase but kept in MACRO_CASE because changing all names is too troublesome.
/// And most of the time you may not need to care about the error code.
/// NOTICE: All numbers are reversed to positive, required by wasm_bindgen.
#[wasm_bindgen]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum BrotliDecStreamErrCode {
    /* Errors caused by invalid input */
    BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE = 1,
    BROTLI_DECODER_ERROR_FORMAT_RESERVED = 2,
    BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE = 3,
    BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET = 4,
    BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME = 5,
    BROTLI_DECODER_ERROR_FORMAT_CL_SPACE = 6,
    BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE = 7,
    BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT = 8,
    BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1 = 9,
    BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2 = 10,
    BROTLI_DECODER_ERROR_FORMAT_TRANSFORM = 11,
    BROTLI_DECODER_ERROR_FORMAT_DICTIONARY = 12,
    BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS = 13,
    BROTLI_DECODER_ERROR_FORMAT_PADDING_1 = 14,
    BROTLI_DECODER_ERROR_FORMAT_PADDING_2 = 15,
    BROTLI_DECODER_ERROR_FORMAT_DISTANCE = 16,

    /* -17..-18 codes are reserved */
    BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET = 19,
    BROTLI_DECODER_ERROR_INVALID_ARGUMENTS = 20,

    /* Memory allocation problems */
    BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES = 21,
    /* Literal = insert and distance trees together */
    BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS = 22,
    /* -23..-24 codes are reserved for distinct tree groups */
    BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP = 25,
    BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1 = 26,
    BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2 = 27,
    /* -28..-29 codes are reserved for dynamic ring-buffer allocation */
    BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES = 30,

    /* "Impossible" states */
    BROTLI_DECODER_ERROR_UNREACHABLE = 31,
}
