pub struct ErrorCode;

extern "C" {
	pub fn exit(exit_code: u8, offset: usize) -> !;
}

#[allow(non_upper_case_globals)]
impl ErrorCode {
	pub(crate) const UnableToWriteToIPC: u8 = 1;
	pub(crate) const LogDataTooLong: u8 = 2;
}
