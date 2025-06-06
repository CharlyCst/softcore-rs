#![allow(warnings)]

use softcore_prelude::*;

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