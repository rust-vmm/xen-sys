// Copyright 2023 Linaro, All Rights Reserved.
// SPDX-License-Identifier: (BSD-3-Clause OR Apache-2.0)
//
// Tailored after the work done for the rust-vmm/vm-virtio/virtio-bindings

#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

#[cfg(target_arch = "aarch64")]
mod bindings_aarch64;
#[cfg(target_arch = "x86_64")]
mod bindings_x86_64;

pub mod bindings {
    #[cfg(target_arch = "aarch64")]
    pub use super::bindings_aarch64::*;
    #[cfg(target_arch = "x86_64")]
    pub use super::bindings_x86_64::*;
}
