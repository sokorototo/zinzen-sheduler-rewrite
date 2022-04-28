#![no_std]

extern "C" {
    fn exit(error_type: u8) -> !;
}

#[no_mangle]
pub fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

#[panic_handler]
fn panic_exit(_: &core::panic::PanicInfo) -> ! {
    unsafe { exit(1) }
}
