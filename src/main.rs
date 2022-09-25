#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use x86_64::structures::paging::page_table::PageTableLevel;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory::active_level_4_table;
    use x86_64::VirtAddr;
    
    
    
    println!("DIRTY HARD CRAPS AND SHITS{}", "!");
    blog_os::init(); 

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
       if !entry.is_unused() {
            use x86_64::structures::paging::PageTable;

            if !entry.is_unused() {
                println!("L4 Entry {}: {:?}", i, entry);
                
                //get the physical address from the entry and convert it
                let phys = entry.frame().unwrap().start_address();
                let virt = phys.as_u64() + boot_info.physical_memory_offset;
                let ptr = VirtAddr::new(virt).as_mut_ptr();
                let l3_table: &PageTable = unsafe { &*ptr };

                //print for non empty entries in the l3 table
                for (i, entry) in l3_table.iter().enumerate() {
                    if !entry.is_unused() {
                        println!("  L3 Entry {}: {:?}", i, entry);
                    }
                }
            }
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }




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