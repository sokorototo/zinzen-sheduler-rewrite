pub struct ErrorCode;

#[allow(non_upper_case_globals)]
impl ErrorCode {
	pub(crate) const UnableToWriteToIPC: u8 = 1;
	pub(crate) const LogDataTooLong: u8 = 2;
}