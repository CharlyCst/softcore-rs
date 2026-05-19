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
    pub mtval: xlenbits,
    pub stval: xlenbits,
    pub utval: xlenbits,
    pub mscratch: xlenbits,
    pub sscratch: xlenbits,
    pub mepc: xlenbits,
    pub sepc: xlenbits,
    pub uepc: xlenbits,
    pub medeleg: Medeleg,
    pub mcause: Mcause,
    pub scause: Mcause,
    pub ucause: Mcause,
    pub mstatus: Mstatus,
    pub mtvec: Mtvec,
    pub stvec: Mtvec,
    pub utvec: Mtvec,
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

/// signed_extend
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L10.
pub fn signed_extend(m: i128, v: BitDynamic) -> BitDynamic {
    sail_sign_extend(v, m)
}

/// bit_to_bool
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L19-22.
pub fn bit_to_bool(b: bool) -> bool {
    match b {
        true => {true}
        false => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_to_bit
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L25.
pub fn bool_to_bit(x: bool) -> bool {
    if {x} {
        true
    } else {
        false
    }
}

/// bool_to_bits
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L28.
pub fn bool_to_bits(x: bool) -> BitDynamic {
    [bool_to_bit(x)]
}

/// (operator >=_u)
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L42.
pub fn _operator_biggerequal_u_(x: BitDynamic, y: BitDynamic) -> bool {
    (x.unsigned() >= y.unsigned())
}

/// (operator <_u)
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L45.
pub fn _operator_smaller_u_(x: BitDynamic, y: BitDynamic) -> bool {
    (x.unsigned() < y.unsigned())
}

/// xlen
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L50.
pub const xlen: i128 = 64;

/// xlen_bytes
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L51.
pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitDynamic;

pub type priv_level = BitDynamic;

/// Privilege
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L56.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

/// privLevel_to_bits
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L59-64.
pub fn privLevel_to_bits(p: Privilege) -> BitDynamic {
    match p {
        Privilege::User => {BitDynamic::new(2, 0b00)}
        Privilege::Supervisor => {BitDynamic::new(2, 0b01)}
        Privilege::Machine => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// haveSupMode
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L66.
pub const fn haveSupMode(unit_arg: ()) -> bool {
    true
}

/// haveUsrMode
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L67.
pub const fn haveUsrMode(unit_arg: ()) -> bool {
    true
}

/// exception
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L73-76.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_internal_error(())
}

pub type regidx = BitDynamic;

pub type cregidx = BitDynamic;

pub type csreg = BitDynamic;

/// Medeleg
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L99-114.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Medeleg {
    pub bits: BitDynamic,
}

/// Mcause
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L117-120.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mcause {
    pub bits: BitDynamic,
}

/// Mstatus
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L125-149.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitDynamic,
}

/// Mtvec
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L152-155.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mtvec {
    pub bits: BitDynamic,
}

/// _get_Mcause_Cause
///
/// Generated from the Sail sources.
pub fn _get_Mcause_Cause(v: Mcause) -> BitDynamic {
    v.bits.subrange::<0, 63, 63>()
}

/// _get_Mcause_IsInterrupt
///
/// Generated from the Sail sources.
pub fn _get_Mcause_IsInterrupt(v: Mcause) -> BitDynamic {
    v.bits.subrange::<63, 64, 1>()
}

/// _get_Mstatus_MIE
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_MIE(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<3, 4, 1>()
}

/// _get_Mstatus_SIE
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_SIE(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<1, 2, 1>()
}

/// _get_Mstatus_UIE
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_UIE(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<0, 1, 1>()
}

/// _get_Mtvec_Base
///
/// Generated from the Sail sources.
pub fn _get_Mtvec_Base(v: Mtvec) -> BitDynamic {
    v.bits.subrange::<2, 64, 62>()
}

/// _get_Mtvec_Mode
///
/// Generated from the Sail sources.
pub fn _get_Mtvec_Mode(v: Mtvec) -> BitDynamic {
    v.bits.subrange::<0, 2, 2>()
}

/// rX
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L168-172.
pub fn rX(core_ctx: &mut Core, r: BitDynamic) -> BitDynamic {
    match r {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00000))} => {BitDynamic::new(4, 0b0000).zero_extend_dyn(64)}
        _ => {core_ctx.Xs[(r.unsigned() as usize)]}
        _ => {panic!("Unreachable code")}
    }
}

