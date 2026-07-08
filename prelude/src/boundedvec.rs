use crate::{BitDynamic, BitStatic};
use std::ops::{Deref, DerefMut};
use std::slice::SliceIndex;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct BoundedVec<T, const BOUND: usize> {
    pub len: usize,
    pub vec: [T; BOUND],
}

impl<T: Default + Copy, const BOUND: usize> BoundedVec<T, BOUND> {
    pub fn new() -> Self {
        Self {
            len: 0,
            vec: [T::default(); BOUND],
        }
    }

    pub const fn push(&mut self, item: T) {
        assert!(self.len < BOUND, "BoundedVec capacity exceeded");

        self.vec[self.len] = item;
        self.len += 1;
    }

    pub fn as_slice(&self) -> &[T] {
        &self.vec[..self.len as usize]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.vec[..self.len as usize]
    }

    pub fn slice<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[T]>,
    {
        &self.as_slice()[index]
    }

    pub const fn take(mut self, vl: usize) -> Self {
        self.len = if self.len < vl { self.len } else { vl };
        self
    }

    pub const fn truncate(&mut self, vl: usize) {
        self.len = if self.len < vl { self.len } else { vl };
    }
}

impl<T: Default, const BOUND: usize> Deref for BoundedVec<T, BOUND> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        // Cast i128 to usize safely for Rust array indexing
        &self.vec[..self.len as usize]
    }
}

impl<T: Default, const BOUND: usize> DerefMut for BoundedVec<T, BOUND> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.vec[..self.len as usize]
    }
}

impl<T: Default + Copy, const BOUND: usize> From<Vec<T>> for BoundedVec<T, BOUND> {
    fn from(v: Vec<T>) -> Self {
        assert!(v.len() <= BOUND, "BoundedVec capacity exceeded");
        let mut vec = [T::default(); BOUND];
        for i in 0..v.len() {
            vec[i] = v[i];
        }
        Self {
            len: v.len(),
            vec: vec,
        }
    }
}

// TODO(Gurvan): From slice functions

impl<const LEN: i128, const BOUND: usize> From<BoundedVec<BitDynamic, BOUND>>
    for BoundedVec<BitStatic<LEN>, BOUND>
{
    fn from(dynamic_bvec: BoundedVec<BitDynamic, BOUND>) -> Self {
        let mut static_vec = [BitStatic::<LEN>::zeros(); BOUND];

        let mut i = 0;
        while i < dynamic_bvec.len {
            static_vec[i] = BitStatic::<LEN>::from(dynamic_bvec.vec[i]);
            i += 1;
        }

        Self {
            len: dynamic_bvec.len,
            vec: static_vec,
        }
    }
}
impl<const LEN: i128, const BOUND: usize> From<BoundedVec<BitStatic<LEN>, BOUND>>
    for BoundedVec<BitDynamic, BOUND>
{
    fn from(static_bvec: BoundedVec<BitStatic<LEN>, BOUND>) -> Self {
        let mut dynamic_vec = [BitDynamic::default(); BOUND];

        let mut i = 0;
        while i < static_bvec.len {
            dynamic_vec[i] = BitDynamic::from(static_bvec.vec[i]);
            i += 1;
        }

        Self {
            len: static_bvec.len,
            vec: dynamic_vec,
        }
    }
}

// TODO(Gurvan): Test functions
