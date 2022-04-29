import fs from "fs";

// RAW wasm source
const buffer = fs.readFileSync("./out/output.wasm");

// Compiled wasm module
const module = await WebAssembly.compile(buffer);

// The current WASM instance
const instance = await WebAssembly.instantiate(module, {
	env: {
		console_log(dataOffset) {
			let readResult = new Uint8Array(wasmMemory.buffer, dataStart, dataOffset);
			let decoder = new TextDecoder();
			let string = decoder.decode(readResult);
			console.log(string);
		},
		exit(error_code) {
			if (error_code != 0) {
				throw new Error(`[WASM_ERROR; ErrorCode:${error_code}]`);
			} else {
				console.info("Webassembly has finished execution")
			}
		}
	},
});

// Where the IPC buffer pointer starts at
const dataStart = instance.exports.get_data_pointer();

// The wasm memory
const wasmMemory = instance.exports.memory;

// Call main
if (instance.exports.main() != 0) {
	console.log("WASM has experienced an unexpected error!");
}
