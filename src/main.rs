#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Implement panicking
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// Add an entrypoint for our program
// Must be named _start exactly, uses C calling convention
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 0xb8000 is the location of the VGA Buffer
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
