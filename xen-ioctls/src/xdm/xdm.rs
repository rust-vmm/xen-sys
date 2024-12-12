/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::{
    fs::{File, OpenOptions},
    io::Error,
    os::unix::io::AsRawFd,
};

use libc::{c_void, ioctl};
use vmm_sys_util::eventfd::EventFd;
use xen_bindings::bindings::ioreq;

#[cfg(target_arch = "aarch64")]
use crate::aarch64::types::*;
#[cfg(target_arch = "x86_64")]
use crate::x86_64::types::*;
use crate::{private::*, xdm::types::*};

fn do_dm_op(
    fd: &File,
    domid: u16,
    privcmd_dm_op_buffers: &mut Vec<PrivcmdDeviceModelOpBuffer>,
) -> Result<(), std::io::Error> {
    let mut privcmd_dm_op = PrivcmdDeviceModelOp::new(domid, privcmd_dm_op_buffers);
    /*
     * The expression "&mut privcmd_dm_op as *mut _" creates a reference
     * to privcmd_dm_op before casting it to a *mut c_void
     */
    let privcmd_dm_op_ptr: *mut c_void = &mut privcmd_dm_op as *mut _ as *mut c_void;

    unsafe {
        let ret = ioctl(fd.as_raw_fd(), IOCTL_PRIVCMD_DM_OP(), privcmd_dm_op_ptr);
        if ret < 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(())
}

pub struct XenDeviceModelHandle {
    fd: File,
}

impl XenDeviceModelHandle {
    pub fn new() -> Result<Self, std::io::Error> {
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .open(HYPERCALL_PRIVCMD)?;

        let mut privcmd_dm_op_buffers = Vec::new();

        do_dm_op(&fd, DOM_INVALID, &mut privcmd_dm_op_buffers).map(|_| XenDeviceModelHandle { fd })
    }

    pub fn nr_vcpus(&self, domid: u16) -> Result<u32, std::io::Error> {
        let mut dm_op = XenDeviceModelOp {
            op: XEN_DMOP_nr_vcpus,
            pad: 0,
            u: XenDeviceModelOpPayload {
                xen_dm_op_nr_vcpus: XenDeviceModelNrVcpus { vcpus: 0 },
            },
        };

        let mut privcmd_dm_op_buffers = vec![PrivcmdDeviceModelOpBuffer {
            uptr: &mut dm_op as *mut _ as *mut c_void,
            size: std::mem::size_of::<XenDeviceModelOp>(),
        }];

        do_dm_op(&self.fd, domid, &mut privcmd_dm_op_buffers)
            .map(|_| unsafe { dm_op.u.xen_dm_op_nr_vcpus.vcpus })
    }

    pub fn create_ioreq_server(
        &self,
        domid: u16,
        handle_bufioreq: u8,
    ) -> Result<u16, std::io::Error> {
        let mut dm_op = XenDeviceModelOp {
            op: XEN_DMOP_create_ioreq_server,
            pad: 0,
            u: XenDeviceModelOpPayload {
                xen_create_ioreq_server: XenDeviceModelCreateIoreqServer {
                    handle_bufioreq,
                    ..Default::default()
                },
            },
        };

        let mut privcmd_dm_op_buffers = vec![PrivcmdDeviceModelOpBuffer {
            uptr: &mut dm_op as *mut _ as *mut c_void,
            size: std::mem::size_of::<XenDeviceModelOp>(),
        }];

        do_dm_op(&self.fd, domid, &mut privcmd_dm_op_buffers)
            .map(|_| unsafe { dm_op.u.xen_create_ioreq_server.id })
    }

    fn do_io_range_to_ioreq_server(
        &self,
        op: u32,
        domid: u16,
        id: u16,
        is_mmio: i32,
        start: u64,
        end: u64,
    ) -> Result<(), std::io::Error> {
        let r#type = match is_mmio {
            0 => XEN_DMOP_IO_RANGE_PORT,
            _ => XEN_DMOP_IO_RANGE_MEMORY,
        };

        let mut dm_op = XenDeviceModelOp {
            op,
            pad: 0,
            u: XenDeviceModelOpPayload {
                xen_ioreq_server_range: XenDeviceModelIoreqServerRange {
                    id,
                    pad: 0,
                    r#type,
                    start: U64Aligned { v: start },
                    end: U64Aligned { v: end },
                },
            },
        };

        let mut privcmd_dm_op_buffers = vec![PrivcmdDeviceModelOpBuffer {
            uptr: &mut dm_op as *mut _ as *mut c_void,
            size: std::mem::size_of::<XenDeviceModelOp>(),
        }];

        do_dm_op(&self.fd, domid, &mut privcmd_dm_op_buffers)
    }

    pub fn map_io_range_to_ioreq_server(
        &self,
        domid: u16,
        id: u16,
        is_mmio: i32,
        start: u64,
        end: u64,
    ) -> Result<(), std::io::Error> {
        self.do_io_range_to_ioreq_server(
            XEN_DMOP_map_io_range_to_ioreq_server,
            domid,
            id,
            is_mmio,
            start,
            end,
        )
    }

    pub fn unmap_io_range_from_ioreq_server(
        &self,
        domid: u16,
        id: u16,
        is_mmio: i32,
        start: u64,
        end: u64,
    ) -> Result<(), std::io::Error> {
        self.do_io_range_to_ioreq_server(
            XEN_DMOP_unmap_io_range_from_ioreq_server,
            domid,
            id,
            is_mmio,
            start,
            end,
        )
    }

    pub fn set_ioreq_server_state(
        &self,
        domid: u16,
        id: u16,
        enabled: i32,
    ) -> Result<(), std::io::Error> {
        let mut dm_op = XenDeviceModelOp {
            op: XEN_DMOP_set_ioreq_server_state,
            pad: 0,
            u: XenDeviceModelOpPayload {
                xen_set_ioreq_server_state: XenDeviceModelSetIoreqServerState {
                    id,
                    enabled: enabled as u8,
                    ..Default::default()
                },
            },
        };

        let mut privcmd_dm_op_buffers = vec![PrivcmdDeviceModelOpBuffer {
            uptr: &mut dm_op as *mut _ as *mut c_void,
            size: std::mem::size_of::<XenDeviceModelOp>(),
        }];

        do_dm_op(&self.fd, domid, &mut privcmd_dm_op_buffers)
    }

    pub fn destroy_ioreq_server(&self, domid: u16, id: u16) -> Result<(), std::io::Error> {
        let mut dm_op = XenDeviceModelOp {
            op: XEN_DMOP_destroy_ioreq_server,
            pad: 0,
            u: XenDeviceModelOpPayload {
                xen_destroy_ioreq_server: XenDeviceModelDestroyIoreqServer {
                    id,
                    ..Default::default()
                },
            },
        };

        let mut privcmd_dm_op_buffers = vec![PrivcmdDeviceModelOpBuffer {
            uptr: &mut dm_op as *mut _ as *mut c_void,
            size: std::mem::size_of::<XenDeviceModelOp>(),
        }];

        do_dm_op(&self.fd, domid, &mut privcmd_dm_op_buffers)
    }

    pub fn set_irq_level(&self, domid: u16, irq: u32, level: u32) -> Result<(), std::io::Error> {
        let mut dm_op = XenDeviceModelOp {
            op: XEN_DMOP_set_irq_level,
            pad: 0,
            u: XenDeviceModelOpPayload {
                xen_set_irq_level: XenDeviceModelSetIrqLevel {
                    irq,
                    level: level as u8,
                    ..Default::default()
                },
            },
        };

        let mut privcmd_dm_op_buffers = vec![PrivcmdDeviceModelOpBuffer {
            uptr: &mut dm_op as *mut _ as *mut c_void,
            size: std::mem::size_of::<XenDeviceModelOp>(),
        }];

        do_dm_op(&self.fd, domid, &mut privcmd_dm_op_buffers)
    }

    fn config_irqfd(
        &self,
        fd: EventFd,
        domid: u16,
        irq: u32,
        level: u8,
        flags: u32,
    ) -> Result<(), std::io::Error> {
        let mut dm_op = XenDeviceModelOp {
            op: XEN_DMOP_set_irq_level,
            pad: 0,
            u: XenDeviceModelOpPayload {
                xen_set_irq_level: XenDeviceModelSetIrqLevel {
                    irq,
                    level,
                    ..Default::default()
                },
            },
        };

        let mut irqfd = PrivcmdDeviceModelIrqFd {
            dm_op: &mut dm_op as *mut _ as *mut c_void,
            size: std::mem::size_of::<XenDeviceModelOp>() as u32,
            fd: fd.as_raw_fd() as u32,
            flags,
            domid,
            pad: [0; 2],
        };

        let ret = unsafe {
            // The expression "&mut irqfd as *mut _" creates a reference to irqfd before
            // casting it to a *mut c_void.
            ioctl(
                self.fd.as_raw_fd(),
                IOCTL_PRIVCMD_IRQFD(),
                &mut irqfd as *mut _ as *mut c_void,
            )
        };

        if ret < 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn set_irqfd(
        &self,
        fd: EventFd,
        domid: u16,
        irq: u32,
        level: u8,
    ) -> Result<(), std::io::Error> {
        self.config_irqfd(fd, domid, irq, level, 0)
    }

    pub fn clear_irqfd(
        &self,
        fd: EventFd,
        domid: u16,
        irq: u32,
        level: u8,
    ) -> Result<(), std::io::Error> {
        self.config_irqfd(fd, domid, irq, level, PRIVCMD_IRQFD_FLAG_DEASSIGN)
    }

    fn config_ioeventfd(
        &self,
        kick: &EventFd,
        ioreq: &mut ioreq,
        ports: &[u32],
        addr: u64,
        addr_len: u32,
        vq: u32,
        vcpus: u32,
        domid: u16,
        flags: u32,
    ) -> Result<(), std::io::Error> {
        let mut ioeventfd = PrivcmdDeviceModelIoeventFd {
            ioreq: ioreq as *mut _ as *mut c_void,
            ports: ports as *const _ as *const u32,
            addr,
            addr_len,
            event_fd: kick.as_raw_fd() as u32,
            vcpus,
            vq,
            flags,
            domid,
            pad: [0; 2],
        };

        let ret = unsafe {
            // The expression "&mut ioeventfd as *mut _" creates a reference to ioeventfd
            // before casting it to a *mut c_void.
            ioctl(
                self.fd.as_raw_fd(),
                IOCTL_PRIVCMD_IOEVENTFD(),
                &mut ioeventfd as *mut _ as *mut c_void,
            )
        };

        if ret < 0 {
            return Err(Error::last_os_error());
        }

        Ok(())
    }

    pub fn set_ioeventfd(
        &self,
        kick: &EventFd,
        ioreq: &mut ioreq,
        ports: &[u32],
        addr: u64,
        addr_len: u32,
        vq: u32,
        vcpus: u32,
        domid: u16,
    ) -> Result<(), std::io::Error> {
        self.config_ioeventfd(kick, ioreq, ports, addr, addr_len, vq, vcpus, domid, 0)
    }

    pub fn clear_ioeventfd(
        &self,
        kick: &EventFd,
        ioreq: &mut ioreq,
        ports: &[u32],
        addr: u64,
        addr_len: u32,
        vq: u32,
        vcpus: u32,
        domid: u16,
    ) -> Result<(), std::io::Error> {
        self.config_ioeventfd(
            kick,
            ioreq,
            ports,
            addr,
            addr_len,
            vq,
            vcpus,
            domid,
            PRIVCMD_IOEVENTFD_FLAG_DEASSIGN,
        )
    }
}
