use core::ops;

// BitStorage (Common ground between different BitVector implementation) ---------------------------

pub trait BitStorage:
    Eq
    + PartialEq
    + Clone
    + Copy
    + std::fmt::Debug
    + Default
    + PartialOrd
{
    fn len(&self) -> i128;
    fn unsigned(&self) -> i128;
    fn signed(&self) -> i128;
    fn new(len: i128, val: u64) -> Self;

    fn zeros(len: i128) -> Self;
    fn ones(len: i128) -> Self;

    fn bitand(self, rhs: Self) -> Self;
    fn bitor(self, rhs: Self) -> Self;
    fn bitxor(self, rhs: Self) -> Self;
    fn not(self) -> Self;
    fn add(self, rhs: i64) -> Self;

    fn shl(self, rhs: u128) -> Self;
    fn shr(self, rhs: u128) -> Self;

    fn set_bit(self, idx: i128, value: bool) -> Self;
    fn get_bit(self, idx: i128) -> bool;

    /* TODO(Gurvan):
        We have a bunch of functions we need which require different types trickery.
        We might want to implement them at the BitVector level rather than the BitStorage:
        - `subrange`
        - `set_subrange`
        - `zero_extend`
        - `concat`
        - `sign_extend`
    */
}

// BitDynamic (BitVector with dynamically known size) ----------------------------------------------

const BITDYNAMIC_SIZE: usize = 8;
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitDynamic {
    pub len: i128,
    pub bits: [u64; BITDYNAMIC_SIZE], /* Little endian */
}

impl BitStorage for BitDynamic {
    fn len(&self) -> i128 {
        self.len
    }

    fn signed(&self) -> i128 {
        todo!()
    }

    fn unsigned(&self) -> i128 {
        assert!(self.len <= 128);
        ((self.bits[0] as u128) | ((self.bits[1] as u128) << 64)) as i128
    }

    fn new(len: i128, val: u64) -> Self {
        let mut bits = [0; BITDYNAMIC_SIZE];
        bits[0] = val;
        Self { len, bits }
    }

    fn zeros(len: i128) -> Self {
        Self::new(len, 0)
    }

    fn ones(len: i128) -> Self {
        Self { len: len, bits: Self::bit_mask(len) }
    }

    fn bitand(mut self, rhs: Self) -> Self {
        for i in 0..BITDYNAMIC_SIZE {
           self.bits[i] = self.bits[i] & rhs.bits[i];
        }
        self
    }

    fn bitor(mut self, rhs: Self) -> Self {
        for i in 0..BITDYNAMIC_SIZE {
            self.bits[i] = self.bits[i] | rhs.bits[i];
        }
        self
    }

    fn bitxor(mut self, rhs: Self) -> Self {
        for i in 0..BITDYNAMIC_SIZE {
            self.bits[i] = self.bits[i] ^ rhs.bits[i];
        }
        self
    }

    fn not(mut self) -> Self {
        let bitmask = Self::bit_mask(self.len);
        for i in 0..BITDYNAMIC_SIZE {
            self.bits[i] = !self.bits[i] & bitmask[i];
        }
        self
    }

    fn add(mut self, rhs: i64) -> Self {
        /* TODO(Gurvan): Check that this is correct */
        let mut remainder = rhs as i128;
        let mask = Self::bit_mask(self.len);
        for i in 0..BITDYNAMIC_SIZE {
            remainder = (self.bits[i] as i128) + remainder;
            self.bits[i] = ((remainder & (u64::MAX as i128)) as u64) & mask[i];
            remainder = remainder >> 64;
        }
        self
    }

    fn shl(self, rhs: u128) -> Self {
        todo!();
    }

    fn shr(self, rhs: u128) -> Self {
        todo!();
    }

    fn set_bit(mut self, idx: i128, value: bool) -> Self {
        if idx >= 0 && idx < self.len {
            let limb = (idx / 64) as usize;
            let bit = (idx % 64) as u32;
            if value { self.bits[limb] |= 1 << bit; }
            else { self.bits[limb] &= !(1 << bit); }
        }
        self
    }

