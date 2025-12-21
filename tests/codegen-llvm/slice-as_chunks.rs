//@ compile-flags: -Copt-level=3
//@ only-64bit (because the LLVM type of i64 for usize shows up)

#![crate_type = "lib"]
#![no_std]

// Hack to get the correct usize
// CHECK: @helper([[USIZE:i32|i64]]
#[no_mangle]
pub fn helper(_: usize) {}

// CHECK-LABEL: @chunks4
#[no_mangle]
pub fn chunks4(x: &[u8]) -> &[[u8; 4]] {
    // CHECK-NEXT: start:
    // CHECK-NEXT: lshr [[USIZE]] %x.1, 2
    // CHECK-NOT: shl
    // CHECK-NOT: mul
    // CHECK-NOT: udiv
    // CHECK-NOT: urem
    // CHECK: ret
    x.as_chunks().0
}

// CHECK-LABEL: @chunks4_with_remainder
#[no_mangle]
pub fn chunks4_with_remainder(x: &[u8]) -> (&[[u8; 4]], &[u8]) {
    // FIXME(xdoardo): Find a way to use numeric substitution in regex blocks (FileCheck)
    // CHECK-DAG: and [[USIZE]] %x.1, {{2147483644|9223372036854775804}}
    // CHECK-DAG: and [[USIZE]] %x.1, 3
    // CHECK-DAG: lshr
    // CHECK-NOT: mul
    // CHECK-NOT: udiv
    // CHECK-NOT: urem
    // CHECK: ret
    x.as_chunks()
}
