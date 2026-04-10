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

pub fn bitvector_concat(e1: BitVector, e2: BitVector) -> BitVector {
    bv(e1.len + e2.len, (e1.bits() << e2.len) | e2.bits())
}

pub const fn get_slice_int<BS: BitStorage>(l: i128, n: i128, start: i128) -> BitVector<BS> {
    let val = (n >> start) & (mask128(l as usize) as i128);
    bv(l, val as u64)
}

/* TODO(Gurvan): This type signature is wrong */
pub const fn slice<BS: BitStorage>(bits: BitVector<BS>, start: i128, len: i128) -> BitVector<BS> {
    let mask = mask(len as usize);
    bv(len, (bits.bits() >> start) & mask)
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

pub fn sail_sign_extend<BS: BitStorage>(input: BitVector<BS>, n: i128) -> BitVector<BS> {
    assert!(n >= input.len(), "Cannot sign extend to smaller size");
    assert!(n <= 64, "Maximum supported size is 64 for now");

    // Special case: when extending from same size to same size, it's a no-op
    if input.len() == n {
        return input
    }

    // Check if the sign bit (MSB) is set
    let sign_bit = (input.bits() >> (input.len() - 1)) & 1;

    if sign_bit == 0 {
        // Positive number - just zero extend
        BitVector::new(n, input.bits())
    } else {
        // Negative number - fill upper bits with 1s
        // Handle the case where M=64 to avoid shift overflow
        let mask = if input.len() == 64 {
            0u64
        } else {
            (1u64 << input.len()) - 1
        };
        let extension_bits = !mask & if n == 64 { u64::MAX } else { (1u64 << n) - 1 };
        BitVector::new(n, input.bits() | extension_bits)
    }
}

pub fn sail_ones<BS: BitStorage>(len: i128) -> BitVector<BS> {
    BitVector::ones(len)
}

pub fn sail_zeros<BS:BitStorage>(len: i128) -> BitVector<BS> {
    BitVector::zeros(len)
}

pub fn sail_shiftright<BS: BitStorage>(bits: BitVector<BS>, shift: i128) -> BitVector<BS> {
    bits.shr(shift)
}

pub fn sail_shiftleft<BS: BitStorage>(bits: BitVector<BS>, shift: i128) -> BitVector<BS> {
    bits.shl(shift)
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

pub fn subrange_bits<BS: BitStorage>(vec: BitVector<BS>, end: i128, start: i128) -> BitVector<BS> {
    let out = end - start + 1;
    bv(out, (vec.bits >> start) & mask(out as usize))
}

pub fn update_subrange_bits<BS: BitStorage>(bits: BitVector<BS>, to: u64, from: u64, value: BitVector<BS>) -> BitVector<BS> {
    assert!(to - from + 1 == value.len as u64, "size don't match");

    // Generate the 111111 mask
    let mut mask = (1 << value.len) - 1;
    // Shit and invert it
    mask = !(mask << from);

    // Now we can update and return the updated value
    bv(bits.len, (bits.bits & mask) | (value.bits() << from))
}

pub fn bitvector_update<BS: BitStorage>(v: BitVector<BS>, pos: i128, value: bool) -> BitVector<BS> {
    v.set_bit(pos, value)
}

pub fn undefined_bitvector<BS: BitStorage>(len: i128) -> BitVector<BS> {
    BitVector::zeros(len)
}

/* TODO(Gurvan): Maybe the following should take i128 as a parameter */

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
