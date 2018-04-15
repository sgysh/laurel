use alloc::heap::{Alloc, AllocErr, Layout};

use core;
use chunk;
use util;

pub struct MyAllocator {
    head: core::cell::RefCell<chunk::ChunkHeader>,
}

impl MyAllocator {
    pub const fn empty() -> MyAllocator {
        MyAllocator {
            head: core::cell::RefCell::new(chunk::ChunkHeader::empty()),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.head = core::cell::RefCell::new(chunk::ChunkHeader::new(heap_start, heap_size));
    }

    pub const fn new() -> MyAllocator {
        MyAllocator {head: core::cell::RefCell::new(chunk::ChunkHeader::empty())}
    }
}

unsafe impl<'a> Alloc for &'a MyAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        util::irq_disable();

        let alloc_start = self.head.borrow_mut().add(layout.size());

        util::irq_enable();

        if alloc_start.is_some() {
            return Ok(alloc_start.unwrap() as *mut u8);
        } else {
            debug_assert!(false);
            return Err(AllocErr::Exhausted{request: layout})
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        util::irq_disable();

        self.head.borrow_mut().del(ptr, layout.size());

        util::irq_enable();
    }
}
