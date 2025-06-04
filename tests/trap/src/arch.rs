#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub PC: xlenbits,
    pub nextPC: xlenbits,
    pub mtval: xlenbits,
    pub mscratch: xlenbits,
    pub sscratch: xlenbits,
    pub medeleg: Medeleg,
    pub mcause: Mcause,
    pub scause: Mcause,
    pub ucause: Mcause,
    pub mstatus: Mstatus,
    pub mtvec: Mtvec,
    pub stvec: Mtvec,
    pub utvec: Mtvec,
    pub cur_privilege: Privilege,
    pub Xs: [xlenbits;32],
    pub config: SailConfig,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfig {

}

pub fn bool_to_bit(sail_ctx: &mut SailVirtCtx, x: bool) -> bool {
    if {x} {
        true
    } else {
        false
    }
}

pub fn bool_to_bits(sail_ctx: &mut SailVirtCtx, x: bool) -> BitVector<1> {
    let mut __generated_vector: BitVector<1> = BitVector::<1>::new_empty();
    {
        let var_1 = 0;
        let var_2 = bool_to_bit(sail_ctx, x);
        __generated_vector.set_vector_entry(var_1, var_2)
    };
    __generated_vector
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

pub fn privLevel_to_bits(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<2> {
    match p {
        Privilege::User => {BitVector::<2>::new(0b00)}
        Privilege::Supervisor => {BitVector::<2>::new(0b01)}
        Privilege::Machine => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

pub fn haveSupMode(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> bool {
    true
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_internal_error(())
}

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Medeleg {
    pub bits: BitVector<64>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mcause {
    pub bits: BitVector<64>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitVector<64>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mtvec {
    pub bits: BitVector<64>,
}

pub fn _get_Mstatus_MIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<3, 4, 1>()
}

pub fn _get_Mstatus_SIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

pub fn _get_Mstatus_UIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<0, 1, 1>()
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
    E_SAMO_Page_Fault(())
}

pub type exc_code = BitVector<8>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct sync_exception {
    pub trap: ExceptionType,
    pub excinfo: Option<xlenbits>,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ctl_result {
    CTL_TRAP(sync_exception)
}

pub type tv_mode = BitVector<2>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TrapVectorMode {
    TV_Direct,
    TV_Vector,
    TV_Reserved
}

pub fn tval(sail_ctx: &mut SailVirtCtx, excinfo: Option<BitVector<64>>) -> BitVector<64> {
    match excinfo {
        Some(e) => {e}
        None => {BitVector::<1>::new(0b0).zero_extend::<64>()}
        _ => {panic!("Unreachable code")}
    }
}

pub fn trap_handler(sail_ctx: &mut SailVirtCtx, del_priv: Privilege, intr: bool, c: BitVector<8>, pc: BitVector<64>, info: Option<BitVector<64>>) -> BitVector<64> {
    match del_priv {
        Privilege::Machine => {{
            sail_ctx.mcause.bits = {
                let var_1 = bool_to_bits(sail_ctx, intr);
                sail_ctx.mcause.bits.set_subrange::<63, 64, 1>(var_1)
            };
            sail_ctx.mcause.bits = {
                let var_2 = c.zero_extend::<63>();
                sail_ctx.mcause.bits.set_subrange::<0, 63, 63>(var_2)
            };
            sail_ctx.mstatus.bits = {
                let var_3 = _get_Mstatus_MIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.bits.set_subrange::<7, 8, 1>(var_3)
            };
            sail_ctx.mstatus.bits = sail_ctx.mstatus.bits.set_subrange::<3, 4, 1>(BitVector::<1>::new(0b0));
            sail_ctx.mstatus.bits = {
                let var_4 = privLevel_to_bits(sail_ctx, sail_ctx.cur_privilege);
                sail_ctx.mstatus.bits.set_subrange::<11, 13, 2>(var_4)
            };
            sail_ctx.mtval = tval(sail_ctx, info);
            todo!("E_var")
        }}
        Privilege::Supervisor => {{
            assert!(haveSupMode(sail_ctx, ()), "Process message");
            sail_ctx.scause.bits = {
                let var_5 = bool_to_bits(sail_ctx, intr);
                sail_ctx.scause.bits.set_subrange::<63, 64, 1>(var_5)
            };
            sail_ctx.scause.bits = {
                let var_6 = c.zero_extend::<63>();
                sail_ctx.scause.bits.set_subrange::<0, 63, 63>(var_6)
            };
            sail_ctx.mstatus.bits = {
                let var_7 = _get_Mstatus_SIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.bits.set_subrange::<5, 6, 1>(var_7)
            };
            sail_ctx.mstatus.bits = sail_ctx.mstatus.bits.set_subrange::<1, 2, 1>(BitVector::<1>::new(0b0));
            sail_ctx.mstatus.bits = sail_ctx.mstatus.bits.set_subrange::<8, 9, 1>(match sail_ctx.cur_privilege {
                Privilege::User => {BitVector::<1>::new(0b0)}
                Privilege::Supervisor => {BitVector::<1>::new(0b1)}
                Privilege::Machine => {panic!("todo_process_panic_type")}
                _ => {panic!("Unreachable code")}
            });
            todo!("E_var")
        }}
        Privilege::User => {{
            sail_ctx.ucause.bits = {
                let var_8 = bool_to_bits(sail_ctx, intr);
                sail_ctx.ucause.bits.set_subrange::<63, 64, 1>(var_8)
            };
            sail_ctx.ucause.bits = {
                let var_9 = c.zero_extend::<63>();
                sail_ctx.ucause.bits.set_subrange::<0, 63, 63>(var_9)
            };
            sail_ctx.mstatus.bits = {
                let var_10 = _get_Mstatus_UIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.bits.set_subrange::<4, 5, 1>(var_10)
            };
            sail_ctx.mstatus.bits = sail_ctx.mstatus.bits.set_subrange::<0, 1, 1>(BitVector::<1>::new(0b0));
            todo!("E_var")
        }}
        _ => {panic!("Unreachable code")}
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    RISCV_ADDI,
    RISCV_SLTI,
    RISCV_SLTIU,
    RISCV_XORI,
    RISCV_ORI,
    RISCV_ANDI
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum csrop {
    CSRRW,
    CSRRS,
    CSRRC
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ITYPE((BitVector<12>, regidx, regidx, iop)),
    CSR((BitVector<12>, regidx, regidx, bool, csrop))
}

pub type csrRW = BitVector<2>;