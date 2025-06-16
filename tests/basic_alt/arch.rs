#![allow(warnings)]

use softcore_prelude::*;

/// The software core.
/// 
/// This struct represents a software core, and holds all the registers as well as the core configuration.
/// The core is the main abstraction exposed by the softcore library and represents a single execution thread.
/// 
/// The raw functions translated directly from the specification are available in the `raw` module, whereas higher-level wrappers are implemented as methods on the [Core] struct directly.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Core {
    pub PC: xlenbits,
    pub nextPC: xlenbits,
    pub Xs: [xlenbits; (32 as usize)],
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {

}

/// EXTZ
/// 
/// Generated from the Sail sources at `tests/basic_alt/arch.sail` L7.
pub fn EXTZ<const N: i128, const M: i128>(m: i128, v: BitVector<N>) -> BitVector<M> {
    v.zero_extend()
}

pub const xlen: i128 = 64;

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitVector<xlen>;

pub type regbits = BitVector<5>;

/// rX
/// 
/// Generated from the Sail sources at `tests/basic_alt/arch.sail` L31-35.
pub fn rX(core_ctx: &mut Core, r: BitVector<5>) -> BitVector<64> {
    match r {
        b__0 if {(b__0 == BitVector::<5>::new(0b00000))} => {EXTZ(64, BitVector::<4>::new(0b0000))}
        _ => {core_ctx.Xs[(r.unsigned() as usize)]}
        _ => {panic!("Unreachable code")}
    }
}

/// wX
/// 
/// Generated from the Sail sources at `tests/basic_alt/arch.sail` L38-41.
pub fn wX(core_ctx: &mut Core, r: BitVector<5>, v: BitVector<64>) {
    if {(r != BitVector::<5>::new(0b00000))} {
        core_ctx.Xs[(r.unsigned() as usize)] = v
    } else {
        ()
    }
}

/// iop
/// 
/// Generated from the Sail sources at `tests/basic_alt/arch.sail` L61.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    RISCV_ADDI,
    RISCV_SLTI,
    RISCV_SLTIU,
    RISCV_XORI,
    RISCV_ORI,
    RISCV_ANDI
}

/// ast
/// 
/// Generated from the Sail sources at `tests/basic_alt/arch.sail` L63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ITYPE((BitVector<12>, BitVector<5>, BitVector<5>, iop)),
    LOAD((BitVector<12>, BitVector<5>, BitVector<5>))
}