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

/*
 * flush_dcache_range(start, end)
 * - x0(start) - start address of a region
 * - x1(end)   - end address of a region
 * Clobbers: x2, x3, x4
 */
ENTRY flush_dcache_range
    /* Do not modify x0 */
    mov    x4, x0
    /* Get the minimum D-cache line size */
    mrs    x3, ctr_el0
    ubfm   x3, x3, #16, #19
    mov    x2, #4
    lsl    x2, x2, x3
    sub    x3, x2, #1
    bic    x4, x4, x3
    /* Clean and invalidate D-cache line */
1:  dc     civac, x4
    add    x4, x4, x2
    cmp    x4, x1
    b.lo   1b
    dsb    sy
    ret
ENDFUNC flush_dcache_range
