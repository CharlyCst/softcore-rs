#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub PC: xlenbits,
    pub nextPC: xlenbits,
    pub mepc: xlenbits,
    pub sepc: xlenbits,
    pub uepc: xlenbits,
    pub mstatus: Mstatus,
    pub cur_privilege: Privilege,
    pub Xs: [xlenbits;32],
    pub config: SailConfig,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfig {

}

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

pub fn haveUsrMode(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> bool {
    true
}

pub fn privLevel_to_bits(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<2> {
    match p {
        Privilege::User => {BitVector::<2>::new(0b00)}
        Privilege::Supervisor => {BitVector::<2>::new(0b01)}
        Privilege::Machine => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

pub fn privLevel_of_bits(sail_ctx: &mut SailVirtCtx, p: BitVector<2>) -> Privilege {
    match p {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {Privilege::User}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {Privilege::Supervisor}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {Privilege::Machine}
        _ => {not_implemented("Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

pub fn pc_alignment_mask(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> BitVector<64> {
    !(BitVector::<2>::new(0b10).zero_extend::<64>())
}

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitVector<64>,
}

pub fn _get_Mstatus_MPIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<7, 8, 1>()
}

pub fn _get_Mstatus_MPP(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<2> {
    v.bits.subrange::<11, 13, 2>()
}

pub fn set_next_pc(sail_ctx: &mut SailVirtCtx, pc: BitVector<64>) {
    sail_ctx.nextPC = pc
}

pub fn handle_illegal(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    
}

pub fn get_xret_target(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<64> {
    match p {
        Privilege::Machine => {sail_ctx.mepc}
        Privilege::Supervisor => {sail_ctx.sepc}
        Privilege::User => {sail_ctx.uepc}
        _ => {panic!("Unreachable code")}
    }
}

pub fn prepare_xret_target(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<64> {
    get_xret_target(sail_ctx, p)
}

pub fn exception_handler(sail_ctx: &mut SailVirtCtx, cur_priv: Privilege, pc: BitVector<64>) -> BitVector<64> {
    let prev_priv = sail_ctx.cur_privilege;
    sail_ctx.mstatus.bits = {
        let var_1 = _get_Mstatus_MPIE(sail_ctx, sail_ctx.mstatus);
        sail_ctx.mstatus.bits.set_subrange::<3, 4, 1>(var_1)
    };
    sail_ctx.mstatus.bits = sail_ctx.mstatus.bits.set_subrange::<7, 8, 1>(BitVector::<1>::new(0b1));
    sail_ctx.cur_privilege = {
        let var_2 = _get_Mstatus_MPP(sail_ctx, sail_ctx.mstatus);
        privLevel_of_bits(sail_ctx, var_2)
    };
    sail_ctx.mstatus.bits = {
        let var_3 = {
            let var_4 = if {haveUsrMode(sail_ctx, ())} {
                Privilege::User
            } else {
                Privilege::Machine
            };
            privLevel_to_bits(sail_ctx, var_4)
        };
        sail_ctx.mstatus.bits.set_subrange::<11, 13, 2>(var_3)
    };
    if {(sail_ctx.cur_privilege != Privilege::Machine)} {
        sail_ctx.mstatus.bits = sail_ctx.mstatus.bits.set_subrange::<17, 18, 1>(BitVector::<1>::new(0b0))
    } else {
        ()
    };
    (prepare_xret_target(sail_ctx, Privilege::Machine) & pc_alignment_mask(sail_ctx, ()))
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    MRET(())
}

pub fn ext_check_xret_priv(sail_ctx: &mut SailVirtCtx, p: Privilege) -> bool {
    true
}

pub fn ext_fail_xret_priv(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    ()
}

pub fn execute(sail_ctx: &mut SailVirtCtx, ast::MRET(()): ast) -> Retired {
    if {(sail_ctx.cur_privilege != Privilege::Machine)} {
        handle_illegal(sail_ctx, ());
        Retired::RETIRE_FAIL
    } else if {!(ext_check_xret_priv(sail_ctx, Privilege::Machine))} {
        ext_fail_xret_priv(sail_ctx, ());
        Retired::RETIRE_FAIL
    } else {
        {
            let var_1 = exception_handler(sail_ctx, sail_ctx.cur_privilege, sail_ctx.PC);
            set_next_pc(sail_ctx, var_1)
        };
        Retired::RETIRE_SUCCESS
    }
}