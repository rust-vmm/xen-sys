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
use libc::{c_void, ioctl, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};

use crate::privcmd::*;
use crate::sysctl_types::XenSysctl;

const HYPERCALL_PRIVCMD: &str = "/dev/xen/privcmd";
const HYPERCALL_BUFFER_FILE: &str = "/dev/xen/hypercall";

fn round_up(value: u64, scale: u64) -> u64
{
    let mut ceiling: u64 = scale;

    while ceiling < value {
        ceiling += scale;
    }

    ceiling
}

unsafe fn do_ioctl(data: *mut PrivCmdHypercall) -> Result<(), std::io::Error>
{
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(HYPERCALL_PRIVCMD)?;

    let ret = ioctl(fd.as_raw_fd(), crate::privcmd::IOCTL_PRIVCMD_HYPERCALL, data);

    if ret == 0 {
        return Ok(());
    }

    Err(Error::last_os_error())
}

pub fn do_sysctl(xen_sys_ctl: &mut XenSysctl) ->  Result<(), std::io::Error>
{
    let mut ret:Result<(), std::io::Error> = Ok(());
    let size = round_up(std::mem::size_of::<XenSysctl>() as u64, PAGE_SIZE.into());
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(HYPERCALL_BUFFER_FILE)?;

    unsafe {
        // Setup a bounce buffer for Xen to use.
        let vaddr: *mut XenSysctl = mmap(0 as *mut c_void, size as usize, PROT_READ | PROT_WRITE, MAP_SHARED, fd.as_raw_fd(), 0) as *mut XenSysctl;

        // Function mmap() returns -1 in case of error.  Casting to i16 or i64
        // yield the same result.
        if *(vaddr as *mut i32) == -1 {
            return Err(Error::last_os_error());
        }

        // Write content of XenSysctl to the bounce buffer so that Xen knows what
        // we are asking for.
        vaddr.write(*xen_sys_ctl);

        let mut privcmd_hypercall = PrivCmdHypercall {
            op: __HYPERVISOR_sysctl,
            arg: [vaddr as u64, 0, 0, 0, 0],
        };

        match do_ioctl(&mut privcmd_hypercall) {
            // Read back content from bounce buffer if no errors.
            Ok(_) => *xen_sys_ctl = vaddr.read(),
            Err(err) => ret = Err(err),
        };

        if munmap(vaddr as *mut c_void, size as usize) < 0 {
            ret = Err(Error::last_os_error());
        }
    }

    ret
}