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

/* 4kB pages */
.equ PAGE_SHIFT,            12
.equ PAGE_SIZE,             (1 << PAGE_SHIFT)
.equ PAGE_MASK,             (~(PAGE_SIZE - 1))

.equ STACK_ORDER,           2
.equ STACK_SIZE,            (PAGE_SIZE << STACK_ORDER)

/* 1 if BE, 0 if LE */
.equ HEAD_FLAG_ENDIANNESS,  0
.equ HEAD_FLAG_PAGE_SIZE,   ((PAGE_SHIFT - 10) / 2)
.equ HEAD_FLAG_PHYS_BASE,   1
.equ HEAD_FLAGS,            ((HEAD_FLAG_ENDIANNESS << 0) | (HEAD_FLAG_PAGE_SIZE << 1) | (HEAD_FLAG_PHYS_BASE << 3))

/* Memory attributes */
.equ MAIR0VAL,              0xeeaa4400
.equ MAIR1VAL,              0xff000004
.equ MAIRVAL,               (MAIR1VAL << 32 | MAIR0VAL)

/* Translation control register */
.equ TCR_EL1,               0x10b5193119

.section ".bss.page_aligned"
.p2align PAGE_SHIFT

stack_start:
    .space STACK_SIZE
stack_end:

.text
    b       _start                  /* branch to kernel start, magic */
    .long   0                       /* Executable code */
    .quad   0x0                     /* Image load offset from start of RAM */
    .quad   _end - _start           /* Effective Image size */
    .quad   HEAD_FLAGS              /* Informative flags, little-endian */
    .quad   0                       /* reserved */
    .quad   0                       /* reserved */
    .quad   0                       /* reserved */
    .byte   0x41                    /* Magic number, "ARM\x64" */
    .byte   0x52
    .byte   0x4d
    .byte   0x64
    .long   0                       /* reserved */


ENTRY _start
    /* Disable all IRQs */
    msr     daifset, #0xf

/* Disable MMU */
    bl      _mmu_disable

    /* Calculate where we are */
    ldr     x22, =_start        /* x22 := vaddr(_start) */
    adr     x21, _start         /* x21 := paddr(_start) */
    sub     x21, x21, x22       /* x21 := phys-offset */

    bl      _cpu_setup

    /* Load the vector table */
    ldr     x2, =vector_table
    msr     vbar_el1, x2

    ldr     x0, =__start_bss
    ldr     x1, =__end_bss

    bl      flush_dcache_range
1:  str     xzr, [x0], #8
    cmp     x0, x1
    b.lo    1b

    /* Load BSS start address again as x0 has been modified in the upper loop */
    ldr     x0, =__start_bss
    bl      flush_dcache_range

    ldr     x1, =stack_end
    mov     sp, x1

    b       kernel_main
ENDFUNC _start

ENTRY _mmu_disable
    dsb     sy

    /* Turn off D-cache and MMU */
    mrs     x2, sctlr_el1
    bic     x2, x2, #1 << 0     /* clear SCTLR.M */
    bic     x2, x2, #1 << 2     /* clear SCTLR.C */
    msr     sctlr_el1, x2
    isb

    ret
ENDFUNC _mmu_disable

ENTRY _cpu_setup
    dsb     sy

    /* Set up memory attribute type tables */
    ldr     x0, =MAIRVAL
    msr     mair_el1, x0

    /* Set up TCR_EL1 register */
    ldr     x0, =TCR_EL1
    mrs     x1, ID_AA64MMFR0_EL1
    /* Set TCR_EL1.IPS to ID_AA64MMFR0_EL1.PARange */
    bfi     x0, x1, #32, #3
    msr     tcr_el1, x0
    isb

    ret
ENDFUNC _cpu_setup