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
