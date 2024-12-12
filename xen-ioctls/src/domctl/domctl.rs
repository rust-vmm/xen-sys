/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::convert::TryFrom;

use crate::{
    domctl::{types::*, xc_types::XcDominfo},
    private::*,
};

fn do_domctl(xen_domctl: &mut XenDomctl) -> Result<(), std::io::Error> {
    let bouncebuffer = BounceBuffer::new(std::mem::size_of::<XenDomctl>())?;
    let vaddr = bouncebuffer.vaddr() as *mut XenDomctl;
    let mut privcmd = PrivCmdHypercall {
        op: __HYPERVISOR_DOMCTL,
        arg: [vaddr as u64, 0, 0, 0, 0],
    };
    // Write content of XenDomctl to the bounce buffer so that Xen knows what
    // we are asking for.
    // SAFETY: vaddr is a valid bounce buffer of XenDomctl size
    unsafe { vaddr.write(*xen_domctl) };

    // SAFETY: we pass a PrivCmdHypercall sized value to an IOCTL_PRIVCMD_HYPERCALL
    // ioctl
    unsafe {
        do_ioctl(
            IOCTL_PRIVCMD_HYPERCALL(),
            std::ptr::addr_of_mut!(privcmd).cast(),
        )?
    };
    // Read back content from bounce buffer if no errors.
    // SAFETY: vaddr is a valid bounce buffer of XenDomctl size
    *xen_domctl = unsafe { vaddr.read() };
    Ok(())
}

pub fn xc_domain_info(first_domain: u16, max_domain: u32) -> Vec<XcDominfo> {
    let mut vec = Vec::new();
    let mut domain = first_domain;

    for _ in 0..max_domain {
        let mut domctl = XenDomctl {
            cmd: XEN_DOMCTL_getdomaininfo,
            interface_version: XEN_DOMCTL_INTERFACE_VERSION,
            domain,
            pad: [0; 3],
            u: XenDomctlPayload {
                domaininfo: XenDomctlGetDomainInfo::default(),
            },
        };

        match do_domctl(&mut domctl) {
            Ok(()) => {
                if let Ok(dominfo) = XcDominfo::try_from(
                    // SAFETY: domctl was successful and the union is a XenDomctlPayload variant
                    unsafe { domctl.u.domaininfo },
                ) {
                    vec.push(dominfo);
                }
            }
            Err(err) if err.raw_os_error() == Some(libc::EACCES) => {
                eprintln!(
                    "Xen DOMCTL failed: {}\nCheck if XEN_DOMCTL_INTERFACE_VERSION in your Xen \
                     build matches the expected value of this xen-ioctls build: {:#04x}",
                    err, XEN_DOMCTL_INTERFACE_VERSION
                );
            }
            Err(err) => {
                eprintln!("Xen DOMCTL failed: {}", err);
            }
        }

        domain += 1;
    }

    vec
}
