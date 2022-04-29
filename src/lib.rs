#![no_main]

use std::io::Read;

mod error;
mod repetition;
mod task;
mod tests;

#[no_mangle]
#[cfg(debug_assertions)]
pub unsafe extern "C" fn main() {
	log("Hello World!");
}

extern "C" {
	fn console_log(offset: usize);
	fn exit(exit_code: u8, offset: usize) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn get_data_pointer() -> *const u8 {
	IPC_BUFFER.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn get_ipc_buffer_size() -> usize {
	IPC_BUFFER_SIZE
}

/// Log data to the console
pub fn log<S: AsRef<str>>(msg: S) {
	let data = msg.as_ref().as_bytes();

	unsafe {
		if data.len() >= IPC_BUFFER_SIZE {
			let error_msg: &[u8] = b"The length of data to be logged to the console exceeds the size of the IPC_BUFFER";
			write_to_ipc(error_msg);
			exit(error::ErrorCode::LogDataTooLong, error_msg.len())
		};

		write_to_ipc(data);
		console_log(data.len());
	}
}

/// A buffer where IPC Data is written to by Rust and read from by Javascript
/// At any one moment, only one read and write is done to this buffer, WASM is single-threaded anyway
pub const IPC_BUFFER_SIZE: usize = 1024 * 64;
pub static mut IPC_BUFFER: [u8; IPC_BUFFER_SIZE] = [0; IPC_BUFFER_SIZE];

/// This writes some data to the IPC buffer, then returns a pointer and an offset to the data
pub(crate) unsafe fn write_to_ipc<R: Read>(mut source: R) {
	if let Err(error) = source.read(&mut IPC_BUFFER) {
		let data = error.to_string();
		IPC_BUFFER[..data.len()].copy_from_slice(data.as_bytes());

		exit(error::ErrorCode::UnableToWriteToIPC, data.len());
	}
}
