#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub PC: xlenbits,
    pub config: SailConfig,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfig {

}

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

pub type regbits = BitVector<5>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

pub fn execute(sail_ctx: &mut SailVirtCtx, TodoArgsApp: ast) {
    if {let foo_var_1 = BitVector::<3>::new(0b101);
    (BitVector::<3>::new(0b101) != foo_var_1)} {
        assert!(false, "Process message")
    } else {
        ()
    }
}