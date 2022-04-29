use std::io::Read;

mod tests;

mod console;
mod error;

mod goal;
mod preprocessor;
mod schedule;
mod task;

#[no_mangle]
extern "C" fn entry() {}

#[no_mangle]
pub unsafe extern "C" fn get_data_pointer() -> *const u8 {
	IPC_BUFFER.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn get_ipc_buffer_size() -> usize {
	IPC_BUFFER_SIZE
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

		error::exit(error::ErrorCode::UnableToWriteToIPC, data.len());
	}
}
