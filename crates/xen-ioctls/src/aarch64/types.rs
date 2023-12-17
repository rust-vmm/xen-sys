/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![allow(non_upper_case_globals)]

use std::fmt;

#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct U64Aligned {
    pub v: u64,
}

impl fmt::LowerHex for U64Aligned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.v;

        // delegate to i64's implementation
        fmt::LowerHex::fmt(&val, f)
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct XenArchDomainconfig {
    // IN/OUT
    pub gic_version: u8,
    // IN
    pub tee_type: u16,
    // In
    pub nr_spis: u32,
    // OUT
    // Based on the property clock-frequency in the DT timer node.
    // The property may be present when the bootloader/firmware doesn't
    // set correctly CNTFRQ which hold the timer freqency.
    //
    // As it's not possible to trap this register, we have to replicate
    // the value in the guest DT.
    //
    // = 0 => property not present
    // > 0 => Value of the property
    //
    pub clock_frequency: u32,
}
