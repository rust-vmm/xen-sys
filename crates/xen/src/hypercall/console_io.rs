/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use xen_sys::hypercall::{console_io, ConsoleIO};

/// writes to the system serial console which
/// is disabled for non-dom0 domains unless
/// Xen is built with CONFIG_VERBOSE
pub fn write(out: &[u8]) {
    unsafe { console_io(ConsoleIO::Write, out) };
}
