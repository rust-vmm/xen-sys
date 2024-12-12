/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use std::{
    convert::TryInto,
    fs::{File, OpenOptions},
    io::{Error, Read, Write},
    os::unix::io::AsRawFd,
};

use crate::{private::*, xec::types::*};

pub struct XenEventChannelHandle {
    fd: File,
}

impl XenEventChannelHandle {
    pub fn new() -> Result<Self, std::io::Error> {
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .open(HYPERCALL_EVTCHN)?;

        Ok(XenEventChannelHandle { fd })
    }

    pub fn bind_interdomain(&self, domid: u32, remote_port: u32) -> Result<u32, std::io::Error> {
        let mut bind = XenIoctlEvtchnBindInterdomain {
            remote_domain: domid,
            remote_port,
        };

        // SAFETY: self.fd is a valid HYPERCALL_EVTCHN descriptor, and we pass a
        // XenIoctlEvtchnBindInterdomain to the IOCTL_EVTCHN_BIND_INTERDOMAIN ioctl
        match unsafe {
            libc::ioctl(
                self.fd.as_raw_fd(),
                #[allow(clippy::useless_conversion)]
                IOCTL_EVTCHN_BIND_INTERDOMAIN().try_into().unwrap(),
                std::ptr::addr_of_mut!(bind),
            )
        } {
            ret if ret < 0 => Err(Error::last_os_error()),
            ret => Ok(ret as u32),
        }
    }

    pub fn unbind(&self, port: u32) -> Result<u32, std::io::Error> {
        let mut unbind = XenIoctlEvtchnUnbind { port };

        // SAFETY: self.fd is a valid HYPERCALL_EVTCHN descriptor, and we pass a
        // XenIoctlEvtchnUnbind to the IOCTL_EVTCHN_UNBIND ioctl
        match unsafe {
            libc::ioctl(
                self.fd.as_raw_fd(),
                #[allow(clippy::useless_conversion)]
                IOCTL_EVTCHN_UNBIND().try_into().unwrap(),
                std::ptr::addr_of_mut!(unbind),
            )
        } {
            ret if ret < 0 => Err(Error::last_os_error()),
            ret => Ok(ret as u32),
        }
    }

    pub fn fd(&self) -> Result<i32, std::io::Error> {
        Ok(self.fd.as_raw_fd())
    }

    pub fn notify(&self, port: u32) -> Result<u32, std::io::Error> {
        let mut notify = XenIoctlEvtchnNotify { port };

        // SAFETY: self.fd is a valid HYPERCALL_EVTCHN descriptor, and we pass a
        // XenIoctlEvtchnNotify to the IOCTL_EVTCHN_NOTIFY ioctl
        match unsafe {
            libc::ioctl(
                self.fd.as_raw_fd(),
                #[allow(clippy::useless_conversion)]
                IOCTL_EVTCHN_NOTIFY().try_into().unwrap(),
                std::ptr::addr_of_mut!(notify),
            )
        } {
            ret if ret < 0 => Err(Error::last_os_error()),
            ret => Ok(ret as u32),
        }
    }

    pub fn pending(&mut self) -> Result<u32, std::io::Error> {
        let mut buffer = [0; 4];

        self.fd
            .read_exact(&mut buffer[..])
            .map(|_| u32::from_ne_bytes(buffer))
    }

    pub fn unmask(&mut self, port: u32) -> Result<(), std::io::Error> {
        self.fd.write_all(&port.to_ne_bytes())
    }
}
