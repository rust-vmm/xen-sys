/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![feature(asm)]
#![no_std]

/// public types from xen/xen.h

type c_char = u8; /// really i8 but easier for porting
type c_long = i64;
type c_ulong = u64;
type xen_pfn_t = c_ulong;
type event_port = u32;

// export functionality
pub mod hypercall;
mod start_info;
pub use start_info::start_info as start_info_t;
