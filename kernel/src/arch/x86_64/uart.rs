/* ref: https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/uart.c */

use core::arch::asm;
use core::fmt::Write;

const UART0: u16 = 0x3F8;

pub struct Uart {
    base: u16,
}

impl Uart {
    fn send_byte(&mut self, byte: u8) {
        // TODO: QEMU上では動作するが、実機では初期化が必要。また、同期処理が必要
        unsafe {
            asm!("out dx, al", in("dx") self.base, in("al") byte);
        }
    }
    fn send_str(&mut self, s: &str) {
        for c in s.bytes() {
            self.send_byte(c);
        }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.send_str(s);
        Ok(())
    }
}

impl Default for Uart {
    fn default() -> Self {
        Self { base: UART0 }
    }
}

pub fn _uart_print(args: core::fmt::Arguments) {
    let mut uart = Uart::default();
    uart.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => ($crate::arch::x86_64::uart::_uart_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! uart_println{
    () => ($crate::uart_print!("\r\n"));
    ($($arg:tt)*) => ($crate::uart_print!("{}\r\n",format_args!($($arg)*)));
}