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

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitVector<xlen>;

pub const xlen: i128 = 64;

/// known_match_branch
/// 
/// Generated from the Sail sources at `tests/optimization/arch.sail` L17-22.
pub const fn known_match_branch(unit_arg: ()) -> BitVector<64> {
    sail_zeros::<64>(64)
}

/// ast
/// 
/// Generated from the Sail sources at `tests/optimization/arch.sail` L26.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

/// execute
/// 
/// Generated from the Sail sources at `tests/optimization/arch.sail` L30-32.
pub fn execute(ast::TEST(()): ast) {
    let a = known_match_branch(());
    ()
}
