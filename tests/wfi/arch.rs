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
    pub mepc: xlenbits,
    pub sepc: xlenbits,
    pub uepc: xlenbits,
    pub mtimecmp: BitStatic::<64>,
    pub mtime: BitStatic::<64>,
    pub mcycle: BitStatic::<64>,
    pub mstatus: Mstatus,
    pub cur_privilege: Privilege,
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {

}

/// Initialize all registers.
///
/// This function should be called before using a fresh core, otherwise the core might not be in a valid state.
pub fn _reset_all_registers() {
    
}

/// (operator <_u)
///
/// Generated from the Sail sources at `tests/wfi/arch.sail` L9.
pub fn _operator_smaller_u_(x: BitDynamic, y: BitDynamic) -> bool {
    (x.unsigned() < y.unsigned())
}

pub const xlen: i128 = 64;

pub type xlenbits = BitStatic::<xlen>;

pub type priv_level = BitStatic::<2>;

/// Privilege
///
/// Generated from the Sail sources at `tests/wfi/arch.sail` L21.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

pub type regidx = BitStatic::<5>;

pub type cregidx = BitStatic::<3>;

pub type csreg = BitStatic::<12>;

/// Mstatus
///
/// Generated from the Sail sources at `tests/wfi/arch.sail` L39-63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitStatic::<64>,
}

/// _get_Mstatus_TW
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_TW(v: Mstatus) -> BitStatic::<1> {
    v.bits.subrange::<21, 22, 1>()
}

/// handle_illegal
///
/// Generated from the Sail sources at `tests/wfi/arch.sail` L71-74.
pub fn handle_illegal(unit_arg: ()) {
    
}

/// platform_wfi
///
/// Generated from the Sail sources at `tests/wfi/arch.sail` L76-84.
pub fn platform_wfi(core_ctx: &mut Core, unit_arg: ()) {
    if {{
        let var_1 = core_ctx.mtime;
        let var_2 = core_ctx.mtimecmp;
        _operator_smaller_u_(var_1.into(), var_2.into())
    }} {
        core_ctx.mtime = core_ctx.mtimecmp;
        core_ctx.mcycle = core_ctx.mtimecmp
    } else {
        ()
    }
}

/// Retired
///
/// Generated from the Sail sources at `tests/wfi/arch.sail` L87.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

/// ast
///
/// Generated from the Sail sources at `tests/wfi/arch.sail` L89.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    WFI(())
}

/// encdec_forwards
///
/// Generated from the Sail sources.
pub fn encdec_forwards(arg_hashtag_: ast) -> BitStatic::<32> {
    match arg_hashtag_ {
        ast::WFI(()) => {bitvector_concat(BitDynamic::from(BitStatic::<12>::new(12, 0b000100000101)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitStatic::<5>::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitStatic::<3>::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitStatic::<5>::new(5, 0b00000)), BitDynamic::from(BitStatic::<7>::new(7, 0b1110011)))))))))}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_backwards
///
/// Generated from the Sail sources.
pub fn encdec_backwards(arg_hashtag_: BitStatic::<32>) -> ast {
    match arg_hashtag_ {
        v__0 if {(v__0 == BitStatic::<32>::new(32, 0b00010000010100000000000001110011))} => {ast::WFI(())}
        _ => {panic!("Unreachable code")}
    }
}

/// execute
///
/// Generated from the Sail sources at `tests/wfi/arch.sail` L102-109.
pub fn execute(core_ctx: &mut Core, ast::WFI(()): ast) -> Retired {
    match core_ctx.cur_privilege {
        Privilege::Machine => {{
            platform_wfi(core_ctx, ());
            Retired::RETIRE_SUCCESS
        }}
        Privilege::Supervisor => {if {({
            let var_1 = core_ctx.mstatus;
            _get_Mstatus_TW(var_1)
        } == BitStatic::<1>::new(1, 0b1))} {
            ();
            Retired::RETIRE_FAIL
        } else {
            platform_wfi(core_ctx, ());
            Retired::RETIRE_SUCCESS
        }}
        Privilege::User => {{
            Retired::RETIRE_FAIL
        }}
        _ => {panic!("Unreachable code")}
    }
}
