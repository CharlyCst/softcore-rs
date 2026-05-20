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

    pub fn push(&mut self, item: T) {
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

    pub fn take(mut self, vl: usize) -> Self {
        self.len = std::cmp::min(self.len, vl);
        self
    }

    pub fn truncate(&mut self, vl: usize) {
        self.len = std::cmp::min(self.len, vl);
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
        assert!(v.len() < BOUND, "BoundedVec capacity exceeded");
        let mut vec = [T::default(); BOUND];
        for i in 0..v.len() {
            vec[i] = v[i];
        }
        Self {
            len: vec.len(),
            vec: vec,
        }
    }
}

// TODO(Gurvan): Test functions
