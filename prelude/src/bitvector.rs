use core::ops;

/* BitStorage ------------------------------------------------------------------------- */

pub trait BitStorage: Eq + PartialEq + Clone + Copy + std::fmt::Debug + Default {
    fn len(&self) -> i128;
    fn as_i128(&self) -> i128;
    fn new(len: i128, val: u64) -> Self;
}

/* Dynamic BitVectors ----------------------------------------------------------------- */

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitDynamic {
    pub len: i128,
    pub bits: u64, /* TODO: Move to arbitrary length here, array of bytes? */
}

impl BitStorage for BitDynamic {
    fn len(&self) -> i128 { self.len }
    fn as_i128(&self) -> i128 {
        self.bits as i128
    }

    fn new(len: i128, val: u64) -> Self {
        if len < 64 {
            Self { len: len, bits: val & ((1 << len) - 1) }
        } else {
            Self { len: len, bits: val }
        }
    }

}

impl BitDynamic {
    fn to_static<const LEN: i128>(&self) -> BitStatic<LEN> {
        assert!(self.len <= LEN);
        BitStatic {
            bits: self.bits,
        }
    }
}

/* Static BitVectors ------------------------------------------------------------------ */

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitStatic<const N: i128> {
    pub bits: u64,
}

impl<const LEN: i128> BitStorage for BitStatic<LEN> {
    fn len(&self) -> i128 { LEN }
    fn as_i128(&self) -> i128 { self.bits as i128 }
    fn new(len: i128, val: u64) -> Self {
        assert!(len == LEN);
        if LEN < 64 {
            Self { bits: val & ((1 << LEN) - 1) }
        } else {
            Self { bits: val }
        }
    }
}

impl <const LEN: i128> BitStatic<LEN> {
    fn to_dynamic(&self) -> BitDynamic {
        BitDynamic {
            len: LEN,
            bits: self.bits,
        }
    }
}

/* General BitVectors ------------------------------------------------------------------ */

#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct BitVector<S: BitStorage> {
    pub storage: S,
}

impl <BS: BitStorage> BitVector<BS> {
    pub fn new(len: i128, val: u64) -> Self {
        Self { storage: BS::new(len, val) }
    }

    pub fn new_empty(len: i128) -> Self {
        Self { storage: BS::new(len, 0) }
    }

    pub fn len(self) -> i128 {
        self.storage.len()
    }

    pub fn bits(self) -> u64 {
        self.storage.as_i128().try_into().unwrap()
    }

    pub fn as_i128(self) -> i128 {
        self.storage.as_i128()
    }

    pub fn update(self, pos: i128, value: bool) -> Self {
        let mask = 1 << pos;
        Self::new(self.len(), (self.bits() & !mask) | (value as u64) << pos)
    }
}

pub fn bv<const LEN: i128>(val: u64) -> BitVector<BitStatic<LEN>> {
    BitVector::new(LEN, val)
}
