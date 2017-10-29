/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use xen_sys::hypercall::Hypercall;

const SCHEDOP_yield: u64 = 0;
const SCHEDOP_shutdown: u64 = 2;
const SCHEDOP_poll: u64 = 3;

pub fn shutdown(reason: u32) {
    unsafe {
        hypercall!(Hypercall::sched_op, SCHEDOP_shutdown, 0)
    };
}

pub fn yield_slice() {
    unsafe {
        hypercall!(Hypercall::sched_op, SCHEDOP_yield, 0);
    };
}
