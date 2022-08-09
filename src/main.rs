#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec::Vec;
use pc98_rs::{dos, print, println};

#[link_section = ".startup"]
#[no_mangle]
fn _start() -> ! {
    main();
    dos::exit(0);
}

fn main() {
    {
        let mut vec: Vec<usize> = Vec::new();
        for v in 0..16 {
            vec.push(v);
        }
        println!("{:?}", vec);
    }
    println!("Finished!");
}
