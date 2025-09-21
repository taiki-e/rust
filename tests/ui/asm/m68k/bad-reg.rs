//@ add-core-stubs
//@ needs-asm-support
//@ revisions: m68k
//@[m68k] compile-flags: --target m68k-unknown-linux-gnu
//@[m68k] needs-llvm-components: m68k

#![crate_type = "lib"]
#![feature(no_core, asm_experimental_arch)]
#![no_core]

extern crate minicore;
use minicore::*;

fn f() {
    let mut f = 0.0_f32;
    let mut d = 0.0_f64;
    unsafe {
        // Unsupported registers
        asm!("", out("a4") _);
        //~^ ERROR used internally by LLVM and cannot be used as an operand for inline asm
        asm!("", out("a5") _);
        //~^ ERROR used internally by LLVM and cannot be used as an operand for inline asm
        asm!("", out("bp") _);
        //~^ ERROR used internally by LLVM and cannot be used as an operand for inline asm
        asm!("", out("a6") _);
        //~^ ERROR used internally by LLVM and cannot be used as an operand for inline asm
        asm!("", out("fp") _);
        //~^ ERROR used internally by LLVM and cannot be used as an operand for inline asm
        asm!("", out("a7") _);
        //~^ ERROR the stack pointer cannot be used as an operand for inline asm
        asm!("", out("sp") _);
        //~^ ERROR the stack pointer cannot be used as an operand for inline asm
        asm!("", out("usp") _);
        //~^ ERROR the stack pointer cannot be used as an operand for inline asm
        asm!("", out("ssp") _);
        //~^ ERROR the stack pointer cannot be used as an operand for inline asm
        asm!("", out("isp") _);
        //~^ ERROR the stack pointer cannot be used as an operand for inline asm

        asm!("", out("fp0") _); // ok
        asm!("/* {} */", in(freg) f);
        //~^ ERROR register class `freg` can only be used as a clobber, not as an input or output
        //~| ERROR type `f32` cannot be used with this register class
        asm!("/* {} */", out(freg) _);
        //~^ ERROR register class `freg` can only be used as a clobber, not as an input or output
        asm!("/* {} */", in(freg) d);
        //~^ ERROR register class `freg` can only be used as a clobber, not as an input or output
        //~| ERROR type `f64` cannot be used with this register class
        asm!("/* {} */", out(freg) d);
        //~^ ERROR register class `freg` can only be used as a clobber, not as an input or output
        //~| ERROR type `f64` cannot be used with this register class
    }
}
