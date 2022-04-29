use std::io::Read;

mod tests;

/// A buffer where IPC Data is written to by Rust and read from by Javascript
/// At any one moment, only one read and write is done to this buffer, WASM is single-threaded anyway
pub const IPC_BUFFER_SIZE: usize = 1024 * 64;
pub static mut IPC_BUFFER: [u8; IPC_BUFFER_SIZE] = [0; IPC_BUFFER_SIZE];

/// This writes some data to the IPC buffer, then returns a pointer and an offset to the data
pub(crate) fn write_to_ipc<R: Read>(mut source: R) -> Result<usize, String> {
	match unsafe { source.read(&mut IPC_BUFFER) } {
		Ok(offset) => Ok(offset),
		Err(err) => Err(err.to_string()),
	}
}

#[no_mangle]
pub unsafe extern "C" fn get_data_pointer() -> *const u8 {
	IPC_BUFFER.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn get_ipc_buffer_size() -> usize {
	IPC_BUFFER_SIZE
}

extern "C" {
	fn console_log(offset: usize);
}

pub fn log<S: AsRef<str>>(msg: S) -> Result<(), String> {
	let data = msg.as_ref().as_bytes();

	if data.len() >= IPC_BUFFER_SIZE {
		return Err("The length of data to be logged to the console exceeds the size of the IPC_BUFFER".into());
	};

	let data_offset = write_to_ipc(data)?;
	unsafe {
		console_log(data_offset);
	}

	Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i8 {
	for _ in 0..12 {
		if log("A log called at will from Rust").is_err() {
			return -1;
		};
	}

	0
}
