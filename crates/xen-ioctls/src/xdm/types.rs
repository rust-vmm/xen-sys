/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use libc::c_void;
use std::ptr;

#[cfg(target_arch = "aarch64")]
use crate::aarch64::types::*;
#[cfg(target_arch = "x86_64")]
use crate::x86_64::types::*;

pub const DOM_INVALID: u16 = 0x7FF4;
pub const XEN_DMOP_create_ioreq_server: u32 = 1;
pub const XEN_DMOP_map_io_range_to_ioreq_server: u32 = 3;
pub const XEN_DMOP_unmap_io_range_from_ioreq_server: u32 = 4;
pub const XEN_DMOP_set_ioreq_server_state: u32 = 5;
pub const XEN_DMOP_destroy_ioreq_server: u32 = 6;
pub const XEN_DMOP_set_irq_level: u32 = 19;
pub const XEN_DMOP_nr_vcpus: u32 = 20;

pub const HVM_IOREQSRV_BUFIOREQ_OFF: u8 = 0;
pub const HVM_IOREQSRV_BUFIOREQ_LEGACY: u8 = 1;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// xen/include/public/hvm/dm_op.h::struct xen_dm_op_create_ioreq_server
pub struct XenDeviceModelCreateIoreqServer {
    pub handle_bufioreq: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub pad2: u8,
    pub id: u16,
}

pub const XEN_DMOP_IO_RANGE_PORT: u32 = 0; /* I/O port range */
pub const XEN_DMOP_IO_RANGE_MEMORY: u32 = 1; /* MMIO range */

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// xen/include/public/hvm/dm_op.h::struct xen_dm_op_ioreq_server_range
pub struct XenDeviceModelIoreqServerRange {
    pub id: u16,
    pub pad: u16,
    pub r#type: u32,
    pub start: U64Aligned,
    pub end: U64Aligned,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// xen/include/public/hvm/dm_op.h::struct xen_dm_op_set_ioreq_server_state
pub struct XenDeviceModelSetIoreqServerState {
    pub id: u16,
    pub enabled: u8,
    pub pad: u8,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// xen/include/public/hvm/dm_op.h::struct xen_dm_op_destroy_ioreq_server
pub struct XenDeviceModelDestroyIoreqServer {
    pub id: u16,
    pub pad: u16,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// xen/include/public/hvm/dm_op.h::struct xen_dm_op_set_irq_level
pub struct XenDeviceModelSetIrqLevel {
    pub irq: u32,
    pub level: u8,
    pub pad0: u8,
    pub pad1: u8,
    pub pad2: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// xen/include/public/hvm/dm_op.h::struct xen_dm_op_nr_vcpus
pub struct XenDeviceModelNrVcpus {
    pub vcpus: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union XenDeviceModelOpPayload {
    pub xen_create_ioreq_server: XenDeviceModelCreateIoreqServer,
    pub xen_ioreq_server_range: XenDeviceModelIoreqServerRange,
    pub xen_set_ioreq_server_state: XenDeviceModelSetIoreqServerState,
    pub xen_destroy_ioreq_server: XenDeviceModelDestroyIoreqServer,
    pub xen_set_irq_level: XenDeviceModelSetIrqLevel,
    pub xen_dm_op_nr_vcpus: XenDeviceModelNrVcpus,
}

#[repr(C)]
#[derive(Copy, Clone)]
// xen/include/public/hvm/dm_op.h::struct xen_dm_op
pub struct XenDeviceModelOp {
    pub op: u32,
    pub pad: u32,
    pub u: XenDeviceModelOpPayload,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// tools/include/xen-sys/Linux/privcmd.h::struct privcmd_dm_op_buf
pub struct PrivcmdDeviceModelOpBuffer {
    pub uptr: *mut c_void,
    pub size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// tools/include/xen-sys/Linux/privcmd.h::struct privcmd_dm_op
// sizeof(struct privcmd_dm_op) == 96
pub(crate) struct PrivcmdDeviceModelOp {
    pub domid: u16,
    pub num: u16,
    pub ubufs: *mut PrivcmdDeviceModelOpBuffer,
}

impl PrivcmdDeviceModelOp {
    pub(crate) fn new(
        domid: u16,
        privcmd_dm_op_buffers: &mut Vec<PrivcmdDeviceModelOpBuffer>,
    ) -> Self {
        let num = privcmd_dm_op_buffers.len() as u16;
        let ubufs = match num {
            0 => ptr::null_mut(),
            _ => privcmd_dm_op_buffers.as_mut_ptr(),
        };

        PrivcmdDeviceModelOp { domid, num, ubufs }
    }
}

pub const PRIVCMD_IRQFD_FLAG_DEASSIGN: u32 = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// tools/include/xen-sys/Linux/privcmd.h::privcmd_irqfd
pub struct PrivcmdDeviceModelIrqFd {
    pub dm_op: *mut c_void,
    pub size: u32,
    pub fd: u32,
    pub flags: u32,
    pub domid: u16,
    pub pad: [u8; 2],
}
