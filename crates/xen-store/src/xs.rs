/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use libc::writev;
use nix::libc::iovec;
use std::collections::VecDeque;
use std::ffi::CString;
use std::io::{Error, ErrorKind};
use std::io::{Read, Write};
use std::mem;
use std::net::Shutdown;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::UnixStream;
use std::slice;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::thread::JoinHandle;
use vmm_sys_util::eventfd::{EventFd, EFD_SEMAPHORE};

use crate::types::*;

pub const XS_DIRECTORY: u32 = 1;
pub const XS_READ: u32 = 2;
pub const XS_WATCH: u32 = 4;
pub const XS_WRITE: u32 = 11;
pub const XS_WATCH_EVENT: u32 = 15;

fn queue_message(
    condvar: &Arc<(
        Mutex<VecDeque<Result<XenStoreMessage, std::io::Error>>>,
        Condvar,
    )>,
    eventfd: Option<EventFd>,
    message: Result<XenStoreMessage, std::io::Error>,
) {
    let (lock, cvar) = &**condvar;

    let mut queue = lock.lock().unwrap();

    if eventfd.is_some() {
        /* Increment evenfd counter to be consumed in read_watch() */
        let _ = eventfd
            .unwrap()
            .write(1)
            .map_err(|e| println!("queue_message: error: {}", e));
    }

    queue.push_back(message);
    cvar.notify_one();
}

fn thread_function(
    mut rx_socket: UnixStream,
    tx_eventfd: EventFd,
    reply_condvar: Arc<(
        Mutex<VecDeque<Result<XenStoreMessage, std::io::Error>>>,
        Condvar,
    )>,
    watch_condvar: Arc<(
        Mutex<VecDeque<Result<XenStoreMessage, std::io::Error>>>,
        Condvar,
    )>,
) -> Result<(), std::io::Error> {
    let xen_socket_msg_size = mem::size_of::<XenSocketMessage>();

    loop {
        let mut xen_socket_reply_msg = XenSocketMessage::default();
        let mut buffer: Vec<u8> = vec![0];
        let mut condvar = reply_condvar.clone();
        let mut eventfd: Option<EventFd> = None;

        unsafe {
            let xen_socket_reply_msg_slice = slice::from_raw_parts_mut(
                &mut xen_socket_reply_msg as *mut _ as *mut u8,
                xen_socket_msg_size,
            );

            rx_socket.read_exact(xen_socket_reply_msg_slice)?;

            if xen_socket_reply_msg.r#type == XS_READ && xen_socket_reply_msg.len == 0 {
                queue_message(
                    &condvar,
                    eventfd,
                    Ok(XenStoreMessage {
                        r#type: xen_socket_reply_msg.r#type,
                        body: "".to_string(),
                    }),
                );
                continue;
            }

            buffer.resize(xen_socket_reply_msg.len as usize, 0);

            rx_socket.read_exact(buffer.as_mut_slice())?;

            if xen_socket_reply_msg.r#type != XS_READ
                && xen_socket_reply_msg.r#type != XS_WRITE
                && xen_socket_reply_msg.r#type != XS_WATCH
                && xen_socket_reply_msg.r#type != XS_WATCH_EVENT
                && xen_socket_reply_msg.r#type != XS_DIRECTORY
            {
                queue_message(
                    &condvar,
                    eventfd,
                    Err(Error::new(ErrorKind::Other, "Xen Store transaction error")),
                );
                continue;
            }

            if xen_socket_reply_msg.r#type == XS_WATCH_EVENT {
                condvar = watch_condvar.clone();
                eventfd = Some(tx_eventfd.try_clone()?);
            }

            match String::from_utf8(buffer) {
                Ok(result) => {
                    if result.len() != xen_socket_reply_msg.len as usize {
                        queue_message(&condvar, eventfd, Err(Error::from(ErrorKind::InvalidData)));
                        continue;
                    }

                    queue_message(
                        &condvar,
                        eventfd,
                        Ok(XenStoreMessage {
                            r#type: xen_socket_reply_msg.r#type,
                            body: result,
                        }),
                    );
                }
                Err(e) => {
                    queue_message(&condvar, eventfd, Err(Error::new(ErrorKind::Other, e)));
                }
            };
        }
    }
}

pub struct XenStoreHandle {
    handler: Option<JoinHandle<Result<(), std::io::Error>>>,
    reply_condvar: Arc<(
        Mutex<VecDeque<Result<XenStoreMessage, std::io::Error>>>,
        Condvar,
    )>,
    watch_condvar: Arc<(
        Mutex<VecDeque<Result<XenStoreMessage, std::io::Error>>>,
        Condvar,
    )>,
    tx_socket: Mutex<UnixStream>,
    rx_eventfd: EventFd,
}

impl XenStoreHandle {
    pub fn new() -> Result<Self, std::io::Error> {
        let tx_socket = UnixStream::connect(XENSTORED_SOCKET)?;
        let rx_socket = tx_socket.try_clone()?;
        let tx_eventfd = EventFd::new(EFD_SEMAPHORE)?;
        let rx_eventfd = tx_eventfd.try_clone()?;
        let reply_condvar = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let reply_condvar_cloned = Arc::clone(&reply_condvar);
        let watch_condvar = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let watch_condvar_cloned = Arc::clone(&watch_condvar);

        let handler = thread::spawn(|| {
            thread_function(
                rx_socket,
                tx_eventfd,
                reply_condvar_cloned,
                watch_condvar_cloned,
            )
        });

        Ok(XenStoreHandle {
            handler: Some(handler),
            reply_condvar,
            watch_condvar,
            tx_socket: Mutex::new(tx_socket),
            rx_eventfd,
        })
    }

