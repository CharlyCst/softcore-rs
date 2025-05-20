#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub mepc: xlenbits,
    pub sepc: xlenbits,
    pub uepc: xlenbits,
    pub mtimecmp: BitVector<64>,
    pub mtime: BitVector<64>,
    pub mcycle: BitVector<64>,
    pub mstatus: Mstatus,
    pub cur_privilege: Privilege,
}

pub fn _operator_smaller_u_<const N: usize>(sail_ctx: &mut SailVirtCtx, x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.as_usize() < y.as_usize())
}

pub const xlen: usize = 64;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

pub fn _get_Mstatus_TW(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<21, 22, 1>()
}

pub fn handle_illegal(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    ()
}

pub fn platform_wfi(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    if {_operator_smaller_u_(sail_ctx, sail_ctx.mtime, sail_ctx.mtimecmp)} {
        sail_ctx.mtime = sail_ctx.mtimecmp;
        sail_ctx.mcycle = sail_ctx.mtimecmp
    } else {
        ()
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    WFI(())
}

pub fn execute_WFI(sail_ctx: &mut SailVirtCtx) -> Retired {
    match sail_ctx.cur_privilege {
        Privilege::Machine => {{
            platform_wfi(sail_ctx, ());
            Retired::RETIRE_SUCCESS
        }}
        Privilege::Supervisor => {if {(_get_Mstatus_TW(sail_ctx, sail_ctx.mstatus) == BitVector::<1>::new(0b1))} {
            handle_illegal(sail_ctx, ());
            Retired::RETIRE_FAIL
        } else {
            platform_wfi(sail_ctx, ());
            Retired::RETIRE_SUCCESS
        }}
        Privilege::User => {{
            handle_illegal(sail_ctx, ());
            Retired::RETIRE_FAIL
        }}
        _ => {panic!("Unreachable code")}
    }
}