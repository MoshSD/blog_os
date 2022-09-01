// main


#![no_std]   //not using the standard library
#![no_main]  //disabling main (all rust-level entry points)

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]  //disabling mangling for this fn so that it can be used by the linker - linker will look for fn named _start without main
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}


//Called on panic
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}