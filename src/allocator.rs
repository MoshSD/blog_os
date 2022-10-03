use alloc::{alloc::{GlobalAlloc, Layout}, collections::LinkedList};
use core::ptr::null_mut;
use linked_list_allocator::LockedHeap;
use bump::BumpAllocator;
use linked_list::LinkedListAllocator;

pub mod fixed_size_block;
pub mod linked_list;
pub mod bump;

// #[global_allocator]
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());


// #[global_allocator]
// static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

#[global_allocator]
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());
    

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; //100KiB

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, frame,
    },
    VirtAddr,
};

use self::fixed_size_block::FixedSizeBlockAllocator;

//Initialising the kernel heap with new pages
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {

    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}



// Wrapper around spin::Mutex to allow for trait implementations
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self  {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

//Allign the given address 'addr' upwards to alignment 'align'
fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr // addr already aligned
    } else {
        addr - remainder + align
    }
}


// pub struct Dummy;
// #[global_allocator]
// static ALLOCATOR: Dummy = Dummy;

// unsafe impl GlobalAlloc for Dummy {
//     unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
//         null_mut()
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         panic!("dealloc should never be called")
//     }
// }   