#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
// This is called when a panic occurs. '!' is the "never" return type.
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

// This is our binary entrypoint. "C" denotes the C calling convention.
// A function called "_start" is automatically looked for by the linker at compile time.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	loop {}
}
