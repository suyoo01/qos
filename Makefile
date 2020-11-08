KERNEL=target/armv7a-none-eabi/debug/os
all:
	cargo build
	cargo objdump -- -D > target/kern.obj
gdb: all
	gdb-multiarch $(KERNEL)
qemu: all
	qemu-system-arm -M xilinx-zynq-a9 -serial /dev/null -serial mon:stdio -s -nographic -kernel $(KERNEL)

qemu-gdb: all
	qemu-system-arm -M xilinx-zynq-a9 -serial /dev/null -serial mon:stdio -s -nographic -kernel $(KERNEL) -S