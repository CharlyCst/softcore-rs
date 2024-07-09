use core::ops;
use std::usize;

#[derive(Clone, Copy, Debug)]
pub struct BitVector<const N: usize> {
    bits: u64,
}

impl<const N: usize> BitVector<N> {
    pub const fn new(val: u64) -> Self {
        // First check that there is no more than N bits
        assert!(val >> N == 0, "Too many bits in BitVector");

        // If the check pass it is safe to construct
        Self { bits: val }
    }

    pub fn subrange<const A: usize, const B: usize, const C: usize>(self) -> BitVector<C> {
        assert_eq!(A - B, C, "Invalid subrange parameters");
        assert!(A >= N, "Invalid subrange");

        let mut val = self.bits;           // The current value
        val &= BitVector::<A>::bit_mask(); // Remove top bits
        val >>= B;                         // Shift all the bits
        BitVector::new(val)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitvec_masks() {
        assert_eq!(BitVector::<1>::bit_mask(), 0b1);
        assert_eq!(BitVector::<2>::bit_mask(), 0b11);
        assert_eq!(BitVector::<8>::bit_mask(), 0b11111111);
        assert_eq!(BitVector::<64>::bit_mask(), 0xffffffffffffffff);
    }
}
