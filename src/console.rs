use crate::{
	error::{self, ErrorCode},
	write_to_ipc, IPC_BUFFER_SIZE,
};

extern "C" {
	fn console_log(is_string: bool, offset: usize);
}

/// Log a string to the console
pub fn log_str<S: AsRef<str>>(msg: S) {
	let data = msg.as_ref().as_bytes();

	unsafe {
		if data.len() >= IPC_BUFFER_SIZE {
			let error_msg: &[u8] = b"The length of data to be logged to the console exceeds the size of the IPC_BUFFER";
			write_to_ipc(error_msg);
			error::exit(ErrorCode::LogDataTooLong, error_msg.len())
		};

		write_to_ipc(data);
		console_log(true, data.len());
	}
}

/// Log a string to the console
pub fn log_buf(data: &[u8]) {
	unsafe {
		if data.len() >= IPC_BUFFER_SIZE {
			let error_msg: &[u8] = b"The length of data to be logged to the console exceeds the size of the IPC_BUFFER";
			write_to_ipc(error_msg);
			error::exit(ErrorCode::LogDataTooLong, error_msg.len())
		};

		write_to_ipc(data);
		console_log(false, data.len());
	}
}
