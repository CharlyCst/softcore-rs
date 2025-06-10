#![allow(incomplete_features, non_camel_case_types)]

use core::ops;
use std::cmp::min;

// NOTE: Ideally we would use unbounded integers for natural numbers. Yet in practice this would
// mess up with things such as the SMT solver during symbolic execution.
// After manual inspection, u128 are big enough for all the RISC-V use cases, so we keep that until
// a better solution is needed.
pub type nat = u128;

pub fn sail_branch_announce(_value: usize, _pc: BitVector<64>) {}

pub fn lteq_int(e1: usize, e2: usize) -> bool {
    e1 <= e2
}

pub fn gt_int(e1: usize, e2: usize) -> bool {
    e1 > e2
}

pub fn bitvector_length<const N: usize>(_e: BitVector<N>) -> usize {
    N
}

pub fn parse_hex_bits<const N: usize>(_n: usize, _hex_str: &str) -> BitVector<N> {
    todo!("'parse_hex_bits' is not yet implemented");
}

pub fn bitvector_concat<const N: usize, const M: usize, const NM: usize>(
    e1: BitVector<N>,
    e2: BitVector<M>,
) -> BitVector<{ NM }> {
    BitVector::<{ NM }>::new((e1.bits() << M) | e2.bits())
}

pub fn get_slice_int<const L: usize>(l: usize, n: usize, start: usize) -> BitVector<L> {
    let val = (n >> start) & (mask(l) as usize);
    BitVector::new(val as u64)
}

pub fn get_16_random_bits(_unit: ()) -> BitVector<16> {
    BitVector::<16>::new(0)
}

pub fn not_implemented<T>(_any: T) -> ! {
    panic!("Feature not implemented yet");
}

pub fn internal_error(_file: String, _line: usize, _s: String) -> ! {
    panic!("Softcore: internal error")
}

pub fn print_output<const N: usize>(text: String, _csr: BitVector<N>) {
    println!("{}", text)
}

pub fn print_platform(text: String) {
    println!("{}", text)
}

pub fn bits_str<const N: usize>(val: BitVector<N>) -> String {
    format!("{:b}", val.bits())
}

pub fn bitvector_access<const N: usize>(vec: BitVector<N>, idx: usize) -> bool {
    (vec.bits() & (1 << idx)) > 0
}

// Todo: implement truncate for other sizes if required
pub fn truncate(v: BitVector<64>, size: usize) -> BitVector<64> {
    assert!(size == 64);
    v
}

pub fn sign_extend<const M: usize>(value: usize, input: BitVector<M>) -> BitVector<64> {
    assert!(
        value == 64,
        "handle the case where sign_extend has value not equal 64"
    );
    let extension = ((1 << (64 - M)) - 1) << M;
    BitVector::<64>::new(extension | input.bits)
}

pub const fn sail_ones<const N: usize>(n: usize) -> BitVector<N> {
    assert!(n <= 64);
    BitVector::<N>::new(mask(n))
}

pub const fn sail_zeros<const N: usize>(_n: usize) -> BitVector<N> {
    BitVector::<N>::new(0)
}