    fn get_bit(self, idx: i128) -> bool {
        todo!()
    }
}

impl PartialOrd for BitDynamic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl<const LEN: i128> From<BitStatic<LEN>> for BitDynamic {
    fn from(bv: BitStatic<LEN>) -> Self {
        let mut bits = [ 0u64 ; BITDYNAMIC_SIZE ];
        bits[0] = bv.bits;
        Self { len: LEN, bits }
    }
}

impl BitDynamic {
    pub const fn bit_mask(len: i128) -> [u64; BITDYNAMIC_SIZE] {
        let mut mask = [0u64; BITDYNAMIC_SIZE];
        let mut i = 0;
        /* Must use while because can't use for i in 0..BITDYNAMIC_SIZE in constant function */
        while i < BITDYNAMIC_SIZE {
            let lower_bound = (i as i128) * 64;
            if len <= lower_bound {
                mask[i] = 0;
            } else if len >= lower_bound + 64 {
                mask[i] = u64::MAX;
            } else {
                mask[i] = (1 << (len % 64)) - 1;
            }
            i += 1;
        }
        mask
    }
}

// BitStatic (BitVector with statically known small [<= 64] size) ---------------------------------

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitStatic<const LEN: i128> {
    pub bits: u64,
}

impl<const LEN: i128> BitStorage for BitStatic<LEN> {
    fn len(&self) -> i128 {
        LEN
    }

    fn signed(&self) -> i128 {
        let value = self.bits as u128;
        let sign_bit_mask = 1 << (LEN - 1);
        if value & sign_bit_mask == 0 {
            // The number is positive, nothing to do
            value as i128
        } else {
            // The number is negative, need to fill upper bits with 1s
            let fill_mask = !((1 << LEN) - 1);
            (value | fill_mask) as i128
        }
    }

    fn unsigned(&self) -> i128 {
        self.bits as i128
    }

    fn new(len: i128, val: u64) -> Self {
        assert!(len == LEN);
        let _ = Self::ASSERT_LEN_VALID;
        if LEN < 64 {
            Self {
                bits: val & ((1 << LEN) - 1),
            }
        } else {
            Self { bits: val }
        }
    }

    fn zeros(len: i128) -> Self {
        Self::new(len, 0)
    }

    fn ones(len: i128) -> Self {
        Self::new(len, Self::BIT_MASK)
    }

    fn bitand(self, rhs: Self) -> Self{
        Self { bits: self.bits & rhs.bits }
    }

    fn bitor(self, rhs: Self) -> Self{
        Self { bits: self.bits | rhs.bits }
    }

    fn bitxor(self, rhs: Self) -> Self{
        Self { bits: self.bits ^ rhs.bits }
    }

    fn not(self) -> Self {
        Self { bits: !self.bits & Self::BIT_MASK }
    }

    fn add(self, rhs: i64) -> Self {
        let result = self.bits as i64 + rhs;
        // If the result is out of bounds, we may want to handle overflow
        Self { bits: result as u64 }
    }

    fn shl(self, rhs: u128) -> Self {
        Self { bits: self.bits << rhs }
    }

    fn shr(self, rhs: u128) -> Self {
        Self { bits: self.bits >> rhs }
    }

    fn set_bit(self, idx: i128, value: bool) -> Self {
        assert!(idx < LEN, "Out of bounds array check");
        let bits = if value {
            self.bits | 1u64 << idx
        } else {
            self.bits & !(1u64 << idx)
        };
        Self { bits }
    }

    fn get_bit(self, idx: i128) -> bool {
        assert!(idx < LEN);
        self.bits & (1 << idx) > 0
    }
}

impl<const LEN: i128> PartialOrd for BitStatic<LEN> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl<const LEN: i128> From<BitDynamic> for BitStatic<LEN> {
    fn from(bv: BitDynamic) -> Self {
        assert!(bv.len == LEN);
        Self { bits: bv.bits[0] }
    }
}

