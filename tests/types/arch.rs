#![allow(warnings)]

use softcore_prelude::*;

/// The software core.
/// 
/// This struct represents a software core, and holds all the registers as well as the core configuration.
/// The core is the main abstraction exposed by the softcore library and represents a single execution thread.
/// 
/// The raw functions translated directly from the specification are available in the `raw` module, whereas higher-level wrappers are implemented as methods on the [Core] struct directly.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Core {
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {
    pub unknown_at_compile_time: i128,
}

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitVector<xlen>;

pub const xlen: i128 = 64;

/// ExceptionType
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L15-23.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExceptionType {
    E_Fetch_Addr_Align(()),
    E_Fetch_Access_Fault(()),
    E_Illegal_Instr(()),
    E_Breakpoint(()),
    E_Extension(i128)
}

/// physaddr
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L25.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum physaddr {
    Physaddr(xlenbits)
}

/// pmpMatchAddr
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L27-29.
pub fn pmpMatchAddr(physaddr::Physaddr(addr): physaddr) -> bool {
    (addr != BitVector::<64>::new(0b0000000000000000000000000000000000000000000000000000000000000000))
}

/// handle_int
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L33-35.
pub fn handle_int(a1: i128) -> i128 {
    (a1 + 4)
}

/// handle_int_int
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L37-39.
pub fn handle_int_int(a1: i128, a2: i128) -> bool {
    false
}

/// handle_int_int_bool_int
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L41-43.
pub fn handle_int_int_bool_int(a1: i128, a2: i128, a3: bool, a4: i128) -> i128 {
    131
}

/// handle_bool
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L46-48.
pub fn handle_bool(factor_bool: bool) {
    
}

/// handle_union
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L50-52.
pub fn handle_union(unit_arg: ()) -> ExceptionType {
    ExceptionType::E_Illegal_Instr(())
}

/// handle_empty
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L54-57.
pub fn handle_empty(unit_arg: ()) {
    
}

/// ast
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L59.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

/// zeros
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L66.
pub const fn zeros<const N: i128>(n: i128) -> BitVector<N> {
    sail_zeros(n)
}

/// hex_bits_backwards
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L77.
pub fn hex_bits_backwards<const M: i128>(m: i128, str: &'static str) -> BitVector<M> {
    parse_hex_bits(m, str)
}

/// validDoubleRegs
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L80-85.
pub fn validDoubleRegs<const N: usize>(n: i128, regs: [BitVector<5>; N]) -> bool {
    todo!("E_for");
    true
}

/// Access_kind
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L87-90.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_kind<ARCH_AK> {
    AK_ttw(()),
    AK_arch(ARCH_AK)
}

/// Mem_read_request
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L92-98.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mem_read_request<const N: i128, const VASIZE: i128, PA, ARCH_AK> {
    pub access_kind: Access_kind<ARCH_AK>,
    pub va: Option<BitVector<VASIZE>>,
    pub pa: PA,
    pub size: i128,
    pub tag: bool,
}

/// Retired
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L106.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

/// handle_retired
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L107-109.
pub fn handle_retired(unit_arg: ()) -> Retired {
    Retired::RETIRE_SUCCESS
}

/// exception
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L111-114.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_not_implemented(&'static str),
    Error_internal_error(())
}

/// My_struct
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L116-120.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct My_struct {
    pub field1: BitVector<5>,
    pub field2: i128,
    pub field3: &'static str,
}

/// My_struct_generic
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L122-124.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct My_struct_generic<const N: i128> {
    pub foo: BitVector<N>,
}

/// exceptionType_to_bits
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L127-134.
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

/// execute_TEST
/// 
/// Generated from the Sail sources at `tests/types/arch.sail` L136-178.
pub fn execute_TEST(core_ctx: &mut Core) {
    handle_empty(());
    handle_bool(true);
    let a = handle_int(1234);
    let b = handle_int_int(1234, 12345);
    let c = handle_int_int_bool_int(1234, 12345, false, 2);
    let d = handle_retired(());
    let e = handle_union(());
    let f = hex_bits_backwards(8, "00");
    let g = pmpMatchAddr(physaddr::Physaddr(BitVector::<64>::new(0b0000000000000000000000000000000011011110101011011011111011101111)));
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
    let ok = validDoubleRegs(2, [BitVector::<5>::new(0b11011), BitVector::<5>::new(0b01111)]);
    let s: My_struct = My_struct {
        field1: BitVector::<5>::new(0b11111),
        field2: 5,
        field3: "test"
    };
    let s2: My_struct_generic<4> = My_struct_generic {
        foo: BitVector::<4>::new(0b1010)
    };
    let G: i128 = core_ctx.config.unknown_at_compile_time;
    let mask: xlenbits = sail_ones::<64>(min_int(G, 64)).zero_extend::<64>();
    let mask2 = sail_ones::<8>(8);
    let value = exceptionType_to_bits(ExceptionType::E_Fetch_Addr_Align(()));
    ()
}
