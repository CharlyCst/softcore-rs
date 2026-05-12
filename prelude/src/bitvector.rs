use core::ops::*;

// Utils -------------------------------------------------------------------------------------------

const fn assert_eq_range<const START: i128, const END: i128, const LEN: i128>() {
    assert!(END - START == LEN);
}
const fn assert_eq_sum<const X: i128, const Y: i128, const Z: i128>() {
    assert!(X + Y == Z);
}

const fn assert_leq<const X: i128, const Y: i128>() {
    assert!(X <= Y);
}

// BitDynamic (BitVector with dynamically known size) ----------------------------------------------

const BITDYNAMIC_SIZE: usize = 8;
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitDynamic {
    pub len: i128,
    // Little-Endian
    pub bits: [u64; BITDYNAMIC_SIZE],
}

impl BitDynamic {
    pub const fn len(self) -> i128 {
        self.len
    }

    pub const fn signed(self) -> i128 {
        assert!(self.len <= 128);
        let val = self.unsigned();
        let sign_bit_mask = 1i128 << (self.len - 1);
        if (val & sign_bit_mask) == 0 {
            val
        } else {
            // Sign extend: fill bits from self.len up to 128 with 1s
            let fill_mask = !((1i128 << self.len) - 1);
            val | fill_mask
        }
    }

    pub const fn unsigned(self) -> i128 {
        assert!(self.len <= 128);
        ((self.bits[0] as u128) | ((self.bits[1] as u128) << 64)) as i128
    }

    pub const fn new(len: i128, val: u64) -> Self {
        assert!(len < (BITDYNAMIC_SIZE * 64) as i128);
        let mut bits = [0; BITDYNAMIC_SIZE];
        bits[0] = val;
        Self { len, bits }
    }

    pub const fn zeros(len: i128) -> Self {
        Self::new(len, 0)
    }

    pub const fn ones(len: i128) -> Self {
        Self {
            len: len,
            bits: Self::bit_mask(len),
        }
    }

    pub const fn set_bit(mut self, idx: i128, value: bool) -> Self {
        assert!(0 <= idx && idx < self.len);
        let limb = (idx / 64) as usize;
        let bit = (idx % 64) as u32;
        if value {
            self.bits[limb] |= 1 << bit;
        } else {
            self.bits[limb] &= !(1 << bit);
        }
        self
    }

    pub const fn get_bit(self, idx: i128) -> bool {
        assert!(0 <= idx && idx < self.len);
        let limb = (idx / 64) as usize;
        let bit = (idx % 64) as u32;
        (self.bits[limb] & (1 << bit)) != 0
    }

    pub const fn bitand(mut self, rhs: Self) -> Self {
        let mut i: usize = 0;
        while i < BITDYNAMIC_SIZE {
            self.bits[i] = self.bits[i] & rhs.bits[i];
            i += 1;
        }
        self
    }

    pub const fn bitor(mut self, rhs: Self) -> Self {
        let mut i: usize = 0;
        while i < BITDYNAMIC_SIZE {
            self.bits[i] = self.bits[i] | rhs.bits[i];
            i += 1;
        }
        self
    }

    pub const fn bitxor(mut self, rhs: Self) -> Self {
        let mut i: usize = 0;
        while i < BITDYNAMIC_SIZE {
            self.bits[i] = self.bits[i] ^ rhs.bits[i];
            i += 1;
        }
        self
    }

    pub const fn not(mut self) -> Self {
        let bitmask = Self::bit_mask(self.len);
        let mut i: usize = 0;
        while i < BITDYNAMIC_SIZE {
            self.bits[i] = !self.bits[i] & bitmask[i];
            i += 1;
        }
        self
    }

    pub const fn add(self, rhs: u64) -> Self {
        self.wrapped_add(Self::new(self.len(), rhs))
    }

    pub const fn wrapped_add(mut self, rhs: Self) -> Self {
        assert!(self.len == rhs.len);
        let mut carry: u128 = 0;
        let mask = Self::bit_mask(self.len);
        let mut i: usize = 0;
        while i < BITDYNAMIC_SIZE {
            let sum = (self.bits[i] as u128) + (rhs.bits[i] as u128) + carry;
            self.bits[i] = (sum as u64) & mask[i];
            carry = sum >> 64;
            i += 1;
        }
        self
    }

    pub const fn shl(self, rhs: u128) -> Self {
        let mut res = Self::zeros(self.len);
        let shift_limbs = (rhs / 64) as usize;
        let shift_bits = (rhs % 64) as u32;
        let mask = Self::bit_mask(self.len);

        // TODO: We could assert the following as this might be undefined behavior? Check sail spec
        if shift_limbs >= BITDYNAMIC_SIZE {
            return res;
        }

        let mut i = BITDYNAMIC_SIZE - 1;
        while i >= shift_limbs {
            let src_idx = i - shift_limbs;
            let mut val = self.bits[src_idx] << shift_bits;
            if shift_bits > 0 && src_idx > 0 {
                val |= self.bits[src_idx - 1] >> (64 - shift_bits);
            }
            res.bits[i] = val & mask[i];

            if i == 0 {
                break;
            }
            i -= 1;
        }
        res
    }

    pub const fn shr(self, rhs: u128) -> Self {
        let mut res = Self::zeros(self.len);
        let shift_limbs = (rhs / 64) as usize;
        let shift_bits = (rhs % 64) as u32;
        let mask = Self::bit_mask(self.len);

        let mut i = 0;
        while i + shift_limbs < BITDYNAMIC_SIZE {
            let src_idx = i + shift_limbs;
            res.bits[i] = self.bits[src_idx] >> shift_bits;
            if shift_bits > 0 && src_idx + 1 < BITDYNAMIC_SIZE {
                res.bits[i] |= self.bits[src_idx + 1] << (64 - shift_bits);
            }
            res.bits[i] &= mask[i];
            i += 1;
        }
        res
    }