    fn xs_transaction(
        &self,
        r#type: u32,
        iovec_buffers: &mut Vec<iovec>,
    ) -> Result<String, std::io::Error> {
        let mut xen_socket_msg = XenSocketMessage::new(r#type, iovec_buffers)?;
        let xen_socket_msg_size = mem::size_of::<XenSocketMessage>();
        let (lock, cvar) = &*self.reply_condvar;

        unsafe {
            let xen_socket_msg_slice = slice::from_raw_parts(
                &mut xen_socket_msg as *mut _ as *mut u8,
                xen_socket_msg_size,
            );

            /*
             * Grabbing the mutex guarantees there will only be
             * one active transcation at a time.
             */
            let mut tx_socket = self.tx_socket.lock().unwrap();
            tx_socket.write_all(xen_socket_msg_slice)?;

            let ret = writev(
                tx_socket.as_raw_fd(),
                iovec_buffers.as_ptr(),
                iovec_buffers.len() as i32,
            );

            if ret < 0 {
                return Err(Error::last_os_error());
            }

            let mut reply_vec = lock.lock().unwrap();
            while reply_vec.is_empty() {
                reply_vec = cvar.wait(reply_vec).unwrap();
            }

            match reply_vec.pop_front() {
                Some(result) => match result {
                    Ok(xsm) => {
                        if xsm.r#type != r#type {
                            return Err(Error::from(ErrorKind::InvalidData));
                        }
                        Ok(xsm.body)
                    }
                    Err(e) => Err(e),
                },
                None => Err(Error::new(ErrorKind::Other, "Xen Store transaction error")),
            }
        }
    }

    pub fn read_str(&self, path: &str) -> Result<String, std::io::Error> {
        let c_path = CString::new(format!("{}", path))?;
        let mut iovec_buffers = vec![iovec {
            iov_base: c_path.as_ptr() as *mut _,
            iov_len: path.len() + 1,
        }];

        self.xs_transaction(XS_READ, &mut iovec_buffers)
    }

    pub fn write_str(&self, path: &str, val: &str) -> Result<(), std::io::Error> {
        let cpath = CString::new(format!("{}", path))?;
        let cval = CString::new(format!("{}", val))?;
        let mut iovec_buffers = vec![
            iovec {
                iov_base: cpath.as_ptr() as *mut _,
                iov_len: path.len() + 1,
            },
            iovec {
                iov_base: cval.as_ptr() as *mut _,
                iov_len: val.len(),
            },
        ];

        self.xs_transaction(XS_WRITE, &mut iovec_buffers)
            .map(|_| ())
    }

    pub fn create_watch(&self, path: &str, token: &str) -> Result<(), std::io::Error> {
        let cpath = CString::new(format!("{}", path))?;
        let ctoken = CString::new(format!("{}", token))?;
        let mut iovec_buffers = vec![
            iovec {
                iov_base: cpath.as_ptr() as *mut _,
                iov_len: path.len() + 1,
            },
            iovec {
                iov_base: ctoken.as_ptr() as *mut _,
                iov_len: token.len() + 1,
            },
        ];

        self.xs_transaction(XS_WATCH, &mut iovec_buffers)
            .map(|_| ())
    }

    pub fn read_watch(&self, index: usize) -> Result<String, std::io::Error> {
        let (lock, cvar) = &*self.watch_condvar;

        let mut watch_vec = lock.lock().unwrap();
        while watch_vec.is_empty() {
            watch_vec = cvar.wait(watch_vec).unwrap();
        }

        /* Consume eventfd counter incremented in queue_message() */
        let _ = self.rx_eventfd.read().unwrap();

        match watch_vec.pop_front() {
            Some(result) => match result {
                Ok(mut xsm) => {
                    if xsm.r#type != XS_WATCH_EVENT {
                        return Err(Error::from(ErrorKind::InvalidData));
                    }

                    let body = xsm.body.as_mut_str();
                    let v: Vec<&str> = body.split('\0').collect();
                    if index >= v.len() {
                        return Err(Error::from(ErrorKind::InvalidInput));
                    }

                    Ok(String::from(v[index]))
                }
                Err(e) => Err(e),
            },
            None => Err(Error::new(ErrorKind::Other, "Xen Store transaction error")),
        }
    }

    pub fn fileno(&self) -> Result<i32, std::io::Error> {
        Ok(self.rx_eventfd.as_raw_fd())
    }

    pub fn directory(&self, path: &str) -> Result<Vec<i32>, std::io::Error> {
        let c_path = CString::new(format!("{}", path))?;
        let mut iovec_buffers = vec![iovec {
            iov_base: c_path.as_ptr() as *mut _,
            iov_len: path.len() + 1,
        }];

        match self.xs_transaction(XS_DIRECTORY, &mut iovec_buffers) {
            Ok(mut res) => {
                let mut vi32 = vec![];
                let body = res.as_mut_str();
                let vstr: Vec<&str> = body.split('\0').collect();
                for str in vstr {
                    if !str.eq("") {
                        vi32.push(str.parse::<i32>().unwrap());
                    }
                }
                Ok(vi32)
            }
            Err(e) => Err(e),
        }
    }
}

impl Drop for XenStoreHandle {
    fn drop(&mut self) {
        let tx_socket = self.tx_socket.lock().unwrap();

        /*
         * Calling shutdown() on the socket will cause the blocking
         * rx_socket in thread_function() to return with an error
         * condition, something that will automatically break the
         * loop and cause the thread to stop.
         */
        let _ = tx_socket.shutdown(Shutdown::Both);

        /* Wait for it to stop */
        let _ = self.handler.take().unwrap().join();
    }
}