pub fn min_int(v1: usize, v2: usize) -> usize {
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

pub fn subrange_bits<const IN: usize, const OUT: usize>(
    vec: BitVector<IN>,
    end: usize,
    start: usize,
) -> BitVector<OUT> {
    assert_eq!(end - start + 1, OUT);
    assert!(OUT <= IN);

    BitVector::new((vec.bits >> start) & mask(OUT))
}

pub fn subrange_bits_8(vec: BitVector<64>, end: usize, start: usize) -> BitVector<8> {
    assert!(end - start + 1 == 8); // todo: In the future, we should improve the subrange bits function
    BitVector::<8>::new((vec.bits >> start) & 0xFF)
}

pub fn update_subrange_bits<const N: usize, const M: usize>(
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
    BitVector::new((bits.bits & mask) | (value.bits() << from))
}

// TODO: Make this function more generic in the future.
pub fn bitvector_update(v: BitVector<64>, pos: usize, value: bool) -> BitVector<64> {
    let mask = 1 << pos;
    BitVector::<64>::new((v.bits() & !mask) | (value as u64) << pos)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitVector<const N: usize> {
    bits: u64,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitField<const T: usize> {
    pub bits: BitVector<T>,
}

impl<const N: usize> BitField<N> {
    pub const fn new(value: u64) -> Self {
        BitField {
            bits: BitVector::new(value),
        }
    }

    pub const fn subrange<const A: usize, const B: usize, const C: usize>(self) -> BitVector<C> {
        assert!(B - A == C, "Invalid subrange parameters");
        assert!(B <= N, "Invalid subrange");

        self.bits.subrange::<A, B, C>()
    }

    pub const fn set_subrange<const A: usize, const B: usize, const C: usize>(
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

impl<const N: usize> PartialOrd for BitVector<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl<const N: usize> BitVector<N> {
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

    pub const fn as_usize(self) -> usize {
        self.bits as usize
    }

    pub const fn zero_extend<const M: usize>(self) -> BitVector<M> {
        assert!(M >= N, "Can not zero-extend to a smaller size!");
        assert!(M <= 64, "Maximum zero-extend supported size if 64");

        // Here we have nothing to do, we already use 64 bits with zeroes for MSBs
        BitVector { bits: self.bits }
    }

    pub fn set_bit(self, idx: usize, value: bool) -> Self {
        assert!(idx < N, "Out of bounds array check");
        let new_value = if value {
            self.bits | 1u64 << idx
        } else {
            self.bits & !(1u64 << idx)
        };
        BitVector { bits: new_value }
    }

    pub const fn subrange<const A: usize, const B: usize, const C: usize>(self) -> BitVector<C> {
        assert!(B - A == C, "Invalid subrange parameters");
        assert!(B <= N, "Invalid subrange");

        let mut val = self.bits; // The current value
        val &= BitVector::<B>::bit_mask(); // Remove top bits
        val >>= A; // Shift all the bits
        BitVector::new(val)
    }

    pub const fn set_subrange<const A: usize, const B: usize, const C: usize>(
        self,
        bits: BitVector<C>,
    ) -> Self {
        assert!(B - A == C, "Invalid set_subrange parameters");
        assert!(B <= N, "Invalid subrange");

        let mask = !(BitVector::<C>::bit_mask() << A);
        let new_bits = bits.bits() << A;
        BitVector::new((self.bits & mask) | new_bits)
    }

    pub const fn wrapped_add(self, other: BitVector<N>) -> BitVector<N> {
        BitVector::<N>::new(self.bits.wrapping_add(other.bits))
    }

    /// Returns a bit mask with 1 for the first [N] bits.
    const fn bit_mask() -> u64 {
        assert!(N <= 64);

        if N == 64 { u64::MAX } else { (1 << N) - 1 }
    }
}

impl<const N: usize> ops::BitAnd for BitVector<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits & rhs.bits,
        }
    }
}

impl<const N: usize> ops::BitOr for BitVector<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits | rhs.bits,
        }
    }
}

impl<const N: usize> ops::BitXor for BitVector<N> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            bits: self.bits ^ rhs.bits,
        }
    }
}

impl<const N: usize> ops::Shl<usize> for BitVector<N> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self {
            bits: self.bits << rhs,
        }
    }
}

impl<const N: usize> ops::Shr<usize> for BitVector<N> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self {
            bits: self.bits >> rhs,
        }
    }
}

impl<const N: usize> ops::Not for BitVector<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitVector::new((!self.bits) & Self::bit_mask())
    }
}

impl<const N: usize> std::ops::Add<i64> for BitVector<N> {
    type Output = Self;

    fn add(self, rhs: i64) -> BitVector<N> {
        let result = self.bits as i64 + rhs;
        // If the result is out of bounds, we may want to handle overflow
        BitVector::<N>::new(result as u64) // Returning the result as BitVector
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
        assert_eq!((!BitVector::<1>::new(0b1)).bits(), 0b0);
        assert_eq!((!BitVector::<1>::new(0b0)).bits(), 0b1);
        assert_eq!((!BitVector::<2>::new(0b01)).bits(), 0b10);
        assert_eq!((!BitVector::<2>::new(0b11)).bits(), 0b00);
    }

