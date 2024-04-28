#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sros::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    sros::init();

    use x86_64::registers::control::Cr3;

    let (level_4_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_table.start_address());

    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    sros::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    sros::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sros::test_panic_handler(info)
}
