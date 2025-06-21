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
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {

}

pub const xlen: i128 = 64;

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

/// Privilege
/// 
/// Generated from the Sail sources at `tests/decoder/arch.sail` L34.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

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
/// Generated from the Sail sources at `tests/decoder/arch.sail` L58.
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
/// Generated from the Sail sources at `tests/decoder/arch.sail` L59.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum csrop {
    CSRRW,
    CSRRS,
    CSRRC
}

/// Retired
/// 
/// Generated from the Sail sources at `tests/decoder/arch.sail` L60.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

/// ast
/// 
/// Generated from the Sail sources at `tests/decoder/arch.sail` L62.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    CSR((BitVector<12>, regidx, regidx, bool, csrop)),
    MRET(()),
    SRET(()),
    WFI(()),
    SFENCE_VMA((BitVector<5>, BitVector<5>)),
    HFENCE_VVMA((BitVector<5>, BitVector<5>)),
    HFENCE_GVMA((BitVector<5>, BitVector<5>))
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
        v__35 if {let mapping1_hashtag__var_1: BitVector<2> = v__35.subrange::<12, 14, 2>();
        let mapping0_hashtag__var_2: BitVector<1> = v__35.subrange::<14, 15, 1>();
        ((bool_bits_backwards_matches(mapping0_hashtag__var_2) && encdec_csrop_backwards_matches(mapping1_hashtag__var_1)) && (v__35.subrange::<0, 7, 7>() == BitVector::<7>::new(0b1110011)))} => {let csr: BitVector<12> = v__35.subrange::<20, 32, 12>();
        let rs1: BitVector<5> = v__35.subrange::<15, 20, 5>();
        let rd: BitVector<5> = v__35.subrange::<7, 12, 5>();
        let mapping1_hashtag_: BitVector<2> = v__35.subrange::<12, 14, 2>();
        let mapping0_hashtag_: BitVector<1> = v__35.subrange::<14, 15, 1>();
        let csr: BitVector<12> = v__35.subrange::<20, 32, 12>();
        match (bool_bits_backwards(mapping0_hashtag_), encdec_csrop_backwards(mapping1_hashtag_)) {
            (is_imm, op) => {Some(ast::CSR((csr, rs1, rd, is_imm, op)))}
            _ => {None}
            _ => {panic!("Unreachable code")}
        }}
        _ => {None}
        _ => {panic!("Unreachable code")}
    } {
        Some(result) => {result}
        None => {match head_exp_hashtag_ {
            v__0 if {(v__0 == BitVector::<32>::new(0b00110000001000000000000001110011))} => {ast::MRET(())}
            v__7 if {(v__7 == BitVector::<32>::new(0b00010000001000000000000001110011))} => {ast::SRET(())}
            v__14 if {(v__14 == BitVector::<32>::new(0b00010000010100000000000001110011))} => {ast::WFI(())}
            v__20 if {((v__20.subrange::<25, 32, 7>() == BitVector::<7>::new(0b0001001)) && (v__20.subrange::<0, 15, 15>() == BitVector::<15>::new(0b000000001110011)))} => {let rs2: BitVector<5> = v__20.subrange::<20, 25, 5>();
            let rs1: BitVector<5> = v__20.subrange::<15, 20, 5>();
            ast::SFENCE_VMA((rs1, rs2))}
            v__25 if {((v__25.subrange::<25, 32, 7>() == BitVector::<7>::new(0b0010001)) && (v__25.subrange::<0, 15, 15>() == BitVector::<15>::new(0b000000001110011)))} => {let rs2: BitVector<5> = v__25.subrange::<20, 25, 5>();
            let rs1: BitVector<5> = v__25.subrange::<15, 20, 5>();
            ast::HFENCE_VVMA((rs1, rs2))}
            v__30 if {((v__30.subrange::<25, 32, 7>() == BitVector::<7>::new(0b0110001)) && (v__30.subrange::<0, 15, 15>() == BitVector::<15>::new(0b000000001110011)))} => {let rs2: BitVector<5> = v__30.subrange::<20, 25, 5>();
            let rs1: BitVector<5> = v__30.subrange::<15, 20, 5>();
            ast::HFENCE_GVMA((rs1, rs2))}
            _ => {panic!("Unreachable code")}
        }}
        _ => {panic!("Unreachable code")}
    }
}
