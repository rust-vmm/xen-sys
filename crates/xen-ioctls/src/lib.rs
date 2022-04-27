/*
 * Copyright 2021-22 Mathieu Poirier <mathieu.poirier@linaro.org>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

pub(crate) mod private;
mod sysctl;
mod domctl;
mod xfm;


#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "aarch64")]
mod aarch64;

pub use domctl::*;
pub use sysctl::*;
pub use xfm::*;