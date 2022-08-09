extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::cell::UnsafeCell;

use crate::{print, println};

pub struct Alloc {
    pub alloc: UnsafeCell<usize>,
}

unsafe impl Sync for Alloc {}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc = self.alloc.get();

        let align = layout.align();

        *alloc += align;
        println!("alloc: {:x}", *alloc);
        asm!(
            "mov ah, 0x48",
            "int 0x21",
            in("bx") align
        );
        alloc as *mut u8
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        let alloc = self.alloc.get();
        println!("dealloc: {:?}", alloc);
        asm!(
            "
            mov ah,0x4a
            int 0x21
            ",
            in("bx") alloc,
        );
        *alloc = 0;
    }
}
