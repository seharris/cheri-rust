//@ compile-flags: -Copt-level=3

#![crate_type = "lib"]
#![feature(slice_from_ptr_range)]
#![no_std]

// Hack to get the correct usize
// CHECK: @helper([[USIZE:i[0-9]+]]
#[no_mangle]
pub fn helper(_: usize) {}

// This is intentionally using a non-power-of-two array length,
// as that's where the optimization differences show up

// CHECK-LABEL: @flatten_via_ptr_range
#[no_mangle]
pub fn flatten_via_ptr_range(slice_of_arrays: &[[i32; 13]]) -> &[i32] {
    // CHECK-NOT: lshr
    // CHECK-NOT: udiv
    // CHECK: mul nuw nsw [[USIZE]] %{{.+}}, 13
    // CHECK-NOT: lshr
    // CHECK-NOT: udiv
    let r = slice_of_arrays.as_ptr_range();
    let r = r.start.cast()..r.end.cast();
    unsafe { core::slice::from_ptr_range(r) }
}
