#![no_std]
#![no_main]

mod arch;

use core::arch::{asm, global_asm};

use crate::arch::riscv64::exception::illegal_instruction;

extern "C" {
    fn exception_entry();
}

#[no_mangle]
fn kernel_main() -> ! {
    println!("Hello, world!");
    uart_println!("Hello, world!");
    unsafe {
        asm!("csrw stvec , {0}", in(reg) exception_entry as usize);
        illegal_instruction();
    }
    loop {}
}

global_asm!(
    r#"
    .section ".text.boot"
    .global boot
    boot:
        la sp, __stack_top
        j kernel_main
    "#
);

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("panic: {:?}", info);
    loop {}
}
