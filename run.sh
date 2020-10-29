#!/bin/sh

~/Xilinx/SDK/2017.3/bin/qemu-system-aarch64 -M arm-generic-fdt-7series -serial /dev/null -serial mon:stdio -nographic\
  -dtb zybo.dtb -machine linux=on -kernel $1 -s 
