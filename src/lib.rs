use std::io::Read;

mod tests;

/// A buffer where IPC Data is written to by Rust and read from by Javascript
/// At any one moment, only one read and write is done to this buffer, so it's wrapped in a Mutex
pub static mut IPC_BUFFER: [u8; 1024] = [0; 1024];

/// This writes some data to the IPC buffer, then returns a pointer and an offset to the data
pub(crate) unsafe fn write_to_ipc<R: Read>(mut source: R) -> isize {
	if let Ok(offset) = source.read(&mut IPC_BUFFER) {
		return offset as isize;
	} else {
		return -1;
	}
}

#[no_mangle]
pub unsafe extern "C" fn get_data_pointer() -> *const u8 {
	IPC_BUFFER.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn start() -> isize {
	let mut data: &[u8] = &[2, 1, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3];
	write_to_ipc(&mut data)
}
