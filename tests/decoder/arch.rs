#![allow(warnings)]

use softcore_prelude::*;

use crate::arch_prelude::*;

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

/// Initialize all registers.
///
/// This function should be called before using a fresh core, otherwise the core might not be in a valid state.
pub fn _reset_all_registers() {
    
}

/// xlen
///
/// Generated from the Sail sources at `tests/decoder/arch.sail` L28.
pub const xlen: i128 = 64;

/// xlen_bytes
///
/// Generated from the Sail sources at `tests/decoder/arch.sail` L29.
pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitStatic::<xlen>;

pub type priv_level = BitStatic::<2>;

/// Privilege
///
/// Generated from the Sail sources at `tests/decoder/arch.sail` L34.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

pub type regidx = BitStatic::<5>;

pub type cregidx = BitStatic::<3>;

pub type csreg = BitStatic::<12>;

/// bool_bits_forwards
///
/// Generated from the Sail sources.
pub fn bool_bits_forwards(arg_hashtag_: bool) -> BitStatic::<1> {
    match arg_hashtag_ {
        true => {BitDynamic::new(1, 0b1)}
        false => {BitDynamic::new(1, 0b0)}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_bits_backwards
///
/// Generated from the Sail sources.
pub fn bool_bits_backwards(arg_hashtag_: BitStatic::<1>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(1, 0b1))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_bits_backwards_matches
///
/// Generated from the Sail sources.
pub fn bool_bits_backwards_matches(arg_hashtag_: BitStatic::<1>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(1, 0b1))} => {true}
        b__1 if {(b__1 == BitDynamic::new(1, 0b0))} => {true}
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
    CSR((BitStatic::<12>, regidx, regidx, bool, csrop)),
    MRET(()),
    SRET(()),
    WFI(()),
    SFENCE_VMA((BitStatic::<5>, BitStatic::<5>)),
    HFENCE_VVMA((BitStatic::<5>, BitStatic::<5>)),
    HFENCE_GVMA((BitStatic::<5>, BitStatic::<5>))
}

/// encdec_csrop_forwards
///
/// Generated from the Sail sources.
pub fn encdec_csrop_forwards(arg_hashtag_: csrop) -> BitStatic::<2> {
    match arg_hashtag_ {
        csrop::CSRRW => {BitDynamic::new(2, 0b01)}
        csrop::CSRRS => {BitDynamic::new(2, 0b10)}
        csrop::CSRRC => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_csrop_backwards
///
/// Generated from the Sail sources.
pub fn encdec_csrop_backwards(arg_hashtag_: BitStatic::<2>) -> csrop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b01))} => {csrop::CSRRW}
        b__1 if {(b__1 == BitDynamic::new(2, 0b10))} => {csrop::CSRRS}
        b__2 if {(b__2 == BitDynamic::new(2, 0b11))} => {csrop::CSRRC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_csrop_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_csrop_backwards_matches(arg_hashtag_: BitStatic::<2>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b01))} => {true}
        b__1 if {(b__1 == BitDynamic::new(2, 0b10))} => {true}
        b__2 if {(b__2 == BitDynamic::new(2, 0b11))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

pub type csrRW = BitStatic::<2>;

/// encdec_forwards
///
/// Generated from the Sail sources.
pub fn encdec_forwards(arg_hashtag_: ast) -> BitStatic::<32> {
    match arg_hashtag_ {
        ast::CSR((csr, rs1, rd, is_imm, op)) => {bitvector_concat(BitDynamic::from((csr as BitStatic::<12>)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs1 as BitStatic::<5>)), BitDynamic::from(bitvector_concat(BitDynamic::from(bool_bits_forwards(is_imm)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_csrop_forwards(op)), BitDynamic::from(bitvector_concat(BitDynamic::from((rd as BitStatic::<5>)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::MRET(()) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0011000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00010)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::SRET(()) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0001000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00010)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::WFI(()) => {bitvector_concat(BitDynamic::from(BitDynamic::new(12, 0b000100000101)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))}
        ast::SFENCE_VMA((rs1, rs2)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0001001)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs2 as BitStatic::<5>)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs1 as BitStatic::<5>)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::HFENCE_VVMA((rs1, rs2)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010001)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs2 as BitStatic::<5>)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs1 as BitStatic::<5>)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::HFENCE_GVMA((rs1, rs2)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0110001)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs2 as BitStatic::<5>)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs1 as BitStatic::<5>)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_backwards
///
/// Generated from the Sail sources.
pub fn encdec_backwards(arg_hashtag_: BitStatic::<32>) -> ast {
    let head_exp_hashtag_: BitStatic::<32> = arg_hashtag_;
    match match head_exp_hashtag_ {
        v__35 if {let mapping1_hashtag__var_1: BitStatic::<2> = v__35.subrange::<12, 14, 2>();
        let mapping0_hashtag__var_2: BitStatic::<1> = v__35.subrange::<14, 15, 1>();
        ((bool_bits_backwards_matches(mapping0_hashtag__var_2) && encdec_csrop_backwards_matches(mapping1_hashtag__var_1)) && (v__35.subrange::<0, 7, 7>() == BitDynamic::new(7, 0b1110011)))} => {let csr: BitStatic::<12> = v__35.subrange::<20, 32, 12>();
        let rs1: BitStatic::<5> = v__35.subrange::<15, 20, 5>();
        let rd: BitStatic::<5> = v__35.subrange::<7, 12, 5>();
        let mapping1_hashtag_: BitStatic::<2> = v__35.subrange::<12, 14, 2>();
        let mapping0_hashtag_: BitStatic::<1> = v__35.subrange::<14, 15, 1>();
        let csr: BitStatic::<12> = v__35.subrange::<20, 32, 12>();
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
            v__0 if {(v__0 == BitDynamic::new(32, 0b00110000001000000000000001110011))} => {ast::MRET(())}
            v__7 if {(v__7 == BitDynamic::new(32, 0b00010000001000000000000001110011))} => {ast::SRET(())}
            v__14 if {(v__14 == BitDynamic::new(32, 0b00010000010100000000000001110011))} => {ast::WFI(())}
            v__20 if {((v__20.subrange::<25, 32, 7>() == BitDynamic::new(7, 0b0001001)) && (v__20.subrange::<0, 15, 15>() == BitDynamic::new(15, 0b000000001110011)))} => {let rs2: BitStatic::<5> = v__20.subrange::<20, 25, 5>();
            let rs1: BitStatic::<5> = v__20.subrange::<15, 20, 5>();
            ast::SFENCE_VMA((rs1, rs2))}
            v__25 if {((v__25.subrange::<25, 32, 7>() == BitDynamic::new(7, 0b0010001)) && (v__25.subrange::<0, 15, 15>() == BitDynamic::new(15, 0b000000001110011)))} => {let rs2: BitStatic::<5> = v__25.subrange::<20, 25, 5>();
            let rs1: BitStatic::<5> = v__25.subrange::<15, 20, 5>();
            ast::HFENCE_VVMA((rs1, rs2))}
            v__30 if {((v__30.subrange::<25, 32, 7>() == BitDynamic::new(7, 0b0110001)) && (v__30.subrange::<0, 15, 15>() == BitDynamic::new(15, 0b000000001110011)))} => {let rs2: BitStatic::<5> = v__30.subrange::<20, 25, 5>();
            let rs1: BitStatic::<5> = v__30.subrange::<15, 20, 5>();
            ast::HFENCE_GVMA((rs1, rs2))}
            _ => {panic!("Unreachable code")}
        }}
        _ => {panic!("Unreachable code")}
    }
}
