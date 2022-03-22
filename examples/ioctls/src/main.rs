/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use xen_ioctls::sysctl_types::*;
use xen_ioctls::xen_types::*;
use xen_ioctls::private::PrivCmdHypercall;
use xen_ioctls::xc_misc::*;

fn main() {

    println!("size of XenSysctlPhysinfo: {}", std::mem::size_of::<XenSysctlPhysinfo>());
    println!("size of XenSysctlGetDomaininfolist: {}", std::mem::size_of::<XenSysctlGetdomaininfolist>());
    println!("size of PrivCmdHypercall: {}", std::mem::size_of::<PrivCmdHypercall>());
    println!("size of XenSysctl: {}", std::mem::size_of::<XenSysctl>());
    println!("size of XenDomctlGetDomainInfo: {}", std::mem::size_of::<XenDomctlGetDomainInfo>());

    match get_physinfo() {
        Ok(physinfo) => println!("physinfo: {:?}", physinfo),
        Err(err) => println!("err: {}", err),
    }

    let domain_list = get_domain_infolist(0, 1024).unwrap();
    for domain in domain_list {
        println!("domain: {} flags: {:x}", domain.domain, domain.flags);
        println!("tot_pages: {:x} max_pages: {:x}", domain.tot_pages, domain.max_pages);
        println!("nr_online_vcpus: {} max_vcpuid: {}", domain.nr_online_vcpus, domain.max_vcpu_id);
    }
}
