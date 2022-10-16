#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]

#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rust_os::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }
    /* reading the page tables */
    //use x86_64::registers::control::Cr3;
    //let (level_4_page_table, _) = Cr3::read();

    //println!("Level 4 page table at: {:?}", level_4_page_table.start_address());


    /* trigger as page fault */
    // let ptr = 0xACABbabe as *mut u32
    // let ptr = 0x205546 as *mut u32;
    
    /* read from a code page */
    // unsafe { let x = *ptr; }
    // println!("read worked");
    
    /* write to a code page */
    // unsafe { *ptr = 42; };
    // println!("write worked");
    

    /* trigger a stack overflow */
    // fn stack_overflow() { stack_overflow(); }
    // stack_overflow();
    

    /* invoke a breakpoint exception */
    // x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
    /* provoking deadlock */
    /* to prevent deadlock
     * we disable interrupts
     *  during print
     * disabling interrupts
     *  should only be
     *  used in wost-
     *  case scenarios */
    // loop {
        // for _ in 0 .. 10000 {}
        // use rust_os::print;
        // print!("-");
    // }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo)  -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
    //loop {}
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
