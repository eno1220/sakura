run:
	cd kernel && cargo build && cd ..
	mkdir -p mnt
	cp kernel/target/riscv64gc-unknown-none-elf/debug/kernel mnt/kernel.elf
	qemu-system-riscv64 -machine virt -bios default -nographic --no-reboot -serial mon:stdio -kernel mnt/kernel.elf