    #[test]
    fn subrange_bitvector() {
        let v = BitVector::<32>::new(0b10110111);

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

        assert_eq!(
            BitVector::<32>::new(0xffffffff)
                .subrange::<7, 23, 16>()
                .bits(),
            0xffff
        );
        assert_eq!(v.subrange::<2, 7, 5>().bits(), 0b01101);

        let v = BitVector::<32>::new(0b10110111);
        assert_eq!(
            v.set_subrange::<0, 1, 1>(BitVector::new(0b0)).bits(),
            0b10110110
        );
        assert_eq!(
            v.set_subrange::<0, 1, 1>(BitVector::new(0b1)).bits(),
            0b10110111
        );
        assert_eq!(
            v.set_subrange::<0, 2, 2>(BitVector::new(0b00)).bits(),
            0b10110100
        );
        assert_eq!(
            v.set_subrange::<2, 5, 3>(BitVector::new(0b010)).bits(),
            0b10101011
        );

        assert_eq!(
            BitVector::<64>::new(0x0000000000000000)
                .subrange::<60, 64, 4>()
                .bits(),
            0x0
        );
        assert_eq!(
            BitVector::<64>::new(0xa000000000000000)
                .subrange::<60, 64, 4>()
                .bits(),
            0xa
        );
        assert_eq!(
            BitVector::<64>::new(0xb000000000000000)
                .subrange::<60, 64, 4>()
                .bits(),
            0xb
        );
        assert_eq!(
            BitVector::<64>::new(0xc000000000000000)
                .subrange::<60, 64, 4>()
                .bits(),
            0xc
        );
        assert_eq!(
            BitVector::<64>::new(0xd000000000000000)
                .subrange::<60, 64, 4>()
                .bits(),
            0xd
        );
        assert_eq!(
            BitVector::<64>::new(0xe000000000000000)
                .subrange::<60, 64, 4>()
                .bits(),
            0xe
        );
        assert_eq!(
            BitVector::<64>::new(0xf000000000000000)
                .subrange::<60, 64, 4>()
                .bits(),
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

        let v = BitVector::<32>::new(0b10110111);
        assert_eq!(
            v.set_subrange::<0, 1, 1>(BitVector::new(0b0)).bits(),
            0b10110110
        );
        assert_eq!(
            v.set_subrange::<0, 1, 1>(BitVector::new(0b1)).bits(),
            0b10110111
        );
        assert_eq!(
            v.set_subrange::<0, 2, 2>(BitVector::new(0b00)).bits(),
            0b10110100
        );
        assert_eq!(
            v.set_subrange::<2, 5, 3>(BitVector::new(0b010)).bits(),
            0b10101011
        );
    }

    #[test]
    fn test_update_subrange_bits() {
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b11111100),
                1,
                0,
                BitVector::<2>::new(0b11)
            )
            .bits,
            0b11111111
        );
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b00000000),
                0,
                0,
                BitVector::<1>::new(0b1)
            )
            .bits,
            0b00000001
        );
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b00000000),
                1,
                1,
                BitVector::<1>::new(0b1)
            )
            .bits,
            0b00000010
        );
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b00000000),
                2,
                2,
                BitVector::<1>::new(0b1)
            )
            .bits,
            0b00000100
        );
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b00000000),
                3,
                3,
                BitVector::<1>::new(0b1)
            )
            .bits,
            0b00001000
        );
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b00000000),
                4,
                4,
                BitVector::<1>::new(0b1)
            )
            .bits,
            0b00010000
        );
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b00000000),
                5,
                5,
                BitVector::<1>::new(0b1)
            )
            .bits,
            0b00100000
        );
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b00000000),
                6,
                6,
                BitVector::<1>::new(0b1)
            )
            .bits,
            0b01000000
        );
        assert_eq!(
            update_subrange_bits(
                BitVector::<8>::new(0b00000000),
                7,
                7,
                BitVector::<1>::new(0b1)
            )
            .bits,
            0b10000000
        );
    }

    #[test]
    fn bitwise_operators() {
        let v = BitVector::<32>::new(0b1);

        assert_eq!(v, v | v);
        assert_eq!(v, v & v);
        assert_eq!(v, v ^ v ^ v);
        assert_eq!(v, !!v);

        for i in 0..30 {
            assert_eq!(v, (v << i) >> i);
        }
    }

    #[test]
    fn test_zero_extend() {
        let v = BitVector::<8>::new(0b1010);

        assert_eq!(v.bits, v.zero_extend::<16>().bits);
        assert_eq!(v.bits, v.zero_extend::<63>().bits);
        assert_eq!(v.bits, v.zero_extend::<64>().bits);
    }

    #[test]
    fn test_bitvector_concat() {
        const SIZE: usize = 20;
        const NEW_SIZE: usize = 40;

        for i in 0..(1 << SIZE) {
            let v = BitVector::<SIZE>::new(i);
            assert_eq!(
                bitvector_concat::<SIZE, SIZE, NEW_SIZE>(v, v).bits,
                i + (i << SIZE)
            );
        }
    }

    #[test]
    fn test_bitvector_access() {
        const SIZE: usize = 10;

        for i in 0..(1 << SIZE) {
            let v = BitVector::<SIZE>::new(i);
            for idx in 0..SIZE {
                assert_eq!((i & (1 << idx)) > 0, bitvector_access(v, idx))
            }
        }
    }

    #[test]
    fn test_set_bit() {
        const SIZE: usize = 60;

        let mut v = BitVector::<SIZE>::new(0);
        let mut val: u64 = 0;
        for idx in 0..SIZE {
            val |= 1u64 << idx;
            v = v.set_bit(idx, true);

            assert_eq!(v.bits, val);
        }

        for i in 0..SIZE {
            v = v.set_bit(i, false);
        }

        assert_eq!(v.bits, 0);
    }
}
