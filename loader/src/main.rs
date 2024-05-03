#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use uefi::prelude::*;
use uefi::table::boot::MemoryMap;

#[entry]
fn efi_main(_image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();
    uefi::println!("Hello, world!");

    let memmap_size = system_table.boot_services().memory_map_size().map_size + 4096;
    uefi::println!("Memory map size: {}", memmap_size);

    let mut memmap_buf = vec![0; memmap_size];
    let memmap = system_table
        .boot_services()
        .memory_map(&mut memmap_buf)
        .unwrap();
    print_memory_map(&memmap);
    loop {}

    #[allow(unreachable_code)]
    Status::SUCCESS
}

fn print_memory_map(memmap: &MemoryMap) {
    for entry in memmap.entries() {
        uefi::println!(
            "Type: {:?}, Addr: {:#010x} - {:#010x}, Number of pages: {:#06}, Len: {:#06} KiB",
            entry.ty,
            entry.phys_start,
            entry.phys_start + entry.page_count * 4 * 1024 - 1,
            entry.page_count,
            entry.page_count * 4
        );
    }
}
