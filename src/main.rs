#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Implement panicking
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Add an entrypoint for our program
// Must be named _start exactly, uses C calling convention
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
