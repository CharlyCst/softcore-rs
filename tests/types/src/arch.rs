#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub config: SailConfig,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfig {
    pub unknown_at_compile_time: usize,
}

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExceptionType {
    E_Fetch_Addr_Align(()),
    E_Fetch_Access_Fault(()),
    E_Illegal_Instr(()),
    E_Breakpoint(()),
    E_Extension(usize)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum physaddr {
    Physaddr(xlenbits)
}

pub fn pmpMatchAddr(physaddr::Physaddr(addr): physaddr) -> bool {
    (addr != BitVector::<64>::new(0b0000000000000000000000000000000000000000000000000000000000000000))
}

pub fn handle_int(a1: usize) -> usize {
    (a1 + 4)
}

pub fn handle_int_int(a1: usize, a2: usize) -> bool {
    false
}

pub fn handle_int_int_bool_int(a1: usize, a2: usize, a3: bool, a4: usize) -> usize {
    131
}

pub fn handle_bool(factor_bool: bool) {
    
}

pub fn handle_union(unit_arg: ()) -> ExceptionType {
    ExceptionType::E_Illegal_Instr(())
}

pub fn handle_empty(unit_arg: ()) {
    
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

pub fn zeros<const N: usize>(n: usize) -> BitVector<N> {
    sail_zeros(n)
}

pub fn hex_bits_backwards<const M: usize>(m: usize, str: &'static str) -> BitVector<M> {
    parse_hex_bits(m, str)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_kind<ARCH_AK> {
    AK_ttw(()),
    AK_arch(ARCH_AK)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mem_read_request<const N: usize, const VASIZE: usize, PA, ARCH_AK> {
    pub access_kind: Access_kind<ARCH_AK>,
    pub va: Option<BitVector<VASIZE>>,
    pub pa: PA,
    pub size: usize,
    pub tag: bool,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

pub fn handle_retired(unit_arg: ()) -> Retired {
    Retired::RETIRE_SUCCESS
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_not_implemented(&'static str),
    Error_internal_error(())
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct My_struct {
    pub field1: BitVector<5>,
    pub field2: usize,
    pub field3: &'static str,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct My_struct_generic<const N: usize> {
    pub foo: BitVector<N>,
}

pub fn exceptionType_to_bits(e: ExceptionType) -> BitVector<8> {
    match e {
        ExceptionType::E_Fetch_Addr_Align(()) => {BitVector::<8>::new(0b00000000)}
        ExceptionType::E_Fetch_Access_Fault(()) => {BitVector::<8>::new(0b00000001)}
        ExceptionType::E_Illegal_Instr(()) => {BitVector::<8>::new(0b00000010)}
        ExceptionType::E_Breakpoint(()) => {BitVector::<8>::new(0b00000011)}
        ExceptionType::E_Extension(_) => {BitVector::<8>::new(0b00000100)}
        _ => {panic!("Unreachable code")}
    }
}

pub fn execute_TEST(sail_ctx: &mut SailVirtCtx) {
    handle_empty(());
    handle_bool(true);
    let a = handle_int(1234);
    let b = handle_int_int(1234, 12345);
    let c = handle_int_int_bool_int(1234, 12345, false, 2);
    let d = handle_retired(());
    let e = handle_union(());
    let f = hex_bits_backwards(8, "00");
    let g = {
        let var_2 = physaddr::Physaddr(BitVector::<64>::new(0b0000000000000000000000000000000011011110101011011011111011101111));
        pmpMatchAddr(var_2)
    };
    if {(f != BitVector::<8>::new(0b00000000))} {
        assert!(false, "failed to parse hex)")
    } else {
        ()
    };
    assert!(true, "works");
    panic!("todo_process_panic_type");
    for i in 0..=3 {
        let idx = i;
        ()
    };
    let s: My_struct = My_struct {
        field1: BitVector::<5>::new(0b11111),
        field2: 5,
        field3: "test"
    };
    let s2: My_struct_generic<4> = My_struct_generic {
        foo: BitVector::<4>::new(0b1010)
    };
    let G: usize = sail_ctx.config.unknown_at_compile_time;
    let mask: xlenbits = sail_ones::<64>(min_int(G, 64)).zero_extend::<64>();
    let mask2 = sail_ones::<8>(8);
    let value = {
        let var_1 = ExceptionType::E_Fetch_Addr_Align(());
        exceptionType_to_bits(var_1)
    };
    ()
}