    pub const fn concat(self, other: BitDynamic) -> BitDynamic {
        let new_len = self.len + other.len;
        let res = other.zero_extend_dyn(new_len);
        let shifted_self = self.zero_extend_dyn(new_len).shl(other.len as u128);
        res.bitor(shifted_self)
    }

    pub const fn get_subrange(self, end: i128, start: i128) -> BitDynamic {
        assert!(0 <= start && start <= end && end <= self.len);
        let len = end - start;
        let shifted = self.shr(start as u128);
        let mask = Self::bit_mask(len);
        let mut res = BitDynamic::zeros(len);
        let mut i = 0;
        while i < BITDYNAMIC_SIZE {
            res.bits[i] = shifted.bits[i] & mask[i];
            i += 1;
        }
        res
    }

    pub const fn sign_extend<const RESULT_LEN: i128>(self) -> BitStatic<RESULT_LEN> {
        assert!(self.len <= RESULT_LEN);
        let val = self.signed();
        BitStatic { bits: val as u64 }
    }

    pub const fn sign_extend_dyn(self, len: i128) -> BitDynamic {
        assert!(self.len <= len);
        if self.len == 0 {
            return BitDynamic::zeros(len);
        }
        let is_negative = self.get_bit(self.len - 1);
        let mut res = self.zero_extend_dyn(len);
        if is_negative {
            let full_mask = Self::bit_mask(len);
            let existing_mask = Self::bit_mask(self.len);
            let mut i = 0;
            while i < BITDYNAMIC_SIZE {
                let extension_bits = full_mask[i] & !existing_mask[i];
                res.bits[i] |= extension_bits;
                i += 1;
            }
        }
        res
    }

    pub const fn zero_extend<const RESULT_LEN: i128>(self) -> BitStatic<RESULT_LEN> {
        assert!(self.len <= RESULT_LEN);
        BitStatic { bits: self.bits[0] }
    }

    pub const fn zero_extend_dyn(self, len: i128) -> BitDynamic {
        assert!(self.len <= len);
        let mut res = self;
        res.len = len; // Bits outside original len are already 0
        res
    }

    // TODO: Move to i128
    pub const fn set_subrange(mut self, bits: BitDynamic, to: u64, from: u64) -> Self {
        let to = to as i128;
        let from = from as i128;
        assert!(0 <= from && from <= to && to < self.len);
        let len = to - from + 1;
        assert!(bits.len == len);
        let range_mask = BitDynamic {
            len: self.len,
            bits: Self::bit_mask(len),
        }
        .shl(from as u128);

        let mut i = 0;
        while i < BITDYNAMIC_SIZE {
            self.bits[i] &= !range_mask.bits[i];
            i += 1;
        }

        let shifted_bits = bits.zero_extend_dyn(self.len).shl(from as u128);
        self.bitor(shifted_bits)
    }

    pub const fn subrange<const START: i128, const END: i128, const LEN: i128>(self) -> BitDynamic {
        assert_eq_range::<START, END, LEN>();
        self.get_subrange(END, START)
    }

    pub const fn bit_mask(len: i128) -> [u64; BITDYNAMIC_SIZE] {
        let mut mask = [0u64; BITDYNAMIC_SIZE];
        let mut i = 0;
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

    pub fn to_raw_le(self) -> Vec<u8> {
        assert!(self.len % 8 == 0, "Length must be byte-aligned");
        let num_bytes = (self.len / 8) as usize;
        let mut raw_bytes = Vec::with_capacity(num_bytes);

        raw_bytes.extend(
            self.bits
                .iter()
                .flat_map(|limb| limb.to_le_bytes())
                .take(num_bytes),
        );

        raw_bytes
    }
}

impl PartialOrd for BitDynamic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl<const LEN: i128> From<BitStatic<LEN>> for BitDynamic {
    fn from(bv: BitStatic<LEN>) -> Self {
        let mut bits = [0u64; BITDYNAMIC_SIZE];
        bits[0] = bv.bits;
        Self { len: LEN, bits }
    }
}

pub const fn bvd(len: i128, val: u64) -> BitDynamic {
    BitDynamic::new(len, val)
}

// BitStatic (BitVector with statically known small [<= 64] size) ---------------------------------

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitStatic<const LEN: i128> {
    /// Little endian
    pub bits: u64,
}

impl<const LEN: i128> BitStatic<LEN> {
    pub const fn len(self) -> i128 {
        LEN
    }

