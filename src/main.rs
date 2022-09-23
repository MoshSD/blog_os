#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use x86_64::structures::paging::page_table::PageTableLevel;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("DIRTY HARD CRAPS AND SHITS{}", "!");
    blog_os::init(); 


    #[cfg(test)]
    test_main();

    blog_os::hlt_loop();   
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop(); 
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}