/// wX
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L175-178.
pub fn wX(core_ctx: &mut Core, r: BitDynamic, v: BitDynamic) {
    if {(r != BitDynamic::new(5, 0b00000))} {
        core_ctx.Xs[(r.unsigned() as usize)] = v
    } else {
        ()
    }
}

/// bool_bits_forwards
///
/// Generated from the Sail sources.
pub fn bool_bits_forwards(arg_hashtag_: bool) -> BitDynamic {
    match arg_hashtag_ {
        true => {BitDynamic::new(1, 0b1)}
        false => {BitDynamic::new(1, 0b0)}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_bits_backwards
///
/// Generated from the Sail sources.
pub fn bool_bits_backwards(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(1, 0b1))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_bits_backwards_matches
///
/// Generated from the Sail sources.
pub fn bool_bits_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(1, 0b1))} => {true}
        b__1 if {(b__1 == BitDynamic::new(1, 0b0))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// ExceptionType
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L193-210.
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

pub type exc_code = BitDynamic;

/// num_of_ExceptionType
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L215-233.
pub fn num_of_ExceptionType(e: ExceptionType) -> i128 {
    match e {
        ExceptionType::E_Fetch_Addr_Align(()) => {0}
        ExceptionType::E_Fetch_Access_Fault(()) => {1}
        ExceptionType::E_Illegal_Instr(()) => {2}
        ExceptionType::E_Breakpoint(()) => {3}
        ExceptionType::E_Load_Addr_Align(()) => {4}
        ExceptionType::E_Load_Access_Fault(()) => {5}
        ExceptionType::E_SAMO_Addr_Align(()) => {6}
        ExceptionType::E_SAMO_Access_Fault(()) => {7}
        ExceptionType::E_U_EnvCall(()) => {8}
        ExceptionType::E_S_EnvCall(()) => {9}
        ExceptionType::E_Reserved_10(()) => {10}
        ExceptionType::E_M_EnvCall(()) => {11}
        ExceptionType::E_Fetch_Page_Fault(()) => {12}
        ExceptionType::E_Load_Page_Fault(()) => {13}
        ExceptionType::E_Reserved_14(()) => {14}
        ExceptionType::E_SAMO_Page_Fault(()) => {15}
        _ => {panic!("Unreachable code")}
    }
}

/// exceptionType_to_bits
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L236-254.
pub fn exceptionType_to_bits(e: ExceptionType) -> BitDynamic {
    match e {
        ExceptionType::E_Fetch_Addr_Align(()) => {BitDynamic::new(8, 0b00000000)}
        ExceptionType::E_Fetch_Access_Fault(()) => {BitDynamic::new(8, 0b00000001)}
        ExceptionType::E_Illegal_Instr(()) => {BitDynamic::new(8, 0b00000010)}
        ExceptionType::E_Breakpoint(()) => {BitDynamic::new(8, 0b00000011)}
        ExceptionType::E_Load_Addr_Align(()) => {BitDynamic::new(8, 0b00000100)}
        ExceptionType::E_Load_Access_Fault(()) => {BitDynamic::new(8, 0b00000101)}
        ExceptionType::E_SAMO_Addr_Align(()) => {BitDynamic::new(8, 0b00000110)}
        ExceptionType::E_SAMO_Access_Fault(()) => {BitDynamic::new(8, 0b00000111)}
        ExceptionType::E_U_EnvCall(()) => {BitDynamic::new(8, 0b00001000)}
        ExceptionType::E_S_EnvCall(()) => {BitDynamic::new(8, 0b00001001)}
        ExceptionType::E_Reserved_10(()) => {BitDynamic::new(8, 0b00001010)}
        ExceptionType::E_M_EnvCall(()) => {BitDynamic::new(8, 0b00001011)}
        ExceptionType::E_Fetch_Page_Fault(()) => {BitDynamic::new(8, 0b00001100)}
        ExceptionType::E_Load_Page_Fault(()) => {BitDynamic::new(8, 0b00001101)}
        ExceptionType::E_Reserved_14(()) => {BitDynamic::new(8, 0b00001110)}
        ExceptionType::E_SAMO_Page_Fault(()) => {BitDynamic::new(8, 0b00001111)}
        _ => {panic!("Unreachable code")}
    }
}

/// sync_exception
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L256-259.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct sync_exception {
    pub trap: ExceptionType,
    pub excinfo: Option<xlenbits>,
}

/// ctl_result
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L262-264.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ctl_result {
    CTL_TRAP(sync_exception)
}

pub type tv_mode = BitDynamic;

/// TrapVectorMode
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L267.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TrapVectorMode {
    TV_Direct,
    TV_Vector,
    TV_Reserved
}

/// set_next_pc
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L270-272.
pub fn set_next_pc(core_ctx: &mut Core, pc: BitDynamic) {
    core_ctx.nextPC = pc
}

/// tval
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L274-279.
pub fn tval(excinfo: Option<BitDynamic>) -> BitDynamic {
    match excinfo {
        Some(e) => {e}
        None => {BitDynamic::new(1, 0b0).zero_extend_dyn(64)}
        _ => {panic!("Unreachable code")}
    }
}

/// trapVectorMode_of_bits
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L282-287.
pub fn trapVectorMode_of_bits(m: BitDynamic) -> TrapVectorMode {
    match m {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {TrapVectorMode::TV_Direct}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {TrapVectorMode::TV_Vector}
        _ => {TrapVectorMode::TV_Reserved}
        _ => {panic!("Unreachable code")}
    }
}

/// tvec_addr
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L290-299.
pub fn tvec_addr(m: Mtvec, c: Mcause) -> Option<BitDynamic> {
    let base: xlenbits = bitvector_concat(BitDynamic::from(_get_Mtvec_Base(m)), BitDynamic::from(BitDynamic::new(2, 0b00)));
    match trapVectorMode_of_bits(_get_Mtvec_Mode(m)) {
        TrapVectorMode::TV_Direct => {Some(base)}
        TrapVectorMode::TV_Vector => {if {(_get_Mcause_IsInterrupt(c) == BitDynamic::new(1, 0b1))} {
            Some(base.wrapped_add((_get_Mcause_Cause(c).zero_extend_dyn(64) << 2)))
        } else {
            Some(base)
        }}
        TrapVectorMode::TV_Reserved => {None}
        _ => {panic!("Unreachable code")}
    }
}

/// prepare_trap_vector
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L302-312.
pub fn prepare_trap_vector(core_ctx: &mut Core, p: Privilege, cause: Mcause) -> BitDynamic {
    let tvec: Mtvec = match p {
        Privilege::Machine => {core_ctx.mtvec}
        Privilege::Supervisor => {core_ctx.stvec}
        Privilege::User => {core_ctx.utvec}
        _ => {panic!("Unreachable code")}
    };
    match tvec_addr(tvec, cause) {
        Some(epc) => {epc}
        None => {panic!("todo_process_panic_type")}
        _ => {panic!("Unreachable code")}
    }
}

/// trap_handler
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L314-365.
pub fn trap_handler(core_ctx: &mut Core, del_priv: Privilege, intr: bool, c: BitDynamic, pc: BitDynamic, info: Option<BitDynamic>) -> BitDynamic {
    match del_priv {
        Privilege::Machine => {{
            core_ctx.mcause.bits = core_ctx.mcause.bits.set_subrange(bool_to_bits(intr), 63, 63);
            core_ctx.mcause.bits = core_ctx.mcause.bits.set_subrange(c.zero_extend_dyn(63), 62, 0);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange({
                let var_1: Mstatus = core_ctx.mstatus;
                _get_Mstatus_MIE(var_1)
            }, 7, 7);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 3, 3);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange({
                let var_2: Privilege = core_ctx.cur_privilege;
                privLevel_to_bits(var_2)
            }, 12, 11);
            core_ctx.mtval = tval(info);
            core_ctx.mepc = pc;
            core_ctx.cur_privilege = del_priv;
            {
                let var_3: Mcause = core_ctx.mcause;
                prepare_trap_vector(core_ctx, del_priv, var_3)
            }
        }}
        Privilege::Supervisor => {{
            assert!(true, "no supervisor mode present for delegation");
            core_ctx.scause.bits = core_ctx.scause.bits.set_subrange(bool_to_bits(intr), 63, 63);
            core_ctx.scause.bits = core_ctx.scause.bits.set_subrange(c.zero_extend_dyn(63), 62, 0);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange({
                let var_4: Mstatus = core_ctx.mstatus;
                _get_Mstatus_SIE(var_4)
            }, 5, 5);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 1, 1);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(match core_ctx.cur_privilege {
                Privilege::User => {BitDynamic::new(1, 0b0)}
                Privilege::Supervisor => {BitDynamic::new(1, 0b1)}
                Privilege::Machine => {panic!("todo_process_panic_type")}
                _ => {panic!("Unreachable code")}
            }, 8, 8);
            core_ctx.stval = tval(info);
            core_ctx.sepc = pc;
            core_ctx.cur_privilege = del_priv;
            {
                let var_5: Mcause = core_ctx.scause;
                prepare_trap_vector(core_ctx, del_priv, var_5)
            }
        }}
        Privilege::User => {{
            core_ctx.ucause.bits = core_ctx.ucause.bits.set_subrange(bool_to_bits(intr), 63, 63);
            core_ctx.ucause.bits = core_ctx.ucause.bits.set_subrange(c.zero_extend_dyn(63), 62, 0);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange({
                let var_6: Mstatus = core_ctx.mstatus;
                _get_Mstatus_UIE(var_6)
            }, 4, 4);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 0, 0);
            core_ctx.utval = tval(info);
            core_ctx.uepc = pc;
            core_ctx.cur_privilege = del_priv;
            {
                let var_7: Mcause = core_ctx.ucause;
                prepare_trap_vector(core_ctx, del_priv, var_7)
            }
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// exception_delegatee
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L370-380.
pub fn exception_delegatee(core_ctx: &mut Core, e: ExceptionType, p: Privilege) -> Privilege {
    let idx: i128 = num_of_ExceptionType(e);
    let _super_: bool = {
        let var_1: bool = bitvector_access(core_ctx.medeleg.bits, idx);
        bit_to_bool(var_1)
    };
    let deleg: Privilege = if {_super_} {
        Privilege::Supervisor
    } else {
        Privilege::Machine
    };
    if {_operator_smaller_u_(privLevel_to_bits(deleg), privLevel_to_bits(p))} {
        p
    } else {
        deleg
    }
}

/// exception_handler
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L382-390.
pub fn exception_handler(core_ctx: &mut Core, cur_priv: Privilege, ctl: ctl_result, pc: BitDynamic) -> BitDynamic {
    match (cur_priv, ctl) {
        (_, ctl_result::CTL_TRAP(e)) => {{
            let del_priv: Privilege = exception_delegatee(core_ctx, e.trap, cur_priv);
            trap_handler(core_ctx, del_priv, false, exceptionType_to_bits(e.trap), pc, e.excinfo)
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// handle_illegal
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L392-396.
pub fn handle_illegal(core_ctx: &mut Core, unit_arg: ()) {
    let t: sync_exception = sync_exception {
        trap: ExceptionType::E_Illegal_Instr(()),
        excinfo: None
    };
    {
        let var_1: BitDynamic = {
            let var_2: Privilege = core_ctx.cur_privilege;
            let var_3: BitDynamic = core_ctx.PC;
            exception_handler(core_ctx, var_2, ctl_result::CTL_TRAP(t), var_3)
        };
        set_next_pc(core_ctx, var_1)
    }
}

/// iop
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L400.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    RISCV_ADDI,
    RISCV_SLTI,
    RISCV_SLTIU,
    RISCV_XORI,
    RISCV_ORI,
    RISCV_ANDI
}

/// csrop
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L401.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum csrop {
    CSRRW,
    CSRRS,
    CSRRC
}

/// Retired
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L402.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL
}

/// ast
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L404.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ITYPE((BitDynamic, regidx, regidx, iop)),
    CSR((BitDynamic, regidx, regidx, bool, csrop))
}

/// encdec_csrop_forwards
///
/// Generated from the Sail sources.
pub fn encdec_csrop_forwards(arg_hashtag_: csrop) -> BitDynamic {
    match arg_hashtag_ {
        csrop::CSRRW => {BitDynamic::new(2, 0b01)}
        csrop::CSRRS => {BitDynamic::new(2, 0b10)}
        csrop::CSRRC => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_csrop_backwards
///
/// Generated from the Sail sources.
pub fn encdec_csrop_backwards(arg_hashtag_: BitDynamic) -> csrop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b01))} => {csrop::CSRRW}
        b__1 if {(b__1 == BitDynamic::new(2, 0b10))} => {csrop::CSRRS}
        b__2 if {(b__2 == BitDynamic::new(2, 0b11))} => {csrop::CSRRC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_csrop_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_csrop_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b01))} => {true}
        b__1 if {(b__1 == BitDynamic::new(2, 0b10))} => {true}
        b__2 if {(b__2 == BitDynamic::new(2, 0b11))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

pub type csrRW = BitDynamic;

/// csrAccess
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L436.
pub fn csrAccess(csr: BitDynamic) -> BitDynamic {
    csr.subrange::<10, 12, 2>()
}

/// csrPriv
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L437.
pub fn csrPriv(csr: BitDynamic) -> BitDynamic {
    csr.subrange::<8, 10, 2>()
}

/// encdec_forwards
///
/// Generated from the Sail sources.
pub fn encdec_forwards(arg_hashtag_: ast) -> BitDynamic {
    match arg_hashtag_ {
        ast::ITYPE((imm, rs1, rd, iop::RISCV_ADDI)) => {bitvector_concat(BitDynamic::from((imm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs1 as regidx)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from((rd as regidx)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))}
        ast::CSR((csr, rs1, rd, is_imm, op)) => {bitvector_concat(BitDynamic::from((csr as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((rs1 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(bool_bits_forwards(is_imm)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_csrop_forwards(op)), BitDynamic::from(bitvector_concat(BitDynamic::from((rd as BitDynamic)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_backwards
///
/// Generated from the Sail sources.
pub fn encdec_backwards(arg_hashtag_: BitDynamic) -> ast {
    let head_exp_hashtag_: BitDynamic = arg_hashtag_;
    match match head_exp_hashtag_ {
        v__0 if {((v__0.subrange::<12, 15, 3>() == BitDynamic::new(3, 0b000)) && (v__0.subrange::<0, 7, 7>() == BitDynamic::new(7, 0b0010011)))} => {let imm: BitDynamic = v__0.subrange::<20, 32, 12>();
        let rs1: regidx = v__0.subrange::<15, 20, 5>();
        let rd: regidx = v__0.subrange::<7, 12, 5>();
        let imm: BitDynamic = v__0.subrange::<20, 32, 12>();
        Some(ast::ITYPE((imm, rs1, rd, iop::RISCV_ADDI)))}
        v__3 if {let mapping1_hashtag__var_1: BitDynamic = v__3.subrange::<12, 14, 2>();
        let mapping0_hashtag__var_2: BitDynamic = v__3.subrange::<14, 15, 1>();
        ((bool_bits_backwards_matches(mapping0_hashtag__var_2) && encdec_csrop_backwards_matches(mapping1_hashtag__var_1)) && (v__3.subrange::<0, 7, 7>() == BitDynamic::new(7, 0b1110011)))} => {let csr: BitDynamic = v__3.subrange::<20, 32, 12>();
        let rs1: BitDynamic = v__3.subrange::<15, 20, 5>();
        let rd: BitDynamic = v__3.subrange::<7, 12, 5>();
        let mapping1_hashtag_: BitDynamic = v__3.subrange::<12, 14, 2>();
        let mapping0_hashtag_: BitDynamic = v__3.subrange::<14, 15, 1>();
        let csr: BitDynamic = v__3.subrange::<20, 32, 12>();
        match (bool_bits_backwards(mapping0_hashtag_), encdec_csrop_backwards(mapping1_hashtag_)) {
            (is_imm, op) => {Some(ast::CSR((csr, rs1, rd, is_imm, op)))}
            _ => {None}
            _ => {panic!("Unreachable code")}
        }}
        _ => {None}
        _ => {panic!("Unreachable code")}
    } {
        Some(result) => {result}
        _ => {panic!("Unreachable code")}
    }
}

/// is_CSR_defined
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L442-447.
pub fn is_CSR_defined(csr: BitDynamic, p: Privilege) -> bool {
    match csr {
        b__0 if {(b__0 == BitDynamic::new(12, 0b001101000000))} => {(p == Privilege::Machine)}
        b__1 if {(b__1 == BitDynamic::new(12, 0b000101000000))} => {((p == Privilege::Machine) || (p == Privilege::Supervisor))}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// check_CSR_access
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L450-452.
pub fn check_CSR_access(csrrw: BitDynamic, csrpr: BitDynamic, p: Privilege, isWrite: bool) -> bool {
    (!(((isWrite == true) && (csrrw == BitDynamic::new(2, 0b11)))) && _operator_biggerequal_u_(privLevel_to_bits(p), csrpr))
}

/// check_CSR
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L454-455.
pub fn check_CSR(csr: BitDynamic, p: Privilege, isWrite: bool) -> bool {
    (is_CSR_defined(csr, p) && check_CSR_access(csrAccess(csr), csrPriv(csr), p, isWrite))
}

/// readCSR
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L457-465.
pub fn readCSR(core_ctx: &mut Core, csr: BitDynamic) -> BitDynamic {
    let res: xlenbits = match (csr, 64) {
        (b__0, _) if {(b__0 == BitDynamic::new(12, 0b001101000000))} => {core_ctx.mscratch}
        (b__1, _) if {(b__1 == BitDynamic::new(12, 0b000101000000))} => {core_ctx.sscratch}
        _ => {BitDynamic::new(4, 0b0000).zero_extend_dyn(64)}
        _ => {panic!("Unreachable code")}
    };
    res
}

/// writeCSR
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L467-474.
pub fn writeCSR(core_ctx: &mut Core, csr: BitDynamic, value: BitDynamic) {
    let res: Option<xlenbits> = match (csr, 64) {
        (b__0, _) if {(b__0 == BitDynamic::new(12, 0b001101000000))} => {{
            core_ctx.mscratch = value;
            Some(core_ctx.mscratch)
        }}
        (b__1, _) if {(b__1 == BitDynamic::new(12, 0b000101000000))} => {{
            core_ctx.sscratch = value;
            Some(core_ctx.sscratch)
        }}
        _ => {None}
        _ => {panic!("Unreachable code")}
    };
    ()
}

/// execute
///
/// Generated from the Sail sources at `tests/trap/arch.sail` L417-423.
pub fn execute(core_ctx: &mut Core, merge_hashtag_var: ast) -> Retired {
    match merge_hashtag_var {
        ast::ITYPE((imm, rs1, rd, iop::RISCV_ADDI)) => {{
            let rs1_val: BitDynamic = rX(core_ctx, rs1);
            let imm_ext: xlenbits = signed_extend(64, imm);
            let result: BitDynamic = rs1_val.wrapped_add(imm_ext);
            wX(core_ctx, rd, result);
            Retired::RETIRE_SUCCESS
        }}
        ast::CSR((csr, rs1, rd, is_imm, op)) => {{
            let rs1_val: xlenbits = if {is_imm} {
                rs1.zero_extend_dyn(64)
            } else {
                rX(core_ctx, rs1)
            };
            let isWrite: bool = match op {
                csrop::CSRRW => {true}
                _ => {if {is_imm} {
                    (rs1_val.unsigned() != 0)
                } else {
                    (rs1.unsigned() != 0)
                }}
                _ => {panic!("Unreachable code")}
            };
            if {!({
                let var_1: Privilege = core_ctx.cur_privilege;
                check_CSR(csr, var_1, isWrite)
            })} {
                handle_illegal(core_ctx, ());
                Retired::RETIRE_FAIL
            } else {
                let csr_val: BitDynamic = readCSR(core_ctx, csr);
                if {isWrite} {
                    let new_val: xlenbits = match op {
                        csrop::CSRRW => {rs1_val}
                        csrop::CSRRS => {(csr_val | rs1_val)}
                        csrop::CSRRC => {(csr_val & !(rs1_val))}
                        _ => {panic!("Unreachable code")}
                    };
                    writeCSR(core_ctx, csr, new_val)
                } else {
                    ()
                };
                wX(core_ctx, rd, csr_val);
                Retired::RETIRE_SUCCESS
            }
        }}
        _ => {panic!("Unreachable code")}
    }
}
