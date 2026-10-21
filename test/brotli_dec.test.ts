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

describe("custom dictionary", () => {
  // Generated with:
  //   printf '$DICT_TEXT' > dict.txt
  //   printf '$PAYLOAD_TEXT' > payload.txt
  //   brotli -q 5 -D dict.txt --stdout payload.txt | base64
  const dict = base64ToData(
    "bWl0b2Nob25kcmlhbCBlbmVyZ3kgY29udmVyc2lvbi4gb3hpZGF0aXZlIHBob3NwaG9yeWxhdGlvbi4gZWxlY3Ryb24gdHJhbnNwb3J0IGNoYWluLiBrcmVicyBjeWNsZSByZWd1bGF0aW9uLiBhZGVub3NpbmUgdHJpcGhvc3BoYXRlIHN5bnRoZXNpcy4gY2hsb3JvcGh5bGwgYWJzb3JwdGlvbiBzcGVjdHJhLiBlbnp5bWUgc3Vic3RyYXRlIGFmZmluaXR5LiBnbHljb2x5c2lzIHBhdGh3YXkgY29udHJvbC4gY2VsbHVsYXIgcmVzcGlyYXRpb24gYmFsYW5jZS4gcGhvdG9zeW50aGVzaXMgZWZmaWNpZW5jeS4g",
  );
  const compressed = base64ToData("oXgFACDgkCO9QoC1BVngHAu9azQmYeHlcPrBY/h8nimn4qeGNbirsTtZnXW4y6FI4I7IoS+CJJPBqZwH");
  const payload =
    "The electron transport chain drives adenosine triphosphate synthesis, while the krebs cycle regulation gates the glycolysis pathway control across cellular respiration balance.";

  it("can decompress custom-dictionary data with the dictionary", () => {
    const result = brotliDec.decompress(compressed, dict);
    expect(textDecoder.decode(result)).toBe(payload);
  });

  it("cleanly fails when decompressing custom-dictionary data without the dictionary", () => {
    expect(() => brotliDec.decompress(compressed)).toThrow("Brotli decompress failed");
  });

  it("treats an empty custom dictionary as absent", () => {
    expect(() => brotliDec.decompress(compressed, new Uint8Array(0))).toThrow("Brotli decompress failed");
  });

  it("can streamingly decompress custom-dictionary data with the dictionary", () => {
    const input1 = compressed.slice(0, compressed.length / 2);
    const input2 = compressed.slice(compressed.length / 2);
    const stream = new brotliDec.DecompressStream(dict);
    const result1 = stream.decompress(input1, 256);
    expect(result1.code).toBe(brotliDec.BrotliStreamResultCode.NeedsMoreInput);
    const result2 = stream.decompress(input2, 256);
    expect(result2.code).toBe(brotliDec.BrotliStreamResultCode.ResultSuccess);
    expect(textDecoder.decode(new Uint8Array([...result1.buf, ...result2.buf]))).toBe(payload);
  });

  it("cleanly fails when streamingly decompressing custom-dictionary data without the dictionary", () => {
    const stream = new brotliDec.DecompressStream();
    expect(() => stream.decompress(compressed, 256)).toThrow("Brotli streaming decompress failed");
  });
});
