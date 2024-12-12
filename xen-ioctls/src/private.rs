/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::{convert::TryInto, fs::OpenOptions, io::Error, os::unix::io::AsRawFd};

use libc::{c_ulong, c_void, ioctl, mmap, munmap, MAP_SHARED, PROT_READ, PROT_WRITE};
use vmm_sys_util::ioctl::{_IOC_NONE, _IOC_WRITE};

use crate::{
    xdm::types::{PrivcmdDeviceModelIoeventFd, PrivcmdDeviceModelIrqFd, PrivcmdDeviceModelOp},
    xfm::types::{PrivCmdMmapBatchV2, PrivCmdMmapResource},
};

pub const PAGE_SHIFT: u32 = 12;
pub const PAGE_SIZE: u32 = 1 << PAGE_SHIFT;

pub const __HYPERVISOR_SYSCTL: u64 = 35;
pub const __HYPERVISOR_DOMCTL: u64 = 36;

pub const XEN_PRIVCMD_TYPE: u32 = 'P' as u32;

/*
 * #define IOCTL_PRIVCMD_HYPERCALL \
 *      _IOC(_IOC_NONE, 'P', 0, sizeof(privcmd_hypercall_t))
 */
ioctl_ioc_nr!(
    IOCTL_PRIVCMD_HYPERCALL,
    _IOC_NONE,
    XEN_PRIVCMD_TYPE,
    0_u32,
    std::mem::size_of::<PrivCmdHypercall>() as u32
);

/*
 * #define IOCTL_PRIVCMD_MMAPBATCH_V2 \
 *      _IOC(_IOC_NONE, 'P', 4, sizeof(privcmd_mmapbatch_v2_t))
 */
ioctl_ioc_nr!(
    IOCTL_PRIVCMD_MMAPBATCH_V2,
    _IOC_NONE,
    XEN_PRIVCMD_TYPE,
    4_u32,
    std::mem::size_of::<PrivCmdMmapBatchV2>() as u32
);

/*
 * #define IOCTL_PRIVCMD_DM_OP \
 *      _IOC(_IOC_NONE, 'P', 5, sizeof(privcmd_dm_op))
 */
ioctl_ioc_nr!(
    IOCTL_PRIVCMD_DM_OP,
    _IOC_NONE,
    XEN_PRIVCMD_TYPE,
    5_u32,
    std::mem::size_of::<PrivcmdDeviceModelOp>() as u32
);

/*
 * #define IOCTL_PRIVCMD_MMAP_RESOURCE \
 *      _IOC(_IOC_NONE, 'P', 7, sizeof(privcmd_mmapbatch_v2_t))
 */
ioctl_ioc_nr!(
    IOCTL_PRIVCMD_MMAP_RESOURCE,
    _IOC_NONE,
    XEN_PRIVCMD_TYPE,
    7_u32,
    std::mem::size_of::<PrivCmdMmapResource>() as u32
);

/*
 * #define IOCTL_PRIVCMD_IRQFD  _IOW('P', 8, privcmd_irqfd_t)
 */
ioctl_ioc_nr!(
    IOCTL_PRIVCMD_IRQFD,
    _IOC_WRITE,
    XEN_PRIVCMD_TYPE,
    8_u32,
    std::mem::size_of::<PrivcmdDeviceModelIrqFd>() as u32
);

/*
 * #define IOCTL_PRIVCMD_IOEVENTFD  _IOW('P', 9, privcmd_ioeventfd_t)
 */
ioctl_ioc_nr!(
    IOCTL_PRIVCMD_IOEVENTFD,
    _IOC_WRITE,
    XEN_PRIVCMD_TYPE,
    9_u32,
    std::mem::size_of::<PrivcmdDeviceModelIoeventFd>() as u32
);

pub const HYPERCALL_EVTCHN: &str = "/dev/xen/evtchn";
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

        // Setup a bounce buffer for Xen to use.
        // SAFETY: `fd is a valid HYPERCALL_BUFFER_FILE descriptor
        let vaddr = unsafe {
            mmap(
                core::ptr::null_mut::<c_void>(),
                bounce_buffer_size,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd.as_raw_fd(),
                0,
            )
        } as *mut u8;

        // Function mmap() returns -1 in case of error.  Casting to i16 or i64
        // yield the same result.
        if vaddr as i8 == -1 {
            return Err(Error::last_os_error());
        }

        // Zero-out the memory we got from Xen.  This will give us a clean slate and add
        // the pages in the EL1 and EL2 page tables.  Otherwise the MMU
        // throws an exception and Xen will abort the transfer.
        // SAFETY: vaddr was mmapped and we checked mmap return value for errors.
        unsafe { vaddr.write_bytes(0, bounce_buffer_size) };

        Ok(BounceBuffer {
            vaddr: vaddr as *mut c_void,
            size: bounce_buffer_size,
        })
    }

    pub(crate) fn vaddr(&self) -> *mut c_void {
        self.vaddr
    }

    pub(crate) unsafe fn into_vec<T: Clone>(self, len: usize) -> Vec<T> {
        assert!(len * std::mem::size_of::<T>() < self.size);
        core::slice::from_raw_parts::<T>(self.vaddr.cast(), len).to_vec()
    }
}

impl Drop for BounceBuffer {
    fn drop(&mut self) {
        // SAFETY: we allocated self.vaddr in Self::new
        if unsafe { munmap(self.vaddr, self.size) } < 0 {
            println!(
                "Error {} unmapping vaddr: {:?}",
                Error::last_os_error(),
                self.vaddr
            );
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

    let ret = ioctl(
        fd.as_raw_fd(),
        #[allow(clippy::useless_conversion)]
        request.try_into().unwrap(),
        data,
    );

    if ret == 0 {
        return Ok(());
    }

    Err(Error::last_os_error())
}
