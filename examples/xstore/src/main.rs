/*
 * Copyright 2022-23 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use xen_store::XenStoreHandle;

fn main() -> Result<(), std::io::Error> {
    let xsh = XenStoreHandle::new()?;

    if let Err(e) = xsh.read_str("domid") {
        println!("Got error: {}", e);
    };

    if let Err(e) = xsh.create_watch("backend/i2c", "backend/i2c") {
        println!("Got error: {}", e);
    }

    if let Err(e) = xsh.read_str("/local/domain/1/device/i2c/0") {
        println!("Got error: {}", e);
    }

    if let Err(e) = xsh.write_str("/local/domain/1/device/i2c/0", "that") {
        println!("Got error: {}", e);
    }

    if let Err(e) = xsh.read_str("/local/domain/1/device/i2c/0") {
        println!("Got error: {}", e);
    }

    if let Err(e) = xsh.read_watch(0) {
        println!("Got error: {}", e);
    }

    Ok(())
}
