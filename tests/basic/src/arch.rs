#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

pub type regbits = BitVector<5>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub PC: xlenbits,
    pub nextPC: xlenbits,
    pub Xs: [xlenbits;32],
}

pub fn rX(sail_ctx: &mut SailVirtCtx, r: BitVector<5>) -> BitVector<64> {
    match r {
        b__0 if {(b__0 == BitVector::<5>::new(0b00000))} => {BitVector::<4>::new(0b0000).zero_extend::<64>()}
        _ => {sail_ctx.Xs[r.as_usize()]}
        _ => {panic!("Unreachable code")}
    }
}

pub fn wX(sail_ctx: &mut SailVirtCtx, r: BitVector<5>, v: BitVector<64>) {
    if {(r != BitVector::<5>::new(0b00000))} {
        sail_ctx.Xs = {
            let var_1 = sail_ctx.Xs;
            let var_2 = r.as_usize();
            let var_3 = v;
            plain_vector_update(sail_ctx, var_1, var_2, var_3)
        }
    } else {
        ()
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    RISCV_ADDI,
    RISCV_SLTI,
    RISCV_SLTIU,
    RISCV_XORI,
    RISCV_ORI,
    RISCV_ANDI,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ITYPE((BitVector<12>, BitVector<5>, BitVector<5>, iop)),
    LOAD((BitVector<12>, BitVector<5>, BitVector<5>)),
}

pub fn execute_ITYPE(sail_ctx: &mut SailVirtCtx, imm: BitVector<12>, rs1: BitVector<5>, rd: BitVector<5>) {
    let rs1_val = rX(sail_ctx, rs1);
    let imm_ext: xlenbits = BitVector::<64>::new(imm.bits());
    let result = rs1_val.wrapped_add(imm_ext);
    wX(sail_ctx, rd, result)
}