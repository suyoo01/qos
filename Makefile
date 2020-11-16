
KERNEL=target/armv7a-none-eabi/release/os
all:
	cargo build --release
	cargo objdump --release -- -D > target/kern.obj
gdb: all
	gdb-multiarch $(KERNEL)
qemu: all
	qemu-system-arm -M xilinx-zynq-a9 -serial /dev/null -serial mon:stdio -s -nographic -kernel $(KERNEL)

qemu-gdb: all
	qemu-system-arm -M xilinx-zynq-a9 -serial /dev/null -serial mon:stdio -s -nographic -kernel $(KERNEL) -S