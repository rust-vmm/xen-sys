/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use libc::{c_ulong};

pub const PAGE_SHIFT:u32 = 12;
pub const PAGE_SIZE:u32 = 1 << PAGE_SHIFT;
pub const PAGE_MASK:u64 = !(PAGE_SIZE as u64 - 1);

pub const __HYPERVISOR_sysctl:u64 = 35;

pub const IOCTL_PRIVCMD_HYPERCALL:c_ulong = 0x305000;

pub const XC_HYPERCALL_BUFFER_BOUNCE_NONE:u32 = 0;
pub const XC_HYPERCALL_BUFFER_BOUNCE_IN:u32 = 1;
pub const XC_HYPERCALL_BUFFER_BOUNCE_OUT:u32 = 2;
pub const XC_HYPERCALL_BUFFER_BOUNCE_BOTH:u32 = 3;

pub const HYPERCALL_PRIVCMD: &str = "/dev/xen/privcmd";
pub const HYPERCALL_BUFFER_FILE: &str = "/dev/xen/hypercall";

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct PrivCmdHypercall
{
	pub op: u64,
	pub arg: [u64; 5],
}

pub fn round_up(value: u64, scale: u64) -> u64
{
    let mut ceiling: u64 = scale;

    while ceiling < value {
        ceiling += scale;
    }

    ceiling
}