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
    pub PC: xlenbits,
    pub nextPC: xlenbits,
    pub mepc: xlenbits,
    pub sepc: xlenbits,
    pub uepc: xlenbits,
    pub mstatus: Mstatus,
    pub cur_privilege: Privilege,
    pub Xs: [xlenbits;32],
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {

}

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

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
pub fn privLevel_to_bits(p: Privilege) -> BitVector<2> {
    match p {
        Privilege::User => {BitVector::<2>::new(0b00)}
        Privilege::Supervisor => {BitVector::<2>::new(0b01)}
        Privilege::Machine => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// privLevel_of_bits
/// 
/// Generated from the Sail sources at `tests/mret/arch.sail` L54-60.
pub fn privLevel_of_bits(p: BitVector<2>) -> Privilege {
    match p {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {Privilege::User}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {Privilege::Supervisor}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {Privilege::Machine}
        _ => {not_implemented("Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

/// pc_alignment_mask
/// 
/// Generated from the Sail sources at `tests/mret/arch.sail` L62-63.
pub fn pc_alignment_mask(unit_arg: ()) -> BitVector<64> {
    !(BitVector::<2>::new(0b10).zero_extend::<64>())
}

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

/// Mstatus
/// 
/// Generated from the Sail sources at `tests/mret/arch.sail` L81-105.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitVector<64>,
}

/// _get_Mstatus_MPIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_Mstatus_MPP
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPP(v: Mstatus) -> BitVector<2> {
    v.bits.subrange::<11, 13, 2>()
}

/// set_next_pc
/// 
/// Generated from the Sail sources at `tests/mret/arch.sail` L142-144.
pub fn set_next_pc(core_ctx: &mut Core, pc: BitVector<64>) {
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
pub fn get_xret_target(core_ctx: &mut Core, p: Privilege) -> BitVector<64> {
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
pub fn prepare_xret_target(core_ctx: &mut Core, p: Privilege) -> BitVector<64> {
    get_xret_target(core_ctx, p)
}

/// exception_handler
/// 
/// Generated from the Sail sources at `tests/mret/arch.sail` L174-184.
pub fn exception_handler(core_ctx: &mut Core, cur_priv: Privilege, pc: BitVector<64>) -> BitVector<64> {
    let prev_priv = core_ctx.cur_privilege;
    core_ctx.mstatus.bits = {
        let var_1 = {
            let var_2 = core_ctx.mstatus;
            _get_Mstatus_MPIE(var_2)
        };
        core_ctx.mstatus.bits.set_subrange::<3, 4, 1>(var_1)
    };
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<7, 8, 1>(BitVector::<1>::new(0b1));
    core_ctx.cur_privilege = {
        let var_3 = {
            let var_4 = core_ctx.mstatus;
            _get_Mstatus_MPP(var_4)
        };
        privLevel_of_bits(var_3)
    };
    core_ctx.mstatus.bits = {
        let var_5 = {
            let var_6 = if {haveUsrMode(())} {
                Privilege::User
            } else {
                Privilege::Machine
            };
            privLevel_to_bits(var_6)
        };
        core_ctx.mstatus.bits.set_subrange::<11, 13, 2>(var_5)
    };
    if {(core_ctx.cur_privilege != Privilege::Machine)} {
        core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<17, 18, 1>(BitVector::<1>::new(0b0))
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

/// execute_MRET
/// 
/// Generated from the Sail sources at `tests/mret/arch.sail` L208-217.
pub fn execute_MRET(core_ctx: &mut Core) -> Retired {
    if {(core_ctx.cur_privilege != Privilege::Machine)} {
        handle_illegal(());
        Retired::RETIRE_FAIL
    } else if {!(ext_check_xret_priv(Privilege::Machine))} {
        ext_fail_xret_priv(());
        Retired::RETIRE_FAIL
    } else {
        {
            let var_1 = {
                let var_2 = core_ctx.cur_privilege;
                let var_3 = core_ctx.PC;
                exception_handler(core_ctx, var_2, var_3)
            };
            set_next_pc(core_ctx, var_1)
        };
        Retired::RETIRE_SUCCESS
    }
}