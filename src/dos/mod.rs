pub mod console;

use core::arch::asm;

pub fn exit(rt: u8) -> ! {
    unsafe {
        asm!("mov ah, 0x4c",
            "int 0x21",
            in("al") rt,
            lateout("ax") _);
    }
    loop {}
}
