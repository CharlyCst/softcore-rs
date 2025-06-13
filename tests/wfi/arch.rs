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
    pub mepc: xlenbits,
    pub sepc: xlenbits,
    pub uepc: xlenbits,
    pub mtimecmp: BitVector<64>,
    pub mtime: BitVector<64>,
    pub mcycle: BitVector<64>,
    pub mstatus: Mstatus,
    pub cur_privilege: Privilege,
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {

}

/// (operator <_u)
/// 
/// Generated from the Sail sources at `tests/wfi/arch.sail` L9.
pub fn _operator_smaller_u_<const N: i128>(x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.unsigned() < y.unsigned())
}

pub const xlen: i128 = 64;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

/// Privilege
/// 
/// Generated from the Sail sources at `tests/wfi/arch.sail` L21.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

/// Mstatus
/// 
/// Generated from the Sail sources at `tests/wfi/arch.sail` L39-63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitVector<64>,
}

/// _get_Mstatus_TW
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_TW(v: Mstatus) -> BitVector<1> {
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
        _operator_smaller_u_(var_1, var_2)
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

/// encdec_backwards
/// 
/// Generated from the Sail sources.
pub fn encdec_backwards(arg_hashtag_: BitVector<32>) -> ast {
    match arg_hashtag_ {
        v__0 if {(v__0 == BitVector::<32>::new(0b00010000010100000000000001110011))} => {ast::WFI(())}
        _ => {panic!("Unreachable code")}
    }
}

/// execute_WFI
/// 
/// Generated from the Sail sources at `tests/wfi/arch.sail` L102-109.
pub fn execute_WFI(core_ctx: &mut Core) -> Retired {
    match core_ctx.cur_privilege {
        Privilege::Machine => {{
            platform_wfi(core_ctx, ());
            Retired::RETIRE_SUCCESS
        }}
        Privilege::Supervisor => {if {({
            let var_1 = core_ctx.mstatus;
            _get_Mstatus_TW(var_1)
        } == BitVector::<1>::new(0b1))} {
            handle_illegal(());
            Retired::RETIRE_FAIL
        } else {
            platform_wfi(core_ctx, ());
            Retired::RETIRE_SUCCESS
        }}
        Privilege::User => {{
            handle_illegal(());
            Retired::RETIRE_FAIL
        }}
        _ => {panic!("Unreachable code")}
    }
}