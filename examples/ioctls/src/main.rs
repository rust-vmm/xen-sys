/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use xen_ioctls::sysctl::types::*;
use xen_ioctls::domctl::types::*;
use xen_ioctls::domctl::domctl::*;
use xen_ioctls::sysctl::sysctl::*;

fn main() {

    println!("size of XenSysctlPhysinfo: {}", std::mem::size_of::<XenSysctlPhysinfo>());
    println!("size of XenSysctlGetDomaininfolist: {}", std::mem::size_of::<XenSysctlGetdomaininfolist>());
    println!("size of XenSysctl: {}", std::mem::size_of::<XenSysctl>());
    println!("size of XenDomctl: {}", std::mem::size_of::<XenDomctl>());
    println!("size of XenDomctlGetDomainInfo: {}", std::mem::size_of::<XenDomctlGetDomainInfo>());

    match xc_physinfo() {
        Ok(physinfo) => println!("physinfo: {:?}", physinfo),
        Err(err) => println!("err: {}", err),
    }

    let dominfo_list = xc_domain_info(0, 10);
    for dominfo in dominfo_list {
        println!("domaininfo: {:?}", dominfo);
    }

    let domain_list = xc_domain_getinfolist(0, 1024).unwrap();
    for domain in domain_list {
        println!("domainlist: {:?}", domain);
    }
}
