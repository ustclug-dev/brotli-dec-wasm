// Test cases derived from brotli-wasm (https://github.com/httptoolkit/brotli-wasm).
// Licensed under Apache License, Version 2.0.
// Modified: adapted for brotli-dec-wasm (decompress-only) and vitest

import { describe, it, expect, beforeAll } from "vitest";
import brotliDecPromise from "../index.ts";

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder();
const base64ToData = (base64: string) => Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));

type BrotliDecWasm = Awaited<typeof brotliDecPromise>;
let brotliDec: BrotliDecWasm;

beforeAll(async () => {
  brotliDec = await brotliDecPromise;
});

describe("brotli-dec-wasm", () => {
  it("can decompress data", () => {
    // Generated with: echo -n '$CONTENT' | brotli --stdout - | base64
    const input = base64ToData("GxoAABypU587dC0k9ianQOgqjS32iUTcCA==");
    const result = brotliDec.decompress(input);
    expect(textDecoder.decode(result)).toBe("Brotli brotli brotli brotli");
  });

  it("cleanly fails when decompressing garbage", () => {
    const input = textEncoder.encode("This is not brotli data, it's just a string");
    expect(() => brotliDec.decompress(input)).toThrow("Brotli decompress failed");
  });

  it("can streamingly decompress data", () => {
    // Generated with: echo -n '$CONTENT' | brotli --stdout - | base64
    const input = base64ToData("GxoAABypU587dC0k9ianQOgqjS32iUTcCA==");
    const input1 = input.slice(0, input.length / 2);
    const input2 = input.slice(input.length / 2);
    const stream = new brotliDec.DecompressStream();
    const result1 = stream.decompress(input1, 100);
    const output1 = result1.buf;
    expect(result1.code).toBe(brotliDec.BrotliStreamResultCode.NeedsMoreInput);
    const result2 = stream.decompress(input2, 100);
    const output2 = result2.buf;
    expect(result2.code).toBe(brotliDec.BrotliStreamResultCode.ResultSuccess);
    expect(textDecoder.decode(new Uint8Array([...output1, ...output2]))).toBe("Brotli brotli brotli brotli");
  });

  it("cleanly fails when streamingly decompressing garbage", () => {
    const input = textEncoder.encode("This is not brotli data, it's just a string");
    const stream = new brotliDec.DecompressStream();
    expect(() => stream.decompress(input, 100)).toThrow("Brotli streaming decompress failed");
  });

  it("streaming decompressing can handle needing more output", () => {
    const input = base64ToData("GxoAABypU587dC0k9ianQOgqjS32iUTcCA==");
    const stream = new brotliDec.DecompressStream();
    const result1 = stream.decompress(input, 1);
    const output1 = result1.buf;
    expect(result1.code).toBe(brotliDec.BrotliStreamResultCode.NeedsMoreOutput);
    const result2 = stream.decompress(input.slice(result1.input_offset), 100);
    const output2 = result2.buf;
    expect(result2.code).toBe(brotliDec.BrotliStreamResultCode.ResultSuccess);
    expect(textDecoder.decode(new Uint8Array([...output1, ...output2]))).toBe("Brotli brotli brotli brotli");
  });

  it("can decompress pre-compressed data back to original", () => {
    // "Test input data" compressed with brotli at default quality
    const compressed = base64ToData("Gw4A+KWpyubolCCjVAjmxJ4D");
    const result = brotliDec.decompress(compressed);
    expect(textDecoder.decode(result)).toBe("Test input data");
  });

  it("can decompress data compressed at different quality", () => {
    // "Test input data" compressed with brotli at quality 1
    const compressed = base64ToData("CweAVGVzdCBpbnB1dCBkYXRhAw==");
    const result = brotliDec.decompress(compressed);
    expect(textDecoder.decode(result)).toBe("Test input data");
  });
});
