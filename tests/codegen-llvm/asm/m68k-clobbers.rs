//@ add-core-stubs
//@ revisions: m68k
//@[m68k] compile-flags: --target m68k-unknown-linux-gnu
//@[m68k] needs-llvm-components: m68k

#![crate_type = "rlib"]
#![feature(no_core, asm_experimental_arch)]
#![no_core]

extern crate minicore;
use minicore::*;

// CHECK-LABEL: @flags_clobber
// CHECK: call void asm sideeffect "", "~{ccr}"()
#[no_mangle]
pub unsafe fn flags_clobber() {
    asm!("", options(nostack, nomem));
}

// CHECK-LABEL: @no_clobber
// CHECK: call void asm sideeffect "", ""()
#[no_mangle]
pub unsafe fn no_clobber() {
    asm!("", options(nostack, nomem, preserves_flags));
}

// CHECK-LABEL: @clobber_abi
// CHECK: asm sideeffect "", "={d0},={d1},={a0},={a1},~{fp0},~{fp1}"()
#[no_mangle]
pub unsafe fn clobber_abi() {
    asm!("", clobber_abi("C"), options(nostack, nomem, preserves_flags));
}
