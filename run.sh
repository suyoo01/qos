qemu-system-aarch64 -M arm-generic-fdt-7series -serial /dev/null -serial mon:stdio -display none \
  -kernel target/armv7a-none-eabi/debug/os -dtb zybo.dtb
