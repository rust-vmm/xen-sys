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
use libc::ioctl;

use crate::private::*;
use crate::sysctl_types::XenSysctl;

unsafe fn do_ioctl(data: *mut PrivCmdHypercall) -> Result<(), std::io::Error>
{
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(HYPERCALL_PRIVCMD)?;

    let ret = ioctl(fd.as_raw_fd(), crate::private::IOCTL_PRIVCMD_HYPERCALL, data);

    if ret == 0 {
        return Ok(());
    }

    Err(Error::last_os_error())
}

pub fn do_sysctl(xen_sys_ctl: &mut XenSysctl) ->  Result<(), std::io::Error> {
    let bouncebuffer = BounceBuffer::new(std::mem::size_of::<XenSysctl>())?;
    let vaddr = bouncebuffer.to_vaddr() as *mut XenSysctl;
    let mut privcmd_hypercall = PrivCmdHypercall {
        op: __HYPERVISOR_sysctl,
        arg: [vaddr as u64, 0, 0, 0, 0],
    };

    unsafe {
        // Write content of XenSysctl to the bounce buffer so that Xen knows what
        // we are asking for.
        vaddr.write(*xen_sys_ctl);

        do_ioctl(&mut privcmd_hypercall).map(|_| {
            // Read back content from bounce buffer if no errors.
            *xen_sys_ctl = vaddr.read();
        })
    }
}