#![feature(alloc_error_handler)]
#![no_std]

pub mod alloc;
pub mod dos;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    dos::exit(1);
}
