// main


#![no_std]   //not using the standard library
#![no_main]  //disabling main (all rust-level entry points)

mod vga_buffer;

#[no_mangle]  //disabling mangling for this fn so that it can be used by the linker - linker will look for fn named _start without main
pub extern "C" fn _start() -> ! {
    println!("BIG FAT STINKING CRAPS AND FARTS ALSO{}", "!");
    
    !("asihofqweginuwguyinw3inugy uinywe gnuiewr g")
    panic!("crapping and also farting");

    loop{}
}


//Called on panic
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}