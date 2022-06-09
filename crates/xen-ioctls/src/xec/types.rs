/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use vmm_sys_util::ioctl::_IOC_NONE;

pub const XEN_EVTCHN_TYPE: u32 = 'E' as u32;

/*
 * #define IOCTL_EVTCHN_BIND_INTERDOMAIN \
 *      _IOC(_IOC_NONE, 'E', 1, sizeof(ioctl_evtchn_bind_interdomain))
 */
ioctl_ioc_nr!(
    IOCTL_EVTCHN_BIND_INTERDOMAIN,
    _IOC_NONE,
    XEN_EVTCHN_TYPE,
    1 as u32,
    std::mem::size_of::<XenIoctlEvtchnBindInterdomain>() as u32
);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// tools/include/xen-sys/Linux/evtchn.h::struct ioctl_evtchn_bind_interdomain
// sizeof(struct ioctl_evtchn_bind_interdomain) == 8
pub struct XenIoctlEvtchnBindInterdomain {
    pub remote_domain: u32,
    pub remote_port: u32,
}

/*
 * #define IOCTL_EVTCHN_UNBIND \
 *      _IOC(_IOC_NONE, 'E', 3, sizeof(struct ioctl_evtchn_unbind))
 */
ioctl_ioc_nr!(
    IOCTL_EVTCHN_UNBIND,
    _IOC_NONE,
    XEN_EVTCHN_TYPE,
    3 as u32,
    std::mem::size_of::<XenIoctlEvtchnUnbind>() as u32
);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// tools/include/xen-sys/Linux/evtchn.h::struct ioctl_evtchn_unbind
// sizeof(struct ioctl_evtchn_unbind) == 4
pub struct XenIoctlEvtchnUnbind {
    pub port: u32,
}

/*
 * #define IOCTL_EVTCHN_NOTIFY \
 *      _IOC(_IOC_NONE, 'E', 4, sizeof(struct ioctl_evtchn_notify))
 */
ioctl_ioc_nr!(
    IOCTL_EVTCHN_NOTIFY,
    _IOC_NONE,
    XEN_EVTCHN_TYPE,
    4 as u32,
    std::mem::size_of::<XenIoctlEvtchnNotify>() as u32
);

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
// tools/include/xen-sys/Linux/evtchn.h::struct ioctl_evtchn_unbind
// sizeof(struct ioctl_evtchn_unbind) == 4
pub struct XenIoctlEvtchnNotify {
    pub port: u32,
}
