mod syscall;

extern crate alloc;
use alloc::alloc::Layout;
use core::cell::UnsafeCell;

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("Memory allocation error: {:?}", layout);
}

#[global_allocator]
static HEAP: syscall::Alloc = syscall::Alloc {
    alloc: UnsafeCell::new(0x1000),
};
