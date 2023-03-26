#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(djent_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use djent_os::println;

/// Entry point for our program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    println!("Welcome to DjentOS");

    #[cfg(test)]
    test_main();

    loop {}
}

/// Implement panicking
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Implement panicking in host console for unit testing
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    djent_os::test_panic_handler(info)
}
