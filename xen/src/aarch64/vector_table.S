/*
 * Copyright 2021-2022 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Inspired by the XTF project:
 *
 *    https://xenbits.xen.org/gitweb/?p=xtf.git;a=summary
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

/* Save state */
.macro entry_trap
    sub     sp, sp, #(272 - 240)     /* offset: spsr_el1 - lr */
    stp     x28, x29, [sp, #-16]!
    stp     x26, x27, [sp, #-16]!
    stp     x24, x25, [sp, #-16]!
    stp     x22, x23, [sp, #-16]!
    stp     x20, x21, [sp, #-16]!
    stp     x18, x19, [sp, #-16]!
    stp     x16, x17, [sp, #-16]!
    stp     x14, x15, [sp, #-16]!
    stp     x12, x13, [sp, #-16]!
    stp     x10, x11, [sp, #-16]!
    stp     x8, x9, [sp, #-16]!
    stp     x6, x7, [sp, #-16]!
    stp     x4, x5, [sp, #-16]!
    stp     x2, x3, [sp, #-16]!
    stp     x0, x1, [sp, #-16]!

    add     x21, sp, #272           /* offset: spsr_el1 */
    stp     lr, x21, [sp, #240]     /* offset: lr */
    mrs     x21, elr_el1
    mrs     x22, spsr_el1
    stp     x21, x22, [sp, #256]    /* offset: pc */
.endm

/* Restore state */
.macro exit_trap
    ldp     x21, x22, [sp, #256]    /* offset: pc */
    ldp     x0, x1, [sp], #16
    ldp     x2, x3, [sp], #16
    ldp     x4, x5, [sp], #16
    ldp     x6, x7, [sp], #16
    ldp     x8, x9, [sp], #16

    msr     elr_el1, x21            /* set up the return data */
    msr     spsr_el1, x22

    ldp     x10, x11, [sp], #16
    ldp     x12, x13, [sp], #16
    ldp     x14, x15, [sp], #16
    ldp     x16, x17, [sp], #16
    ldp     x18, x19, [sp], #16
    ldp     x20, x21, [sp], #16
    ldp     x22, x23, [sp], #16
    ldp     x24, x25, [sp], #16
    ldp     x26, x27, [sp], #16
    ldp     x28, x29, [sp], #16

    ldr     lr, [sp], #32           /* offset: spsr_el1 - lr */
    eret
.endm

/* Bad abort numbers */
.equ BAD_SYNC,      0
.equ BAD_IRQ,       1
.equ BAD_FIQ,       2
.equ BAD_ERROR,     3

.macro invalid, reason
    mov     x0, sp
    mov     x1, #\reason
    b       do_bad_mode
.endm

/* BAD_ERROR */
el0_error_invalid:
    invalid BAD_ERROR
ENDFUNC el0_error_invalid

/* BAD_SYNC */
el1_sync_invalid:
    invalid BAD_SYNC
ENDFUNC el1_sync_invalid

/* BAD_IRQ */
el1_irq_invalid:
    invalid BAD_IRQ
ENDFUNC el1_irq_invalid

/* BAD_FIQ */
el1_fiq_invalid:
    invalid BAD_FIQ
ENDFUNC el1_fiq_invalid

/* BAD_ERROR */
el1_error_invalid:
    invalid BAD_ERROR
ENDFUNC el1_error_invalid

/* SYNC exception handler */
    .align 6
el1_sync:
    entry_trap
    mov     x0, sp
    bl      do_trap_sync
    exit_trap
ENDFUNC el1_sync

/* IRQ exception handler */
    .align 6
el1_irq:
    entry_trap
    mov     x0, sp
    bl      do_trap_irq
    exit_trap
ENDFUNC el1_irq

/* Exception vectors */
.macro ventry label
    .align  7
    b       \label
.endm

    .align  11
ENTRY vector_table
    /* Synchronous EL1t */
    ventry el1_sync_invalid
    /* IRQ EL1t */
    ventry el1_irq_invalid
    /* FIQ EL1t */
    ventry el1_fiq_invalid
    /* Error EL1t */
    ventry el1_error_invalid

    /* Synchronous EL1h */
    ventry el1_sync
    /* IRQ EL1h */
    ventry el1_irq
    /* FIQ EL1h */
    ventry el1_fiq_invalid
    /* Error EL1h */
    ventry el1_error_invalid

    /* Synchronous 64-bit EL0 */
    ventry el0_error_invalid
    /* IRQ 64-bit EL0 */
    ventry el0_error_invalid
    /* FIQ 64-bit EL0 */
    ventry el0_error_invalid
    /* Error 64-bit EL0 */
    ventry el0_error_invalid

    /* Synchronous 32-bit EL0 */
    ventry el0_error_invalid
    /* IRQ 32-bit EL0 */
    ventry el0_error_invalid
    /* FIQ 32-bit EL0 */
    ventry el0_error_invalid
    /* Error 32-bit EL0 */
    ventry el0_error_invalid
ENDFUNC vector_table