    pub const fn signed(self) -> i128 {
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

    pub const fn unsigned(self) -> i128 {
        self.bits as i128
    }

    pub const fn new(val: u64) -> Self {
        let _ = Self::ASSERT_LEN_VALID;
        Self {
            bits: val & Self::BIT_MASK,
        }
    }

    pub const fn zeros() -> Self {
        Self::new(0)
    }

    pub const fn ones() -> Self {
        Self::new(Self::BIT_MASK)
    }

    pub const fn set_bit(mut self, idx: i128, value: bool) -> Self {
        assert!(idx < LEN, "Out of bounds array check");
        if value {
            self.bits |= 1u64 << idx
        } else {
            self.bits &= !(1u64 << idx)
        }
        self
    }

    pub const fn get_bit(self, idx: i128) -> bool {
        assert!(idx < LEN);
        self.bits & (1 << idx) > 0
    }

    pub const fn bitand(mut self, rhs: Self) -> Self {
        self.bits &= rhs.bits;
        self
    }

    pub const fn bitor(mut self, rhs: Self) -> Self {
        self.bits |= rhs.bits;
        self
    }

    pub const fn bitxor(mut self, rhs: Self) -> Self {
        self.bits ^= rhs.bits;
        self
    }

    pub const fn not(self) -> Self {
        Self {
            bits: !self.bits & Self::BIT_MASK,
        }
    }

    pub const fn add(self, rhs: u64) -> Self {
        self.wrapped_add(Self { bits: rhs })
    }

    pub const fn wrapped_add(self, rhs: Self) -> Self {
        Self {
            bits: (self.bits as u64).wrapping_add(rhs.bits),
        }
    }

    pub const fn shl(self, rhs: u128) -> Self {
        Self {
            bits: self.bits << rhs,
        }
    }

    pub const fn shr(self, rhs: u128) -> Self {
        Self {
            bits: self.bits >> rhs,
        }
    }

    pub const ASSERT_LEN_VALID: () = assert!(
        0 <= LEN && LEN <= 64,
        "Length of BitStatic must be less than 64"
    );

    pub const BIT_MASK: u64 = if LEN == 64 { u64::MAX } else { (1 << LEN) - 1 };

    pub const fn to_dynamic(self) -> BitDynamic {
        BitDynamic::new(LEN, self.bits)
    }

    pub const fn sign_extend<const RESULT_LEN: i128>(self) -> BitStatic<RESULT_LEN> {
        assert_leq::<LEN, RESULT_LEN>();
        BitStatic::new(self.signed() as u64)
    }

    pub const fn sign_extend_dyn(self, _len: i128) -> BitDynamic {
        todo!()
    }

    pub const fn zero_extend<const RESULT_LEN: i128>(self) -> BitStatic<RESULT_LEN> {
        assert_leq::<LEN, RESULT_LEN>();
        BitStatic::<RESULT_LEN>::new(self.bits)
    }

    pub const fn zero_extend_dyn(self, len: i128) -> BitDynamic {
        assert!(LEN < len);
        BitDynamic::new(len, self.bits)
    }

    pub const fn subrange<const START: i128, const END: i128, const RESULT_LEN: i128>(
        self,
    ) -> BitStatic<RESULT_LEN> {
        assert_eq_range::<START, END, RESULT_LEN>();
        BitStatic::new(self.shr(START as u128).bits)
    }

    pub const fn concat<const LEN2: i128, const LEN3: i128>(
        self,
        other: BitStatic<LEN2>,
    ) -> BitStatic<LEN3> {
        assert_eq_sum::<LEN, LEN2, LEN3>();
        self.zero_extend::<LEN3>()
            .shl(LEN2 as u128)
            .bitor(other.zero_extend())
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

pub const fn bv<const LEN: i128>(val: u64) -> BitStatic<LEN> {
    BitStatic::new(val)
}

// Implement core::ops for BitStatic and BitDynamic ------------------------------------------------

macro_rules! impl_ops_for_storage {
    ($target:ty, $($header:tt)*) => {
        impl $($header)* BitAnd for $target {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self {
                <$target>::bitand(self, rhs)
            }
        }
        impl $($header)* BitOr for $target {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self {
                <$target>::bitor(self, rhs)
            }
        }
        impl $($header)* BitXor for $target {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self {
                <$target>::bitxor(self, rhs)
            }
        }
        impl $($header)* Not for $target {
            type Output = Self;
            fn not(self) -> Self {
                <$target>::not(self)
            }
        }
        impl $($header)* Add<u64> for $target {
            type Output = Self;
            fn add(self, rhs: u64) -> Self {
                <$target>::add(self, rhs)
            }
        }
        impl $($header)* Shl<u128> for $target {
            type Output = Self;
            fn shl(self, rhs: u128) -> Self {
                <$target>::shl(self, rhs)
            }
        }
        impl $($header)* Shl<i128> for $target {
            type Output = Self;
            fn shl(self, rhs: i128) -> Self {
                <$target>::shl(self, rhs as u128)
            }
        }
        impl $($header)* Shl<i32> for $target {
            type Output = Self;
            fn shl(self, rhs: i32) -> Self {
                <$target>::shl(self, rhs as u128)
            }
        }
        impl $($header)* Shr<u128> for $target {
            type Output = Self;
            fn shr(self, rhs: u128) -> Self {
                <$target>::shr(self, rhs)
            }
        }
        impl $($header)* Shr<i128> for $target {
            type Output = Self;
            fn shr(self, rhs: i128) -> Self {
                <$target>::shr(self, rhs as u128)
            }
        }
        impl $($header)* Shr<i32> for $target {
            type Output = Self;
            fn shr(self, rhs: i32) -> Self {
                <$target>::shr(self, rhs as u128)
            }
        }
    };
}

impl_ops_for_storage!(BitDynamic,);
impl_ops_for_storage!(BitStatic<LEN>, <const LEN: i128>);

// Tests -------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests_bitstatic {
    use super::*;

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
        assert_eq!((!BitStatic::<1>::new(0b1)).unsigned(), 0b0);
        assert_eq!((!BitStatic::<1>::new(0b0)).unsigned(), 0b1);
        assert_eq!((!BitStatic::<2>::new(0b01)).unsigned(), 0b10);
        assert_eq!((!BitStatic::<2>::new(0b11)).unsigned(), 0b00);
    }

    // #[test]
    // fn subrange_bitvector() {
    //     let v = BitStatic::<32>::new(32, 0b10110111);

    //     assert_eq!(v.subrange::<0, 1, 1>().bits(), 0b1);
    //     assert_eq!(v.subrange::<0, 2, 2>().bits(), 0b11);
    //     assert_eq!(v.subrange::<0, 3, 3>().bits(), 0b111);
    //     assert_eq!(v.subrange::<0, 4, 4>().bits(), 0b0111);
    //     assert_eq!(v.subrange::<0, 5, 5>().bits(), 0b10111);

    //     assert_eq!(v.subrange::<2, 3, 1>().bits(), 0b1);
    //     assert_eq!(v.subrange::<2, 4, 2>().bits(), 0b01);
    //     assert_eq!(v.subrange::<2, 5, 3>().bits(), 0b101);
    //     assert_eq!(v.subrange::<2, 6, 4>().bits(), 0b1101);
    //     assert_eq!(v.subrange::<2, 7, 5>().bits(), 0b01101);

    //     assert_eq!(bv(32, 0xffffffff).subrange::<7, 23, 16>().bits(), 0xffff);
    //     assert_eq!(v.subrange::<2, 7, 5>().bits(), 0b01101);

    //     let v = bv(32, 0b10110111);
    //     assert_eq!(v.set_subrange::<0, 1, 1>(BitStatic::<1>::new(1, 0b0)).bits(), 0b10110110);
    //     assert_eq!(v.set_subrange::<0, 1, 1>(BitStatic::<1>::new(1, 0b1)).bits(), 0b10110111);
    //     assert_eq!(v.set_subrange::<0, 2, 2>(BitStatic::<2>::new(2, 0b00)).bits(), 0b10110100);
    //     assert_eq!(v.set_subrange::<2, 5, 3>(BitStatic::<3>::new(3, 0b010)).bits(), 0b10101011);

    //     assert_eq!(
    //         bv(64, 0x0000000000000000).subrange::<60, 64, 4>().bits(),
    //         0x0
    //     );
    //     assert_eq!(
    //         bv(64, 0xa000000000000000).subrange::<60, 64, 4>().bits(),
    //         0xa
    //     );
    //     assert_eq!(
    //         bv(64, 0xb000000000000000).subrange::<60, 64, 4>().bits(),
    //         0xb
    //     );
    //     assert_eq!(
    //         bv(64, 0xc000000000000000).subrange::<60, 64, 4>().bits(),
    //         0xc
    //     );
    //     assert_eq!(
    //         bv(64, 0xd000000000000000).subrange::<60, 64, 4>().bits(),
    //         0xd
    //     );
    //     assert_eq!(
    //         bv(64, 0xe000000000000000).subrange::<60, 64, 4>().bits(),
    //         0xe
    //     );
    //     assert_eq!(
    //         bv(64, 0xf000000000000000).subrange::<60, 64, 4>().bits(),
    //         0xf
    //     );
    // }

    // #[test]
    // fn test_update_subrange_bits() {
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b11111100), 1, 0, BitStatic::<2>::new(2, 0b11)).bits,
    //         0b11111111
    //     );
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 0, 0, BitStatic::<1>::new(1, 0b1)).bits,
    //         0b00000001
    //     );
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 1, 1, BitStatic::<1>::new(1, 0b1)).bits,
    //         0b00000010
    //     );
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 2, 2, BitStatic::<1>::new(1, 0b1)).bits,
    //         0b00000100
    //     );
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 3, 3, BitStatic::<1>::new(1, 0b1)).bits,
    //         0b00001000
    //     );
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 4, 4, BitStatic::<1>::new(1, 0b1)).bits,
    //         0b00010000
    //     );
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 5, 5, BitStatic::<1>::new(1, 0b1)).bits,
    //         0b00100000
    //     );
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 6, 6, BitStatic::<1>::new(1, 0b1)).bits,
    //         0b01000000
    //     );
    //     assert_eq!(
    //         update_subrange_bits(BitStatic::<8>::new(8, 0b00000000), 7, 7, BitStatic::<1>::new(1, 0b1)).bits,
    //         0b10000000
    //     );
    // }

    #[test]
    fn bitwise_operators() {
        let v = BitStatic::<32>::new(0b1);

        assert_eq!(v, v | v);
        assert_eq!(v, v & v);
        assert_eq!(v, v ^ v ^ v);
        assert_eq!(v, !!v);

        for i in 0u128..30u128 {
            assert_eq!(v, (v << i) >> i);
        }
    }

    #[test]
    fn test_zero_extend() {
        const SIZE: i128 = 8;
        let v = BitStatic::<SIZE>::new(0b1010);

        let vs1: BitStatic<16> = v.zero_extend();
        let vs2: BitStatic<63> = v.zero_extend();
        let vs3: BitStatic<64> = v.zero_extend();

        assert_eq!(vs1.unsigned(), v.unsigned());
        assert_eq!(vs2.unsigned(), v.unsigned());
        assert_eq!(vs3.unsigned(), v.unsigned());
    }

    #[test]
    fn test_concat() {
        const SIZE: i128 = 20;

        for i in 0..(1 << (SIZE as usize)) {
            let v = BitStatic::<SIZE>::new(i);
            let res: BitStatic<{ SIZE + SIZE }> = v.concat(v);
            assert_eq!(res.bits, i + (i << (SIZE as usize)));
        }
    }

    #[test]
    fn test_get_bit() {
        const SIZE: i128 = 10;

        for i in 0..(1 << (SIZE as usize)) {
            let v = BitStatic::<SIZE>::new(i);
            for idx in 0..(SIZE as usize) {
                assert_eq!((i & (1 << idx)) > 0, v.get_bit(idx as i128))
            }
        }
    }

    #[test]
    fn test_set_bit() {
        const SIZE: i128 = 60;

        let mut v = BitStatic::<SIZE>::new(0);
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
        assert_eq!(BitStatic::<1>::new(0b0).signed(), 0);
        assert_eq!(BitStatic::<1>::new(0b1).signed(), -1);

        // Test 2-bit signed values
        assert_eq!(BitStatic::<2>::new(0b00).signed(), 0);
        assert_eq!(BitStatic::<2>::new(0b01).signed(), 1);
        assert_eq!(BitStatic::<2>::new(0b10).signed(), -2);
        assert_eq!(BitStatic::<2>::new(0b11).signed(), -1);

        // Test 3-bit signed values
        assert_eq!(BitStatic::<3>::new(0b000).signed(), 0);
        assert_eq!(BitStatic::<3>::new(0b001).signed(), 1);
        assert_eq!(BitStatic::<3>::new(0b010).signed(), 2);
        assert_eq!(BitStatic::<3>::new(0b011).signed(), 3);
        assert_eq!(BitStatic::<3>::new(0b100).signed(), -4);
        assert_eq!(BitStatic::<3>::new(0b101).signed(), -3);
        assert_eq!(BitStatic::<3>::new(0b110).signed(), -2);
        assert_eq!(BitStatic::<3>::new(0b111).signed(), -1);

        // Test 4-bit signed values
        assert_eq!(BitStatic::<4>::new(0b0000).signed(), 0);
        assert_eq!(BitStatic::<4>::new(0b0001).signed(), 1);
        assert_eq!(BitStatic::<4>::new(0b0111).signed(), 7);
        assert_eq!(BitStatic::<4>::new(0b1000).signed(), -8);
        assert_eq!(BitStatic::<4>::new(0b1001).signed(), -7);
        assert_eq!(BitStatic::<4>::new(0b1111).signed(), -1);

        // Test 8-bit signed values
        assert_eq!(BitStatic::<8>::new(0x00).signed(), 0);
        assert_eq!(BitStatic::<8>::new(0x01).signed(), 1);
        assert_eq!(BitStatic::<8>::new(0x7F).signed(), 127);
        assert_eq!(BitStatic::<8>::new(0x80).signed(), -128);
        assert_eq!(BitStatic::<8>::new(0xFF).signed(), -1);

        // Test 16-bit signed values
        assert_eq!(BitStatic::<16>::new(0x0000).signed(), 0);
        assert_eq!(BitStatic::<16>::new(0x0001).signed(), 1);
        assert_eq!(BitStatic::<16>::new(0x7FFF).signed(), 32767);
        assert_eq!(BitStatic::<16>::new(0x8000).signed(), -32768);
        assert_eq!(BitStatic::<16>::new(0xFFFF).signed(), -1);

        // Test 32-bit signed values
        assert_eq!(BitStatic::<32>::new(0x00000000).signed(), 0);
        assert_eq!(BitStatic::<32>::new(0x00000001).signed(), 1);
        assert_eq!(BitStatic::<32>::new(0x7FFFFFFF).signed(), 2147483647);
        assert_eq!(BitStatic::<32>::new(0x80000000).signed(), -2147483648);
        assert_eq!(BitStatic::<32>::new(0xFFFFFFFF).signed(), -1);

        // Test 64-bit signed values
        assert_eq!(BitStatic::<64>::new(0x0000000000000000).signed(), 0);
        assert_eq!(BitStatic::<64>::new(0x0000000000000001).signed(), 1);
        assert_eq!(
            BitStatic::<64>::new(0x7FFFFFFFFFFFFFFF).signed(),
            9223372036854775807
        );
        assert_eq!(
            BitStatic::<64>::new(0x8000000000000000).signed(),
            -9223372036854775808
        );
        assert_eq!(BitStatic::<64>::new(0xFFFFFFFFFFFFFFFF).signed(), -1);
    }

    #[test]
    fn test_signed_vs_unsigned() {
        // Test that unsigned and signed give different results for negative values
        let v = BitStatic::<8>::new(0xFF);
        assert_eq!(v.unsigned(), 255);
        assert_eq!(v.signed(), -1);

        let v = BitStatic::<8>::new(0x80);
        assert_eq!(v.unsigned(), 128);
        assert_eq!(v.signed(), -128);

        let v = BitStatic::<16>::new(0x8000);
        assert_eq!(v.unsigned(), 32768);
        assert_eq!(v.signed(), -32768);

        // Test that unsigned and signed give same results for positive values
        let v = BitStatic::<8>::new(0x7F);
        assert_eq!(v.unsigned(), 127);
        assert_eq!(v.signed(), 127);

        let v = BitStatic::<8>::new(0x00);
        assert_eq!(v.unsigned(), 0);
        assert_eq!(v.signed(), 0);
    }

    #[test]
    fn test_sign_extend() {
        // Test sign extending positive values from 4 to 8 bits
        let input = BitStatic::<4>::new(0b0111); // 7 in 4 bits
        let result: BitStatic<8> = input.sign_extend();
        assert_eq!(result.unsigned(), 0b00000111); // Should remain 7 in 8 bits

        // Test sign extending negative values from 4 to 8 bits
        let input = BitStatic::<4>::new(0b1000); // -8 in 4 bits (two's complement)
        let result: BitStatic<8> = input.sign_extend();
        assert_eq!(result.unsigned(), 0b11111000); // Should become -8 in 8 bits

        let input = BitStatic::<4>::new(0b1111); // -1 in 4 bits
        let result: BitStatic<8> = input.sign_extend();
        assert_eq!(result.unsigned(), 0b11111111); // Should become -1 in 8 bits

        // Test sign extending from 8 to 16 bits
        let input = BitStatic::<8>::new(0x7F); // 127 in 8 bits (positive)
        let result: BitStatic<16> = input.sign_extend();
        assert_eq!(result.unsigned(), 0x007F); // Should remain 127 in 16 bits

        let input = BitStatic::<8>::new(0x80); // -128 in 8 bits (negative)
        let result: BitStatic<16> = input.sign_extend();
        assert_eq!(result.unsigned(), 0xFF80); // Should become -128 in 16 bits

        let input = BitStatic::<8>::new(0xFF); // -1 in 8 bits
        let result: BitStatic<16> = input.sign_extend();
        assert_eq!(result.unsigned(), 0xFFFF); // Should become -1 in 16 bits

        // Test sign extending from 16 to 32 bits
        let input = BitStatic::<16>::new(0x7FFF); // 32767 in 16 bits (positive)
        let result: BitStatic<32> = input.sign_extend();
        assert_eq!(result.unsigned(), 0x00007FFF); // Should remain 32767 in 32 bits

        let input = BitStatic::<16>::new(0x8000); // -32768 in 16 bits (negative)
        let result: BitStatic<32> = input.sign_extend();
        assert_eq!(result.unsigned(), 0xFFFF8000); // Should become -32768 in 32 bits

        // Test sign extending from 32 to 64 bits
        let input = BitStatic::<32>::new(0x7FFFFFFF); // Positive value
        let result: BitStatic<64> = input.sign_extend();
        assert_eq!(result.unsigned(), 0x000000007FFFFFFF);

        let input = BitStatic::<32>::new(0x80000000); // Negative value
        let result: BitStatic<64> = input.sign_extend();
        assert_eq!(result.unsigned(), 0xFFFFFFFF80000000);

        // Test edge cases - extending by 1 bit
        let input = BitStatic::<1>::new(0b0); // 0 in 1 bit
        let result: BitStatic<2> = input.sign_extend();
        assert_eq!(result.unsigned(), 0b00); // Should remain 0

        let input = BitStatic::<1>::new(0b1); // -1 in 1 bit
        let result: BitStatic<2> = input.sign_extend();
        assert_eq!(result.unsigned(), 0b11); // Should become -1 in 2 bits

        // Test extending smaller values
        let input = BitStatic::<3>::new(0b101); // -3 in 3 bits
        let result: BitStatic<8> = input.sign_extend();
        assert_eq!(result.unsigned(), 0b11111101); // Should become -3 in 8 bits

        let input = BitStatic::<3>::new(0b011); // 3 in 3 bits
        let result: BitStatic<8> = input.sign_extend();
        assert_eq!(result.unsigned(), 0b00000011); // Should remain 3 in 8 bits

        // Test extending from 64 to 64 bits (no-op, but widely used)
        let input = BitStatic::<64>::new(0x7FFFFFFFFFFFFFFF); // Maximum positive 64-bit value
        let result: BitStatic<64> = input.sign_extend();
        assert_eq!(result.unsigned(), 0x7FFFFFFFFFFFFFFF); // Should remain unchanged

        let input = BitStatic::<64>::new(0x8000000000000000); // Minimum negative 64-bit value
        let result: BitStatic<64> = input.sign_extend();
        assert_eq!(result.unsigned(), 0x8000000000000000); // Should remain unchanged

        let input = BitStatic::<64>::new(0xFFFFFFFFFFFFFFFF); // -1 in 64 bits
        let result: BitStatic<64> = input.sign_extend();
        assert_eq!(result.unsigned(), 0xFFFFFFFFFFFFFFFF); // Should remain unchanged

        let input = BitStatic::<64>::new(0x0000000000000000); // 0 in 64 bits
        let result: BitStatic<64> = input.sign_extend();
        assert_eq!(result.unsigned(), 0x0000000000000000); // Should remain unchanged
    }
}

