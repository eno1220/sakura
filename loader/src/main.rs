#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec;
use elf::endian::AnyEndian;
use elf::ElfBytes;
// use elf::{endian::AnyEndian, ElfBytes};
use uefi::prelude::*;
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::table::boot::MemoryMap;

#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();

    uefi_services::println!("Hello, world!");

    let memmap_size = system_table.boot_services().memory_map_size().map_size + 4096;
    uefi_services::println!("Memory map size: {}", memmap_size);

    let mut memmap_buf = vec![0; memmap_size];
    let memmap = system_table
        .boot_services()
        .memory_map(&mut memmap_buf)
        .unwrap();
    // print_memory_map(&memmap);

    let mut file_protocol = match boot_services.get_image_file_system(image) {
        Ok(file) => file,
        Err(err) => {
            uefi_services::println!("Failed to get image file system: {:?}", err);
            return Status::ABORTED;
        }
    };

    let root_dir = match file_protocol.read_dir(cstr16!(".")) {
        Ok(root_dir) => root_dir,
        Err(error) => {
            uefi_services::println!("Failed to open root directory: {:?}", error);
            return Status::ABORTED;
        }
    };

    let kernel_file_info = root_dir
        .filter_map(|entry| match entry {
            Ok(entry) => {
                if entry.file_name() == cstr16!("kernel.elf") {
                    Some(entry)
                } else {
                    None
                }
            }
            Err(error) => {
                uefi_services::println!("Failed to read entry: {:?}", error);
                None
            }
        })
        .next()
        .unwrap();
    uefi_services::println!("kernel_file_info: {:?}", kernel_file_info);

    let kernel_file = match file_protocol.read(cstr16!("kernel.elf")) {
        Ok(file) => file,
        Err(error) => {
            uefi_services::println!("Failed to open kernel file: {:?}", error);
            return Status::ABORTED;
        }
    };

    let file = match ElfBytes::<AnyEndian>::minimal_parse(&kernel_file) {
        Ok(file) => {
            for section in file.section_headers().unwrap() {
                uefi_services::println!("Section: {:?}", section);
            }
            file
        }
        Err(error) => {
            uefi_services::println!("Failed to parse ELF file: {:?}", error);
            return Status::ABORTED;
        }
    };

    let entry_point = file.ehdr.e_entry;
    uefi_services::println!("Kernel entry point: {:#010x}", entry_point);

    let (load_first_addr, load_last_addr) = calc_load_size(&file);
    let load_page_size = calc_size_in_pages_from_bytes((load_last_addr - load_first_addr) as usize);

    let physical_addr = match boot_services.allocate_pages(
        uefi::table::boot::AllocateType::Address(load_first_addr),
        uefi::table::boot::MemoryType::LOADER_DATA,
        load_page_size,
    ) {
        Ok(physical_addr) => physical_addr,
        Err(error) => {
            uefi_services::println!("Failed to allocate pages: {:?}", error);
            return Status::ABORTED;
        }
    };

    uefi_services::println!("physical_addr: {:x}", physical_addr);

    for program_header in file.segments().unwrap() {
        if program_header.p_type == elf::abi::PT_LOAD {
            let segment_addr = kernel_file.as_ptr() as u64 + program_header.p_offset;
            let segment_size = program_header.p_filesz;
            let copy_to = program_header.p_vaddr;
            unsafe {
                core::ptr::copy_nonoverlapping(
                    segment_addr as *const u8,
                    copy_to as *mut u8,
                    segment_size as usize,
                );
            }
            let zero_size = program_header.p_memsz - program_header.p_filesz;
            unsafe {
                core::ptr::write_bytes(
                    (segment_size + segment_addr) as *mut u8,
                    0,
                    zero_size as usize,
                );
            }
        }
    }

    let kernel_stack = match boot_services.allocate_pages(
        uefi::table::boot::AllocateType::AnyPages,
        uefi::table::boot::MemoryType::LOADER_DATA,
        calc_size_in_pages_from_bytes(1024 * 1024),
    ) {
        Ok(kernel_stack) => kernel_stack,
        Err(error) => {
            panic!("Failed to allocate pages: {:?}", error);
        }
    };

    let kernel_stack =
        unsafe { core::slice::from_raw_parts_mut(kernel_stack as *mut u8, 1024 * 1024) };
    let new_rsp = kernel_stack.as_ptr() as u64 + 1024 * 1024;
    uefi_services::println!("new_rsp: {:x}", new_rsp);

    drop(file_protocol);
    let (_, memory_map) = system_table.exit_boot_services();

    unsafe {
        let entry_point: extern "sysv64" fn(new_rsp: u64) -> ! = core::mem::transmute(entry_point);
        entry_point(new_rsp);
    }

    #[allow(unreachable_code)]
    Status::SUCCESS
}

fn calc_load_size(file: &ElfBytes<AnyEndian>) -> (u64, u64) {
    let mut first_addr = u64::MAX;
    let mut last_addr = u64::MIN;
    for program_header in file.segments().unwrap() {
        if program_header.p_type == elf::abi::PT_LOAD {
            let start_addr = program_header.p_vaddr;
            let end_addr = start_addr + program_header.p_memsz;
            if start_addr < first_addr {
                first_addr = start_addr;
            }
            if end_addr > last_addr {
                last_addr = end_addr;
            }
        }
    }
    (first_addr, last_addr)
}

fn calc_size_in_pages_from_bytes(bytes: usize) -> usize {
    (bytes + 0xfff) / 0x1000
}

fn print_memory_map(memmap: &MemoryMap) {
    for entry in memmap.entries() {
        uefi_services::println!(
            "Type: {:?}, Addr: {:#010x} - {:#010x}, Number of pages: {:#06}, Len: {:#06} KiB",
            entry.ty,
            entry.phys_start,
            entry.phys_start + entry.page_count * 4 * 1024 - 1,
            entry.page_count,
            entry.page_count * 4
        );
    }
}