impl<const LEN: i128> BitStatic<LEN> {
    pub const ASSERT_LEN_VALID: () = assert!(LEN < 64, "Length of BitStatic must be less than 64");

    pub const BIT_MASK: u64 = if LEN == 64 { u64::MAX } else { (1 << LEN) - 1 };
}

// BitVector ---------------------------------------------------------------------------------------

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitVector<S: BitStorage> {
    pub storage: S,
}

impl<BS: BitStorage> BitVector<BS> {
    pub fn new(len: i128, val: u64) -> Self {
        Self {
            storage: BS::new(len, val),
        }
    }

    pub fn zeros(len: i128) -> Self {
        Self { storage: BS::zeros(len) }
    }

    pub fn ones(len: i128) -> Self {
        Self { storage: BS::ones(len) }
    }

    pub fn len(self) -> i128 {
        self.storage.len()
    }

    // TODO(Gurvan): Remove if unused
    // pub fn bits(self) -> u64 {
    //     self.storage.unsigned().try_into().unwrap()
    // }

    pub fn signed(self) -> i128 {
        self.storage.signed()
    }

    pub fn unsigned(self) -> i128 {
        self.storage.unsigned()
    }

    pub fn set_bit(self, pos: i128, value: bool) -> Self {
        Self { storage: self.storage.set_bit(pos, value) }
    }

    pub fn get_bit(self, pos: i128) -> bool {
        self.storage.get_bit(pos)
    }
}

/// Create a fresh [BitVector].
///
/// This is equivalent to [BitVector::new], with a shorted syntax.
pub fn bv<const LEN: i128>(val: u64) -> BitVector<BitStatic<LEN>> {
    BitVector::new(LEN, val)
}

impl<BS: BitStorage> ops::BitAnd for BitVector<BS> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self { storage: self.storage.bitand(rhs.storage) }
    }
}

impl<BS: BitStorage> ops::BitOr for BitVector<BS> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { storage: self.storage.bitor(rhs.storage) }
    }
}

impl<BS: BitStorage> ops::BitXor for BitVector<BS> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { storage: self.storage.bitxor(rhs.storage) }
    }
}

impl<BS: BitStorage> ops::Shl<usize> for BitVector<BS> {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self { storage: self.storage.shl(rhs as u128) }
    }
}

impl<BS: BitStorage> ops::Shl<u128> for BitVector<BS> {
    type Output = Self;

    fn shl(self, rhs: u128) -> Self::Output {
        Self { storage: self.storage.shl(rhs) }
    }
}

impl<BS: BitStorage> ops::Shl<i128> for BitVector<BS> {
    type Output = Self;

    fn shl(self, rhs: i128) -> Self::Output {
        Self { storage: self.storage.shl(rhs as u128) }
    }
}

impl<BS: BitStorage> ops::Shl<i32> for BitVector<BS> {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        Self { storage: self.storage.shl(rhs as u128) }
    }
}

impl<BS: BitStorage> ops::Shr<usize> for BitVector<BS> {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self { storage: self.storage.shr(rhs as u128) }
    }
}

impl<BS: BitStorage> ops::Shr<u128> for BitVector<BS> {
    type Output = Self;

    fn shr(self, rhs: u128) -> Self::Output {
        Self { storage: self.storage.shr(rhs as u128) }
    }
}

impl<BS: BitStorage> ops::Shr<i128> for BitVector<BS> {
    type Output = Self;

    fn shr(self, rhs: i128) -> Self::Output {
        Self { storage: self.storage.shr(rhs as u128) }
    }
}

impl<BS: BitStorage> ops::Shr<i32> for BitVector<BS> {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        Self { storage: self.storage.shr(rhs as u128) }
    }
}

impl<BS: BitStorage> ops::Not for BitVector<BS> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self { storage: self.storage.not() }
    }
}

impl<BS: BitStorage> ops::Add<i64> for BitVector<BS> {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        Self { storage: self.storage.add(rhs) }
    }
}

