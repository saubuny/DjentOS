#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// Implement panicking
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Add an entrypoint for our program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}
