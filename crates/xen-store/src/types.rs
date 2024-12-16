/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use nix::libc::{iovec, E2BIG};
use std::io;

pub const XENSTORED_SOCKET: &str = "/var/run/xenstored/socket";
pub const XENSTORE_PAYLOAD_MAX: u32 = 4096;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// xen/include/public/io/xs_wire.h::struct xsd_sockmsg
pub(crate) struct XenSocketMessage {
    pub r#type: u32,
    pub req_id: u32,
    pub tx_id: u32,
    pub len: u32,
}

impl XenSocketMessage {
    pub(crate) fn new(r#type: u32, iovec_buffers: &mut Vec<iovec>) -> Result<Self, std::io::Error> {
        let msg = XenSocketMessage {
            r#type,
            req_id: 0,
            tx_id: 0,
            len: iovec_buffers
                .iter()
                .fold(0, |acc, iovec| acc + iovec.iov_len as u32),
        };

        if msg.len > XENSTORE_PAYLOAD_MAX {
            return Err(io::Error::from_raw_os_error(E2BIG));
        }

        Ok(msg)
    }
}

#[derive(Debug, Default, Clone)]
pub(crate) struct XenStoreMessage {
    pub r#type: u32,
    pub body: String,
}
