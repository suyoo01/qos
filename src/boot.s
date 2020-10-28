.section .text.boot
    b reset_handler
    b . /* 0x4 Undefined Instruction */
    b . /* 0x8 Software Intrrupt */
    b . /* 0xc Prefetch Abort */
    b . /* 0x10 Data Abort */
    b . /* 0x14 Reserved */
    b . /* 0x18 IRQ */
    b . /* 0x1c FIQ */

.section .text
.global reset_handler
reset_handler:
/* FIQ stack setup */
    msr cpsr_c, #0x11
    ldr r1, =_fiq_stack_start
    ldr sp, =_fiq_stack_end
    movw r0, #0xFEFE
    movt r0, #0xFEFE
fiq_loop:
    cmp r1, sp
    strlt r0, [r1], #4
    blt fiq_loop

/* IRQ stack setup */
    msr cpsr_c, #0x12
    ldr r1, =_irq_stack_start
    ldr sp, =_irq_stack_end
irq_loop:
    cmp r1, sp
    strlt r0, [r1], #4
    blt irq_loop

/* Supervisor stack setup */
    msr cpsr_c, #0x13
    ldr r1, =_svc_stack_start
    ldr sp, =_svc_stack_end
svc_loop:
    cmp r1, sp
    strlt r0, [r1], #4
    blt svc_loop

/* Kernel bss setup */
    mov r0, #0
    ldr r1, =_bss_start
    ldr r2, =_bss_end
bss_loop:
    cmp r1, r2
    strlt r0, [r1], #4
    blt bss_loop

    b entry /* branch to rust */