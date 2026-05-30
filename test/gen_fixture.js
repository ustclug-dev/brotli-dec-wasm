// Generate compressed brotli fixtures for benchmarks
import { brotliCompressSync } from "node:zlib";
import { randomBytes } from "node:crypto";
import { writeFileSync } from "node:fs";

const size = 1_000_000;

// Random data (incompressible)
const randomInput = randomBytes(size);
const randomCompressed = brotliCompressSync(randomInput);
writeFileSync(new URL("./fixture_1m_random.bin", import.meta.url), randomCompressed);
console.log(`Random:   ${randomInput.length} bytes -> ${randomCompressed.length} bytes compressed`);

// Repeated data (highly compressible)
const repeatedInput = Buffer.from("Brotli benchmark data. ".repeat(Math.ceil(size / 23)).slice(0, size));
const repeatedCompressed = brotliCompressSync(repeatedInput);
writeFileSync(new URL("./fixture_1m_repeated.bin", import.meta.url), repeatedCompressed);
console.log(`Repeated: ${repeatedInput.length} bytes -> ${repeatedCompressed.length} bytes compressed`);
