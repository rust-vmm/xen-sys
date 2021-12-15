/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![allow(non_camel_case_types)]

#[cfg(target_arch = "x86_64")]
use crate::x86_64::*;

#[cfg(target_arch = "aarch64")]
use crate::aarch64::*;

/// SCHEDOP_ defines from public/sched.h
#[derive(Debug)]
pub enum SchedOp {
    /// SCHEDOP_yield
    r#yield,
    /// SCHEDOP_block
    block,
    /// SCHEDOP_shutdown
    shutdown,
    /// SCHEDOP_poll
    poll,
    /// SCHEDOP_remote_shutdown
    remote_shutdown,
    /// SCHEDOP_shutdown_code
    shutdown_code,
    /// SCHEDOP_watchdog
    watchdog,
    /// SCHEDOP_pin_override
    pin_override,
}

/// CONSOLEIO_ defines from public/xen.h
#[derive(Debug)]
pub enum ConsoleIO {
    /// CONSOLEIO_write
    Write,
    /// CONSOLEIO_read
    Read,
}

macro_rules! hypercall {
    ($op:expr, $a1:expr) => {
        $crate::hypercall_1($op, $a1 as u64)
    };

    ($op:expr, $a1:expr, $a2:expr) => {
        $crate::hypercall_2($op, $a1 as u64, $a2 as u64)
    };

    ($op:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::hypercall_3($op, $a1 as u64, $a2 as u64, $a3 as u64)
    };

    ($op:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::hypercall_4($op, $a1 as u64, $a2 as u64, $a3 as u64, $a4 as u64)
    };

    ($op:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::hypercall_5(
            $op, $a1 as u64, $a2 as u64, $a3 as u64, $a4 as u64, $a5 as u64,
        )
    };
}

pub unsafe fn console_io(mode: ConsoleIO, buf: &[u8]) -> i64 {
    match mode {
        ConsoleIO::Write => hypercall!(
            __HYPERVISOR_console_io,
            CONSOLEIO_write,
            buf.len() as u64,
            buf.as_ptr() as u64
        ),
        ConsoleIO::Read => hypercall!(
            __HYPERVISOR_console_io,
            CONSOLEIO_read,
            buf.len() as u64,
            buf.as_ptr() as u64
        ),
    }
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn sched_op(mode: SchedOp, data: u32) {
    match mode {
        SchedOp::r#yield => hypercall!(__HYPERVISOR_sched_op, SCHEDOP_yield, data as u64),
        SchedOp::shutdown => hypercall!(__HYPERVISOR_sched_op, SCHEDOP_shutdown, data as u64),
        _ => panic!(),
    };
}

#[cfg(target_arch = "aarch64")]
pub unsafe fn sched_op(mode: SchedOp, data: u32) {
    let address = &data as *const u32;

    match mode {
        SchedOp::r#yield => hypercall!(__HYPERVISOR_sched_op, SCHEDOP_yield, address as u64),
        SchedOp::shutdown => hypercall!(__HYPERVISOR_sched_op, SCHEDOP_shutdown, address as u64),
        _ => panic!(),
    };
}
