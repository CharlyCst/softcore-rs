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
    pub PC: xlenbits,
    pub nextPC: xlenbits,
    pub mepc: xlenbits,
    pub sepc: xlenbits,
    pub uepc: xlenbits,
    pub mstatus: Mstatus,
    pub cur_privilege: Privilege,
    pub Xs: [xlenbits; (32 as usize)],
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

/// xlen
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L34.
pub const xlen: i128 = 64;

/// xlen_bytes
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L35.
pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitStatic::<xlen>;

pub type priv_level = BitStatic::<2>;

/// Privilege
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L40.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

/// haveUsrMode
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L43.
pub const fn haveUsrMode(unit_arg: ()) -> bool {
    true
}

/// privLevel_to_bits
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L46-51.
pub fn privLevel_to_bits(p: Privilege) -> BitStatic::<2> {
    match p {
        Privilege::User => {BitDynamic::new(2, 0b00)}
        Privilege::Supervisor => {BitDynamic::new(2, 0b01)}
        Privilege::Machine => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// privLevel_of_bits
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L54-60.
pub fn privLevel_of_bits(p: BitStatic::<2>) -> Privilege {
    match p {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {Privilege::User}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {Privilege::Supervisor}
        b__2 if {(b__2 == BitDynamic::new(2, 0b11))} => {Privilege::Machine}
        _ => {not_implemented("Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

/// pc_alignment_mask
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L62-63.
pub fn pc_alignment_mask(unit_arg: ()) -> BitStatic::<64> {
    !(BitDynamic::new(2, 0b10).zero_extend_dyn(64))
}

pub type regidx = BitStatic::<5>;

pub type cregidx = BitStatic::<3>;

pub type csreg = BitStatic::<12>;

/// Mstatus
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L81-105.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitStatic::<64>,
}

/// _get_Mstatus_MPIE
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPIE(v: Mstatus) -> BitStatic::<1> {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_Mstatus_MPP
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPP(v: Mstatus) -> BitStatic::<2> {
    v.bits.subrange::<11, 13, 2>()
}

/// rX
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L116-120.
pub fn rX(core_ctx: &mut Core, r: BitStatic::<5>) -> BitStatic::<64> {
    match r {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00000))} => {BitDynamic::new(4, 0b0000).zero_extend_dyn(64)}
        _ => {core_ctx.Xs[(r.unsigned() as usize)]}
        _ => {panic!("Unreachable code")}
    }
}

/// wX
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L123-126.
pub fn wX(core_ctx: &mut Core, r: BitStatic::<5>, v: BitStatic::<64>) {
    if {(r != BitDynamic::new(5, 0b00000))} {
        core_ctx.Xs[(r.unsigned() as usize)] = v
    } else {
        ()
    }
}

/// set_next_pc
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L142-144.
pub fn set_next_pc(core_ctx: &mut Core, pc: BitStatic::<64>) {
    core_ctx.nextPC = pc
}

/// handle_illegal
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L146-149.
pub fn handle_illegal(unit_arg: ()) {
    
}

/// get_xret_target
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L152-157.
pub fn get_xret_target(core_ctx: &mut Core, p: Privilege) -> BitStatic::<64> {
    match p {
        Privilege::Machine => {core_ctx.mepc}
        Privilege::Supervisor => {core_ctx.sepc}
        Privilege::User => {core_ctx.uepc}
        _ => {panic!("Unreachable code")}
    }
}

/// prepare_xret_target
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L171-172.
pub fn prepare_xret_target(core_ctx: &mut Core, p: Privilege) -> BitStatic::<64> {
    get_xret_target(core_ctx, p)
}

/// exception_handler
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L174-184.
pub fn exception_handler(core_ctx: &mut Core, cur_priv: Privilege, pc: BitStatic::<64>) -> BitStatic::<64> {
    let prev_priv: Privilege = core_ctx.cur_privilege;
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange({
        let var_1: Mstatus = core_ctx.mstatus;
        _get_Mstatus_MPIE(var_1)
    }, 3, 3);
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b1), 7, 7);
    core_ctx.cur_privilege = {
        let var_2: BitStatic::<2> = {
            let var_3: Mstatus = core_ctx.mstatus;
            _get_Mstatus_MPP(var_3)
        };
        privLevel_of_bits(var_2)
    };
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(privLevel_to_bits(Privilege::User), 12, 11);
    if {(core_ctx.cur_privilege != Privilege::Machine)} {
        core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 17, 17)
    } else {
        ()
    };
    (prepare_xret_target(core_ctx, Privilege::Machine) & pc_alignment_mask(()))
}

/// Retired
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L188.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

/// ast
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L190.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    MRET(())
}

/// encdec_forwards
///
/// Generated from the Sail sources.
pub fn encdec_forwards(arg_hashtag_: ast) -> BitStatic::<32> {
    match arg_hashtag_ {
        ast::MRET(()) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0011000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00010)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_backwards
///
/// Generated from the Sail sources.
pub fn encdec_backwards(arg_hashtag_: BitStatic::<32>) -> ast {
    match arg_hashtag_ {
        v__0 if {(v__0 == BitDynamic::new(32, 0b00110000001000000000000001110011))} => {ast::MRET(())}
        _ => {panic!("Unreachable code")}
    }
}

/// ext_check_xret_priv
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L204.
pub const fn ext_check_xret_priv(p: Privilege) -> bool {
    true
}

/// ext_fail_xret_priv
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L206.
pub const fn ext_fail_xret_priv(unit_arg: ()) {
    ()
}

/// execute
///
/// Generated from the Sail sources at `tests/mret/arch.sail` L208-217.
pub fn execute(core_ctx: &mut Core, ast::MRET(()): ast) -> Retired {
    if {(core_ctx.cur_privilege != Privilege::Machine)} {
        ();
        Retired::RETIRE_FAIL
    } else if {!(true)} {
        ();
        Retired::RETIRE_FAIL
    } else {
        {
            let var_1: BitStatic::<64> = {
                let var_2: Privilege = core_ctx.cur_privilege;
                let var_3: BitStatic::<64> = core_ctx.PC;
                exception_handler(core_ctx, var_2, var_3)
            };
            set_next_pc(core_ctx, var_1)
        };
        Retired::RETIRE_SUCCESS
    }
}
