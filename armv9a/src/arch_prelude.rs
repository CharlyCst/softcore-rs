//! Armv9a Prelude
//!
//! This module holds prelude functions that are only required for the Armv9a architecture

#![allow(non_snake_case)]

use crate::prelude::*;

pub fn UInt0<const N: i128>(bits: BitVector<N>) -> i128 {
    bits.unsigned()
}

pub fn SInt0<const N: i128>(bits: BitVector<N>) -> i128 {
    bits.signed()
}

pub fn ZeroExtend0<const N: i128, const M: i128>(bits: BitVector<N>, n: i128) -> BitVector<M> {
    assert_eq!(n, M, "ZeroExtend0 with the wring bit width");
    bits.zero_extend()
}

pub const fn integer_subrange<const L: i128>(i: i128, hi: i128, lo: i128) -> BitVector<L> {
    assert!(
        (hi - lo) + 1 == L,
        "Dimension mismatch in 'integer_subrange', this is a compiler bug"
    );
    get_slice_int((hi - lo) + 1, i, lo)
}

/// Returns an undefined bitvector of the given size.
///
/// For now we return 0, but we might want to track undefined values in the future.
pub const fn undefined_bitvector<const N: i128>(n: i128) -> BitVector<N> {
    assert!(n == N, "Compiler bug");
    bv(0)
}
