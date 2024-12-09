/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use libc::{c_int, c_void, mmap, munmap, ENOENT, MAP_PRIVATE, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io::{Error, ErrorKind};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::{thread, time};

use crate::private::*;
use crate::xfm::types::*;
use crate::xfm::xfm_types::*;

fn map_from_address(
    addr: *mut c_void,
    size: usize,
    prot: i32,
    flags: i32,
    offset: i64,
) -> Result<*mut c_void, std::io::Error> {
    let fd = OpenOptions::new()
        .read(true)
        .write(true)
        .open(HYPERCALL_PRIVCMD)?;

    unsafe {
        let vaddr = mmap(addr, size, prot, flags, fd.as_raw_fd(), offset).cast::<c_void>();

        // Function mmap() returns -1 in case of error.  Casting to i16 or i64
        // yield the same result.
        if vaddr as i8 == -1 {
            return Err(Error::last_os_error());
        }

        Ok(vaddr)
    }
}

pub fn xenforeignmemory_map_resource(
    domid: u16,
    r#type: u32,
    id: u32,
    frame: u32,
    nr_frames: u64,
    addr: *mut c_void,
    prot: i32,
    flags: i32,
) -> Result<XenForeignMemoryResourceHandle, std::io::Error> {
    let mut privcmd_mmapresource = PrivCmdMmapResource {
        dom: domid,
        r#type,
        id,
        idx: frame,
        num: nr_frames,
        addr: ptr::null_mut(),
    };
    /*
     * The expression "&mut privcmd_mmapresource as *mut _" creates a reference
     * to privcmd_mmapresource before casting it to a *mut c_void
     */
    let privcmd_ptr: *mut c_void = &mut privcmd_mmapresource as *mut _ as *mut c_void;

    /* Check flags only contains POSIX defined values */
    if (flags & !(MAP_SHARED | MAP_PRIVATE)) != 0 {
        return Err(Error::new(ErrorKind::Other, "Invalid flags"));
    }

    if addr.is_null() && nr_frames != 0 {
        privcmd_mmapresource.addr = map_from_address(
            addr,
            (nr_frames << PAGE_SHIFT) as usize,
            PROT_READ | PROT_WRITE,
            flags | MAP_SHARED,
            0,
        )?;
    }

    unsafe {
        match do_ioctl(IOCTL_PRIVCMD_MMAP_RESOURCE(), privcmd_ptr) {
            Ok(_) => Ok(XenForeignMemoryResourceHandle {
                domid,
                r#type,
                id,
                frame: frame as u64,
                nr_frames: privcmd_mmapresource.num,
                addr: privcmd_mmapresource.addr,
                prot,
                flags,
            }),
            Err(e) => {
                if !addr.is_null()
                    && munmap(
                        privcmd_mmapresource.addr,
                        (nr_frames << PAGE_SHIFT) as usize,
                    ) < 0
                {
                    println!(
                        "Error {} unmapping vaddr: {:?}",
                        Error::last_os_error(),
                        privcmd_mmapresource.addr
                    );
                }

                Err(e)
            }
        }
    }
}

pub fn xenforeignmemory_unmap_resource(
    resource: &XenForeignMemoryResourceHandle,
) -> Result<(), std::io::Error> {
    unsafe {
        if munmap(resource.addr, (resource.nr_frames << PAGE_SHIFT) as usize) < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn retry_paged_pages(
    domid: u16,
    addr: *mut c_void,
    pages: u64,
    arr: *const u64,
    err: *mut c_int,
) -> Result<(), std::io::Error> {
    let mut i = 0;
    let mut batch_start = 0;

    unsafe {
        while i < pages {
            /* Look for the first page that faulted */
            while i < pages {
                if *err.add(i as usize) == -ENOENT {
                    batch_start = i;
                    break;
                }

                i += 1;
            }

            let mut privcmd_mmapbatch_v2 = PrivCmdMmapBatchV2 {
                num: 1,
                dom: domid,
                addr: (addr as *mut u8).add((i << PAGE_SHIFT) as usize) as *mut c_void,
                arr: arr.add(i as usize),
                err: err.add(i as usize),
            };

            i += 1;

            /* Try to lump as many_ consecutive_ faulted  pages in the same request */
            while i < pages {
                if *err.add(i as usize) != -ENOENT {
                    break;
                }

                i += 1;
                privcmd_mmapbatch_v2.num += 1;
            }

            /*
             * The expression "&mut privcmd_mmapbatch_v2 as *mut _" creates a reference
             * to privcmd_mmapbatch_v2 before casting it to a *mut c_void
             */
            let privcmd_ptr: *mut c_void = &mut privcmd_mmapbatch_v2 as *mut _ as *mut c_void;

            match do_ioctl(IOCTL_PRIVCMD_MMAPBATCH_V2(), privcmd_ptr) {
                Ok(_) => {
                    continue;
                }
                Err(e) => {
                    if e.raw_os_error() == Some(libc::ENOENT) {
                        /* Something went wrong with this batch, retry it */
                        i = batch_start;
                        thread::sleep(time::Duration::from_micros(100));
                        continue;
                    }

                    return Err(e);
                }
            }
        }

        Ok(())
    }
}

pub fn xenforeignmemory_map(
    domid: u16,
    prot: i32,
    pages: u64,
    arr: *const u64,
    err: *mut c_int,
) -> Result<*mut c_void, std::io::Error> {
    let mut err_vec: Vec<c_int> = vec![0; pages as usize];
    let mut err_array: *mut c_int = err;
    let num: u32 = pages.try_into().map_err(|_| ErrorKind::InvalidInput)?;

    let addr: *mut c_void = map_from_address(
        ptr::null_mut(),
        (pages << PAGE_SHIFT) as usize,
        prot,
        MAP_SHARED,
        0,
    )?;

    if err.is_null() {
        err_array = err_vec.as_mut_ptr();
    }

    let mut privcmd_mmapbatch_v2 = PrivCmdMmapBatchV2 {
        num,
        dom: domid,
        addr,
        arr,
        err: err_array,
    };
    /*
     * The expression "&mut privcmd_mmapbatch_v2 as *mut _" creates a reference
     * to privcmd_mmapbatch_v2 before casting it to a *mut c_void
     */
    let privcmd_ptr: *mut c_void = &mut privcmd_mmapbatch_v2 as *mut _ as *mut c_void;

    unsafe {
        match do_ioctl(IOCTL_PRIVCMD_MMAPBATCH_V2(), privcmd_ptr) {
            Ok(_) => Ok(addr),
            Err(e) => {
                if e.raw_os_error() != Some(libc::ENOENT) {
                    return Err(e);
                }

                retry_paged_pages(domid, addr, pages, arr, err_array).map(|_| addr)
            }
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn xenforeignmemory_unmap(addr: *mut c_void, pages: u64) -> Result<(), std::io::Error> {
    unsafe {
        if munmap(addr, (pages << PAGE_SHIFT) as usize) < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}