#[cfg(test)]
mod tests_bitdynamic {
    use super::*;

    #[test]
    fn bitvec_masks() {
        // TODO(Gurvan): More tests
        assert_eq!(BitDynamic::bit_mask(0)[0], 0b0);
        assert_eq!(BitDynamic::bit_mask(1)[0], 0b1);
        assert_eq!(BitDynamic::bit_mask(2)[0], 0b11);
        assert_eq!(BitDynamic::bit_mask(8)[0], 0b11111111);
        assert_eq!(BitDynamic::bit_mask(64)[0], 0xffffffffffffffff);
    }

    #[test]
    fn bitvec_not() {
        assert_eq!((!BitDynamic::new(1, 0b1)).unsigned(), 0b0);
        assert_eq!((!BitDynamic::new(1, 0b0)).unsigned(), 0b1);
        assert_eq!((!BitDynamic::new(2, 0b01)).unsigned(), 0b10);
        assert_eq!((!BitDynamic::new(2, 0b11)).unsigned(), 0b00);
    }

    #[test]
    fn subrange_bitvector() {
        // TODO(Gurvan): More tests
        let v = BitDynamic::new(32, 0b10110111);

        assert_eq!(v.subrange::<0, 1, 1>().unsigned(), 0b1);
        assert_eq!(v.subrange::<0, 2, 2>().unsigned(), 0b11);
        assert_eq!(v.subrange::<0, 3, 3>().unsigned(), 0b111);
        assert_eq!(v.subrange::<0, 4, 4>().unsigned(), 0b0111);
        assert_eq!(v.subrange::<0, 5, 5>().unsigned(), 0b10111);

        assert_eq!(v.subrange::<2, 3, 1>().unsigned(), 0b1);
        assert_eq!(v.subrange::<2, 4, 2>().unsigned(), 0b01);
        assert_eq!(v.subrange::<2, 5, 3>().unsigned(), 0b101);
        assert_eq!(v.subrange::<2, 6, 4>().unsigned(), 0b1101);
        assert_eq!(v.subrange::<2, 7, 5>().unsigned(), 0b01101);

        assert_eq!(
            bvd(32, 0xffffffff).subrange::<7, 23, 16>().unsigned(),
            0xffff
        );
        assert_eq!(v.subrange::<2, 7, 5>().unsigned(), 0b01101);

        let v = bvd(32, 0b10110111);
        assert_eq!(
            v.set_subrange(BitDynamic::new(1, 0b0), 0, 0).unsigned(),
            0b10110110
        );
        assert_eq!(
            v.set_subrange(BitDynamic::new(1, 0b1), 0, 0).unsigned(),
            0b10110111
        );
        assert_eq!(
            v.set_subrange(BitDynamic::new(2, 0b00), 1, 0).unsigned(),
            0b10110100
        );
        assert_eq!(
            v.set_subrange(BitDynamic::new(3, 0b010), 4, 2).unsigned(),
            0b10101011
        );

        assert_eq!(
            bvd(64, 0x0000000000000000)
                .subrange::<60, 64, 4>()
                .unsigned(),
            0x0
        );
        assert_eq!(
            bvd(64, 0xa000000000000000)
                .subrange::<60, 64, 4>()
                .unsigned(),
            0xa
        );
        assert_eq!(
            bvd(64, 0xb000000000000000)
                .subrange::<60, 64, 4>()
                .unsigned(),
            0xb
        );
        assert_eq!(
            bvd(64, 0xc000000000000000)
                .subrange::<60, 64, 4>()
                .unsigned(),
            0xc
        );
        assert_eq!(
            bvd(64, 0xd000000000000000)
                .subrange::<60, 64, 4>()
                .unsigned(),
            0xd
        );
        assert_eq!(
            bvd(64, 0xe000000000000000)
                .subrange::<60, 64, 4>()
                .unsigned(),
            0xe
        );
        assert_eq!(
            bvd(64, 0xf000000000000000)
                .subrange::<60, 64, 4>()
                .unsigned(),
            0xf
        );
    }

