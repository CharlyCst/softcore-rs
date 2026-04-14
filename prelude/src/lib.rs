#![allow(incomplete_features, non_camel_case_types)]

use core::ops::*;
use std::cmp::max;
use std::cmp::min;

mod bitvector;
pub use bitvector::*;

// NOTE: Ideally we would use unbounded integers for natural numbers. Yet in practice this would
// mess up with things such as the SMT solver during symbolic execution.
// After manual inspection, u128 are big enough for all the RISC-V use cases, so we keep that until
// a better solution is needed.
pub type nat = u128;

pub fn sail_branch_announce<BS: BitStorage>(_value: i128, _pc: BitVector<BS>) {}

pub fn lteq_int(e1: i128, e2: i128) -> bool {
    e1 <= e2
}

pub fn gt_int(e1: i128, e2: i128) -> bool {
    e1 > e2
}

pub fn bitvector_length<BS: BitStorage>(e: BitVector<BS>) -> i128 {
    e.len()
}

pub fn parse_hex_bits<BS: BitStorage>(_n: i128, _hex_str: &str) -> BitVector<BS> {
    todo!("'parse_hex_bits' is not yet implemented");
}

pub fn bitvector_concat<BS3: BitStorage, BS2: BitStorage, BS1: BitStorageConcat<BS2, BS3>>(e1: BitVector<BS1>, e2: BitVector<BS2>) -> BitVector<BS3> {
    e1.concat(e2)
}

pub fn get_slice_int(l: i128, n: i128, start: i128) -> BitVector<BitDynamic> {
    let val = (n >> start) & (mask128(l as usize) as i128);
    // TODO: This cast in annoying, bv::new should take a u128 / i128…
    BitVector::<BitDynamic>::new(l, val as u64)
}

pub fn slice<BS2: BitStorage, BS1: BitStorageExtend<BS2>>(bits: BitVector<BS1>, start: i128, len: i128) -> BitVector<BS2> {
    bits.get_subrange(start + len, start)
}

pub fn get_16_random_bits<BS: BitStorage>(_unit: ()) -> BitVector<BS> {
    BitVector::new(16, 0)
}

pub fn not_implemented<T>(_any: T) -> ! {
    panic!("Feature not implemented yet");
}

pub fn internal_error(_file: String, _line: i128, _s: String) -> ! {
    panic!("Softcore: internal error")
}

pub fn print_output<BS: BitStorage>(text: String, _csr: BitVector<BS>) {
    println!("{}", text)
}

pub fn print_platform(text: String) {
    println!("{}", text)
}

pub fn bits_str<BS: BitStorage>(val: BitVector<BS>) -> String {
    format!("{:b}", val.unsigned())
}

pub fn bitvector_access<BS: BitStorage>(vec: BitVector<BS>, idx: i128) -> bool {
    vec.get_bit(idx)
}

// Todo: implement truncate for other sizes if required
pub fn truncate<BS: BitStorage>(v: BitVector<BS>, size: i128) -> BitVector<BS> {
    assert!(size == 64);
    v
}

pub fn sail_sign_extend<BS2: BitStorage, BS1: BitStorageExtend<BS2>>(input: BitVector<BS1>, n: i128) -> BitVector<BS2> {
    input.sign_extend(n)
}

pub fn sail_ones<BS: BitStorage>(len: i128) -> BitVector<BS> {
    BitVector::ones(len)
}

pub fn sail_zeros<BS:BitStorage>(len: i128) -> BitVector<BS> {
    BitVector::zeros(len)
}

pub fn sail_shiftright<BS: BitStorage>(bits: BitVector<BS>, shift: i128) -> BitVector<BS> {
    bits >> shift
}

pub fn sail_shiftleft<BS: BitStorage>(bits: BitVector<BS>, shift: i128) -> BitVector<BS> {
    bits << shift
}

pub fn max_int(v1: i128, v2: i128) -> i128 {
    max(v1, v2)
}

pub fn min_int(v1: i128, v2: i128) -> i128 {
    min(v1, v2)
}

pub fn cancel_reservation(_unit: ()) {
    // In the future, extend this function
}

fn hex_bits<BS: BitStorage>(len: i128, bits: &str) -> BitVector<BS> {
    let parsed = bits.parse::<u64>().expect("Could not parse hex bits");
    BitVector::new(len, parsed)
}

pub fn hex_bits_12_forwards<BS: BitStorage>(_reg: BitVector<BS>) -> ! {
    todo!("Implement this function")
}

pub fn hex_bits_12_backwards<BS: BitStorage>(bits: &str) -> BitVector<BS> {
    hex_bits(12, bits)
}

pub fn hex_bits_12_backwards_matches(bits: &str) -> bool {
    match bits.parse::<u64>() {
        Ok(n) => n < (1 << 12),
        Err(_) => false,
    }
}

pub fn subrange_bits<BS2: BitStorage, BS1: BitStorageExtend<BS2>>(vec: BitVector<BS1>, end: i128, start: i128) -> BitVector<BS2> {
    vec.get_subrange(end, start)
}

pub fn update_subrange_bits<BS2: BitStorage, BS1: BitStorageExtend<BS2>>(bits: BitVector<BS2>, to: u64, from: u64, vec: BitVector<BS1>) -> BitVector<BS1> {
    vec.set_subrange(bits, to, from)
}

pub fn bitvector_update<BS: BitStorage>(v: BitVector<BS>, pos: i128, value: bool) -> BitVector<BS> {
    v.set_bit(pos, value)
}

pub fn undefined_bitvector<BS: BitStorage>(len: i128) -> BitVector<BS> {
    BitVector::zeros(len)
}

// TODO(Gurvan): Maybe the following should take i128 as a parameter

pub fn undefined_array<T: Copy, const N: usize>(v: T) -> [T; N] {
    [v; N]
}

pub fn undefined_vector<T: Copy>(n: usize, v: T) -> Vec<T> {
    vec![v; n]
}

// ———————————————————————————————— Helpers ————————————————————————————————— //

const fn mask(nb_ones: usize) -> u64 {
    assert!(nb_ones <= 64, "Unsupported mask size");
    if nb_ones == 64 {
        u64::MAX
    } else {
        (1 << nb_ones) - 1
    }
}

const fn mask128(nb_ones: usize) -> u128 {
    assert!(nb_ones <= 128, "Unsupported mask size");
    if nb_ones == 128 {
        u128::MAX
    } else {
        (1 << nb_ones) - 1
    }
}
