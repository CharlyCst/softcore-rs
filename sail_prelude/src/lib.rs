use core::ops;
use std::{process, usize};

pub fn not_implemented() -> ! {
    panic!("Feature not implemented yet");
}

pub fn __exit() -> ! {
    println!("Called exit, leaving the program");
    process::exit(1)
}

pub fn print_output(text: &str) {
    println!("{}", text)
}

pub fn dec_str(val: usize) -> String {
    // Format into a normal decimal string
    format!("{}", val) 
}

pub fn hex_str(val: usize) -> String {
    // Format into a hexadecimal string
    format!("{:x}", val) 
}

pub fn bitvector_access<const N: usize>(vec: BitVector<N>, idx: usize) -> bool {
    (vec.bits() & (1 << idx)) > 0
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BitVector<const N: usize> {
    bits: u64,
}

impl<const N: usize> BitVector<N> {
    pub const fn new(val: u64) -> Self {
        // First check that there is no more than N bits
        assert!(
            N == 64 || (N < 64 && (val >> N) == 0),
            "Too many bits in BitVector"
        );

        // If the check pass it is safe to construct
        Self { bits: val }
    }

    pub const fn new_empty() -> Self {
        Self { bits: 0}
    }

    pub const fn bits(self) -> u64 {
        self.bits
    }

    pub const fn as_usize(self) -> usize {
        self.bits as usize
    }

    pub fn set_vector_entry(&mut self, idx: usize, value: bool) {
        assert!(idx < N, "Out of bounds array check");
        if value {
            self.bits |= 1u64 << idx;
        } else {
            self.bits &= !(1u64 << idx);
        }
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

        if N == 64 {
            u64::MAX
        } else {
            (1 << N) - 1
        }
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

impl<const N: usize> ops::Not for BitVector<N> {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitVector::new((!self.bits) & Self::bit_mask())
    }
}

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
    fn subrange() {
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
}
