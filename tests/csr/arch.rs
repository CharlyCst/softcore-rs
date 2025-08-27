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
    pub mscratch: xlenbits,
    pub sscratch: xlenbits,
    pub cur_privilege: Privilege,
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
/// Generated from the Sail sources at `tests/csr/arch.sail` L7.
pub fn EXTZ<const N: i128, const M: i128>(m: i128, v: BitVector<N>) -> BitVector<M> {
    v.zero_extend()
}

/// EXTS
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L10.
pub fn EXTS<const N: i128, const M: i128>(m: i128, v: BitVector<N>) -> BitVector<M> {
    sail_sign_extend(core_ctx, v, m)
}

/// (operator >=_u)
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L23.
pub fn _operator_biggerequal_u_<const N: i128>(x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.unsigned() >= y.unsigned())
}

pub const xlen: i128 = 64;

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

/// Privilege
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L34.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

/// privLevel_to_bits
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L37-42.
pub fn privLevel_to_bits(p: Privilege) -> BitVector<2> {
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

/// rX
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L67-71.
pub fn rX(core_ctx: &mut Core, r: BitVector<5>) -> BitVector<64> {
    match r {
        b__0 if {(b__0 == BitVector::<5>::new(0b00000))} => {EXTZ(64, BitVector::<4>::new(0b0000))}
        _ => {core_ctx.Xs[(r.unsigned() as usize)]}
        _ => {panic!("Unreachable code")}
    }
}

/// wX
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L74-77.
pub fn wX(core_ctx: &mut Core, r: BitVector<5>, v: BitVector<64>) {
    if {(r != BitVector::<5>::new(0b00000))} {
        core_ctx.Xs[(r.unsigned() as usize)] = v
    } else {
        ()
    }
}

/// bool_bits_backwards
/// 
/// Generated from the Sail sources.
pub fn bool_bits_backwards(arg_hashtag_: BitVector<1>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<1>::new(0b1))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_bits_backwards_matches
/// 
/// Generated from the Sail sources.
pub fn bool_bits_backwards_matches(arg_hashtag_: BitVector<1>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<1>::new(0b1))} => {true}
        b__1 if {(b__1 == BitVector::<1>::new(0b0))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// handle_illegal
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L92-95.
pub fn handle_illegal(unit_arg: ()) {
    
}

/// iop
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L99.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    RISCV_ADDI,
    RISCV_SLTI,
    RISCV_SLTIU,
    RISCV_XORI,
    RISCV_ORI,
    RISCV_ANDI
}

/// csrop
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L100.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum csrop {
    CSRRW,
    CSRRS,
    CSRRC
}

/// Retired
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L101.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

/// ast
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L103.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ITYPE((BitVector<12>, regidx, regidx, iop)),
    CSR((BitVector<12>, regidx, regidx, bool, csrop))
}

/// encdec_csrop_backwards
/// 
/// Generated from the Sail sources.
pub fn encdec_csrop_backwards(arg_hashtag_: BitVector<2>) -> csrop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b01))} => {csrop::CSRRW}
        b__1 if {(b__1 == BitVector::<2>::new(0b10))} => {csrop::CSRRS}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {csrop::CSRRC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_csrop_backwards_matches
/// 
/// Generated from the Sail sources.
pub fn encdec_csrop_backwards_matches(arg_hashtag_: BitVector<2>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b01))} => {true}
        b__1 if {(b__1 == BitVector::<2>::new(0b10))} => {true}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

pub type csrRW = BitVector<2>;

/// csrAccess
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L135.
pub fn csrAccess(csr: BitVector<12>) -> BitVector<2> {
    csr.subrange::<10, 12, 2>()
}

/// csrPriv
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L136.
pub fn csrPriv(csr: BitVector<12>) -> BitVector<2> {
    csr.subrange::<8, 10, 2>()
}

/// encdec_backwards
/// 
/// Generated from the Sail sources.
pub fn encdec_backwards(arg_hashtag_: BitVector<32>) -> ast {
    let head_exp_hashtag_ = arg_hashtag_;
    match match head_exp_hashtag_ {
        v__0 if {((v__0.subrange::<12, 15, 3>() == BitVector::<3>::new(0b000)) && (v__0.subrange::<0, 7, 7>() == BitVector::<7>::new(0b0010011)))} => {let imm: BitVector<12> = v__0.subrange::<20, 32, 12>();
        let rs1: regidx = v__0.subrange::<15, 20, 5>();
        let rd: regidx = v__0.subrange::<7, 12, 5>();
        let imm: BitVector<12> = v__0.subrange::<20, 32, 12>();
        Some(ast::ITYPE((imm, rs1, rd, iop::RISCV_ADDI)))}
        v__3 if {let mapping1_hashtag__var_1: BitVector<2> = v__3.subrange::<12, 14, 2>();
        let mapping0_hashtag__var_2: BitVector<1> = v__3.subrange::<14, 15, 1>();
        ((bool_bits_backwards_matches(mapping0_hashtag__var_2) && encdec_csrop_backwards_matches(mapping1_hashtag__var_1)) && (v__3.subrange::<0, 7, 7>() == BitVector::<7>::new(0b1110011)))} => {let csr: BitVector<12> = v__3.subrange::<20, 32, 12>();
        let rs1: BitVector<5> = v__3.subrange::<15, 20, 5>();
        let rd: BitVector<5> = v__3.subrange::<7, 12, 5>();
        let mapping1_hashtag_: BitVector<2> = v__3.subrange::<12, 14, 2>();
        let mapping0_hashtag_: BitVector<1> = v__3.subrange::<14, 15, 1>();
        let csr: BitVector<12> = v__3.subrange::<20, 32, 12>();
        match (bool_bits_backwards(mapping0_hashtag_), encdec_csrop_backwards(mapping1_hashtag_)) {
            (is_imm, op) => {Some(ast::CSR((csr, rs1, rd, is_imm, op)))}
            _ => {None}
            _ => {panic!("Unreachable code")}
        }}
        _ => {None}
        _ => {panic!("Unreachable code")}
    } {
        Some(result) => {result}
        _ => {panic!("Unreachable code")}
    }
}