    #[test]
    fn bitwise_operators() {
        let v = BitDynamic::new(32, 0b1);

        assert_eq!(v, v | v);
        assert_eq!(v, v & v);
        assert_eq!(v, v ^ v ^ v);
        assert_eq!(v, !!v);

        for i in 0u128..30u128 {
            assert_eq!(v, (v << i) >> i);
        }
    }

    #[test]
    fn test_zero_extend() {
        const SIZE: i128 = 8;
        let v = BitStatic::<SIZE>::new(0b1010);

        let vs1: BitStatic<16> = v.zero_extend();
        let vs2: BitStatic<63> = v.zero_extend();
        let vs3: BitStatic<64> = v.zero_extend();

        assert_eq!(vs1.unsigned(), v.unsigned());
        assert_eq!(vs2.unsigned(), v.unsigned());
        assert_eq!(vs3.unsigned(), v.unsigned());
    }

    #[test]
    fn test_zero_extend_dyn() {
        const SIZE: i128 = 8;
        let v = BitStatic::<SIZE>::new(0b1010);

        let vs1: BitDynamic = v.zero_extend_dyn(16);
        let vs2: BitDynamic = v.zero_extend_dyn(63);
        let vs3: BitDynamic = v.zero_extend_dyn(64);

        assert_eq!(vs1.unsigned(), v.unsigned());
        assert_eq!(vs2.unsigned(), v.unsigned());
        assert_eq!(vs3.unsigned(), v.unsigned());
    }

