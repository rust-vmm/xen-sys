/*
 * Copyright 2016-2017 Doug Goldstein <cardoe@cardoe.com>
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use super::super::*;
use super::Hypercall;

/// x86_64 hypercalls are called at the address: 32 * HYPERCALL_NUM
#[repr(C)]
#[derive(Clone, Copy)]
struct hypercall_entry([u8; 32]);

/// pages on x86_64 are 4096 bytes giving us 128 32-byte entries
extern {
    static HYPERCALL_PAGE: [hypercall_entry; 128];
}

#[inline]
pub unsafe fn hypercall_1(op: Hypercall,
                          a1: u64) -> c_long {
    let ret: c_long;
    let _ign1: u64;
    let addr = HYPERCALL_PAGE.as_ptr().offset(op as isize);

    asm!("call *$0"
         : "={rax}" (ret), "={rdi}" (_ign1)
         : "r" (addr),
           "{rdi}" (a1)
         : "memory"
         : "volatile");
    ret
}

#[inline]
pub unsafe fn hypercall_2(op: Hypercall,
                          a1: u64,
                          a2: u64) -> c_long {
    let ret: c_long;
    let _ign1: u64;
    let _ign2: u64;
    let addr = HYPERCALL_PAGE.as_ptr().offset(op as isize);

    asm!("call *$0"
         : "={rax}" (ret), "={rdi}" (_ign1), "={rsi}" (_ign2)
         : "r" (addr),
           "{rdi}" (a1), "{rsi}" (a2)
         : "memory"
         : "volatile");
    ret
}

#[inline]
pub unsafe fn hypercall_3(op: Hypercall,
                          a1: u64,
                          a2: u64,
                          a3: u64) -> c_long {
    let ret: c_long;
    let _ign1: u64;
    let _ign2: u64;
    let _ign3: u64;
    let addr = HYPERCALL_PAGE.as_ptr().offset(op as isize);

    asm!("call *$0"
         : "={rax}" (ret), "={rdi}" (_ign1), "={rsi}" (_ign2), "={rdx}" (_ign3)
         : "r" (addr),
           "{rdi}" (a1), "{rsi}" (a2), "{rdx}" (a3)
         : "memory"
         : "volatile");
    ret
}

#[inline]
pub unsafe fn hypercall_4(op: Hypercall,
                          a1: u64,
                          a2: u64,
                          a3: u64,
                          a4: u64) -> c_long {
    let ret: c_long;
    let _ign1: u64;
    let _ign2: u64;
    let _ign3: u64;
    let _ign4: u64;
    let addr = HYPERCALL_PAGE.as_ptr().offset(op as isize);

    asm!("call *$0"
         : "={rax}" (ret), "={rdi}" (_ign1), "={rsi}" (_ign2), "={rdx}" (_ign3),
           "={r10}" (_ign4)
         : "r" (addr),
           "{rdi}" (a1), "{rsi}" (a2), "{rdx}" (a3), "{r10}" (a4)
         : "memory"
         : "volatile");
    ret
}

#[inline]
pub unsafe fn hypercall_5(op: Hypercall,
                          a1: u64,
                          a2: u64,
                          a3: u64,
                          a4: u64,
                          a5: u64) -> c_long {
    let ret: c_long;
    let _ign1: u64;
    let _ign2: u64;
    let _ign3: u64;
    let _ign4: u64;
    let _ign5: u64;
    let addr = HYPERCALL_PAGE.as_ptr().offset(op as isize);

    asm!("call *$0"
         : "={rax}" (ret), "={rdi}" (_ign1), "={rsi}" (_ign2), "={rdx}" (_ign3),
           "={r10}" (_ign4), "={r9}" (_ign5)
         : "r" (addr),
           "{rdi}" (a1), "{rsi}" (a2), "{rdx}" (a3), "{r10}" (a4), "{r9}" (a5)
         : "memory"
         : "volatile");
    ret
}


#[macro_export]
macro_rules! hypercall {
    ($op:expr, $a1:expr)
        => ( $crate::hypercall::hypercall_1($op, $a1 as u64) );

    ($op:expr, $a1:expr, $a2:expr)
        => ( $crate::hypercall::hypercall_2($op, $a1 as u64, $a2 as u64) );

    ($op:expr, $a1:expr, $a2:expr, $a3:expr)
        => ( $crate::hypercall::hypercall_3($op, $a1 as u64, $a2 as u64, $a3 as u64) );

    ($op:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr)
        => ( $crate::hypercall::hypercall_4($op, $a1 as u64, $a2 as u64, $a3 as u64, $a4 as u64) );

    ($op:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr)
        => ( $crate::hypercall::hypercall_5($op, $a1 as u64, $a2 as u64, $a3 as u64, $a4 as u64, $a5 as u64) );
}
