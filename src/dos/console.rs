use core::arch::asm;
use core::fmt::{self, Write};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::dos::console::print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\r\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\r\n"), $($arg)*)
    };
}

pub fn print(args: fmt::Arguments) {
    let mut wirter = DosWriter {};
    wirter.write_fmt(args).unwrap();
}

struct DosWriter;

impl Write for DosWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            printc(c);
        }
        Ok(())
    }
}

fn printc(ch: u8) {
    unsafe {
        asm!("mov ah, 0x02",
              "int 0x21",
              in("dl") ch,
              lateout("ax") _,
              lateout("dx") _);
    }
}
