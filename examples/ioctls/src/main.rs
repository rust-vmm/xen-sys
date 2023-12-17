/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use xen_ioctls::{xc_physinfo, xc_domain_info};
use xen_ioctls::xc_domain_getinfolist;

fn main() {

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
