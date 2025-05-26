#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub config: SailConfig,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfig {

}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExceptionType {
    E_Fetch_Addr_Align(()),
    E_Fetch_Access_Fault(()),
    E_Illegal_Instr(()),
    E_Breakpoint(()),
    E_Extension(usize)
}

pub fn handle_int(sail_ctx: &mut SailVirtCtx, a1: usize) -> usize {
    (a1 + 4)
}

pub fn handle_int_int(sail_ctx: &mut SailVirtCtx, a1: usize, a2: usize) -> bool {
    false
}

pub fn handle_int_int_bool_int(sail_ctx: &mut SailVirtCtx, a1: usize, a2: usize, a3: bool, a4: usize) -> usize {
    131
}

pub fn handle_bool(sail_ctx: &mut SailVirtCtx, factor_bool: bool) {
    
}

pub fn handle_union(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> ExceptionType {
    ExceptionType::E_Illegal_Instr(())
}

pub fn handle_empty(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

pub fn zeros<const N: usize>(sail_ctx: &mut SailVirtCtx, n: usize) -> BitVector<N> {
    sail_zeros(n)
}

pub fn hex_bits_backwards<const M: usize>(sail_ctx: &mut SailVirtCtx, m: usize, str: &'static str) -> BitVector<M> {
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

pub fn handle_retired(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> Retired {
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

pub fn exceptionType_to_bits(sail_ctx: &mut SailVirtCtx, e: ExceptionType) -> BitVector<8> {
    match e {
        ExceptionType::E_Fetch_Addr_Align(()) => {BitVector::<8>::new(0b00000000)}
        ExceptionType::E_Fetch_Access_Fault(()) => {BitVector::<8>::new(0b00000001)}
        ExceptionType::E_Illegal_Instr(()) => {BitVector::<8>::new(0b00000010)}
        ExceptionType::E_Breakpoint(()) => {BitVector::<8>::new(0b00000011)}
        _ => {panic!("Unreachable code")}
    }
}

pub fn execute(sail_ctx: &mut SailVirtCtx, TodoArgsApp: ast) {
    handle_empty(sail_ctx, ());
    handle_bool(sail_ctx, true);
    let a = handle_int(sail_ctx, 1234);
    let b = handle_int_int(sail_ctx, 1234, 12345);
    let c = handle_int_int_bool_int(sail_ctx, 1234, 12345, false, 2);
    let d = handle_retired(sail_ctx, ());
    let e = handle_union(sail_ctx, ());
    let f = hex_bits_backwards(sail_ctx, 8, "00");
    if {(f != BitVector::<8>::new(0b00000000))} {
        assert!(false, "Process message")
    } else {
        ()
    };
    assert!(true, "Process message");
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
    let value = {
        let var_1 = ExceptionType::E_Fetch_Addr_Align(());
        exceptionType_to_bits(sail_ctx, var_1)
    };
    ()
}