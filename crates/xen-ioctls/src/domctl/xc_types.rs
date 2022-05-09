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

use std::convert::TryFrom;

#[cfg(target_arch = "aarch64")]
use crate::aarch64::types::*;
#[cfg(target_arch = "x86_64")]
use crate::x86_64::types::*;

use crate::domctl::types::*;
use crate::private::PAGE_SHIFT;

#[derive(Debug, Default, Copy, Clone)]
// tools/include/xenctrl.h::xc_dominfo
pub struct XcDominfo {
    pub domid: u16,
    pub ssidref: u32,
    pub dying: bool,
    pub crashed: bool,
    pub shutdown: bool,
    pub paused: bool,
    pub blocked: bool,
    pub running: bool,
    pub hvm: bool,
    pub debugged: bool,
    pub xenstore: bool,
    pub hap: bool,
    pub shutdown_reason: u32, /* only meaningful if shutdown==1 */
    pub nr_pages: u64,        /* current number, not maximum */
    pub nr_outstanding_pages: u64,
    pub nr_shared_pages: u64,
    pub nr_paged_pages: u64,
    pub shared_info_frame: u64,
    pub cpu_time: u64,
    pub max_memkb: u64,
    pub nr_online_vcpus: u32,
    pub max_vcpu_id: u32,
    pub handle: [u8; 16],
    pub cpupool: u32,
    pub gpaddr_bits: u8,
    pub arch_config: XenArchDomainconfig,
}

impl TryFrom<XenDomctlGetDomainInfo> for XcDominfo {
    type Error = ();

    fn try_from(info: XenDomctlGetDomainInfo) -> Result<Self, Self::Error> {
        let shutdown_reason: u32 =
            (info.flags >> XEN_DOMINF_shutdownshift) & XEN_DOMINF_shutdownmask;

        let mut shutdown: bool = (info.flags & XEN_DOMINF_shutdown) != 0;
        let mut crashed: bool = false;
        if shutdown && shutdown_reason == SHUTDOWN_crash {
            shutdown = false;
            crashed = true;
        }

        Ok(XcDominfo {
            domid: info.domain,
            dying: (info.flags & XEN_DOMINF_dying) != 0,
            crashed,
            shutdown,
            paused: (info.flags & XEN_DOMINF_paused) != 0,
            blocked: (info.flags & XEN_DOMINF_blocked) != 0,
            running: (info.flags & XEN_DOMINF_running) != 0,
            hvm: (info.flags & XEN_DOMINF_hvm_guest) != 0,
            debugged: (info.flags & XEN_DOMINF_debugged) != 0,
            xenstore: (info.flags & XEN_DOMINF_xs_domain) != 0,
            hap: (info.flags & XEN_DOMINF_hap) != 0,
            shutdown_reason,
            ssidref: info.ssidref,
            nr_pages: info.tot_pages.v,
            nr_outstanding_pages: info.outstanding_pages.v,
            nr_shared_pages: info.shr_pages.v,
            nr_paged_pages: info.paged_pages.v,
            max_memkb: info.max_pages.v << (PAGE_SHIFT - 10),
            shared_info_frame: info.shared_info_frame.v,
            cpu_time: info.cpu_time.v,
            nr_online_vcpus: info.nr_online_vcpus,
            max_vcpu_id: info.max_vcpu_id,
            handle: info.handle,
            cpupool: info.cpupool,
            gpaddr_bits: info.gpaddr_bits,
            arch_config: info.arch_config,
        })
    }
}
