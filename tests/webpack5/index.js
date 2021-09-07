import {brotliDec} from 'brotli-dec-wasm'

let input = new Uint8Array([27, 23, 0, 248, 141, 148, 169, 61, 94, 181, 142, 183, 224, 96, 98, 170, 98, 161, 150, 56, 160, 232, 44, 0])
let output = new Uint8Array([227, 130, 173, 227, 131, 169, 227, 130, 173, 227, 131, 169, 227, 131, 137, 227, 130, 173, 227, 131, 137, 227, 130, 173])
console.assert(brotliDec(input) === output)
