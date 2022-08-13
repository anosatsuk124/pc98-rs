#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec::Vec;
use core::arch::asm;
use pc98_rs::{dos, print, println};

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    unsafe {
        asm!(
            "
        mov bx, ss
        mov ax, es
        sub bx, ax
        mov ax, sp
        add ax, 0x0f
        shr ax, 4
        add bx, ax
        mov ah, 0x4a
        int 21h
        "
        );
    }
    main();
    dos::exit(0);
}

fn main() {
    {
        let mut vec1: Vec<usize> = Vec::new();
        let mut vec2: Vec<usize> = Vec::new();
        for v in 0..16 {
            println!("vec1: {:?}, ptr: {:?}", &vec1, &vec1.as_ptr());
            vec1.push(v);
            println!("vec2: {:?}, ptr: {:?}", &vec2, &vec2.as_ptr());
            vec2.push(v + 10);
        }
        println!("vec1: {:?}, ptr: {:?}", &vec1, &vec1.as_ptr());
        println!("vec2: {:?}, ptr: {:?}", &vec2, &vec2.as_ptr());
    }
    println!("Finished!");
}
