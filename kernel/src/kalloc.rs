use core::{
    alloc::{GlobalAlloc, Layout},
    cell::Cell,
};

use crate::uart_println;

const HEAP_SIZE: usize = 4096;

pub struct LinerAllocator {
    heap: [u8; HEAP_SIZE],
    used_size: Cell<usize>,
}

unsafe impl Sync for LinerAllocator {}

unsafe impl GlobalAlloc for LinerAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        uart_println!("alloc");
        uart_println!("layout: {:?}", layout);

        let start = self.used_size.get();
        let end = start + layout.size();
        if end > HEAP_SIZE {
            return core::ptr::null_mut();
        }
        self.used_size.set(end);

        uart_println!("start: {}", start);
        uart_println!("end: {}", end);

        self.heap.as_ptr().add(start) as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        uart_println!("dealloc");
        uart_println!("ptr: {:p}", _ptr);
        uart_println!("layout: {:?}", _layout);
    }
}

impl LinerAllocator {
    pub const fn new() -> Self {
        Self {
            heap: [0; HEAP_SIZE],
            used_size: Cell::new(0),
        }
    }
}

#[global_allocator]
static A: LinerAllocator = LinerAllocator::new();
