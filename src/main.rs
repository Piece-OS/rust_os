#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");
    panic!("Holy FUCK; this is a panic message.");

    loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo)  -> ! {
    println!("{}", info);
    loop {}
}
