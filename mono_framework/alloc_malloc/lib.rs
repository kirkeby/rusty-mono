// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "alloc_malloc"]
#![crate_type = "rlib"]
#![no_std]
#![allocator]
#![feature(allocator)]
#![allow(unused_variables)]

// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values. In practice, the alignment is a
// constant at the call site and the branch will be optimized out.
#[cfg(all(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "powerpc",
              target_arch = "powerpc64",
              target_arch = "asmjs",
              target_arch = "wasm32")))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86_64",
              target_arch = "aarch64",
              target_arch = "mips64",
              target_arch = "s390x",
              target_arch = "sparc64")))]
const MIN_ALIGN: usize = 16;

use core::cmp;
use core::ptr;

extern "C" {
    fn malloc(_: usize) -> *mut u8;
    fn realloc(_: *mut u8, _: usize) -> *mut u8;
    fn memalign(_: usize, _: usize) -> *mut u8;
    fn free(_: *mut u8) -> ();
}

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe {
        if align <= MIN_ALIGN {
            malloc(size as usize) as *mut u8
        } else {
            memalign(align, size)
        }
    }
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe {
        free(ptr as *mut u8)
    }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    old_size: usize,
                                    size: usize,
                                    align: usize)
                                    -> *mut u8 {
    unsafe {
        if align <= MIN_ALIGN {
            realloc(ptr as *mut u8, size as usize) as *mut u8
        } else {
            let new_ptr = __rust_allocate(size, align);
            if !new_ptr.is_null() {
                ptr::copy(ptr, new_ptr, cmp::min(size, old_size));
                free(ptr);
            }
            new_ptr
        }
    }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(ptr: *mut u8,
                                            old_size: usize,
                                            size: usize,
                                            align: usize)
                                            -> usize {
    old_size
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, align: usize) -> usize {
    size
}
