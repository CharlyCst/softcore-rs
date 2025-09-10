#![allow(incomplete_features, non_camel_case_types)]

use core::ops;
use std::cmp::min;

// NOTE: Ideally we would use unbounded integers for natural numbers. Yet in practice this would
// mess up with things such as the SMT solver during symbolic execution.
// After manual inspection, u128 are big enough for all the RISC-V use cases, so we keep that until
// a better solution is needed.
pub type nat = u128;

pub fn sail_branch_announce(_value: i128, _pc: BitVector<64>) {}

pub fn lteq_int(e1: i128, e2: i128) -> bool {
    e1 <= e2
}

pub fn gt_int(e1: i128, e2: i128) -> bool {
    e1 > e2
}

pub fn bitvector_length<const N: i128>(_e: BitVector<N>) -> i128 {
    N
}

pub fn parse_hex_bits<const N: i128>(_n: i128, _hex_str: &str) -> BitVector<N> {
    todo!("'parse_hex_bits' is not yet implemented");
}

pub fn bitvector_concat<const N: i128, const M: i128, const NM: i128>(
    e1: BitVector<N>,
    e2: BitVector<M>,
) -> BitVector<{ NM }> {
    bv::<{ NM }>((e1.bits() << M) | e2.bits())
}

pub const fn get_slice_int<const L: i128>(l: i128, n: i128, start: i128) -> BitVector<L> {
    let val = (n >> start) & (mask(l as usize) as i128);
    bv(val as u64)
}

pub fn get_16_random_bits(_unit: ()) -> BitVector<16> {
    bv::<16>(0)
}

pub fn not_implemented<T>(_any: T) -> ! {
    panic!("Feature not implemented yet");
}

pub fn internal_error(_file: String, _line: i128, _s: String) -> ! {
    panic!("Softcore: internal error")
}

pub fn print_output<const N: i128>(text: String, _csr: BitVector<N>) {
    println!("{}", text)
}

pub fn print_platform(text: String) {
    println!("{}", text)
}

pub fn bits_str<const N: i128>(val: BitVector<N>) -> String {
    format!("{:b}", val.bits())
}

pub fn bitvector_access<const N: i128>(vec: BitVector<N>, idx: i128) -> bool {
    (vec.bits() & (1 << idx)) > 0
}

// Todo: implement truncate for other sizes if required
pub fn truncate(v: BitVector<64>, size: i128) -> BitVector<64> {
    assert!(size == 64);
    v
}

pub fn sign_extend<const M: i128, const N: i128>(
    value: i128,
    input: BitVector<M>,
) -> BitVector<N> {
    assert!(value == N, "Mismatch `sign_extend` size");
    assert!(N >= M, "Cannot sign extend to smaller size");
    assert!(N <= 64, "Maximum supported size is 64 for now");
    
    // Special case: when extending from same size to same size, it's a no-op
    if M == N {
        return bv::<N>(input.bits());
    }
    
    // Check if the sign bit (MSB) is set
    let sign_bit = (input.bits() >> (M - 1)) & 1;
    
    if sign_bit == 0 {
        // Positive number - just zero extend
        bv::<N>(input.bits())
    } else {
        // Negative number - fill upper bits with 1s
        // Handle the case where M=64 to avoid shift overflow
        let mask = if M == 64 { 0u64 } else { (1u64 << M) - 1 };
        let extension_bits = !mask & if N == 64 { u64::MAX } else { (1u64 << N) - 1 };
        bv::<N>(input.bits() | extension_bits)
    }
}

pub const fn sail_ones<const N: i128>(n: i128) -> BitVector<N> {
    assert!(n <= 64);
    bv::<N>(mask(n as usize))
}

pub const fn sail_zeros<const N: i128>(_n: i128) -> BitVector<N> {
    bv::<N>(0)
}

pub const fn sail_shiftright<const N: i128>(bits: BitVector<N>, shift: i128) -> BitVector<N> {
    bv(bits.bits() >> (shift as u64))
}

pub const fn sail_shiftleft<const N: i128>(bits: BitVector<N>, shift: i128) -> BitVector<N> {
    bv(bits.bits() << (shift as u64))
}

