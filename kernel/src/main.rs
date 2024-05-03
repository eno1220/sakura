#![no_std]
#![no_main]

mod arch;

use core::arch::{asm, global_asm};

#[cfg(target_arch = "riscv64")]
use crate::arch::riscv64::exception::illegal_instruction;

#[cfg(target_arch = "riscv64")]
extern "C" {
    fn exception_entry();
}

#[no_mangle]
#[cfg(target_arch = "riscv64")]
fn kernel_main() -> ! {
    println!("Hello, world!");
    uart_println!("Hello, world!");
    unsafe {
        asm!("csrw stvec , {0}", in(reg) exception_entry as usize);
        illegal_instruction();
    }
    loop {}
}

#[cfg(target_arch = "riscv64")]
global_asm!(
    r#"
    .section ".text.boot"
    .global boot
    boot:
        la sp, __stack_top
        j kernel_main
    "#
);

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn kernel_entry(new_rsp: u64) {
    unsafe {
        asm!("mov rsp, {}", "call kernel_main", in(reg) new_rsp, clobber_abi("sysv64"));
    }
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    uart_println!("Hello, world!");
    loop {}
}

#[panic_handler]
#[cfg(target_arch = "riscv64")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("panic: {:?}", info);
    loop {}
}

#[panic_handler]
#[cfg(target_arch = "x86_64")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    uart_println!("panic: {:?}", info);
    loop {}
}
