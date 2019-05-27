/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![feature(lang_items)]
#![no_std]

extern crate xen;
extern crate xen_sys;

use core::ptr;
use xen::hypercall;

#[no_mangle]
pub extern "C" fn rust_entry(start_info_page: *mut xen_sys::start_info_t) {
    let start_info = unsafe { ptr::read(start_info_page) };

    let test = b"test";

    hypercall::console_io::write(test);
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