pub fn min_int(v1: i128, v2: i128) -> i128 {
    min(v1, v2)
}

pub fn cancel_reservation(_unit: ()) {
    // In the future, extend this function
}

pub fn hex_bits_12_forwards(_reg: BitVector<12>) -> ! {
    todo!("Implement this function")
}

pub fn hex_bits_12_backwards(_: &'static str) -> BitVector<12> {
    todo!("Implement this function")
}

pub fn subrange_bits<const IN: i128, const OUT: i128>(
    vec: BitVector<IN>,
    end: i128,
    start: i128,
) -> BitVector<OUT> {
    assert_eq!((end - start + 1), OUT);
    assert!(OUT <= IN);

    bv((vec.bits >> start) & mask(OUT as usize))
}

pub fn update_subrange_bits<const N: i128, const M: i128>(
    bits: BitVector<N>,
    to: u64,
    from: u64,
    value: BitVector<M>,
) -> BitVector<N> {
    assert!(to - from + 1 == M as u64, "size don't match");

    // Generate the 111111 mask
    let mut mask = (1 << M) - 1;
    // Shit and invert it
    mask = !(mask << from);

    // Now we can update and return the updated value
    bv((bits.bits & mask) | (value.bits() << from))
}

pub fn bitvector_update<const N: i128>(v: BitVector<N>, pos: i128, value: bool) -> BitVector<N> {
    let mask = 1 << pos;
    bv((v.bits() & !mask) | (value as u64) << pos)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitVector<const N: i128> {
    bits: u64,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitField<const T: i128> {
    pub bits: BitVector<T>,
}

impl<const N: i128> BitField<N> {
    pub const fn new(value: u64) -> Self {
        BitField { bits: bv(value) }
    }

    pub const fn subrange<const A: i128, const B: i128, const C: i128>(self) -> BitVector<C> {
        assert!(B - A == C, "Invalid subrange parameters");
        assert!(B <= N, "Invalid subrange");

        self.bits.subrange::<A, B, C>()
    }

    pub const fn set_subrange<const A: i128, const B: i128, const C: i128>(
        self,
        bitvector: BitVector<C>,
    ) -> Self {
        assert!(B - A == C, "Invalid subrange parameters");
        assert!(A <= B && B <= N, "Invalid subrange");

        BitField::<N> {
            bits: self.bits.set_subrange::<A, B, C>(bitvector),
        }
    }
}

impl<const N: i128> PartialOrd for BitVector<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

/// Create a fresh [BitVector].
///
/// This is equivalent to [BitVector::new], with a shorted syntax.
pub const fn bv<const N: i128>(val: u64) -> BitVector<N> {
    BitVector::new(val)
}

impl<const N: i128> BitVector<N> {
    pub const fn new(val: u64) -> Self {
        if N < 64 {
            Self {
                bits: val & ((1 << N) - 1),
            }
        } else {
            Self { bits: val }
        }
    }

    pub const fn new_empty() -> Self {
        Self { bits: 0 }
    }

    pub const fn bits(self) -> u64 {
        self.bits
    }

    /// Get the bits as an integer.
    ///
    /// The bitvector is interpreted as unsigned.
    pub const fn unsigned(self) -> i128 {
        // Note that bits is unsigned, so converting to a bigger i128 guarantees the result is
        // still positive.
        self.bits as i128
    }

    /// Get the bits as an integer.
    ///
    /// The bitvector is interpreted as signed
    pub const fn signed(self) -> i128 {
        let value = self.bits as u128;
        let sign_bit_mask = 1 << (N - 1);
        if value & sign_bit_mask == 0 {
            // The number is positive, nothing to do
            value as i128
        } else {
            // The number is negative, need to fill upper bits with 1s
            let fill_mask = !((1 << N) - 1);
            (value | fill_mask) as i128
        }
    }

    pub const fn as_usize(self) -> usize {
        self.bits as usize
    }

    pub const fn as_i128(self) -> i128 {
        self.bits as i128
    }

    pub const fn zero_extend<const M: i128>(self) -> BitVector<M> {
        assert!(M >= N, "Can not zero-extend to a smaller size!");
        assert!(M <= 64, "Maximum zero-extend supported size if 64");

        // Here we have nothing to do, we already use 64 bits with zeroes for MSBs
        BitVector { bits: self.bits }
    }

    pub fn set_bit(self, idx: i128, value: bool) -> Self {
        assert!(idx < N, "Out of bounds array check");
        let new_value = if value {
            self.bits | 1u64 << idx
        } else {
            self.bits & !(1u64 << idx)
        };
        BitVector { bits: new_value }
    }

    pub const fn subrange<const A: i128, const B: i128, const C: i128>(self) -> BitVector<C> {
        assert!(B - A == C, "Invalid subrange parameters");
        assert!(B <= N, "Invalid subrange");

        let mut val = self.bits; // The current value
        val &= BitVector::<B>::bit_mask(); // Remove top bits
        val >>= A; // Shift all the bits
        bv(val)
    }

    pub const fn set_subrange<const A: i128, const B: i128, const C: i128>(
        self,
        bits: BitVector<C>,
    ) -> Self {
        assert!(B - A == C, "Invalid set_subrange parameters");
        assert!(B <= N, "Invalid subrange");

        let mask = !(BitVector::<C>::bit_mask() << A);
        let new_bits = bits.bits() << A;
        bv((self.bits & mask) | new_bits)
    }

    pub const fn wrapped_add(self, other: BitVector<N>) -> BitVector<N> {
        bv::<N>(self.bits.wrapping_add(other.bits))
    }

    /// Returns a bit mask with 1 for the first [N] bits.
    const fn bit_mask() -> u64 {
        assert!(N <= 64);

        if N == 64 { u64::MAX } else { (1 << N) - 1 }
    }
}

impl<const N: i128> ops::BitAnd for BitVector<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl<const N: i128> ops::BitOr for BitVector<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl<const N: i128> ops::BitXor for BitVector<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl<const N: i128> ops::Shl<usize> for BitVector<N> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self {
            bits: self.bits << rhs,
        }
    }
}

