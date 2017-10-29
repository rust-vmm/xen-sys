/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#include <oxerun/macros.h>
#include <xen/elfnote.h>

/* Eventually the target crate name and version */
ELFNOTE(XEN_ELFNOTE_GUEST_OS, .asciz "oxerun")
ELFNOTE(XEN_ELFNOTE_GUEST_VERSION, .asciz "0.0.0")

/* We want the PV loader */
ELFNOTE(XEN_ELFNOTE_LOADER, .asciz "generic")
ELFNOTE(XEN_ELFNOTE_HYPERCALL_PAGE, .quad HYPERCALL_PAGE)

/* Xen ABI information */
ELFNOTE(XEN_ELFNOTE_XEN_VERSION, .asciz "xen-3.0")
ELFNOTE(XEN_ELFNOTE_FEATURES, .asciz "!writable_page_tables|pae_pgdir_above_4gb")
ELFNOTE(XEN_ELFNOTE_PAE_MODE, .asciz "yes")

        .text
        .code64

 /* PV entry point. */
GLOBAL(_start)
        /* Move onto our own stack. */
        mov $STACK_TOP, %rbp
        mov %rbp, %rsp

        /* Put the start_info_t pointer from domain build
         * into RDI to be the first arg in rust_entry
         */
        mov %rsi, %rdi

        /* our entry point into Rust code, not the user's app */
        call rust_entry

        /* poweroff() if rust_entry manages to return. */
        call poweroff
ENDFUNC(_start)
