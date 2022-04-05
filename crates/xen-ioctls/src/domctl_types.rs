/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

#[cfg(target_arch = "x86_64")]
use crate::x86_64::types::*;
#[cfg(target_arch = "aarch64")]
use crate::aarch64::types::*;

// xen/include/public/domctl.h::struct xen_domctl_getdomaininfo
// sizeof(struct xen_domctl_get_domaininfo) == 120
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct XenDomctlGetDomainInfo {
    pub domain: u16,
    pad: u16,
    pub flags: u32,
    pub tot_pages: U64Aligned,
    pub max_pages: U64Aligned,
    pub outstanding_pages: U64Aligned,
    pub shr_pages: U64Aligned,
    pub paged_pages: U64Aligned,
    pub shared_info_frame: U64Aligned,
    pub cpu_time: U64Aligned,
    pub nr_online_vcpus: u32,
    pub max_vcpu_id: u32,
    pub ssidref: u32,
    pub handle: [u8; 16],
    pub cpupool: u32,
    pub gpaddr_bits: u8,
    pad2: [u8; 7],
    pub arch_config: XenArchDomainconfig,
}
