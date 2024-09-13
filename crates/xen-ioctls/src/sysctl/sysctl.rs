/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::slice;

use libc::c_void;

#[cfg(target_arch = "aarch64")]
use crate::aarch64::types::*;
#[cfg(target_arch = "x86_64")]
use crate::x86_64::types::*;

use crate::domctl::types::*;
use crate::private::*;
use crate::sysctl::types::*;

fn do_sysctl(xen_sysctl: &mut XenSysctl) -> Result<(), std::io::Error> {
    let bouncebuffer = BounceBuffer::new(std::mem::size_of::<XenSysctl>())?;
    let vaddr = bouncebuffer.to_vaddr() as *mut XenSysctl;
    let mut privcmd_hypercall = PrivCmdHypercall {
        op: __HYPERVISOR_SYSCTL,
        arg: [vaddr as u64, 0, 0, 0, 0],
    };
    /*
     * The expression "&mut privcmd_hypercall as *mut _" creates a reference
     * to privcmd_hypercall before casting it to a *mut c_void
     */
    let privcmd_ptr: *mut c_void = &mut privcmd_hypercall as *mut _ as *mut c_void;

    unsafe {
        // Write content of XenSysctl to the bounce buffer so that Xen knows what
        // we are asking for.
        vaddr.write(*xen_sysctl);

        do_ioctl(IOCTL_PRIVCMD_HYPERCALL(), privcmd_ptr).map(|_| {
            // Read back content from bounce buffer if no errors.
            *xen_sysctl = vaddr.read();
        })
    }
}

pub fn xc_physinfo() -> Result<XenSysctlPhysinfo, std::io::Error> {
    let mut sysctl = XenSysctl {
        cmd: XEN_SYSCTL_physinfo,
        interface_version: XEN_SYSCTL_INTERFACE_VERSION,
        u: XenSysctlPayload {
            physinfo: XenSysctlPhysinfo::default(),
        },
    };

    do_sysctl(&mut sysctl).map(|_| unsafe { sysctl.u.physinfo })
}

pub fn xc_domain_getinfolist(
    first_domain: u16,
    max_domain: u32,
) -> Result<Vec<XenDomctlGetDomainInfo>, std::io::Error> {
    let bouncebuffer =
        BounceBuffer::new(std::mem::size_of::<XenDomctlGetDomainInfo>() * max_domain as usize)?;
    let vaddr = bouncebuffer.to_vaddr() as *mut XenDomctlGetDomainInfo;

    let mut sysctl = XenSysctl {
        cmd: XEN_SYSCTL_getdomaininfolist,
        interface_version: XEN_SYSCTL_INTERFACE_VERSION,
        u: XenSysctlPayload {
            domaininfolist: XenSysctlGetdomaininfolist {
                first_domain,
                max_domain,
                buffer: U64Aligned { v: vaddr as u64 },
                num_domains: 0,
            },
        },
    };

    do_sysctl(&mut sysctl).map(|_| unsafe {
        slice::from_raw_parts(vaddr, sysctl.u.domaininfolist.num_domains as usize).to_vec()
    })
}