impl<const N: i128> ops::Shl<i128> for BitVector<N> {
    type Output = Self;

    fn shl(self, rhs: i128) -> Self::Output {
        Self {
            bits: self.bits << rhs,
        }
    }
}

impl<const N: i128> ops::Shl<i32> for BitVector<N> {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        Self {
            bits: self.bits << rhs,
        }
    }
}

impl<const N: i128> ops::Shr<usize> for BitVector<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self {
            bits: self.bits >> rhs,
        }
    }
}

impl<const N: i128> ops::Shr<i128> for BitVector<N> {
    type Output = Self;

    fn shr(self, rhs: i128) -> Self::Output {
        Self {
            bits: self.bits >> rhs,
        }
    }
}

impl<const N: i128> ops::Shr<i32> for BitVector<N> {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        Self {
            bits: self.bits >> rhs,
        }
    }
}

impl<const N: i128> ops::Not for BitVector<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        bv((!self.bits) & Self::bit_mask())
    }
}

impl<const N: i128> std::ops::Add<i64> for BitVector<N> {
    type Output = Self;

    fn add(self, rhs: i64) -> BitVector<N> {
        let result = self.bits as i64 + rhs;
        // If the result is out of bounds, we may want to handle overflow
        bv::<N>(result as u64) // Returning the result as BitVector
    }
}

// ———————————————————————————————— Helpers ————————————————————————————————— //

const fn mask(nb_ones: usize) -> u64 {
    if nb_ones == 64 {
        u64::MAX
    } else {
        (1 << nb_ones) - 1
    }
}

