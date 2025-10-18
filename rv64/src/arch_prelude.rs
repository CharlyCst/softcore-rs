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

pub fn sign_extend<const M: i128, const N: i128>( n: i128, input: BitVector<M>) -> BitVector<N> {
    sail_sign_extend(input, n)
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

pub fn shift_right_arith<const N: i128>(value: BitVector<N>, shift: i128) -> BitVector<N> {
    assert!(N <= 64, "Maximum supported size is 64 for now");

    // If shift is 0 or negative, return original value
    if shift <= 0 {
        return value;
    }

    // Check if the sign bit (MSB) is set
    let sign_bit = (value.bits() >> (N - 1)) & 1;

    // If shift is >= N, all bits become the sign bit
    if shift >= N {
        if sign_bit == 0 {
            return bv::<N>(0);
        } else {
            // All bits should be 1
            return bv::<N>(if N == 64 { u64::MAX } else { (1u64 << N) - 1 });
        }
    }

    // Perform the right shift
    let shifted = value.bits() >> shift;

    if sign_bit == 0 {
        // Positive number - regular right shift (zero-fill)
        bv::<N>(shifted)
    } else {
        // Negative number - need to fill upper bits with 1s
        let mask = if N == 64 {
            u64::MAX << (64 - shift)
        } else {
            ((1u64 << shift) - 1) << (N - shift)
        };
        bv::<N>(shifted | mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift() {
        // Test shift by 0 - should return original value
        assert_eq!(shift_right_arith(bv::<8>(0b10110111), 0).bits(), 0b10110111);
        assert_eq!(shift_right_arith(bv::<8>(0b01110111), 0).bits(), 0b01110111);

        // Test positive number (MSB = 0) - should zero-fill
        assert_eq!(shift_right_arith(bv::<8>(0b01110111), 1).bits(), 0b00111011);
        assert_eq!(shift_right_arith(bv::<8>(0b01110111), 2).bits(), 0b00011101);
        assert_eq!(shift_right_arith(bv::<8>(0b01110111), 3).bits(), 0b00001110);
        assert_eq!(shift_right_arith(bv::<8>(0b01111111), 4).bits(), 0b00000111);

        // Test negative number (MSB = 1) - should sign-extend with 1s
        assert_eq!(shift_right_arith(bv::<8>(0b10110111), 1).bits(), 0b11011011);
        assert_eq!(shift_right_arith(bv::<8>(0b10110111), 2).bits(), 0b11101101);
        assert_eq!(shift_right_arith(bv::<8>(0b10110111), 3).bits(), 0b11110110);
        assert_eq!(shift_right_arith(bv::<8>(0b11111111), 4).bits(), 0b11111111);

        // Test shift >= N for positive number - should become all zeros
        assert_eq!(shift_right_arith(bv::<8>(0b01110111), 8).bits(), 0b00000000);
        assert_eq!(shift_right_arith(bv::<8>(0b01110111), 10).bits(), 0b00000000);

        // Test shift >= N for negative number - should become all ones
        assert_eq!(shift_right_arith(bv::<8>(0b10110111), 8).bits(), 0b11111111);
        assert_eq!(shift_right_arith(bv::<8>(0b10110111), 10).bits(), 0b11111111);

        // Test with different bit widths - 16-bit
        assert_eq!(
            shift_right_arith(bv::<16>(0b0111111111111111), 4).bits(),
            0b0000011111111111
        );
        assert_eq!(
            shift_right_arith(bv::<16>(0b1111111111111111), 4).bits(),
            0b1111111111111111
        );
        assert_eq!(
            shift_right_arith(bv::<16>(0b1000000000000000), 4).bits(),
            0b1111100000000000
        );

        // Test with 32-bit values
        assert_eq!(shift_right_arith(bv::<32>(0x7FFFFFFF), 16).bits(), 0x00007FFF);
        assert_eq!(shift_right_arith(bv::<32>(0x80000000), 16).bits(), 0xFFFF8000);
        assert_eq!(shift_right_arith(bv::<32>(0xFFFFFFFF), 16).bits(), 0xFFFFFFFF);

        // Test with 64-bit values (edge case for mask calculation)
        assert_eq!(
            shift_right_arith(bv::<64>(0x7FFFFFFFFFFFFFFF), 32).bits(),
            0x000000007FFFFFFF
        );
        assert_eq!(
            shift_right_arith(bv::<64>(0x8000000000000000), 32).bits(),
            0xFFFFFFFF80000000
        );
        assert_eq!(
            shift_right_arith(bv::<64>(0xFFFFFFFFFFFFFFFF), 32).bits(),
            0xFFFFFFFFFFFFFFFF
        );

        // Test small bit widths
        assert_eq!(shift_right_arith(bv::<4>(0b0111), 1).bits(), 0b0011);
        assert_eq!(shift_right_arith(bv::<4>(0b1111), 1).bits(), 0b1111);
        assert_eq!(shift_right_arith(bv::<4>(0b1000), 2).bits(), 0b1110);

        // Test negative shift - should return original value
        assert_eq!(shift_right_arith(bv::<8>(0b10110111), -1).bits(), 0b10110111);
        assert_eq!(shift_right_arith(bv::<8>(0b01110111), -5).bits(), 0b01110111);
    }
}