impl<const LEN: i128> From<BitVector<BitStatic<LEN>>> for BitVector<BitDynamic> {
    fn from(bv: BitVector<BitStatic<LEN>>) -> Self {
        Self {
            storage: BitDynamic::from(bv.storage),
        }
    }
}

impl<const LEN: i128> From<BitVector<BitDynamic>> for BitVector<BitStatic<LEN>> {
    fn from(bv: BitVector<BitDynamic>) -> Self {
        Self {
            storage: BitStatic::from(bv.storage),
        }
    }
}

// Tests -------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests_bitstatic {
    use super::*;
    use super::BitStorage;

    #[test]
    fn bitvec_masks() {
        assert_eq!(BitStatic::<0>::BIT_MASK, 0b0);
        assert_eq!(BitStatic::<1>::BIT_MASK, 0b1);
        assert_eq!(BitStatic::<2>::BIT_MASK, 0b11);
        assert_eq!(BitStatic::<8>::BIT_MASK, 0b11111111);
        assert_eq!(BitStatic::<64>::BIT_MASK, 0xffffffffffffffff);
    }

    #[test]
    fn bitvec_not() {
        assert_eq!(BitStatic::<1>::new(1, 0b1).not().unsigned(), 0b0);
        assert_eq!(BitStatic::<1>::new(1, 0b0).not().unsigned(), 0b1);
        assert_eq!(BitStatic::<2>::new(2, 0b01).not().unsigned(), 0b10);
        assert_eq!(BitStatic::<2>::new(2, 0b11).not().unsigned(), 0b00);
    }

    #[test]
    fn subrange_bitvector() {
        let v = BitStatic::<32>::new(32, 0b10110111);

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

        assert_eq!(bv(32, 0xffffffff).subrange::<7, 23, 16>().bits(), 0xffff);
        assert_eq!(v.subrange::<2, 7, 5>().bits(), 0b01101);

        let v = bv(32, 0b10110111);
        assert_eq!(v.set_subrange::<0, 1, 1>(BitStatic::<1>::new(1, 0b0)).bits(), 0b10110110);
        assert_eq!(v.set_subrange::<0, 1, 1>(BitStatic::<1>::new(1, 0b1)).bits(), 0b10110111);
        assert_eq!(v.set_subrange::<0, 2, 2>(BitStatic::<2>::new(2, 0b00)).bits(), 0b10110100);
        assert_eq!(v.set_subrange::<2, 5, 3>(BitStatic::<3>::new(3, 0b010)).bits(), 0b10101011);

        assert_eq!(
            bv(64, 0x0000000000000000).subrange::<60, 64, 4>().bits(),
            0x0
        );
        assert_eq!(
            bv(64, 0xa000000000000000).subrange::<60, 64, 4>().bits(),
            0xa
        );
        assert_eq!(
            bv(64, 0xb000000000000000).subrange::<60, 64, 4>().bits(),
            0xb
        );
        assert_eq!(
            bv(64, 0xc000000000000000).subrange::<60, 64, 4>().bits(),
            0xc
        );
        assert_eq!(
            bv(64, 0xd000000000000000).subrange::<60, 64, 4>().bits(),
            0xd
        );
        assert_eq!(
            bv(64, 0xe000000000000000).subrange::<60, 64, 4>().bits(),
            0xe
        );
        assert_eq!(
            bv(64, 0xf000000000000000).subrange::<60, 64, 4>().bits(),
            0xf
        );
    }

    #[test]
    fn test_update_subrange_bits() {
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b11111100), 1, 0, BitStatic::<2>::new(2, 0b11)).bits,
            0b11111111
        );
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 0, 0, BitStatic::<1>::new(1, 0b1)).bits,
            0b00000001
        );
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 1, 1, BitStatic::<1>::new(1, 0b1)).bits,
            0b00000010
        );
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 2, 2, BitStatic::<1>::new(1, 0b1)).bits,
            0b00000100
        );
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 3, 3, BitStatic::<1>::new(1, 0b1)).bits,
            0b00001000
        );
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 4, 4, BitStatic::<1>::new(1, 0b1)).bits,
            0b00010000
        );
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 5, 5, BitStatic::<1>::new(1, 0b1)).bits,
            0b00100000
        );
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 6, 6, BitStatic::<1>::new(1, 0b1)).bits,
            0b01000000
        );
        assert_eq!(
            update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 7, 7, BitStatic::<1>::new(1, 0b1)).bits,
            0b10000000
        );
    }

    #[test]
    fn bitwise_operators() {
        let v = BitStatic::<32>::new(32, 0b1);

        assert_eq!(v, v.bitor(v));
        assert_eq!(v, v.bitand(v));
        assert_eq!(v, v.bitxor(v).bitxor(v));
        assert_eq!(v, v.not().not());

        for i in 0u128..30u128 {
            assert_eq!(v, v.shl(i).shr(i));
        }
    }

    #[test]
    fn test_zero_extend() {
        let v = BitStatic::<8>::new(8, 0b1010);

        assert_eq!(v.unsigned(), v.zero_extend(16).unsigned());
        assert_eq!(v.unsigned(), v.zero_extend(63).unsigned());
        assert_eq!(v.unsigned(), v.zero_extend(64).unsigned());
    }

    #[test]
    fn test_concat() {
        const SIZE: i128 = 20;

        for i in 0..(1 << (SIZE as usize)) {
            let v = BitStatic::<SIZE>::new(SIZE, i);
            assert_eq!(bitvector_concat(v, v).bits, i + (i << (SIZE as usize)));
        }
    }

    #[test]
    fn test_get_bit() {
        const SIZE: i128 = 10;

        for i in 0..(1 << (SIZE as usize)) {
            let v = BitStatic::<SIZE>::new(SIZE, i);
            for idx in 0..(SIZE as usize) {
                assert_eq!((i & (1 << idx)) > 0, v.get_bit(idx as i128))
            }
        }
    }

    #[test]
    fn test_set_bit() {
        const SIZE: i128 = 60;

        let mut v = BitStatic::<SIZE>::new(SIZE, 0);
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
        assert_eq!(BitStatic::<1>::new(1, 0b0).signed(), 0);
        assert_eq!(BitStatic::<1>::new(1, 0b1).signed(), -1);

        // Test 2-bit signed values
        assert_eq!(BitStatic::<2>::new(2, 0b00).signed(), 0);
        assert_eq!(BitStatic::<2>::new(2, 0b01).signed(), 1);
        assert_eq!(BitStatic::<2>::new(2, 0b10).signed(), -2);
        assert_eq!(BitStatic::<2>::new(2, 0b11).signed(), -1);

        // Test 3-bit signed values
        assert_eq!(BitStatic::<3>::new(3, 0b000).signed(), 0);
        assert_eq!(BitStatic::<3>::new(3, 0b001).signed(), 1);
        assert_eq!(BitStatic::<3>::new(3, 0b010).signed(), 2);
        assert_eq!(BitStatic::<3>::new(3, 0b011).signed(), 3);
        assert_eq!(BitStatic::<3>::new(3, 0b100).signed(), -4);
        assert_eq!(BitStatic::<3>::new(3, 0b101).signed(), -3);
        assert_eq!(BitStatic::<3>::new(3, 0b110).signed(), -2);
        assert_eq!(BitStatic::<3>::new(3, 0b111).signed(), -1);

        // Test 4-bit signed values
        assert_eq!(BitStatic::<4>::new(4, 0b0000).signed(), 0);
        assert_eq!(BitStatic::<4>::new(4, 0b0001).signed(), 1);
        assert_eq!(BitStatic::<4>::new(4, 0b0111).signed(), 7);
        assert_eq!(BitStatic::<4>::new(4, 0b1000).signed(), -8);
        assert_eq!(BitStatic::<4>::new(4, 0b1001).signed(), -7);
        assert_eq!(BitStatic::<4>::new(4, 0b1111).signed(), -1);

        // Test 8-bit signed values
        assert_eq!(BitStatic::<8>::new(8, 0x00).signed(), 0);
        assert_eq!(BitStatic::<8>::new(8, 0x01).signed(), 1);
        assert_eq!(BitStatic::<8>::new(8, 0x7F).signed(), 127);
        assert_eq!(BitStatic::<8>::new(8, 0x80).signed(), -128);
        assert_eq!(BitStatic::<8>::new(8, 0xFF).signed(), -1);

        // Test 16-bit signed values
        assert_eq!(BitStatic::<16>::new(16, 0x0000).signed(), 0);
        assert_eq!(BitStatic::<16>::new(16, 0x0001).signed(), 1);
        assert_eq!(BitStatic::<16>::new(16, 0x7FFF).signed(), 32767);
        assert_eq!(BitStatic::<16>::new(16, 0x8000).signed(), -32768);
        assert_eq!(BitStatic::<16>::new(16, 0xFFFF).signed(), -1);

        // Test 32-bit signed values
        assert_eq!(BitStatic::<32>::new(32, 0x00000000).signed(), 0);
        assert_eq!(BitStatic::<32>::new(32, 0x00000001).signed(), 1);
        assert_eq!(BitStatic::<32>::new(32, 0x7FFFFFFF).signed(), 2147483647);
        assert_eq!(BitStatic::<32>::new(32, 0x80000000).signed(), -2147483648);
        assert_eq!(BitStatic::<32>::new(32, 0xFFFFFFFF).signed(), -1);

        // Test 64-bit signed values
        assert_eq!(BitStatic::<32>::new(64, 0x0000000000000000).signed(), 0);
        assert_eq!(BitStatic::<32>::new(64, 0x0000000000000001).signed(), 1);
        assert_eq!(BitStatic::<32>::new(64, 0x7FFFFFFFFFFFFFFF).signed(), 9223372036854775807);
        assert_eq!(BitStatic::<32>::new(64, 0x8000000000000000).signed(), -9223372036854775808);
        assert_eq!(BitStatic::<32>::new(64, 0xFFFFFFFFFFFFFFFF).signed(), -1);
    }

    #[test]
    fn test_signed_vs_unsigned() {
        // Test that unsigned and signed give different results for negative values
        let v = BitStatic::<8>::new(8, 0xFF);
        assert_eq!(v.unsigned(), 255);
        assert_eq!(v.signed(), -1);

        let v = BitStatic::<8>::new(8, 0x80);
        assert_eq!(v.unsigned(), 128);
        assert_eq!(v.signed(), -128);

        let v = BitStatic::<16>::new(16, 0x8000);
        assert_eq!(v.unsigned(), 32768);
        assert_eq!(v.signed(), -32768);

        // Test that unsigned and signed give same results for positive values
        let v = BitStatic::<8>::new(8, 0x7F);
        assert_eq!(v.unsigned(), 127);
        assert_eq!(v.signed(), 127);

        let v = BitStatic::<8>::new(8, 0x00);
        assert_eq!(v.unsigned(), 0);
        assert_eq!(v.signed(), 0);
    }

    #[test]
    fn test_sign_extend() {
        // Test sign extending positive values from 4 to 8 bits
        let input = BitStatic::<4>::new(4, 0b0111); // 7 in 4 bits
        let result = sail_sign_extend(input, 8);
        assert_eq!(result.unsigned(), 0b00000111); // Should remain 7 in 8 bits

        // Test sign extending negative values from 4 to 8 bits
        let input = BitStatic::<4>::new(4, 0b1000); // -8 in 4 bits (two's complement)
        let result = sail_sign_extend(input, 8);
        assert_eq!(result.unsigned(), 0b11111000); // Should become -8 in 8 bits

        let input = BitStatic::<4>::new(4, 0b1111); // -1 in 4 bits
        let result = sail_sign_extend(input, 8);
        assert_eq!(result.unsigned(), 0b11111111); // Should become -1 in 8 bits

        // Test sign extending from 8 to 16 bits
        let input = BitStatic::<8>::new(8, 0x7F); // 127 in 8 bits (positive)
        let result = sail_sign_extend(input, 16);
        assert_eq!(result.unsigned(), 0x007F); // Should remain 127 in 16 bits

        let input = BitStatic::<8>::new(8, 0x80); // -128 in 8 bits (negative)
        let result = sail_sign_extend(input, 16);
        assert_eq!(result.unsigned(), 0xFF80); // Should become -128 in 16 bits

        let input = BitStatic::<8>::new(8, 0xFF); // -1 in 8 bits
        let result = sail_sign_extend(input, 16);
        assert_eq!(result.unsigned(), 0xFFFF); // Should become -1 in 16 bits

        // Test sign extending from 16 to 32 bits
        let input = bv(16, 0x7FFF); // 32767 in 16 bits (positive)
        let result = sail_sign_extend(input, 32);
        assert_eq!(result.unsigned(), 0x00007FFF); // Should remain 32767 in 32 bits

        let input = bv(16, 0x8000); // -32768 in 16 bits (negative)
        let result = sail_sign_extend(input, 32);
        assert_eq!(result.unsigned(), 0xFFFF8000); // Should become -32768 in 32 bits

        // Test sign extending from 32 to 64 bits
        let input = bv(32, 0x7FFFFFFF); // Positive value
        let result = sail_sign_extend(input, 64);
        assert_eq!(result.unsigned(), 0x000000007FFFFFFF);

        let input = bv(32, 0x80000000); // Negative value
        let result = sail_sign_extend(input, 64);
        assert_eq!(result.unsigned(), 0xFFFFFFFF80000000);

        // Test edge cases - extending by 1 bit
        let input = BitStatic::<1>::new(1, 0b0); // 0 in 1 bit
        let result = sail_sign_extend(input, 2);
        assert_eq!(result.unsigned(), 0b00); // Should remain 0

        let input = BitStatic::<1>::new(1, 0b1); // -1 in 1 bit
        let result = sail_sign_extend(input, 2);
        assert_eq!(result.unsigned(), 0b11); // Should become -1 in 2 bits

        // Test extending smaller values
        let input = BitStatic::<3>::new(3, 0b101); // -3 in 3 bits
        let result = sail_sign_extend(input, 8);
        assert_eq!(result.unsigned(), 0b11111101); // Should become -3 in 8 bits

        let input = BitStatic::<3>::new(3, 0b011); // 3 in 3 bits
        let result = sail_sign_extend(input, 8);
        assert_eq!(result.unsigned(), 0b00000011); // Should remain 3 in 8 bits

        // Test extending from 64 to 64 bits (no-op, but widely used)
        let input = BitStatic::<64>::new(64, 0x7FFFFFFFFFFFFFFF); // Maximum positive 64-bit value
        let result = sail_sign_extend(input, 64);
        assert_eq!(result.unsigned(), 0x7FFFFFFFFFFFFFFF); // Should remain unchanged

        let input = BitStatic::<64>::new(64, 0x8000000000000000); // Minimum negative 64-bit value
        let result = sail_sign_extend(input, 64);
        assert_eq!(result.unsigned(), 0x8000000000000000); // Should remain unchanged

        let input = BitStatic::<64>::new(64, 0xFFFFFFFFFFFFFFFF); // -1 in 64 bits
        let result = sail_sign_extend(input, 64);
        assert_eq!(result.unsigned(), 0xFFFFFFFFFFFFFFFF); // Should remain unchanged

        let input = BitStatic::<64>::new(64, 0x0000000000000000); // 0 in 64 bits
        let result = sail_sign_extend(input, 64);
        assert_eq!(result.unsigned(), 0x0000000000000000); // Should remain unchanged
    }
}

// TODO(Gurvan): BitVector specific tests
