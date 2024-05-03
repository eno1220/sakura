/* ref: https://github.com/mit-pdos/xv6-riscv/blob/riscv/kernel/uart.c */

use core::fmt::Write;

const UART0: usize = 0x1000_0000;

pub struct Uart {
    base: usize,
}

impl Uart {
    fn send_byte(&mut self, byte: u8) {
        // TODO: QEMU上では動作するが、実機では初期化が必要。また、同期処理が必要
        unsafe {
            core::ptr::write_volatile(self.base as *mut u8, byte);
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
    ($($arg:tt)*) => ($crate::arch::riscv64::uart::_uart_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! uart_println{
    () => ($crate::uart_print!("\r\n"));
    ($($arg:tt)*) => ($crate::uart_print!("{}\r\n",format_args!($($arg)*)));
}
