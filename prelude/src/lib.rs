#![allow(incomplete_features, non_camel_case_types)]
#![feature(never_type)]

use std::cmp::max;
use std::cmp::min;

mod bitvector;
mod boundedvec;
pub use bitvector::*;
pub use boundedvec::*;

// NOTE: Ideally we would use unbounded integers for natural numbers. Yet in practice this would
// mess up with things such as the SMT solver during symbolic execution.
// After manual inspection, u128 are big enough for all the RISC-V use cases, so we keep that until
// a better solution is needed.
pub type nat = i128;

pub fn sail_branch_announce(_value: i128, _pc: BitDynamic) {}

pub const fn lteq_int(e1: i128, e2: i128) -> bool {
    e1 <= e2
}

pub const fn gt_int(e1: i128, e2: i128) -> bool {
    e1 > e2
}

pub const fn bitvector_length(e: BitDynamic) -> i128 {
    e.len()
}

pub fn parse_hex_bits(_n: i128, _hex_str: &str) -> BitDynamic {
    todo!("'parse_hex_bits' is not yet implemented");
}

pub fn bitvector_concat<B1, B2>(e1: B1, e2: B2) -> BitDynamic
where
    B1: Into<BitDynamic>,
    B2: Into<BitDynamic>,
{
    let e1_dyn: BitDynamic = e1.into();
    let e2_dyn: BitDynamic = e2.into();
    e1_dyn.concat(e2_dyn)
}

pub fn get_slice_int(l: i128, n: i128, start: i128) -> BitDynamic {
    let val = (n >> start) & (mask128(l as usize) as i128);
    // TODO(Gurvan): This cast is annoying as it can fail, S::new should take a u128 / i128…
    BitDynamic::new(l, val as u64)
}

pub fn slice(bits: BitDynamic, start: i128, len: i128) -> BitDynamic {
    bits.get_subrange(start + len, start)
}

pub fn get_16_random_bits(_unit: ()) -> BitDynamic {
    BitDynamic::new(16, 0)
}

pub fn not_implemented<T>(_any: T) -> ! {
    panic!("Feature not implemented yet");
}

pub fn internal_error(_file: String, _line: i128, _s: String) -> ! {
    panic!("Softcore: internal error")
}

pub fn print_output(text: String, _csr: BitDynamic) {
    println!("{}", text)
}

pub fn print_platform(text: String) {
    println!("{}", text)
}

pub fn bits_str(val: BitDynamic) -> String {
    format!("{:b}", val.unsigned())
}

pub fn bitvector_access(vec: BitDynamic, idx: i128) -> bool {
    vec.get_bit(idx)
}

// Todo: implement truncate for other sizes if required
pub fn truncate(v: BitDynamic, size: i128) -> BitDynamic {
    assert!(size == 64);
    v
}

pub const fn sail_sign_extend(input: BitDynamic, n: i128) -> BitDynamic {
    input.sign_extend_dyn(n)
}

pub const fn sail_ones(len: i128) -> BitDynamic {
    BitDynamic::ones(len)
}

pub const fn sail_zeros(len: i128) -> BitDynamic {
    BitDynamic::zeros(len)
}

pub const fn sail_shiftright(bits: BitDynamic, shift: i128) -> BitDynamic {
    bits.shr(shift as u128)
}

pub const fn sail_shiftleft(bits: BitDynamic, shift: i128) -> BitDynamic {
    bits.shl(shift as u128)
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

fn hex_bits<const LEN: i128>(bits: &str) -> BitStatic<LEN> {
    let parsed = bits.parse::<u64>().expect("Could not parse hex bits");
    BitStatic::new(parsed)
}

pub fn hex_bits_12_forwards(_reg: BitStatic<12>) -> ! {
    todo!("Implement this function")
}

pub fn hex_bits_12_backwards(bits: &str) -> BitStatic<12> {
    hex_bits::<12>(bits)
}

pub fn hex_bits_12_backwards_matches(bits: &str) -> bool {
    match bits.parse::<u64>() {
        Ok(n) => n < (1 << 12),
        Err(_) => false,
    }
}

pub const fn subrange_bits(vec: BitDynamic, end: i128, start: i128) -> BitDynamic {
    vec.get_subrange(end + 1, start)
}

pub const fn update_subrange_bits(
    vec: BitDynamic,
    to: u64,
    from: u64,
    bits: BitDynamic,
) -> BitDynamic {
    vec.set_subrange(bits, to, from)
}

pub const fn bitvector_update(v: BitDynamic, pos: i128, value: bool) -> BitDynamic {
    v.set_bit(pos, value)
}

pub const fn undefined_bitvector(len: i128) -> BitDynamic {
    BitDynamic::zeros(len)
}

// TODO(Gurvan): Maybe the following should take i128 as a parameter

pub const fn undefined_array<T: Copy, const N: usize>(v: T) -> [T; N] {
    [v; N]
}

pub fn undefined_vector<T: Default + Copy, const BOUND: usize>(
    n: i128,
    v: T,
) -> BoundedVec<T, BOUND> {
    vec![v; n as usize].into()
}

pub const fn vector_length<T>(v: &[T]) -> i128 {
    v.len() as i128
}

// ———————————————————————————————— Helpers ————————————————————————————————— //

const fn mask128(nb_ones: usize) -> u128 {
    assert!(nb_ones <= 128, "Unsupported mask size");
    if nb_ones == 128 {
        u128::MAX
    } else {
        (1 << nb_ones) - 1
    }
}

// TODO(Gurvan): Comment out?
pub fn opt_into<T, V>(opt: Option<T>) -> Option<V>
where
    V: From<T>,
{
    opt.map(V::from)
}

pub fn opt_into_dyn<T>(opt: Option<T>) -> Option<BitDynamic>
where
    T: Into<BitDynamic>,
{
    opt.map(T::into)
}

pub fn opt_into_static<T, const LEN: i128>(opt: Option<T>) -> Option<BitStatic<LEN>>
where
    T: Into<BitStatic<LEN>>,
{
    opt.map(T::into)
}

// TODO: boundedvec_into_static, boundedvec_into_dyn, array_into_static, array_into_dyn
pub fn boundedvec_into_static<T, const LEN: i128, const BOUND: usize>(
    bv: BoundedVec<T, BOUND>,
) -> BoundedVec<BitStatic<LEN>, BOUND>
where
    T: Into<BitStatic<LEN>>,
{
    todo!()
}

pub fn boundedvec_into_dyn<T, const BOUND: usize>(
    bv: BoundedVec<T, BOUND>,
) -> BoundedVec<BitDynamic, BOUND>
where
    T: Into<BitDynamic>,
{
    todo!()
}

pub fn array_into_static<T, const LEN: i128, const SIZE: usize>(
    bv: [T; SIZE],
) -> [BitStatic<LEN>; SIZE]
where
    T: Into<BitDynamic>,
{
    todo!()
}

pub fn array_into_dyn<T, const SIZE: usize>(bv: [T; SIZE]) -> [BitDynamic; SIZE]
where
    T: Into<BitDynamic>,
{
    todo!()
}

pub fn into_dyn<T>(bv: T) -> BitDynamic
where
    T: Into<BitDynamic>,
{
    T::into(bv)
}

pub fn into_static<T, const LEN: i128>(bv: T) -> BitStatic<LEN>
where
    T: Into<BitStatic<LEN>>,
{
    T::into(bv)
}
