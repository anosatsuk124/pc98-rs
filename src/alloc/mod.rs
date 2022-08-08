mod bump_pointer;

extern crate alloc;
use alloc::alloc::Layout;
use core::cell::UnsafeCell;

#[global_allocator]
static HEAP: bump_pointer::BumpPointerAlloc = bump_pointer::BumpPointerAlloc {
    head: UnsafeCell::new(0x100),
    end: 0x200,
};

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("Memory allocation error: {:?}", layout);
}