    #[test]
    fn test_concat() {
        const SIZE: i128 = 20;

        for i in 0..(1 << (SIZE as usize)) {
            let v = BitDynamic::new(SIZE, i);
            let res: BitDynamic = v.concat(v);
            assert_eq!(res.unsigned(), (i + (i << (SIZE as usize))) as i128);
        }
    }

    #[test]
    fn test_get_bit() {
        const SIZE: i128 = 10;

        for i in 0..(1 << (SIZE as usize)) {
            let v = BitStatic::<SIZE>::new(i);
            for idx in 0..(SIZE as usize) {
                assert_eq!((i & (1 << idx)) > 0, v.get_bit(idx as i128))
            }
        }
    }

    #[test]
    fn test_set_bit() {
        const SIZE: i128 = 60;

        let mut v = BitStatic::<SIZE>::new(0);
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
        assert_eq!(BitDynamic::new(1, 0b0).signed(), 0);
        assert_eq!(BitDynamic::new(1, 0b1).signed(), -1);

        // Test 2-bit signed values
        assert_eq!(BitDynamic::new(2, 0b00).signed(), 0);
        assert_eq!(BitDynamic::new(2, 0b01).signed(), 1);
        assert_eq!(BitDynamic::new(2, 0b10).signed(), -2);
        assert_eq!(BitDynamic::new(2, 0b11).signed(), -1);

        // Test 3-bit signed values
        assert_eq!(BitDynamic::new(3, 0b000).signed(), 0);
        assert_eq!(BitDynamic::new(3, 0b001).signed(), 1);
        assert_eq!(BitDynamic::new(3, 0b010).signed(), 2);
        assert_eq!(BitDynamic::new(3, 0b011).signed(), 3);
        assert_eq!(BitDynamic::new(3, 0b100).signed(), -4);
        assert_eq!(BitDynamic::new(3, 0b101).signed(), -3);
        assert_eq!(BitDynamic::new(3, 0b110).signed(), -2);
        assert_eq!(BitDynamic::new(3, 0b111).signed(), -1);

        // Test 4-bit signed values
        assert_eq!(BitDynamic::new(4, 0b0000).signed(), 0);
        assert_eq!(BitDynamic::new(4, 0b0001).signed(), 1);
        assert_eq!(BitDynamic::new(4, 0b0111).signed(), 7);
        assert_eq!(BitDynamic::new(4, 0b1000).signed(), -8);
        assert_eq!(BitDynamic::new(4, 0b1001).signed(), -7);
        assert_eq!(BitDynamic::new(4, 0b1111).signed(), -1);

        // Test 8-bit signed values
        assert_eq!(BitDynamic::new(8, 0x00).signed(), 0);
        assert_eq!(BitDynamic::new(8, 0x01).signed(), 1);
        assert_eq!(BitDynamic::new(8, 0x7F).signed(), 127);
        assert_eq!(BitDynamic::new(8, 0x80).signed(), -128);
        assert_eq!(BitDynamic::new(8, 0xFF).signed(), -1);

        // Test 16-bit signed values
        assert_eq!(BitDynamic::new(16, 0x0000).signed(), 0);
        assert_eq!(BitDynamic::new(16, 0x0001).signed(), 1);
        assert_eq!(BitDynamic::new(16, 0x7FFF).signed(), 32767);
        assert_eq!(BitDynamic::new(16, 0x8000).signed(), -32768);
        assert_eq!(BitDynamic::new(16, 0xFFFF).signed(), -1);

        // Test 32-bit signed values
        assert_eq!(BitDynamic::new(32, 0x00000000).signed(), 0);
        assert_eq!(BitDynamic::new(32, 0x00000001).signed(), 1);
        assert_eq!(BitDynamic::new(32, 0x7FFFFFFF).signed(), 2147483647);
        assert_eq!(BitDynamic::new(32, 0x80000000).signed(), -2147483648);
        assert_eq!(BitDynamic::new(32, 0xFFFFFFFF).signed(), -1);

        // Test 64-bit signed values
        assert_eq!(BitDynamic::new(64, 0x0000000000000000).signed(), 0);
        assert_eq!(BitDynamic::new(64, 0x0000000000000001).signed(), 1);
        assert_eq!(
            BitDynamic::new(64, 0x7FFFFFFFFFFFFFFF).signed(),
            9223372036854775807
        );
        assert_eq!(
            BitDynamic::new(64, 0x8000000000000000).signed(),
            -9223372036854775808
        );
        assert_eq!(BitDynamic::new(64, 0xFFFFFFFFFFFFFFFF).signed(), -1);
    }

