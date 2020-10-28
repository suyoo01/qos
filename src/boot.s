.section .text.boot
    b reset_handler
    b . /* 0x4 Undefined Instruction */
    b . /* 0x8 Software Intrrupt */
    b . /* 0xc Prefetch Abort */
    b . /* 0x10 Data Abort */
    b . /* 0x14 Reserved */
    b . /* 0x18 IRQ */
    b . /* 0x1c FIQ */

.global reset_handler
reset_handler:
    ldr r0, =str1
    b entry
str1: .word 0xdeadbeef