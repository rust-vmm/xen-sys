/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::{slice};

#[cfg(target_arch = "x86_64")]
use crate::x86_64::types::*;
#[cfg(target_arch = "aarch64")]
use crate::aarch64::types::*;

use crate::private::*;
use crate::sysctl_types::*;
use crate::xen_types::*;
use crate::sysctl::do_sysctl;

pub fn get_domain_infolist(first_domain: u16, max_domain: u32) -> Result<Vec<XenDomctlGetDomainInfo>, std::io::Error> {
    let bouncebuffer = BounceBuffer::new(std::mem::size_of::<XenDomctlGetDomainInfo>() * max_domain as usize)?;
    let vaddr = bouncebuffer.to_vaddr() as *mut XenDomctlGetDomainInfo;

    let mut sysctl = XenSysctl {
        cmd: XEN_SYSCTL_getdomaininfolist,
        interface_version: XEN_SYSCTL_INTERFACE_VERSION,
        u: XenSysctlPayload { domaininfolist: XenSysctlGetdomaininfolist {
                first_domain,
                max_domain,
                buffer: U64Aligned {
                    v: vaddr as u64,
                },
                num_domains: 0,
            },
        },
    };

    do_sysctl(&mut sysctl).map(|_| {
        unsafe {
            slice::from_raw_parts(vaddr, sysctl.u.domaininfolist.num_domains as usize).to_vec()
        }
    })
}

pub fn get_physinfo() -> Result<XenSysctlPhysinfo, std::io::Error>
{
    let mut sysctl = XenSysctl {
        cmd: XEN_SYSCTL_physinfo,
        interface_version: XEN_SYSCTL_INTERFACE_VERSION,
        u: XenSysctlPayload { physinfo: XenSysctlPhysinfo::default() },
    };

    do_sysctl(&mut sysctl).map(|_| unsafe { sysctl.u.physinfo.clone() })
}