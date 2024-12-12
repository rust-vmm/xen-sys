/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#[cfg(target_arch = "aarch64")]
use crate::aarch64::types::*;
#[cfg(target_arch = "x86_64")]
use crate::x86_64::types::*;
use crate::{domctl::types::*, private::*, sysctl::types::*};

fn do_sysctl(xen_sysctl: &mut XenSysctl) -> Result<(), std::io::Error> {
    let bouncebuffer = BounceBuffer::new(std::mem::size_of::<XenSysctl>())?;
    let vaddr = bouncebuffer.vaddr() as *mut XenSysctl;
    let mut privcmd = PrivCmdHypercall {
        op: __HYPERVISOR_SYSCTL,
        arg: [vaddr as u64, 0, 0, 0, 0],
    };

    // Write content of XenSysctl to the bounce buffer so that Xen knows what
    // we are asking for.
    // SAFETY: vaddr points to valid memory allocated when we created the bounce
    // buffer
    unsafe { vaddr.write(*xen_sysctl) };

    // SAFETY: we pass a PrivCmdHypercall value to an IOCTL_PRIVCMD_HYPERCALL ioctl
    unsafe {
        do_ioctl(
            IOCTL_PRIVCMD_HYPERCALL(),
            std::ptr::addr_of_mut!(privcmd).cast(),
        )?
    };
    // Read back content from bounce buffer if no errors.
    // SAFETY: the ioctl succeeded and vaddr was previously successfully allocated
    // by us
    *xen_sysctl = unsafe { vaddr.read() };
    Ok(())
}

pub fn xc_physinfo() -> Result<XenSysctlPhysinfo, std::io::Error> {
    let mut sysctl = XenSysctl {
        cmd: XEN_SYSCTL_physinfo,
        interface_version: XEN_SYSCTL_INTERFACE_VERSION,
        u: XenSysctlPayload {
            physinfo: XenSysctlPhysinfo::default(),
        },
    };

    do_sysctl(&mut sysctl)?;
    Ok(
        // SAFETY: sysctl was successful, and we initialized the union ourselves.
        unsafe { sysctl.u.physinfo },
    )
}

pub fn xc_domain_getinfolist(
    first_domain: u16,
    max_domain: u32,
) -> Result<Vec<XenDomctlGetDomainInfo>, std::io::Error> {
    let bouncebuffer =
        BounceBuffer::new(std::mem::size_of::<XenDomctlGetDomainInfo>() * max_domain as usize)?;

    let mut sysctl = XenSysctl {
        cmd: XEN_SYSCTL_getdomaininfolist,
        interface_version: XEN_SYSCTL_INTERFACE_VERSION,
        u: XenSysctlPayload {
            domaininfolist: XenSysctlGetdomaininfolist {
                first_domain,
                max_domain,
                buffer: U64Aligned {
                    v: bouncebuffer.vaddr() as u64,
                },
                num_domains: 0,
            },
        },
    };

    do_sysctl(&mut sysctl)?;
    Ok(
        // SAFETY: sysctl was successful, so bounce buffer must be populated with num_domains
        // elements
        unsafe { bouncebuffer.into_vec(sysctl.u.domaininfolist.num_domains as usize) },
    )
}
