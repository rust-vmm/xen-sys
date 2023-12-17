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

.macro ALIGN
    .align 2
.endm

.macro GLOBAL, name
    .globl \name
\name:
.endm

.macro ENTRY, name
    ALIGN;
    GLOBAL \name
.endm

.macro SIZE, name
    .size \name, . - \name
.endm

.macro ENDFUNC, name
    .type \name, STT_FUNC;
    SIZE \name
.endm
