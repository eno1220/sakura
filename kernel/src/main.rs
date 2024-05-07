#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

mod arch;
mod kalloc;

extern crate alloc;

use alloc::vec;
use core::arch::asm;

#[cfg(target_arch = "riscv64")]
use crate::arch::riscv64::handler;

#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::handler;

#[cfg(target_arch = "riscv64")]
global_asm!(
    r#"
    .section ".text.boot"
    .global kernel_entry
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

fn init() {
    uart_println!("Hello, world!");
    let vec = vec![1, 2, 3];
    uart_println!("{:?}", vec);
    let vec = vec![1, 2, 3, 4];
    uart_println!("{:?}", vec);
    handler::init();
    handler::illegal_instruction();
    loop {}
}

#[no_mangle]
#[cfg(target_arch = "riscv64")]
fn kernel_main() {
    println!("Hello, world!");
    init();
}

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn kernel_main() {
    init();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    uart_println!("panic: {:?}", info);
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
