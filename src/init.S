.section .text.init
.global _start
_start:
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
ldr r1, =_kern_page_table
add r2, r1, #(1<<14)

page_loop:
    cmp r1, r2
    strlt r0, [r1], #4
    blt page_loop

ldr r1, =_kern_page_table
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


/* Enable MMU */
mrc p15, 0, r0, c1, c0, 0
orr r0, r0, #1
mcr p15, 0, r0, c1, c0, 0

/* Enable I-cache and D-cache */
/*movw r0, #0x1004
mcr p15, 0, r0, c1, c0, 0
dsb
isb*/

ldr sp, =bootstack_top



bl entry /* branch to rust */
