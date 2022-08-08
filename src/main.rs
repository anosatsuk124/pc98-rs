#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link_section = ".startup"]
#[no_mangle]
fn _start() {
    unsafe {
        asm!(
            "mov ah, 0x02",
            "mov dl, 0x41",
            "int 0x21",
            "int 0x20",
        )
    }
}

