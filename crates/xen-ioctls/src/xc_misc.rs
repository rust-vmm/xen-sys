/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use crate::sysctl_types::*;
use crate::sysctl::do_sysctl;

pub fn get_physinfo() -> Result<XenSysctlPhysinfo, std::io::Error>
{
    let mut sysctl = XenSysctl {
        cmd: XEN_SYSCTL_physinfo,
        interface_version: XEN_SYSCTL_INTERFACE_VERSION,
        u: XenSysctlPayload { physinfo: XenSysctlPhysinfo::default() },
    };

    do_sysctl(&mut sysctl).map(|_| unsafe { sysctl.u.physinfo.clone() })
}