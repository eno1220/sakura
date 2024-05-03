run:
	cd kernel && cargo build && cd ..
	mkdir -p mnt
	cp kernel/target/riscv64gc-unknown-none-elf/debug/kernel mnt/kernel.elf
	qemu-system-riscv64 -machine virt -bios default -nographic --no-reboot -serial mon:stdio -kernel mnt/kernel.elf

loader-run:
	cd loader && cargo build && cd ..
	mkdir -p mnt/EFI/BOOT
	cp loader/target/x86_64-unknown-uefi/debug/loader.efi mnt/EFI/BOOT/BOOTX64.EFI
	qemu-system-x86_64 -drive if=pflash,file=thirdparty/RELEASEX64_OVMF.fd,format=raw,readonly=on -drive format=raw,file=fat:rw:mnt --no-reboot -nographic -serial mon:stdio
