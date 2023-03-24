#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// Implement panicking
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Add an entrypoint for our program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
