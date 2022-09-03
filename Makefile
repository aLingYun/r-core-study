go:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/ll_os.bin,addr=0x80200000

run:
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/ll_os.bin,addr=0x80200000 \
		-s -S


debug:
	riscv64-unknown-elf-gdb \
		-ex 'file target/riscv64gc-unknown-none-elf/release/os' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'

b:
	cargo build --release
	rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/ll_os --strip-all -O binary target/riscv64gc-unknown-none-elf/release/ll_os.bin

