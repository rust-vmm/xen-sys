/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![allow(internal_features)]
#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate xen;
extern crate xen_sys;

#[cfg(target_arch = "x86_64")]
use core::arch::asm;
#[cfg(target_arch = "x86_64")]
use core::arch::global_asm;

use xen::entry_point;
use xen::hypercall;

entry_point!(hello_world);

pub fn hello_world() -> Result<(), ()> {
    let test = b"test";

    hypercall::console_io::write(test);
    Ok(())
}
