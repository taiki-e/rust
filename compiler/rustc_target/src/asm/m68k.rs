use std::fmt;

use rustc_span::Symbol;

use super::{InlineAsmArch, InlineAsmType, ModifierInfo};

/*
Support clobber_abi and floating-point registers (clobber-only) in m68k inline assembly



This supports `clobber_abi` which is one of the requirements of stabilization mentioned in #93335.

This basically does a similar thing I did in https://github.com/rust-lang/rust/pull/130630 (for s390x) and https://github.com/rust-lang/rust/pull/131341 (for powerpc/powerpc64/powerpc64le), but for m68k.
- This also supports floating-point registers (as `freg`) as clobber-only, which need to support clobbering of them to implement `clobber_abi`.
- `freg` should be able to accept f32 and f64 as input/output if the unstable `isa-68881` target feature is enabled, but LLVM doesn't support constraint (["f" in GCC](https://gcc.gnu.org/onlinedocs/gcc/Machine-Constraints.html)) for this (https://github.com/llvm/llvm-project/issues/61806). So I have not implemented it in this PR.

Refs:
- Calling Convention:
  - https://m680x0.github.io/doc/abi.html#calling-convention
  - GCC https://github.com/gcc-mirror/gcc/blob/releases/gcc-15.2.0/gcc/config/m68k/m68k.h#L356
  - LLVM https://github.com/llvm/llvm-project/blob/llvmorg-21.1.0/llvm/lib/Target/M68k/M68kCallingConv.td
- Register info:
  - LLVM https://github.com/llvm/llvm-project/blob/llvmorg-21.1.0/llvm/lib/Target/M68k/M68kRegisterInfo.td#L71-L77

cc @glaubitz @ricky26 (designated developers of [m68k-unknown-linux-gnu](https://doc.rust-lang.org/nightly/rustc/platform-support/m68k-unknown-linux-gnu.html#designated-developers))
cc @knickish

@rustbot label +O-motorola68k +A-inline-assembly



no f in
https://github.com/llvm/llvm-project/blob/llvmorg-21.1.0/clang/lib/Basic/Targets/M68k.cpp#L147
*/

// https://github.com/llvm/llvm-project/blob/llvmorg-19.1.0/clang/lib/Basic/Targets/M68k.cpp#L128
// https://github.com/gcc-mirror/gcc/blob/980929bdb80f1a1490caab5acc6d9740e0f9b539/gcc/config/m68k/m68k.h#L686

def_reg_class! {
    M68k M68kInlineAsmRegClass {
        reg,
        reg_addr,
        reg_data,
        freg,
    }
}

impl M68kInlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        &[]
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, _ty: InlineAsmType) -> Option<Self> {
        None
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        _ty: InlineAsmType,
    ) -> Option<ModifierInfo> {
        None
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<ModifierInfo> {
        None
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::reg => types! { _: I16, I32; },
            Self::reg_data => types! { _: I8, I16, I32; },
            Self::reg_addr => types! { _: I16, I32; },
            // FIXME: GCC has "f" constraint for this, but not yet supported in LLVM: https://github.com/llvm/llvm-project/issues/61806
            // Self::freg => types! { isa_68881: F32, F64; },
            Self::freg => &[],
        }
    }
}

def_regs! {
    M68k M68kInlineAsmReg M68kInlineAsmRegClass {
        d0: reg, reg_data = ["d0"],
        d1: reg, reg_data = ["d1"],
        d2: reg, reg_data = ["d2"],
        d3: reg, reg_data = ["d3"],
        d4: reg, reg_data = ["d4"],
        d5: reg, reg_data = ["d5"],
        d6: reg, reg_data = ["d6"],
        d7: reg, reg_data = ["d7"],
        a0: reg, reg_addr = ["a0"],
        a1: reg, reg_addr = ["a1"],
        a2: reg, reg_addr = ["a2"],
        a3: reg, reg_addr = ["a3"],
        fp0: freg = ["fp0"],
        fp1: freg = ["fp1"],
        fp2: freg = ["fp2"],
        fp3: freg = ["fp3"],
        fp4: freg = ["fp4"],
        fp5: freg = ["fp5"],
        fp6: freg = ["fp6"],
        fp7: freg = ["fp7"],
        #error = ["a4"] =>
            "a4 is used internally by LLVM and cannot be used as an operand for inline asm",
        #error = ["a5", "bp"] =>
            "a5 is used internally by LLVM and cannot be used as an operand for inline asm",
        #error = ["a6", "fp"] =>
            "a6 is used internally by LLVM and cannot be used as an operand for inline asm",
        #error = ["a7", "sp", "usp", "ssp", "isp"] =>
            "the stack pointer cannot be used as an operand for inline asm",
    }
}

impl M68kInlineAsmReg {
    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        _modifier: Option<char>,
    ) -> fmt::Result {
        out.write_str(self.name())
    }
}
