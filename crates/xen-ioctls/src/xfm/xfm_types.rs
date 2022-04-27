/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use libc::c_void;

pub struct XenForeignMemoryResourceHandle {
    pub domid: u16,
    pub r#type: u32,
    pub id: u32,
    pub frame: u64,
    pub nr_frames: u64,
    pub addr: *mut c_void,
    pub prot: i32,
    pub flags: i32,
}
