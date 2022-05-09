/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use libc::{c_ulong, c_void, ioctl, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::fs::OpenOptions;
use std::io::Error;
use std::os::unix::io::AsRawFd;

pub const PAGE_SHIFT: u32 = 12;
pub const PAGE_SIZE: u32 = 1 << PAGE_SHIFT;

pub const __HYPERVISOR_SYSCTL: u64 = 35;
pub const __HYPERVISOR_DOMCTL: u64 = 36;

pub const IOCTL_PRIVCMD_MMAPBATCH_V2: c_ulong = 0x205004;
pub const IOCTL_MMAP_RESOURCE: c_ulong = 0x205007;
pub const IOCTL_PRIVCMD_HYPERCALL: c_ulong = 0x305000;

pub const HYPERCALL_PRIVCMD: &str = "/dev/xen/privcmd";
pub const HYPERCALL_BUFFER_FILE: &str = "/dev/xen/hypercall";

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct PrivCmdHypercall {
    pub op: u64,
    pub arg: [u64; 5],
}

pub(crate) struct BounceBuffer {
    vaddr: *mut c_void,
    size: usize,
}

impl BounceBuffer {
    pub(crate) fn new(size: usize) -> Result<BounceBuffer, std::io::Error> {
        let bounce_buffer_size = round_up(size as u64, PAGE_SIZE.into());
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .open(HYPERCALL_BUFFER_FILE)?;

        unsafe {
            // Setup a bounce buffer for Xen to use.
            let vaddr = mmap(
                0 as *mut c_void,
                bounce_buffer_size,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd.as_raw_fd(),
                0,
            ) as *mut u8;

            // Function mmap() returns -1 in case of error.  Casting to i16 or i64
            // yield the same result.
            if vaddr as i8 == -1 {
                return Err(Error::last_os_error());
            }

            // Zero-out the memory we got from Xen.  This will give us a clean slate and add the pages
            // in the EL1 and EL2 page tables.  Otherwise the MMU throws and exception and Xen will
            // abort the transfer.
            vaddr.write_bytes(0, bounce_buffer_size);

            Ok(BounceBuffer {
                vaddr: vaddr as *mut c_void,
                size: bounce_buffer_size,
            })
        }
    }

    pub(crate) fn to_vaddr(&self) -> *mut c_void {
        self.vaddr.clone()
    }
}

impl Drop for BounceBuffer {
    fn drop(&mut self) {
        unsafe {
            if munmap(self.vaddr, self.size) < 0 {
                println!(
                    "Error {} unmapping vaddr: {:?}",
                    Error::last_os_error(),
                    self.vaddr
                );
            }
        }
    }
}

pub fn round_up(value: u64, scale: u64) -> usize {
    let mut ceiling: u64 = scale;

    while ceiling < value {
        ceiling += scale;
    }

    ceiling as usize
}

pub(crate) unsafe fn do_ioctl(request: c_ulong, data: *mut c_void) -> Result<(), std::io::Error> {
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(HYPERCALL_PRIVCMD)?;

    let ret = ioctl(fd.as_raw_fd(), request, data);

    if ret == 0 {
        return Ok(());
    }

    Err(Error::last_os_error())
}
