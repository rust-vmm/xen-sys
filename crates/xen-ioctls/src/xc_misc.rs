/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::io::Error;
use std::fs::{OpenOptions};
use std::os::unix::io::AsRawFd;
use std::{slice};
use libc::{c_void, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};

#[cfg(target_arch = "x86_64")]
use crate::x86_64::types::*;
#[cfg(target_arch = "aarch64")]
use crate::aarch64::types::*;

use crate::private::*;
use crate::sysctl_types::*;
use crate::xen_types::*;
use crate::sysctl::do_sysctl;

pub fn get_domain_infolist(first_domain: u16, max_domain: u32) -> Result<Vec<XenDomctlGetDomainInfo>, std::io::Error> {
    let size = round_up((std::mem::size_of::<XenDomctlGetDomainInfo>() * max_domain as usize) as u64, PAGE_SIZE.into());
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(HYPERCALL_BUFFER_FILE)?;

    unsafe {
        // Setup a bounce buffer for Xen to use.
        let vaddr: *mut u8 = mmap(0 as *mut c_void, size as usize,
                                      PROT_READ | PROT_WRITE, MAP_SHARED, fd.as_raw_fd(), 0) as *mut u8;

        // Zero-out the memory we got from Xen.  This will give us a clean slate and add the pages
        // in the EL1 and EL2 page tables.  Otherwise the MMU throws and exception and Xen will
        // abort the transfer.
        vaddr.write_bytes(0, size as usize);

        let domaininfolist = XenSysctlGetdomaininfolist {
            first_domain,
            max_domain,
            buffer: U64Aligned {
                v: vaddr as u64,
            },
            num_domains: 0,
        };

        let mut sysctl = XenSysctl {
            cmd: XEN_SYSCTL_getdomaininfolist,
            interface_version: XEN_SYSCTL_INTERFACE_VERSION,
            u: XenSysctlPayload { domaininfolist },
        };

        match do_sysctl(&mut sysctl) {
            Ok(_) => {
                let domain_ptr: *mut XenDomctlGetDomainInfo = vaddr as *mut XenDomctlGetDomainInfo;
                let infolist = slice::from_raw_parts(domain_ptr, sysctl.u.domaininfolist.num_domains as usize).to_vec();

                if munmap(vaddr as *mut c_void, size as usize) < 0 {
                    Err(Error::last_os_error())
                } else {
                    Ok(infolist)
                }
            },
            Err(err) => {
                munmap(vaddr as *mut c_void, size as usize);
                Err(err)
            }
        }
    }
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