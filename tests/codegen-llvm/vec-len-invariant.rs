//@ compile-flags: -Copt-level=3
//@ only-64bit
//
// This test confirms that we do not reload the length of a Vec after growing it in push.

#![crate_type = "lib"]
#![no_std]

extern crate alloc;
use alloc::vec::Vec;

// Hack to get the correct size for the length part in slices
// CHECK: @helper([[USIZE:i16|i32|i64]]
#[no_mangle]
pub fn helper(_: usize) {}

// CHECK-LABEL: @should_load_once
#[no_mangle]
pub fn should_load_once(v: &mut Vec<u8>) {
    // CHECK: load [[USIZE]]
    // CHECK: call {{.*}}grow_one
    // CHECK-NOT: load [[USIZE]]
    // CHECK: add {{.*}}, 1
    v.push(1);
}
