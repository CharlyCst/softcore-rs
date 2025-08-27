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

/// Initialize all registers.
/// 
/// This function should be called before using a fresh core, otherwise the core might not be in a valid state.
pub fn _reset_all_registers() {
    
}

/// EXTZ
/// 
/// Generated from the Sail sources at `tests/basic/arch.sail` L7.
pub fn EXTZ<const N: i128, const M: i128>(m: i128, v: BitVector<N>) -> BitVector<M> {
    v.zero_extend()
}

/// EXTS
/// 
/// Generated from the Sail sources at `tests/basic/arch.sail` L10.
pub fn EXTS<const N: i128, const M: i128>(m: i128, v: BitVector<N>) -> BitVector<M> {
    sail_sign_extend(core_ctx, v, m)
}

pub const xlen: i128 = 64;

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitVector<xlen>;

pub type regbits = BitVector<5>;

/// rX
/// 
/// Generated from the Sail sources at `tests/basic/arch.sail` L31-35.
pub fn rX(core_ctx: &mut Core, r: BitVector<5>) -> BitVector<64> {
    match r {
        b__0 if {(b__0 == BitVector::<5>::new(0b00000))} => {EXTZ(64, BitVector::<4>::new(0b0000))}
        _ => {core_ctx.Xs[(r.unsigned() as usize)]}
        _ => {panic!("Unreachable code")}
    }
}

/// wX
/// 
/// Generated from the Sail sources at `tests/basic/arch.sail` L38-41.
pub fn wX(core_ctx: &mut Core, r: BitVector<5>, v: BitVector<64>) {
    if {(r != BitVector::<5>::new(0b00000))} {
        core_ctx.Xs[(r.unsigned() as usize)] = v
    } else {
        ()
    }
}

/// iop
/// 
/// Generated from the Sail sources at `tests/basic/arch.sail` L61.
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
/// Generated from the Sail sources at `tests/basic/arch.sail` L63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ITYPE((BitVector<12>, BitVector<5>, BitVector<5>, iop)),
    LOAD((BitVector<12>, BitVector<5>, BitVector<5>))
}

/// execute
/// 
/// Generated from the Sail sources at `tests/basic/arch.sail` L75-79.
pub fn execute(core_ctx: &mut Core, merge_hashtag_var: ast) {
    match merge_hashtag_var {
        ast::ITYPE((imm, rs1, rd, iop::RISCV_ADDI)) => {let rs1_val = rX(core_ctx, rs1);
        let imm_ext: xlenbits = EXTS(64, imm);
        let result = rs1_val.wrapped_add(imm_ext);
        wX(core_ctx, rd, result)}
        ast::LOAD((imm, rs1, rd)) => {let addr: xlenbits = rX(core_ctx, rs1).wrapped_add(EXTS(64, imm));
        let result: xlenbits = EXTS(64, BitVector::<1>::new(0b0));
        wX(core_ctx, rd, result)}
        _ => {panic!("Unreachable code")}
    }
}
