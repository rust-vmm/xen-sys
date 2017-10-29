/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

/// __HYPERVISOR_hypercall defines from xen/xen.h
#[derive(Debug)]
pub enum Hypercall {
    set_trap_table = 0,
    mmu_update = 1,
    set_gdt = 2,
    stack_switch = 3,
    set_callbacks = 4,
    fpu_taskswitch = 5,
    sched_op_compat = 6,
    platform_op = 7,
    set_debugreg = 8,
    get_debugreg = 9,
    update_descriptor = 10,
    memory_op = 12,
    multicall = 13,
    update_va_mapping = 14,
    set_timer_op = 15,
    event_channel_op_compat = 16,
    xen_version = 17,
    /// __HYPERVISOR_console_io
    console_io = 18,
    physdev_op_compat = 19,
    grant_table_op = 20,
    vm_assist = 21,
    update_va_mapping_otherdomain = 22,
    iret = 23,
    vcpu_op = 24,
    set_segment_base = 25,
    mmuext_op = 26,
    xsm_op = 27,
    nmi_op = 28,
    sched_op = 29,
    callback_op = 30,
    xenopref_op = 31,
    event_channel_op = 32,
    physdev_op = 33,
    hvm_op = 34,
    sysctl = 35,
    domctl = 36,
    kexec_op = 37,
    tmem_op = 38,
    xc_reserved_op = 39,
    xenpmu_op = 40,
    arch_0 = 48,
    arch_1 = 49,
    arch_2 = 50,
    arch_3 = 51,
    arch_4 = 52,
    arch_5 = 53,
    arch_6 = 54,
    arch_7 = 55,
}
