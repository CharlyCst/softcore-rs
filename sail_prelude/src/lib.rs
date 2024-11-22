#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use core::ops;
use std::{cmp::min, process, usize};

use rand::Rng;

#[allow(non_upper_case_globals)]
pub const zero_reg: BitVector<64> = BitVector::new(0x0);

pub fn sail_branch_announce(_value: usize, _pc: BitVector<64>) {}

pub fn signed<const N: usize>(e: BitVector<N>) -> BitVector<64> {
    // TODO: Is this function correct?
    BitVector::<64>::new(e.bits())
}

pub fn lteq_int(e1: usize, e2: usize) -> bool {
    e1 <= e2
} 

pub fn gt_int(e1: usize, e2: usize) -> bool {
    e1 > e2
}

pub fn bitvector_length<const N: usize>(_e: BitVector<N>) -> usize {
    N
}

pub fn bitvector_concat<const N: usize, const M: usize>(
    e1: BitVector<N>, 
    e2: BitVector<M>
) -> BitVector<{ N + M }> {
    BitVector::<{ N + M }>::new( (e1.bits() << M) | e2.bits() )
}


pub fn sys_enable_writable_misa() -> bool {
    true
}

pub fn sys_enable_rvc() -> bool {
    true
}

pub fn sys_enable_fdext() -> bool {
    true
}

pub fn  sys_enable_zfinx() -> bool {
    true
}

pub fn sys_enable_writable_fiom() -> bool {
    true
}

pub fn get_16_random_bits() -> BitVector<16> {
    let number: u64 = rand::thread_rng().gen();
    BitVector::<16>::new(number & ((1 << 17) - 1))
}

pub fn not_implemented() -> ! {
    panic!("Feature not implemented yet");
}

pub fn __exit() -> ! {
    println!("Called exit, leaving the program");
    process::exit(1)
}

pub fn print_output(text: String) {
    println!("{}", text)
}

pub fn print_platform(text: String) {
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

pub fn bits_str<const N: usize>(val: BitVector<N>) -> String {
    String::from(format!("{:b}", val.bits()))
}

pub fn print_reg(register: String) {
    print!("{}", register)
}

pub fn sys_pmp_grain() -> usize {
    // TODO: What is this function doing?
    1
}

pub fn bitvector_access<const N: usize>(vec: BitVector<N>, idx: usize) -> bool {
    (vec.bits() & (1 << idx)) > 0
}

pub fn plat_mtval_has_illegal_inst_bits() -> bool {
    // TODO: Implement this function
    false
}

pub fn truncate(v: BitVector<64>, _size: usize) -> BitVector<64> {
    // TODO: What should we do in this function?
    v
}

pub fn sys_pmp_count() -> usize {
    16
}

pub fn zero_extend<const M: usize>(value: usize, input: BitVector<M>) -> BitVector<64> {
    assert!(value == 64, "handle the case where zero_extend has value not equal 64"); 
    BitVector::<64>::new(input.bits())
}

pub fn sign_extend<const M: usize>(value: usize, input: BitVector<M>) -> BitVector<64> {
    assert!(value == 64, "handle the case where sign_extend has value not equal 64");
    BitVector::<64>::new(input.bits())
}

pub fn sail_ones<const N: usize>(_n: usize) -> BitVector<N> {
    !BitVector::<N>::new(0)
}

pub fn min_int(v1: usize, v2: usize) -> usize {
    min(v1, v2)
}

pub fn cancel_reservation() {
    // In the future, extend this function
}

// TODO: This is enough for the risc-v transpilation, but not enought for full sail-to-rust
pub fn subrange_bits(vec: BitVector<64>, _end: usize, _start: usize) -> BitVector<64> {
    vec
}

pub fn update_subrange_bits<const N: usize, const M: usize>(bits: BitVector<N>, from: u64, to: u64, value: BitVector<M>) -> BitVector<N> {
    assert!(from - to + 1 == M as u64, "size don't match");

    // Generate the 111111 mask
    let mut mask = (1 << (M+1)) - 1;
    // Shit and invert it
    mask = !(mask << from);
    
    // Now we can update and return the updated value
    return  BitVector::<N>::new((bits.bits & mask) | (value.bits() << from))
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BitVector<const N: usize> {
    pub bits: u64,
}


#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BitField<const T: usize> {
    pub bits: BitVector<T>
}

impl<const N: usize> BitField<N> {
    pub const fn subrange<const A: usize, const B: usize, const C: usize>(self) -> BitVector<C> {
        assert!(B - A == C, "Invalid subrange parameters");
        assert!(B <= N, "Invalid subrange");

        let mut val = self.bits; // The current value
        val.bits &= BitVector::<B>::bit_mask(); // Remove top bits
        val.bits >>= A; // Shift all the bits
        BitVector::new(val.bits)
    }

    pub const fn set_subrange<const A: usize, const B: usize, const C: usize>(
        self,
        _bits: BitVector<C>,
    ) -> Self {
        assert!(B - A == C, "Invalid subrange parameters");
        assert!(B <= N, "Invalid subrange");

        let mut val = self.bits; // The current value
        val.bits &= BitVector::<B>::bit_mask(); // Remove top bits
        val.bits >>= A; // Shift all the bits
        BitField {
            bits: val,
        }
    }
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
