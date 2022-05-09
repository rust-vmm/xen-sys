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

use libc::{c_int, c_void};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// tools/include/xen-sys/Linux/privcmd.h::privcmd_mmap_resource
// sizeof(struct privcmd_mmap_resource) == 32
pub struct PrivCmdMmapResource {
    pub dom: u16,
    pub r#type: u32,
    pub id: u32,
    pub idx: u32,
    pub num: u64,
    pub addr: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// tools/include/xen-sys/Linux/privcmd.h::privcmd_mmapbatch_v2
// sizeof(privcmd_mmapbatch_v2) == 32
pub struct PrivCmdMmapBatchV2 {
    pub num: u32,
    pub dom: u16,
    pub addr: *mut c_void,
    pub arr: *const u64,
    pub err: *mut c_int,
}
