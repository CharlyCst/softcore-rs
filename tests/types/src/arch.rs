#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {

}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExceptionType {
    E_Fetch_Addr_Align(()),
    E_Fetch_Access_Fault(()),
    E_Illegal_Instr(()),
    E_Breakpoint(()),
    E_Load_Addr_Align(()),
    E_Load_Access_Fault(()),
    E_SAMO_Addr_Align(()),
    E_SAMO_Access_Fault(()),
    E_U_EnvCall(()),
    E_S_EnvCall(()),
    E_Reserved_10(()),
    E_M_EnvCall(()),
    E_Fetch_Page_Fault(()),
    E_Load_Page_Fault(()),
    E_Reserved_14(()),
    E_SAMO_Page_Fault(()),
    E_Extension(usize)
}

pub fn handle_int(sail_ctx: &mut SailVirtCtx, a1: usize) -> usize {
    (a1 + 4)
}

pub fn handle_int_int(sail_ctx: &mut SailVirtCtx, a1: usize, a2: usize) -> bool {
    false
}

pub fn handle_int_int_bool_int(sail_ctx: &mut SailVirtCtx, a1: usize, a2: usize, a3: bool, a4: usize) -> usize {
    (127 + 4)
}

pub fn handle_bool(sail_ctx: &mut SailVirtCtx, factor_bool: bool) {
    ()
}

pub fn handle_union(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> ExceptionType {
    ExceptionType::E_Fetch_Page_Fault(())
}

pub fn handle_empty(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    ()
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
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

pub fn exceptionType_to_bits(sail_ctx: &mut SailVirtCtx, e: ExceptionType) -> BitVector<8> {
    match e {
        ExceptionType::E_Fetch_Addr_Align(()) => {BitVector::<8>::new(0b00000000)}
        ExceptionType::E_Fetch_Access_Fault(()) => {BitVector::<8>::new(0b00000001)}
        ExceptionType::E_Illegal_Instr(()) => {BitVector::<8>::new(0b00000010)}
        ExceptionType::E_Breakpoint(()) => {BitVector::<8>::new(0b00000011)}
        ExceptionType::E_Load_Addr_Align(()) => {BitVector::<8>::new(0b00000100)}
        ExceptionType::E_Load_Access_Fault(()) => {BitVector::<8>::new(0b00000101)}
        ExceptionType::E_SAMO_Addr_Align(()) => {BitVector::<8>::new(0b00000110)}
        ExceptionType::E_SAMO_Access_Fault(()) => {BitVector::<8>::new(0b00000111)}
        ExceptionType::E_U_EnvCall(()) => {BitVector::<8>::new(0b00001000)}
        ExceptionType::E_S_EnvCall(()) => {BitVector::<8>::new(0b00001001)}
        ExceptionType::E_Reserved_10(()) => {BitVector::<8>::new(0b00001010)}
        ExceptionType::E_M_EnvCall(()) => {BitVector::<8>::new(0b00001011)}
        ExceptionType::E_Fetch_Page_Fault(()) => {BitVector::<8>::new(0b00001100)}
        ExceptionType::E_Load_Page_Fault(()) => {BitVector::<8>::new(0b00001101)}
        ExceptionType::E_Reserved_14(()) => {BitVector::<8>::new(0b00001110)}
        ExceptionType::E_SAMO_Page_Fault(()) => {BitVector::<8>::new(0b00001111)}
        _ => {panic!("Unreachable code")}
    }
}

pub fn execute_TEST(sail_ctx: &mut SailVirtCtx) {
    handle_empty(sail_ctx, ());
    handle_bool(sail_ctx, true);
    let a = handle_int(sail_ctx, 1234);
    let b = handle_int_int(sail_ctx, 1234, 12345);
    let c = handle_int_int_bool_int(sail_ctx, 1234, 12345, false, 2);
    let d = handle_retired(sail_ctx, ());
    let e = handle_union(sail_ctx, ());
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
    let value = {
        let var_1 = ExceptionType::E_Fetch_Addr_Align(());
        exceptionType_to_bits(sail_ctx, var_1)
    };
    ()
}