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

pub fn sign_extend(n: i128, input: BitDynamic) -> BitDynamic {
    sail_sign_extend(input, n)
}

pub fn sub_vec(a: BitDynamic, b: BitDynamic) -> BitDynamic {
    assert!(a.len() == 64, "`sub_vec` only support 64 bits for now");
    assert!(
        a.len() == b.len(),
        "`sub_vec` argument are not of equal length"
    );

    // Because we assume 64 bits, we can do a wrapping sub using Rust's 64 bits integers
    BitDynamic::new(
        a.len(),
        (a.unsigned() as i64).wrapping_sub(b.unsigned() as i64) as u64,
    )
}

pub fn shift_bits_left(value: BitDynamic, shift: BitDynamic) -> BitDynamic {
    value.shl(shift.unsigned() as u128)
}

pub fn shift_bits_right(value: BitDynamic, shift: BitDynamic) -> BitDynamic {
    value.shr(shift.unsigned() as u128)
}

pub fn shift_right_arith(value: BitDynamic, shift: i128) -> BitDynamic {
    assert!(value.len() <= 64, "Maximum supported size is 64 for now");

    // If shift is 0 or negative, return original value
    if shift <= 0 {
        return value;
    }

    // Check if the sign bit (MSB) is set
    let sign_bit = (value.unsigned() >> (value.len() - 1)) & 1;

    // If shift is >= N, all bits become the sign bit
    if shift >= value.len() {
        if sign_bit == 0 {
            return BitDynamic::new(value.len(), 0);
        } else {
            // All bits should be 1
            return BitDynamic::new(
                value.len(),
                if value.len() == 64 {
                    u64::MAX
                } else {
                    (1u64 << value.len()) - 1
                },
            );
        }
    }

    // Perform the right shift
    let shifted = value.unsigned() as u64 >> shift;

    if sign_bit == 0 {
        // Positive number - regular right shift (zero-fill)
        BitDynamic::new(value.len(), shifted)
    } else {
        // Negative number - need to fill upper bits with 1s
        let mask = if value.len() == 64 {
            u64::MAX << (64 - shift)
        } else {
            ((1u64 << shift) - 1) << (value.len() - shift)
        };
        BitDynamic::new(value.len(), shifted | mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift() {
        // Test shift by 0 - should return original value
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b10110111), 0).unsigned(),
            0b10110111
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b01110111), 0).unsigned(),
            0b01110111
        );

        // Test positive number (MSB = 0) - should zero-fill
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b01110111), 1).unsigned(),
            0b00111011
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b01110111), 2).unsigned(),
            0b00011101
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b01110111), 3).unsigned(),
            0b00001110
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b01111111), 4).unsigned(),
            0b00000111
        );

        // Test negative number (MSB = 1) - should sign-extend with 1s
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b10110111), 1).unsigned(),
            0b11011011
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b10110111), 2).unsigned(),
            0b11101101
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b10110111), 3).unsigned(),
            0b11110110
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b11111111), 4).unsigned(),
            0b11111111
        );

        // Test shift >= N for positive number - should become all zeros
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b01110111), 8).unsigned(),
            0b00000000
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b01110111), 10).unsigned(),
            0b00000000
        );

        // Test shift >= N for negative number - should become all ones
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b10110111), 8).unsigned(),
            0b11111111
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b10110111), 10).unsigned(),
            0b11111111
        );

        // Test with different bit widths - 16-bit
        assert_eq!(
            shift_right_arith(BitDynamic::new(16, 0b0111111111111111), 4).unsigned(),
            0b0000011111111111
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(16, 0b1111111111111111), 4).unsigned(),
            0b1111111111111111
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(16, 0b1000000000000000), 4).unsigned(),
            0b1111100000000000
        );

        // Test with 32-bit values
        assert_eq!(
            shift_right_arith(BitDynamic::new(32, 0x7FFFFFFF), 16).unsigned(),
            0x00007FFF
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(32, 0x80000000), 16).unsigned(),
            0xFFFF8000
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(32, 0xFFFFFFFF), 16).unsigned(),
            0xFFFFFFFF
        );

        // Test with 64-bit values (edge case for mask calculation)
        assert_eq!(
            shift_right_arith(BitDynamic::new(64, 0x7FFFFFFFFFFFFFFF), 32).unsigned(),
            0x000000007FFFFFFF
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(64, 0x8000000000000000), 32).unsigned(),
            0xFFFFFFFF80000000
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(64, 0xFFFFFFFFFFFFFFFF), 32).unsigned(),
            0xFFFFFFFFFFFFFFFF
        );

        // Test small bit widths
        assert_eq!(
            shift_right_arith(BitDynamic::new(4, 0b0111), 1).unsigned(),
            0b0011
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(4, 0b1111), 1).unsigned(),
            0b1111
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(4, 0b1000), 2).unsigned(),
            0b1110
        );

        // Test negative shift - should return original value
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b10110111), -1).unsigned(),
            0b10110111
        );
        assert_eq!(
            shift_right_arith(BitDynamic::new(8, 0b01110111), -5).unsigned(),
            0b01110111
        );
    }
}