// ————————————————————————————————— Tests —————————————————————————————————— //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitvec_masks() {
        assert_eq!(BitVector::<0>::bit_mask(), 0b0);
        assert_eq!(BitVector::<1>::bit_mask(), 0b1);
        assert_eq!(BitVector::<2>::bit_mask(), 0b11);
        assert_eq!(BitVector::<8>::bit_mask(), 0b11111111);
        assert_eq!(BitVector::<64>::bit_mask(), 0xffffffffffffffff);
    }

    #[test]
    fn bitvec_not() {
        assert_eq!((!bv::<1>(0b1)).bits(), 0b0);
        assert_eq!((!bv::<1>(0b0)).bits(), 0b1);
        assert_eq!((!bv::<2>(0b01)).bits(), 0b10);
        assert_eq!((!bv::<2>(0b11)).bits(), 0b00);
    }

    #[test]
    fn subrange_bitvector() {
        let v = bv::<32>(0b10110111);

        assert_eq!(v.subrange::<0, 1, 1>().bits(), 0b1);
        assert_eq!(v.subrange::<0, 2, 2>().bits(), 0b11);
        assert_eq!(v.subrange::<0, 3, 3>().bits(), 0b111);
        assert_eq!(v.subrange::<0, 4, 4>().bits(), 0b0111);
        assert_eq!(v.subrange::<0, 5, 5>().bits(), 0b10111);

        assert_eq!(v.subrange::<2, 3, 1>().bits(), 0b1);
        assert_eq!(v.subrange::<2, 4, 2>().bits(), 0b01);
        assert_eq!(v.subrange::<2, 5, 3>().bits(), 0b101);
        assert_eq!(v.subrange::<2, 6, 4>().bits(), 0b1101);
        assert_eq!(v.subrange::<2, 7, 5>().bits(), 0b01101);

        assert_eq!(bv::<32>(0xffffffff).subrange::<7, 23, 16>().bits(), 0xffff);
        assert_eq!(v.subrange::<2, 7, 5>().bits(), 0b01101);

        let v = bv::<32>(0b10110111);
        assert_eq!(v.set_subrange::<0, 1, 1>(bv(0b0)).bits(), 0b10110110);
        assert_eq!(v.set_subrange::<0, 1, 1>(bv(0b1)).bits(), 0b10110111);
        assert_eq!(v.set_subrange::<0, 2, 2>(bv(0b00)).bits(), 0b10110100);
        assert_eq!(v.set_subrange::<2, 5, 3>(bv(0b010)).bits(), 0b10101011);

        assert_eq!(
            bv::<64>(0x0000000000000000).subrange::<60, 64, 4>().bits(),
            0x0
        );
        assert_eq!(
            bv::<64>(0xa000000000000000).subrange::<60, 64, 4>().bits(),
            0xa
        );
        assert_eq!(
            bv::<64>(0xb000000000000000).subrange::<60, 64, 4>().bits(),
            0xb
        );
        assert_eq!(
            bv::<64>(0xc000000000000000).subrange::<60, 64, 4>().bits(),
            0xc
        );
        assert_eq!(
            bv::<64>(0xd000000000000000).subrange::<60, 64, 4>().bits(),
            0xd
        );
        assert_eq!(
            bv::<64>(0xe000000000000000).subrange::<60, 64, 4>().bits(),
            0xe
        );
        assert_eq!(
            bv::<64>(0xf000000000000000).subrange::<60, 64, 4>().bits(),
            0xf
        );
    }

    // TODO: In the future squash with the previous function
    #[test]
    fn subrange_bitfield() {
        let bitfield = BitField::<32>::new(0b10110111);

        assert_eq!(bitfield.subrange::<0, 1, 1>().bits(), 0b1);
        assert_eq!(bitfield.subrange::<0, 2, 2>().bits(), 0b11);
        assert_eq!(bitfield.subrange::<0, 3, 3>().bits(), 0b111);
        assert_eq!(bitfield.subrange::<0, 4, 4>().bits(), 0b0111);
        assert_eq!(bitfield.subrange::<0, 5, 5>().bits(), 0b10111);

        assert_eq!(bitfield.subrange::<2, 3, 1>().bits(), 0b1);
        assert_eq!(bitfield.subrange::<2, 4, 2>().bits(), 0b01);
        assert_eq!(bitfield.subrange::<2, 5, 3>().bits(), 0b101);
        assert_eq!(bitfield.subrange::<2, 6, 4>().bits(), 0b1101);
        assert_eq!(bitfield.subrange::<2, 7, 5>().bits(), 0b01101);

        let v = bv::<32>(0b10110111);
        assert_eq!(v.set_subrange::<0, 1, 1>(bv(0b0)).bits(), 0b10110110);
        assert_eq!(v.set_subrange::<0, 1, 1>(bv(0b1)).bits(), 0b10110111);
        assert_eq!(v.set_subrange::<0, 2, 2>(bv(0b00)).bits(), 0b10110100);
        assert_eq!(v.set_subrange::<2, 5, 3>(bv(0b010)).bits(), 0b10101011);
    }

    #[test]
    fn test_update_subrange_bits() {
        assert_eq!(
            update_subrange_bits(bv::<8>(0b11111100), 1, 0, bv::<2>(0b11)).bits,
            0b11111111
        );
        assert_eq!(
            update_subrange_bits(bv::<8>(0b00000000), 0, 0, bv::<1>(0b1)).bits,
            0b00000001
        );
        assert_eq!(
            update_subrange_bits(bv::<8>(0b00000000), 1, 1, bv::<1>(0b1)).bits,
            0b00000010
        );
        assert_eq!(
            update_subrange_bits(bv::<8>(0b00000000), 2, 2, bv::<1>(0b1)).bits,
            0b00000100
        );
        assert_eq!(
            update_subrange_bits(bv::<8>(0b00000000), 3, 3, bv::<1>(0b1)).bits,
            0b00001000
        );
        assert_eq!(
            update_subrange_bits(bv::<8>(0b00000000), 4, 4, bv::<1>(0b1)).bits,
            0b00010000
        );
        assert_eq!(
            update_subrange_bits(bv::<8>(0b00000000), 5, 5, bv::<1>(0b1)).bits,
            0b00100000
        );
        assert_eq!(
            update_subrange_bits(bv::<8>(0b00000000), 6, 6, bv::<1>(0b1)).bits,
            0b01000000
        );
        assert_eq!(
            update_subrange_bits(bv::<8>(0b00000000), 7, 7, bv::<1>(0b1)).bits,
            0b10000000
        );
    }

    #[test]
    fn bitwise_operators() {
        let v = bv::<32>(0b1);

        assert_eq!(v, v | v);
        assert_eq!(v, v & v);
        assert_eq!(v, v ^ v ^ v);
        assert_eq!(v, !!v);

        for i in 0..30 {
            assert_eq!(v, (v << (i as usize)) >> (i as usize));
        }

        // Test i32 shift operators
        for i in 0i32..30i32 {
            assert_eq!(v, (v << i) >> i);
        }
    }

    #[test]
    fn test_zero_extend() {
        let v = bv::<8>(0b1010);

        assert_eq!(v.bits, v.zero_extend::<16>().bits);
        assert_eq!(v.bits, v.zero_extend::<63>().bits);
        assert_eq!(v.bits, v.zero_extend::<64>().bits);
    }

    #[test]
    fn test_bitvector_concat() {
        const SIZE: i128 = 20;
        const NEW_SIZE: i128 = 40;

        for i in 0..(1 << (SIZE as usize)) {
            let v = bv::<SIZE>(i);
            assert_eq!(
                bitvector_concat::<SIZE, SIZE, NEW_SIZE>(v, v).bits,
                i + (i << (SIZE as usize))
            );
        }
    }

    #[test]
    fn test_bitvector_access() {
        const SIZE: i128 = 10;

        for i in 0..(1 << (SIZE as usize)) {
            let v = bv::<SIZE>(i);
            for idx in 0..(SIZE as usize) {
                assert_eq!((i & (1 << idx)) > 0, bitvector_access(v, idx as i128))
            }
        }
    }

    #[test]
    fn test_set_bit() {
        const SIZE: i128 = 60;

        let mut v = bv::<SIZE>(0);
        let mut val: u64 = 0;
        for idx in 0..(SIZE as usize) {
            val |= 1u64 << idx;
            v = v.set_bit(idx as i128, true);

            assert_eq!(v.bits, val);
        }

        for i in 0..(SIZE as usize) {
            v = v.set_bit(i as i128, false);
        }

        assert_eq!(v.bits, 0);
    }

    #[test]
    fn test_signed_interpretation() {
        // Test 1-bit signed values
        assert_eq!(bv::<1>(0b0).signed(), 0);
        assert_eq!(bv::<1>(0b1).signed(), -1);

        // Test 2-bit signed values
        assert_eq!(bv::<2>(0b00).signed(), 0);
        assert_eq!(bv::<2>(0b01).signed(), 1);
        assert_eq!(bv::<2>(0b10).signed(), -2);
        assert_eq!(bv::<2>(0b11).signed(), -1);

        // Test 3-bit signed values
        assert_eq!(bv::<3>(0b000).signed(), 0);
        assert_eq!(bv::<3>(0b001).signed(), 1);
        assert_eq!(bv::<3>(0b010).signed(), 2);
        assert_eq!(bv::<3>(0b011).signed(), 3);
        assert_eq!(bv::<3>(0b100).signed(), -4);
        assert_eq!(bv::<3>(0b101).signed(), -3);
        assert_eq!(bv::<3>(0b110).signed(), -2);
        assert_eq!(bv::<3>(0b111).signed(), -1);

        // Test 4-bit signed values
        assert_eq!(bv::<4>(0b0000).signed(), 0);
        assert_eq!(bv::<4>(0b0001).signed(), 1);
        assert_eq!(bv::<4>(0b0111).signed(), 7);
        assert_eq!(bv::<4>(0b1000).signed(), -8);
        assert_eq!(bv::<4>(0b1001).signed(), -7);
        assert_eq!(bv::<4>(0b1111).signed(), -1);

        // Test 8-bit signed values
        assert_eq!(bv::<8>(0x00).signed(), 0);
        assert_eq!(bv::<8>(0x01).signed(), 1);
        assert_eq!(bv::<8>(0x7F).signed(), 127);
        assert_eq!(bv::<8>(0x80).signed(), -128);
        assert_eq!(bv::<8>(0xFF).signed(), -1);

        // Test 16-bit signed values
        assert_eq!(bv::<16>(0x0000).signed(), 0);
        assert_eq!(bv::<16>(0x0001).signed(), 1);
        assert_eq!(bv::<16>(0x7FFF).signed(), 32767);
        assert_eq!(bv::<16>(0x8000).signed(), -32768);
        assert_eq!(bv::<16>(0xFFFF).signed(), -1);

        // Test 32-bit signed values
        assert_eq!(bv::<32>(0x00000000).signed(), 0);
        assert_eq!(bv::<32>(0x00000001).signed(), 1);
        assert_eq!(bv::<32>(0x7FFFFFFF).signed(), 2147483647);
        assert_eq!(bv::<32>(0x80000000).signed(), -2147483648);
        assert_eq!(bv::<32>(0xFFFFFFFF).signed(), -1);

        // Test 64-bit signed values
        assert_eq!(bv::<64>(0x0000000000000000).signed(), 0);
        assert_eq!(bv::<64>(0x0000000000000001).signed(), 1);
        assert_eq!(bv::<64>(0x7FFFFFFFFFFFFFFF).signed(), 9223372036854775807);
        assert_eq!(bv::<64>(0x8000000000000000).signed(), -9223372036854775808);
        assert_eq!(bv::<64>(0xFFFFFFFFFFFFFFFF).signed(), -1);
    }

    #[test]
    fn test_signed_vs_unsigned() {
        // Test that unsigned and signed give different results for negative values
        let v = bv::<8>(0xFF);
        assert_eq!(v.unsigned(), 255);
        assert_eq!(v.signed(), -1);

        let v = bv::<8>(0x80);
        assert_eq!(v.unsigned(), 128);
        assert_eq!(v.signed(), -128);

        let v = bv::<16>(0x8000);
        assert_eq!(v.unsigned(), 32768);
        assert_eq!(v.signed(), -32768);

        // Test that unsigned and signed give same results for positive values
        let v = bv::<8>(0x7F);
        assert_eq!(v.unsigned(), 127);
        assert_eq!(v.signed(), 127);

        let v = bv::<8>(0x00);
        assert_eq!(v.unsigned(), 0);
        assert_eq!(v.signed(), 0);
    }

    #[test]
    fn test_sign_extend() {
        // Test sign extending positive values from 4 to 8 bits
        let input = bv::<4>(0b0111); // 7 in 4 bits
        let result = sign_extend::<4, 8>(8, input);
        assert_eq!(result.bits(), 0b00000111); // Should remain 7 in 8 bits
        
        // Test sign extending negative values from 4 to 8 bits
        let input = bv::<4>(0b1000); // -8 in 4 bits (two's complement)
        let result = sign_extend::<4, 8>(8, input);
        assert_eq!(result.bits(), 0b11111000); // Should become -8 in 8 bits

        let input = bv::<4>(0b1111); // -1 in 4 bits
        let result = sign_extend::<4, 8>(8, input);
        assert_eq!(result.bits(), 0b11111111); // Should become -1 in 8 bits

        // Test sign extending from 8 to 16 bits
        let input = bv::<8>(0x7F); // 127 in 8 bits (positive)
        let result = sign_extend::<8, 16>(16, input);
        assert_eq!(result.bits(), 0x007F); // Should remain 127 in 16 bits

        let input = bv::<8>(0x80); // -128 in 8 bits (negative)
        let result = sign_extend::<8, 16>(16, input);
        assert_eq!(result.bits(), 0xFF80); // Should become -128 in 16 bits

        let input = bv::<8>(0xFF); // -1 in 8 bits
        let result = sign_extend::<8, 16>(16, input);
        assert_eq!(result.bits(), 0xFFFF); // Should become -1 in 16 bits

        // Test sign extending from 16 to 32 bits
        let input = bv::<16>(0x7FFF); // 32767 in 16 bits (positive)
        let result = sign_extend::<16, 32>(32, input);
        assert_eq!(result.bits(), 0x00007FFF); // Should remain 32767 in 32 bits

        let input = bv::<16>(0x8000); // -32768 in 16 bits (negative)
        let result = sign_extend::<16, 32>(32, input);
        assert_eq!(result.bits(), 0xFFFF8000); // Should become -32768 in 32 bits

        // Test sign extending from 32 to 64 bits
        let input = bv::<32>(0x7FFFFFFF); // Positive value
        let result = sign_extend::<32, 64>(64, input);
        assert_eq!(result.bits(), 0x000000007FFFFFFF);

        let input = bv::<32>(0x80000000); // Negative value
        let result = sign_extend::<32, 64>(64, input);
        assert_eq!(result.bits(), 0xFFFFFFFF80000000);

        // Test edge cases - extending by 1 bit
        let input = bv::<1>(0b0); // 0 in 1 bit
        let result = sign_extend::<1, 2>(2, input);
        assert_eq!(result.bits(), 0b00); // Should remain 0

        let input = bv::<1>(0b1); // -1 in 1 bit
        let result = sign_extend::<1, 2>(2, input);
        assert_eq!(result.bits(), 0b11); // Should become -1 in 2 bits

        // Test extending smaller values
        let input = bv::<3>(0b101); // -3 in 3 bits
        let result = sign_extend::<3, 8>(8, input);
        assert_eq!(result.bits(), 0b11111101); // Should become -3 in 8 bits

        let input = bv::<3>(0b011); // 3 in 3 bits
        let result = sign_extend::<3, 8>(8, input);
        assert_eq!(result.bits(), 0b00000011); // Should remain 3 in 8 bits

        // Test extending from 64 to 64 bits (no-op, but widely used)
        let input = bv::<64>(0x7FFFFFFFFFFFFFFF); // Maximum positive 64-bit value
        let result = sign_extend::<64, 64>(64, input);
        assert_eq!(result.bits(), 0x7FFFFFFFFFFFFFFF); // Should remain unchanged

        let input = bv::<64>(0x8000000000000000); // Minimum negative 64-bit value
        let result = sign_extend::<64, 64>(64, input);
        assert_eq!(result.bits(), 0x8000000000000000); // Should remain unchanged

        let input = bv::<64>(0xFFFFFFFFFFFFFFFF); // -1 in 64 bits
        let result = sign_extend::<64, 64>(64, input);
        assert_eq!(result.bits(), 0xFFFFFFFFFFFFFFFF); // Should remain unchanged

        let input = bv::<64>(0x0000000000000000); // 0 in 64 bits
        let result = sign_extend::<64, 64>(64, input);
        assert_eq!(result.bits(), 0x0000000000000000); // Should remain unchanged
    }
}
