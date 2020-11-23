ifeq ($(DEBUG), 1)
KERNEL=target/armv7a-none-eabi/debug/kernel
else
KERNEL=target/armv7a-none-eabi/release/kernel
CARGOFLAG=--release
endif
all:
	cargo build $(CARGOFLAG)
	cargo objdump $(CARGOFLAG) -- -D > target/kern.obj
gdb: all
	gdb-multiarch $(KERNEL)
qemu: all
	qemu-system-arm -M xilinx-zynq-a9 -serial /dev/null -serial mon:stdio -s -nographic -kernel $(KERNEL)

qemu-gdb: all
	qemu-system-arm -M xilinx-zynq-a9 -serial /dev/null -serial mon:stdio -s -nographic -kernel $(KERNEL) -S
