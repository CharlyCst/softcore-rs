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
    pub PC: xlenbits,
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

pub const xlen: i128 = 64;

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitDynamic;

pub type regbits = BitDynamic;

/// ast
///
/// Generated from the Sail sources at `tests/hoisting/arch.sail` L19.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

/// execute
///
/// Generated from the Sail sources at `tests/hoisting/arch.sail` L24-34.
pub fn execute(ast::TEST(()): ast) {
    if {let foo_var_1: BitDynamic = BitDynamic::new(3, 0b101);
    (BitDynamic::new(3, 0b101) != foo_var_1)} {
        assert!(false, "Branch should not be taken")
    } else {
        ()
    };
    let a: BitDynamic = BitDynamic::new(3, 0b100);
    let a__quote: BitDynamic = BitDynamic::new(3, 0b101);
    assert!((a != a__quote), "Those variables should be different!")
}
