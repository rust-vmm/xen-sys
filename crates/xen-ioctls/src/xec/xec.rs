/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use crate::private::*;
use crate::xec::types::*;
use libc::{c_void, ioctl};
use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};
use std::os::unix::io::AsRawFd;

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

        /*
         * The expression "&mut bind as *mut _" creates a reference
         * to bind before casting it to a *mut c_void
         */
        let bind_ptr: *mut c_void = &mut bind as *mut _ as *mut c_void;

        unsafe {
            match ioctl(
                self.fd.as_raw_fd(),
                IOCTL_EVTCHN_BIND_INTERDOMAIN(),
                bind_ptr,
            ) {
                ret if ret < 0 => Err(Error::last_os_error()),
                ret => Ok(ret as u32),
            }
        }
    }

    pub fn unbind(&self, port: u32) -> Result<u32, std::io::Error> {
        let mut unbind = XenIoctlEvtchnUnbind { port };
        /*
         * The expression "&mut unbind as *mut _" creates a reference
         * to unbind before casting it to a *mut c_void
         */
        let unbind_ptr: *mut c_void = &mut unbind as *mut _ as *mut c_void;

        unsafe {
            match ioctl(self.fd.as_raw_fd(), IOCTL_EVTCHN_UNBIND(), unbind_ptr) {
                ret if ret < 0 => Err(Error::last_os_error()),
                ret => Ok(ret as u32),
            }
        }
    }

    pub fn fd(&self) -> Result<i32, std::io::Error> {
        Ok(self.fd.as_raw_fd())
    }

    pub fn notify(&self, port: u32) -> Result<u32, std::io::Error> {
        let mut notify = XenIoctlEvtchnNotify { port };
        /*
         * The expression "&mut notify as *mut _" creates a reference
         * to notify before casting it to a *mut c_void
         */
        let notify_ptr: *mut c_void = &mut notify as *mut _ as *mut c_void;

        unsafe {
            match ioctl(self.fd.as_raw_fd(), IOCTL_EVTCHN_NOTIFY(), notify_ptr) {
                ret if ret < 0 => Err(Error::last_os_error()),
                ret => Ok(ret as u32),
            }
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
