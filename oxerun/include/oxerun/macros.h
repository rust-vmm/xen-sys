/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#ifndef __XR_MACROS_H__
#define __XR_MACROS_H__

#define GLOBAL(name) \
    .globl name; \
name:

#define SIZE(name) \
    .size name, . - name;

#define ENDFUNC(name) \
    .type name, STT_FUNC; \
    SIZE(name)

#define ELFNOTE(type, desc)         \
    .pushsection .note.Xen;       ; \
    .align 4                      ; \
    .long 2f - 1f /* name size */ ; \
    .long 4f - 3f /* desc size */ ; \
    .long type    /* type */      ; \
1:.asciz "Xen"                    ; \
2:.align 4                        ; \
3:desc                            ; \
4:.align 4                        ; \
    .popsection

#endif /* __XR_MACROS_H__ */
