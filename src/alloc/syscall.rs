extern crate alloc;

use alloc::alloc::{GlobalAlloc, Layout};
use core::arch::asm;
use core::cell::UnsafeCell;
use core::cmp::max;

use crate::{print, println};

pub struct Alloc {
    pub alloc: UnsafeCell<usize>,
}

unsafe impl Sync for Alloc {}

unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alloc = self.alloc.get();

        let mem = max(layout.align(), layout.size());
        let mem = if mem / 16 == 0 { 16 } else { mem };
        let ax: usize;
        asm!(
            "mov ah, 0x48",
            "int 0x21",
            in("bx") mem,
            lateout("ax") ax,
        );
        *alloc += mem;
        println!("mem: {:x}", mem);
        println!("ax: {:x}", ax);
        ax as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        let alloc = self.alloc.get();
        println!("dealloc: {:?}", &ptr);
        asm!(
            "
            mov es, {es}
            mov ah, 0x49
            int 0x21
            ",
            es = in(reg) ptr,
        );
        panic!("error!");
        // *alloc = 0x1000;
    }
}
