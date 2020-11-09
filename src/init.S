.section .text.init
.global _start
_start:
/* Switch to supervisor mode */
    msr cpsr_c, #0x13 
    mov r0, #0
    mov r1, #0
    mov r2, #0
    mov r3, #0
    mov r4, #0
    mov r5, #0
    mov r6, #0
    mov r7, #0
    mov r8, #0
    mov r9, #0
    mov r10, #0
    mov r11, #0
    mov r12, #0
    mov r13, #0
    mov r14, #0
    b setup
wait:
    wfe
    b wait
setup:

/* Set TTBCR.N = 0 */
MRC p15, 0, r1, c2, c0, 2
mov r2, #-4
and r1, r1, r2
MCR p15, 0, r1, c2, c0, 2

/* Setup kernel page table */
mov r0, #0
ldr r1, =_kern_pgdir
/* Kernel page directory */
add r2, r1, #(1<<14)
/* MMIO page table */
add r2, r1, #(1<<10)

page_loop:
    cmp r1, r2
    strlt r0, [r1], #4
    blt page_loop

ldr r1, =_kern_pgdir
/* Register page table */
mcr p15, 0, r1, c2, c0, 0


/* Set [1M, 2M) identity mapping */ 
movw r2, #0x0c02
movt r2, #0x0010
add r1, r1, #4
str r2, [r1]

/* Set [3G+1M, 3G+2M) -> [1M, 2M) offset mapping */
add r1, r1, #(3072 * 4)
str r2, [r1]


/* Map MMIO region [4G-1M, 4G) */
mov r0, #(1022 * 4)
add r1, r1, r0
ldr r2, =_mmio_pgtable
add r2, r2, #1
str r2, [r1]

/* Magic number for UART MMIO */
movw r1, #0x1416
movt r1, #0xe000
add r2, r2, #-1
str r1, [r2]


/* Make kernel domain 0 manager */
mov r0, #3
mcr p15, 0, r0, c3, c0, 0

mov r0, #0
/* Invalidate TLB */
mcr p15, 0, r0, c8, c7, 0
/* Invalidate I-Cache */
mcr p15, 0, r0, c7, c5, 0
/* Invalidate Branch Predictor Array */
mcr p15, 0, r0, c7, c5, 6
/* Invalidate D-cache */
MCR p15, 0, r0, c7, c14, 1

/* Read System Control Register */
mrc p15, 0, r0, c1, c0, 0
/* Enable MMU */
orr r0, r0, #1
/* Enable instruction cache*/
orr r0, r0, #(1<<12)
/* Enable branch prediction */
orr r0, r0, #(1<<11)
/* Enable data cache */
orr r0, r0, #(1<<2)

mcr p15, 0, r0, c1, c0, 0


ldr sp, =_bootstack



bl entry /* branch to rust */
