riscv-run:
	cd kernel && cargo build --target riscv64gc-unknown-none-elf && cd ..
	mkdir -p mnt
	cp kernel/target/riscv64gc-unknown-none-elf/debug/kernel mnt/kernel.elf
	qemu-system-riscv64 -machine virt -bios default -nographic --no-reboot -serial mon:stdio -kernel mnt/kernel.elf

riscv-build:
	cd kernel && cargo build --target riscv64gc-unknown-none-elf && cd ..

x86-run:
	cd loader && cargo build && cd ..
	cd kernel && cargo build --target x86_64-sakuraos-eabi.json && cd ..
	mkdir -p mnt/EFI/BOOT
	cp loader/target/x86_64-unknown-uefi/debug/loader.efi mnt/EFI/BOOT/BOOTX64.EFI
	cp kernel/target/x86_64-sakuraos-eabi/debug/kernel.elf mnt/kernel.elf
	qemu-system-x86_64 -drive if=pflash,file=thirdparty/RELEASEX64_OVMF.fd,format=raw,readonly=on -drive format=raw,file=fat:rw:mnt --no-reboot -nographic -serial mon:stdio

x86-build:
	cd loader && cargo build && cd ..
	cd kernel && cargo build --target x86_64-sakuraos-eabi.json && cd ..

fmt:
	cd kernel && cargo fmt && cd ..
	cd loader && cargo fmt && cd ..

clean:
	cd kernel && cargo clean && cd ..
	cd loader && cargo clean && cd ..
	rm -rf mnt
