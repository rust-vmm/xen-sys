/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use xen_sys::{
    hypercall::{sched_op, SchedOp},
    SHUTDOWN_crash, SHUTDOWN_poweroff, SHUTDOWN_reboot,
};

fn op_shutdown(reason: u32) {
    unsafe { sched_op(SchedOp::shutdown, reason) };
}

#[no_mangle]
pub extern "C" fn poweroff() -> ! {
    op_shutdown(SHUTDOWN_poweroff);
    unreachable!()
}

pub fn reboot() {
    op_shutdown(SHUTDOWN_reboot);
}

pub fn crash() {
    op_shutdown(SHUTDOWN_crash);
}

pub fn yield_slice() {
    unsafe {
        sched_op(SchedOp::r#yield, 0);
    };
}
