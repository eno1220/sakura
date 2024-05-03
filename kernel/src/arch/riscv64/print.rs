use crate::arch::riscv64::sbi;
use core::fmt::Write;

struct StdOut;

impl Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            sbi::putchar(c as u8);
        }
        Ok(())
    }
}

pub fn _print(args: core::fmt::Arguments) {
    StdOut.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print{
    ($($arg:tt)*) => ($crate::arch::riscv64::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println{
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n",format_args!($($arg)*)));
}
