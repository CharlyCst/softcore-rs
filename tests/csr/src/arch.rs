#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub PC: xlenbits,
    pub nextPC: xlenbits,
    pub mscratch: xlenbits,
    pub sscratch: xlenbits,
    pub cur_privilege: Privilege,
    pub Xs: [xlenbits;32],
    pub config: SailConfig,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfig {

}

pub fn _operator_biggerequal_u_<const N: usize>(sail_ctx: &mut SailVirtCtx, x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.as_usize() >= y.as_usize())
}

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

pub fn privLevel_to_bits(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<2> {
    match p {
        Privilege::User => {BitVector::<2>::new(0b00)}
        Privilege::Supervisor => {BitVector::<2>::new(0b01)}
        Privilege::Machine => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

pub fn rX(sail_ctx: &mut SailVirtCtx, r: BitVector<5>) -> BitVector<64> {
    match r {
        b__0 if {(b__0 == BitVector::<5>::new(0b00000))} => {BitVector::<4>::new(0b0000).zero_extend::<64>()}
        _ => {sail_ctx.Xs[r.as_usize()]}
        _ => {panic!("Unreachable code")}
    }
}

pub fn wX(sail_ctx: &mut SailVirtCtx, r: BitVector<5>, v: BitVector<64>) {
    if {(r != BitVector::<5>::new(0b00000))} {
        sail_ctx.Xs[r.as_usize()] = v
    } else {
        ()
    }
}

pub fn handle_illegal(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    ()
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    RISCV_ADDI,
    RISCV_SLTI,
    RISCV_SLTIU,
    RISCV_XORI,
    RISCV_ORI,
    RISCV_ANDI
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum csrop {
    CSRRW,
    CSRRS,
    CSRRC
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ITYPE((BitVector<12>, regidx, regidx, iop)),
    CSR((BitVector<12>, regidx, regidx, bool, csrop))
}

pub type csrRW = BitVector<2>;

pub fn csrAccess(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>) -> BitVector<2> {
    csr.subrange::<10, 12, 2>()
}

pub fn csrPriv(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>) -> BitVector<2> {
    csr.subrange::<8, 10, 2>()
}

pub fn is_CSR_defined(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>, p: Privilege) -> bool {
    match csr {
        b__0 if {(b__0 == BitVector::<12>::new(0b001101000000))} => {(p == Privilege::Machine)}
        b__1 if {(b__1 == BitVector::<12>::new(0b000101000000))} => {((p == Privilege::Machine) || (p == Privilege::Supervisor))}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

pub fn check_CSR_access(sail_ctx: &mut SailVirtCtx, csrrw: BitVector<2>, csrpr: BitVector<2>, p: Privilege, isWrite: bool) -> bool {
    (!(((isWrite == true) && (csrrw == BitVector::<2>::new(0b11)))) && {
        let var_1 = privLevel_to_bits(sail_ctx, p);
        let var_2 = csrpr;
        _operator_biggerequal_u_(sail_ctx, var_1, var_2)
    })
}

pub fn check_CSR(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>, p: Privilege, isWrite: bool) -> bool {
    (is_CSR_defined(sail_ctx, csr, p) && {
        let var_1 = csrAccess(sail_ctx, csr);
        let var_2 = csrPriv(sail_ctx, csr);
        let var_3 = p;
        let var_4 = isWrite;
        check_CSR_access(sail_ctx, var_1, var_2, var_3, var_4)
    })
}

pub fn readCSR(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>) -> BitVector<64> {
    let res: xlenbits = match (csr, 64) {
        (b__0, _) if {(b__0 == BitVector::<12>::new(0b001101000000))} => {sail_ctx.mscratch}
        (b__1, _) if {(b__1 == BitVector::<12>::new(0b000101000000))} => {sail_ctx.sscratch}
        _ => {BitVector::<4>::new(0b0000).zero_extend::<64>()}
        _ => {panic!("Unreachable code")}
    };
    res
}

pub fn writeCSR(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>, value: BitVector<64>) {
    let res: Option<xlenbits> = match (csr, 64) {
        (b__0, _) if {(b__0 == BitVector::<12>::new(0b001101000000))} => {{
            sail_ctx.mscratch = value;
            Some(sail_ctx.mscratch)
        }}
        (b__1, _) if {(b__1 == BitVector::<12>::new(0b000101000000))} => {{
            sail_ctx.sscratch = value;
            Some(sail_ctx.sscratch)
        }}
        _ => {None}
        _ => {panic!("Unreachable code")}
    };
    ()
}

pub fn execute(sail_ctx: &mut SailVirtCtx, merge_hashtag_var: ast) -> Retired {
    match merge_hashtag_var {
        ast::ITYPE((imm, rs1, rd, iop::RISCV_ADDI)) => {{
            let rs1_val = rX(sail_ctx, rs1);
            let imm_ext: xlenbits = BitVector::<64>::new(imm.bits());
            let result = rs1_val.wrapped_add(imm_ext);
            wX(sail_ctx, rd, result);
            Retired::RETIRE_SUCCESS
        }}
        ast::CSR((csr, rs1, rd, is_imm, op)) => {{
            let rs1_val: xlenbits = if {is_imm} {
                rs1.zero_extend::<64>()
            } else {
                rX(sail_ctx, rs1)
            };
            let isWrite: bool = match op {
                csrop::CSRRW => {true}
                _ => {if {is_imm} {
                    (rs1_val.as_usize() != 0)
                } else {
                    (rs1.as_usize() != 0)
                }}
                _ => {panic!("Unreachable code")}
            };
            if {!(check_CSR(sail_ctx, csr, sail_ctx.cur_privilege, isWrite))} {
                handle_illegal(sail_ctx, ());
                Retired::RETIRE_FAIL
            } else {
                let csr_val = readCSR(sail_ctx, csr);
                if {isWrite} {
                    let new_val: xlenbits = match op {
                        csrop::CSRRW => {rs1_val}
                        csrop::CSRRS => {(csr_val | rs1_val)}
                        csrop::CSRRC => {(csr_val & !(rs1_val))}
                        _ => {panic!("Unreachable code")}
                    };
                    writeCSR(sail_ctx, csr, new_val)
                } else {
                    ()
                };
                wX(sail_ctx, rd, csr_val);
                Retired::RETIRE_SUCCESS
            }
        }}
        _ => {panic!("Unreachable code")}
    }
}