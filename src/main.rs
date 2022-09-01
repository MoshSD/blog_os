// main


#![no_std]   //not using the standard library
#![no_main]  //disabling main (all rust-level entry points)

mod vga_buffer;

#[no_mangle]  //disabling mangling for this fn so that it can be used by the linker - linker will look for fn named _start without main
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop{}
}


//Called on panic
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}