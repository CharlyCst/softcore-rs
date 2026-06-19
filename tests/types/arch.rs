#![allow(warnings)]

use softcore_prelude::*;

use crate::arch_prelude::*;

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

/// Initialize all registers.
///
/// This function should be called before using a fresh core, otherwise the core might not be in a valid state.
pub fn _reset_all_registers() {
    
}

/// xlen_bytes
///
/// Generated from the Sail sources at `tests/types/arch.sail` L7.
pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitStatic::<xlen>;

/// xlen
///
/// Generated from the Sail sources at `tests/types/arch.sail` L11.
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
    (addr != BitDynamic::new(64, 0b0000000000000000000000000000000000000000000000000000000000000000))
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
pub const fn zeros(n: i128) -> BitDynamic {
    sail_zeros(n)
}

/// hex_bits_backwards
///
/// Generated from the Sail sources at `tests/types/arch.sail` L77.
pub fn hex_bits_backwards(m: i128, str: &'static str) -> BitDynamic {
    parse_hex_bits(m, str)
}

/// validDoubleRegs
///
/// Generated from the Sail sources at `tests/types/arch.sail` L80-85.
pub fn validDoubleRegs(n: i128, regs: BoundedVec::<BitStatic::<5>, 32>) -> bool {
    for i in 0..=(n - 1) {
        if {(bitvector_access(regs[(i as usize)], 0) == true)} {
            return false;
        } else {
            ()
        }
    };
    true
}

/// X_read
///
/// Generated from the Sail sources at `tests/types/arch.sail` L91-93.
pub fn X_read(n: i128, width: i128) -> BitDynamic {
    zeros(width)
}

/// Access_kind
///
/// Generated from the Sail sources at `tests/types/arch.sail` L95-98.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_kind<ARCH_AK> {
    AK_ttw(()),
    AK_arch(ARCH_AK)
}

/// Mem_read_request
///
/// Generated from the Sail sources at `tests/types/arch.sail` L100-106.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mem_read_request<const N: i128, const VASIZE: i128, PA, ARCH_AK> {
    pub access_kind: Access_kind::<ARCH_AK>,
    pub va: Option<BitDynamic>,
    pub pa: PA,
    pub size: i128,
    pub tag: bool,
}

/// Retired
///
/// Generated from the Sail sources at `tests/types/arch.sail` L114.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

/// handle_retired
///
/// Generated from the Sail sources at `tests/types/arch.sail` L115-117.
pub fn handle_retired(unit_arg: ()) -> Retired {
    Retired::RETIRE_SUCCESS
}

/// exception
///
/// Generated from the Sail sources at `tests/types/arch.sail` L119-122.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_not_implemented(&'static str),
    Error_internal_error(())
}

/// My_struct
///
/// Generated from the Sail sources at `tests/types/arch.sail` L124-128.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct My_struct {
    pub field1: BitStatic::<5>,
    pub field2: i128,
    pub field3: &'static str,
}

/// My_struct_generic
///
/// Generated from the Sail sources at `tests/types/arch.sail` L130-132.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct My_struct_generic<const N: i128> {
    pub foo: BitDynamic,
}

/// exceptionType_to_bits
///
/// Generated from the Sail sources at `tests/types/arch.sail` L135-142.
pub fn exceptionType_to_bits(e: ExceptionType) -> BitStatic::<8> {
    match e {
        ExceptionType::E_Fetch_Addr_Align(()) => {BitDynamic::new(8, 0b00000000)}
        ExceptionType::E_Fetch_Access_Fault(()) => {BitDynamic::new(8, 0b00000001)}
        ExceptionType::E_Illegal_Instr(()) => {BitDynamic::new(8, 0b00000010)}
        ExceptionType::E_Breakpoint(()) => {BitDynamic::new(8, 0b00000011)}
        ExceptionType::E_Extension(_) => {BitDynamic::new(8, 0b00000100)}
        _ => {panic!("Unreachable code")}
    }
}

/// execute
///
/// Generated from the Sail sources at `tests/types/arch.sail` L144-187.
pub fn execute(core_ctx: &mut Core, ast::TEST(()): ast) {
    let a: i128 = handle_int(1234);
    let d: Retired = handle_retired(());
    let e: ExceptionType = handle_union(());
    let f: BitStatic::<8> = hex_bits_backwards(8, "00");
    let g: bool = pmpMatchAddr(physaddr::Physaddr(BitDynamic::new(64, 0b0000000000000000000000000000000011011110101011011011111011101111)));
    let h: BitStatic::<64> = X_read(10, 64);
    if {(f != BitDynamic::new(8, 0b00000000))} {
        assert!(false, "failed to parse hex)")
    } else {
        ()
    };
    assert!(true, "works");
    panic!("todo_process_panic_type");
    for i in 0..=3 {
        let idx: i128 = i;
        ()
    };
    let ok: bool = validDoubleRegs(2, [BitDynamic::new(5, 0b11011), BitDynamic::new(5, 0b01111)].into());
    let s: My_struct = My_struct {
        field1: BitDynamic::new(5, 0b11111),
        field2: 5,
        field3: "test"
    };
    let s2: My_struct_generic::<4> = My_struct_generic {
        foo: BitDynamic::new(4, 0b1010)
    };
    let G: i128 = core_ctx.config.unknown_at_compile_time;
    let mask: xlenbits = sail_ones(min_int(G, 64)).zero_extend_dyn(64);
    let mask2: BitStatic::<8> = sail_ones(8);
    let value: BitStatic::<8> = exceptionType_to_bits(ExceptionType::E_Fetch_Addr_Align(()));
    ()
}
