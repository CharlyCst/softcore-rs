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
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {

}

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

pub type regbits = BitVector<5>;

/// ast
/// 
/// Generated from the Sail sources at `tests/hoisting/arch.sail` L19.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

/// execute_TEST
/// 
/// Generated from the Sail sources at `tests/hoisting/arch.sail` L24-34.
pub fn execute_TEST() {
    if {let foo_var_1 = BitVector::<3>::new(0b101);
    (BitVector::<3>::new(0b101) != foo_var_1)} {
        assert!(false, "Branch should not be taken")
    } else {
        ()
    };
    let a = BitVector::<3>::new(0b100);
    let a__quote = BitVector::<3>::new(0b101);
    assert!((a != a__quote), "Those variables should be different!")
}