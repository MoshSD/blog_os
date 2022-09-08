// main
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_std]   //not using the standard library
#![no_main]  //disabling main (all rust-level entry points)

mod serial;
mod vga_buffer;

#[no_mangle]  //disabling mangling for this fn so that it can be used by the linker - linker will look for fn named _start without main
pub extern "C" fn _start() -> ! {
    println!("BIG FAT STINKING CRAPS AND FARTS ALSO{}", "!");




    //Running tests through _start rather than the default "main"
    #[cfg(test)]
    test_main();

    loop{}
}

 
//Called on panic
use core::panic::PanicInfo;
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
//Setting up custom testing framework
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}