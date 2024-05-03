use x86_64::{
    instructions::interrupts,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::uart_println;
use core::arch::asm;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
    interrupts::disable();
    unsafe {
        IDT.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        IDT.load();
    }
    interrupts::enable();
}

extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    uart_println!("Invalid opcode: {:#?}", stack_frame);
    panic!("Invalid opcode");
}

pub fn illegal_instruction() {
    unsafe {
        asm!("ud2");
    }
}
