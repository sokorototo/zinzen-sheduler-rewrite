import fs from "fs";

let buffer = fs.readFileSync("./out/output.wasm");
let module = await WebAssembly.compile(buffer);
let instance = await WebAssembly.instantiate(module);

// obtain the module memory
let wasmMemory = instance.exports.memory;

// create a buffer starting at the reference to the exported string
let dataStart = instance.exports.get_data_pointer();
let readResult = new Uint8Array(wasmMemory.buffer, dataStart, instance.exports.start());

// Create Decoder, and read text
console.log(readResult);