/// is_CSR_defined
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L141-146.
pub fn is_CSR_defined(csr: BitVector<12>, p: Privilege) -> bool {
    match csr {
        b__0 if {(b__0 == BitVector::<12>::new(0b001101000000))} => {(p == Privilege::Machine)}
        b__1 if {(b__1 == BitVector::<12>::new(0b000101000000))} => {((p == Privilege::Machine) || (p == Privilege::Supervisor))}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// check_CSR_access
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L149-151.
pub fn check_CSR_access(csrrw: BitVector<2>, csrpr: BitVector<2>, p: Privilege, isWrite: bool) -> bool {
    (!(((isWrite == true) && (csrrw == BitVector::<2>::new(0b11)))) && _operator_biggerequal_u_(privLevel_to_bits(p), csrpr))
}

/// check_CSR
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L153-154.
pub fn check_CSR(csr: BitVector<12>, p: Privilege, isWrite: bool) -> bool {
    (is_CSR_defined(csr, p) && check_CSR_access(csrAccess(csr), csrPriv(csr), p, isWrite))
}

/// readCSR
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L156-164.
pub fn readCSR(core_ctx: &mut Core, csr: BitVector<12>) -> BitVector<64> {
    let res: xlenbits = match (csr, 64) {
        (b__0, _) if {(b__0 == BitVector::<12>::new(0b001101000000))} => {core_ctx.mscratch}
        (b__1, _) if {(b__1 == BitVector::<12>::new(0b000101000000))} => {core_ctx.sscratch}
        _ => {EXTZ(64, BitVector::<4>::new(0b0000))}
        _ => {panic!("Unreachable code")}
    };
    res
}

/// writeCSR
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L166-173.
pub fn writeCSR(core_ctx: &mut Core, csr: BitVector<12>, value: BitVector<64>) {
    let res: Option<xlenbits> = match (csr, 64) {
        (b__0, _) if {(b__0 == BitVector::<12>::new(0b001101000000))} => {{
            core_ctx.mscratch = value;
            Some(core_ctx.mscratch)
        }}
        (b__1, _) if {(b__1 == BitVector::<12>::new(0b000101000000))} => {{
            core_ctx.sscratch = value;
            Some(core_ctx.sscratch)
        }}
        _ => {None}
        _ => {panic!("Unreachable code")}
    };
    ()
}

/// execute
/// 
/// Generated from the Sail sources at `tests/csr/arch.sail` L116-122.
pub fn execute(core_ctx: &mut Core, merge_hashtag_var: ast) -> Retired {
    match merge_hashtag_var {
        ast::ITYPE((imm, rs1, rd, iop::RISCV_ADDI)) => {{
            let rs1_val = rX(core_ctx, rs1);
            let imm_ext: xlenbits = EXTS(64, imm);
            let result = rs1_val.wrapped_add(imm_ext);
            wX(core_ctx, rd, result);
            Retired::RETIRE_SUCCESS
        }}
        ast::CSR((csr, rs1, rd, is_imm, op)) => {{
            let rs1_val: xlenbits = if {is_imm} {
                EXTZ(64, rs1)
            } else {
                rX(core_ctx, rs1)
            };
            let isWrite: bool = match op {
                csrop::CSRRW => {true}
                _ => {if {is_imm} {
                    (rs1_val.unsigned() != 0)
                } else {
                    (rs1.unsigned() != 0)
                }}
                _ => {panic!("Unreachable code")}
            };
            if {!({
                let var_1 = core_ctx.cur_privilege;
                check_CSR(csr, var_1, isWrite)
            })} {
                ();
                Retired::RETIRE_FAIL
            } else {
                let csr_val = readCSR(core_ctx, csr);
                if {isWrite} {
                    let new_val: xlenbits = match op {
                        csrop::CSRRW => {rs1_val}
                        csrop::CSRRS => {(csr_val | rs1_val)}
                        csrop::CSRRC => {(csr_val & !(rs1_val))}
                        _ => {panic!("Unreachable code")}
                    };
                    writeCSR(core_ctx, csr, new_val)
                } else {
                    ()
                };
                wX(core_ctx, rd, csr_val);
                Retired::RETIRE_SUCCESS
            }
        }}
        _ => {panic!("Unreachable code")}
    }
}
