// Copyright 2023 Linaro, All Rights Reserved.
// SPDX-License-Identifier: (BSD-3-Clause OR Apache-2.0)
//
// Tailored after the work done for the rust-vmm/vm-virtio/virtio-bindings

#![allow(clippy::all)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Keep this until https://github.com/rust-lang/rust-bindgen/issues/1651 is fixed.
#![cfg_attr(test, allow(deref_nullptr, unaligned_references))]

mod xen_bindings_x86_64_types {
    pub type c_char = i8;
    pub type c_schar = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i64;
    pub type c_ulong = u64;
    pub type c_void = u64;
    pub type c_longlong = i64;
}

include!("xen_bindings_x86_64.rs");
