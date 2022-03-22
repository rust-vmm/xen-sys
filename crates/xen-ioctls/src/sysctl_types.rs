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

pub const XEN_SYSCTL_INTERFACE_VERSION:u32 = 0x14;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// xen/include/public/sysctl.h::struct xen_sysctl_physinfo
pub struct XenSysctlPhysinfo {
    pub threads_per_core: u32,
    pub cores_per_socket: u32,
    pub nr_cpus: u32,
    pub max_cpu_id: u32,
    pub nr_nodes: u32,
    pub max_node_id: u32,
    pub cpu_khz: u32,
    pub capabilites: u32,
    pub total_pages: U64Aligned,
    pub free_pages: U64Aligned,
    pub scrub_pages: U64Aligned,
    pub outstanding_pages: U64Aligned,
    pub max_mfn: U64Aligned,
    pub hw_cap: [u32; 8],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// xen/include/public/sysctl.h::struct xen_sysctl_getdomaininfolist
pub struct XenSysctlGetdomaininfolist {
    pub first_domain: u16,
    pub max_domain: u32,
    pub buffer: U64Aligned,
    pub num_domains: u32,
}

pub const XEN_SYSCTL_readconsole: u32 = 1;
pub const XEN_SYSCTL_tbuf_op: u32 = 2;
pub const XEN_SYSCTL_physinfo: u32 = 3;
pub const XEN_SYSCTL_sched_id: u32 = 4;
pub const XEN_SYSCTL_perfc_op: u32 = 5;
pub const XEN_SYSCTL_getdomaininfolist: u32 = 6;
pub const XEN_SYSCTL_debug_keys: u32 = 7;
pub const XEN_SYSCTL_getcpuinfo: u32 = 8;
pub const XEN_SYSCTL_availheap: u32 = 9;
pub const XEN_SYSCTL_get_pmstat: u32 = 10;
pub const XEN_SYSCTL_cpu_hotplug: u32 = 11;
pub const XEN_SYSCTL_pm_op: u32 = 12;
pub const XEN_SYSCTL_page_offline_op: u32 = 14;
pub const XEN_SYSCTL_lockprof_op: u32 = 15;
pub const XEN_SYSCTL_cputopoinfo: u32 = 16;
pub const XEN_SYSCTL_numainfo: u32 = 17;
pub const XEN_SYSCTL_cpupool_op: u32 = 18;
pub const XEN_SYSCTL_scheduler_op: u32 = 19;
pub const XEN_SYSCTL_coverage_op: u32 = 20;
pub const XEN_SYSCTL_psr_cmt_op: u32 = 21;
pub const XEN_SYSCTL_pcitopoinfo: u32 = 22;
pub const XEN_SYSCTL_psr_alloc: u32 = 23;
/* pub const XEN_SYSCTL_tmem_op: u32 = 24; */
pub const XEN_SYSCTL_get_cpu_levelling_caps: u32 = 25;
pub const XEN_SYSCTL_get_cpu_featureset: u32 = 26;
pub const XEN_SYSCTL_livepatch_op: u32 = 27;
/* pub const XEN_SYSCTL_set_parameter: u32 = 28; */
pub const XEN_SYSCTL_get_cpu_policy: u32 = 29;

#[repr(C)]
#[derive(Copy, Clone)]
pub union XenSysctlPayload {
    pub domaininfolist: XenSysctlGetdomaininfolist,
    pub physinfo: XenSysctlPhysinfo,
    pad: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
// xen/include/public/sysctl.h::struct xen_sysctl
pub struct XenSysctl {
    pub cmd: u32,
    pub interface_version: u32, /* XEN_SYSCTL_INTERFACE_VERSION */
    pub u: XenSysctlPayload,
}