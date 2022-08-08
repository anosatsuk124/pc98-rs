#![no_std]
#![no_main]
extern crate alloc;
extern crate rlibc;

use alloc::vec::Vec;
use pc98_rs::{dos, print, println};

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    main();
    dos::exit(0);
}

fn main() {
    let mut vec = Vec::new();
    vec.push(0);
    println!("vec: {:?}", vec);
    vec.push(1);
    println!("vec: {:?}", vec);
    vec.push(2);
    println!("vec: {:?}", vec);
    vec.push(3);
    println!("vec: {:?}", vec);
}
