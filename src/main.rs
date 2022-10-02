#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_os::init();

    /* trigger a stack overflow */
    // fn stack_overflow() { stack_overflow(); }
    // stack_overflow();
    
    /* trigger as page fault */
    // unsafe { *(0xacabbabe as *mut u64) = 42; };
    /* invoke a breakpoint exception */
    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {
        /* provoking deadlock */
        /* to prevent deadlock
         * we disable interrupts
         *  during print
         * disabling interrupts
         *  should only be
         *  used in wost-
         *  case scenarios */
        for _ in 0 .. 10000 {}
        use rust_os::print;
        print!("-");
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo)  -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo)  -> ! {
    rust_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
