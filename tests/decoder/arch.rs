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

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

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

pub type csrRW = BitVector<2>;