    #[test]
    fn test_signed_vs_unsigned() {
        // Test that unsigned and signed give different results for negative values
        let v = BitDynamic::new(8, 0xFF);
        assert_eq!(v.unsigned(), 255);
        assert_eq!(v.signed(), -1);

        let v = BitDynamic::new(8, 0x80);
        assert_eq!(v.unsigned(), 128);
        assert_eq!(v.signed(), -128);

        let v = BitDynamic::new(16, 0x8000);
        assert_eq!(v.unsigned(), 32768);
        assert_eq!(v.signed(), -32768);

        // Test that unsigned and signed give same results for positive values
        let v = BitDynamic::new(8, 0x7F);
        assert_eq!(v.unsigned(), 127);
        assert_eq!(v.signed(), 127);

        let v = BitDynamic::new(8, 0x00);
        assert_eq!(v.unsigned(), 0);
        assert_eq!(v.signed(), 0);
    }

    #[test]
    fn test_sign_extend_dyn() {
        // Test sign extending positive values from 4 to 8 bits
        let input = BitDynamic::new(4, 0b0111); // 7 in 4 bits
        let result: BitDynamic = input.sign_extend_dyn(8);
        assert_eq!(result.unsigned(), 0b00000111); // Should remain 7 in 8 bits

        // Test sign extending negative values from 4 to 8 bits
        let input = BitDynamic::new(4, 0b1000); // -8 in 4 bits (two's complement)
        let result: BitDynamic = input.sign_extend_dyn(8);
        assert_eq!(result.unsigned(), 0b11111000); // Should become -8 in 8 bits

        let input = BitDynamic::new(4, 0b1111); // -1 in 4 bits
        let result: BitDynamic = input.sign_extend_dyn(8);
        assert_eq!(result.unsigned(), 0b11111111); // Should become -1 in 8 bits

        // Test sign extending from 8 to 16 bits
        let input = BitDynamic::new(8, 0x7F); // 127 in 8 bits (positive)
        let result: BitDynamic = input.sign_extend_dyn(16);
        assert_eq!(result.unsigned(), 0x007F); // Should remain 127 in 16 bits

        let input = BitDynamic::new(8, 0x80); // -128 in 8 bits (negative)
        let result: BitDynamic = input.sign_extend_dyn(16);
        assert_eq!(result.unsigned(), 0xFF80); // Should become -128 in 16 bits

        let input = BitDynamic::new(8, 0xFF); // -1 in 8 bits
        let result: BitDynamic = input.sign_extend_dyn(16);
        assert_eq!(result.unsigned(), 0xFFFF); // Should become -1 in 16 bits

        // Test sign extending from 16 to 32 bits
        let input = BitDynamic::new(16, 0x7FFF); // 32767 in 16 bits (positive)
        let result: BitDynamic = input.sign_extend_dyn(32);
        assert_eq!(result.unsigned(), 0x00007FFF); // Should remain 32767 in 32 bits

        let input = BitDynamic::new(16, 0x8000); // -32768 in 16 bits (negative)
        let result: BitDynamic = input.sign_extend_dyn(32);
        assert_eq!(result.unsigned(), 0xFFFF8000); // Should become -32768 in 32 bits

        // Test sign extending from 32 to 64 bits
        let input = BitDynamic::new(32, 0x7FFFFFFF); // Positive value
        let result: BitDynamic = input.sign_extend_dyn(64);
        assert_eq!(result.unsigned(), 0x000000007FFFFFFF);

        let input = BitDynamic::new(32, 0x80000000); // Negative value
        let result: BitDynamic = input.sign_extend_dyn(64);
        assert_eq!(result.unsigned(), 0xFFFFFFFF80000000);

        // Test edge cases - extending by 1 bit
        let input = BitDynamic::new(1, 0b0); // 0 in 1 bit
        let result: BitDynamic = input.sign_extend_dyn(2);
        assert_eq!(result.unsigned(), 0b00); // Should remain 0

        let input = BitDynamic::new(1, 0b1); // -1 in 1 bit
        let result: BitDynamic = input.sign_extend_dyn(2);
        assert_eq!(result.unsigned(), 0b11); // Should become -1 in 2 bits

        // Test extending smaller values
        let input = BitDynamic::new(3, 0b101); // -3 in 3 bits
        let result: BitDynamic = input.sign_extend_dyn(8);
        assert_eq!(result.unsigned(), 0b11111101); // Should become -3 in 8 bits

        let input = BitDynamic::new(3, 0b011); // 3 in 3 bits
        let result: BitDynamic = input.sign_extend_dyn(8);
        assert_eq!(result.unsigned(), 0b00000011); // Should remain 3 in 8 bits

        // Test extending from 64 to 64 bits (no-op, but widely used)
        let input = BitDynamic::new(64, 0x7FFFFFFFFFFFFFFF); // Maximum positive 64-bit value
        let result: BitDynamic = input.sign_extend_dyn(64);
        assert_eq!(result.unsigned(), 0x7FFFFFFFFFFFFFFF); // Should remain unchanged

        let input = BitDynamic::new(64, 0x8000000000000000); // Minimum negative 64-bit value
        let result: BitDynamic = input.sign_extend_dyn(64);
        assert_eq!(result.unsigned(), 0x8000000000000000); // Should remain unchanged

        let input = BitDynamic::new(64, 0xFFFFFFFFFFFFFFFF); // -1 in 64 bits
        let result: BitDynamic = input.sign_extend_dyn(64);
        assert_eq!(result.unsigned(), 0xFFFFFFFFFFFFFFFF); // Should remain unchanged

        let input = BitDynamic::new(64, 0x0000000000000000); // 0 in 64 bits
        let result: BitDynamic = input.sign_extend_dyn(64);
        assert_eq!(result.unsigned(), 0x0000000000000000); // Should remain unchanged
    }

    #[test]
    fn test_to_raw_le() {
        for i in 0..=0xFF {
            let raw_le = BitDynamic::new(8, i).to_raw_le();
            assert_eq!(raw_le[0] as u64, i);
        }

        for i in 0..=0xFFFF {
            let raw_le = BitDynamic::new(16, i).to_raw_le();
            let low = raw_le[0] as u64;
            let high = raw_le[1] as u64;
            assert_eq!(low + (high << 8), i);
        }
    }

}
