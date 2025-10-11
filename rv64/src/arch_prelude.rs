//! RV64 prelude
//!
//! This module holds prelude functions that are only required for the RV64 model.

use softcore_prelude::*;

pub const fn quot_round_zero(n: i128, m: i128) -> i128 {
    n / m
}

pub const fn rem_round_zero(n: i128, m: i128) -> i128 {
    n % m
}

pub fn sub_vec<const N: i128>(a: BitVector<N>, b: BitVector<N>) -> BitVector<N> {
    assert!(N == 64, "`sub_vec` only support 64 bits for now");

    // Because we assume 64 bits, we can do a wrapping sub using Rust's 64 bits integers
    bv((a.bits() as i64).wrapping_sub(b.bits() as i64) as u64)
}

pub fn shift_bits_left<const N: i128, const M: i128>(
    value: BitVector<N>,
    shift: BitVector<M>,
) -> BitVector<N> {
    bv::<N>(value.bits() << shift.bits())
}

pub fn shift_bits_right<const N: i128, const M: i128>(
    value: BitVector<N>,
    shift: BitVector<M>,
) -> BitVector<N> {
    bv::<N>(value.bits() >> shift.bits())
}
