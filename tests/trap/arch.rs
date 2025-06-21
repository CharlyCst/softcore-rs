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
pub fn bool_to_bits(x: bool) -> BitVector<1> {
    BitVector::new(0).set_bit(0, bool_to_bit(x))
}

/// (operator <_u)
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L45.
pub fn _operator_smaller_u_<const N: i128>(x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.unsigned() < y.unsigned())
}

pub const xlen: i128 = 64;

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

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
pub fn privLevel_to_bits(p: Privilege) -> BitVector<2> {
    match p {
        Privilege::User => {BitVector::<2>::new(0b00)}
        Privilege::Supervisor => {BitVector::<2>::new(0b01)}
        Privilege::Machine => {BitVector::<2>::new(0b11)}
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

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

/// Medeleg
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L99-114.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Medeleg {
    pub bits: BitVector<64>,
}

/// Mcause
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L117-120.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mcause {
    pub bits: BitVector<64>,
}

/// Mstatus
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L125-149.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitVector<64>,
}

/// Mtvec
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L152-155.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mtvec {
    pub bits: BitVector<64>,
}

/// _get_Mcause_Cause
/// 
/// Generated from the Sail sources.
pub fn _get_Mcause_Cause(v: Mcause) -> BitVector<63> {
    v.bits.subrange::<0, 63, 63>()
}

/// _get_Mcause_IsInterrupt
/// 
/// Generated from the Sail sources.
pub fn _get_Mcause_IsInterrupt(v: Mcause) -> BitVector<1> {
    v.bits.subrange::<63, 64, 1>()
}

/// _get_Mstatus_MIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<3, 4, 1>()
}

/// _get_Mstatus_SIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_SIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// _get_Mstatus_UIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_UIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<0, 1, 1>()
}

/// _get_Mtvec_Base
/// 
/// Generated from the Sail sources.
pub fn _get_Mtvec_Base(v: Mtvec) -> BitVector<62> {
    v.bits.subrange::<2, 64, 62>()
}

/// _get_Mtvec_Mode
/// 
/// Generated from the Sail sources.
pub fn _get_Mtvec_Mode(v: Mtvec) -> BitVector<2> {
    v.bits.subrange::<0, 2, 2>()
}

/// rX
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L168-172.
pub fn rX(core_ctx: &mut Core, r: BitVector<5>) -> BitVector<64> {
    match r {
        b__0 if {(b__0 == BitVector::<5>::new(0b00000))} => {BitVector::<4>::new(0b0000).zero_extend::<64>()}
        _ => {core_ctx.Xs[(r.unsigned() as usize)]}
        _ => {panic!("Unreachable code")}
    }
}

/// wX
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L175-178.
pub fn wX(core_ctx: &mut Core, r: BitVector<5>, v: BitVector<64>) {
    if {(r != BitVector::<5>::new(0b00000))} {
        core_ctx.Xs[(r.unsigned() as usize)] = v
    } else {
        ()
    }
}

/// bool_bits_backwards
/// 
/// Generated from the Sail sources.
pub fn bool_bits_backwards(arg_hashtag_: BitVector<1>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<1>::new(0b1))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_bits_backwards_matches
/// 
/// Generated from the Sail sources.
pub fn bool_bits_backwards_matches(arg_hashtag_: BitVector<1>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<1>::new(0b1))} => {true}
        b__1 if {(b__1 == BitVector::<1>::new(0b0))} => {true}
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

pub type exc_code = BitVector<8>;

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
pub fn exceptionType_to_bits(e: ExceptionType) -> BitVector<8> {
    match e {
        ExceptionType::E_Fetch_Addr_Align(()) => {BitVector::<8>::new(0b00000000)}
        ExceptionType::E_Fetch_Access_Fault(()) => {BitVector::<8>::new(0b00000001)}
        ExceptionType::E_Illegal_Instr(()) => {BitVector::<8>::new(0b00000010)}
        ExceptionType::E_Breakpoint(()) => {BitVector::<8>::new(0b00000011)}
        ExceptionType::E_Load_Addr_Align(()) => {BitVector::<8>::new(0b00000100)}
        ExceptionType::E_Load_Access_Fault(()) => {BitVector::<8>::new(0b00000101)}
        ExceptionType::E_SAMO_Addr_Align(()) => {BitVector::<8>::new(0b00000110)}
        ExceptionType::E_SAMO_Access_Fault(()) => {BitVector::<8>::new(0b00000111)}
        ExceptionType::E_U_EnvCall(()) => {BitVector::<8>::new(0b00001000)}
        ExceptionType::E_S_EnvCall(()) => {BitVector::<8>::new(0b00001001)}
        ExceptionType::E_Reserved_10(()) => {BitVector::<8>::new(0b00001010)}
        ExceptionType::E_M_EnvCall(()) => {BitVector::<8>::new(0b00001011)}
        ExceptionType::E_Fetch_Page_Fault(()) => {BitVector::<8>::new(0b00001100)}
        ExceptionType::E_Load_Page_Fault(()) => {BitVector::<8>::new(0b00001101)}
        ExceptionType::E_Reserved_14(()) => {BitVector::<8>::new(0b00001110)}
        ExceptionType::E_SAMO_Page_Fault(()) => {BitVector::<8>::new(0b00001111)}
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

pub type tv_mode = BitVector<2>;

/// TrapVectorMode
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L267.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TrapVectorMode {
    TV_Direct,
    TV_Vector,
    TV_Reserved
}

/// tval
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L274-279.
pub fn tval(excinfo: Option<BitVector<64>>) -> BitVector<64> {
    match excinfo {
        Some(e) => {e}
        None => {BitVector::<1>::new(0b0).zero_extend::<64>()}
        _ => {panic!("Unreachable code")}
    }
}

/// trapVectorMode_of_bits
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L282-287.
pub fn trapVectorMode_of_bits(m: BitVector<2>) -> TrapVectorMode {
    match m {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {TrapVectorMode::TV_Direct}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {TrapVectorMode::TV_Vector}
        _ => {TrapVectorMode::TV_Reserved}
        _ => {panic!("Unreachable code")}
    }
}

/// tvec_addr
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L290-299.
pub fn tvec_addr(m: Mtvec, c: Mcause) -> Option<BitVector<64>> {
    let base: xlenbits = bitvector_concat::<62, 2, 64>(_get_Mtvec_Base(m), BitVector::<2>::new(0b00));
    match trapVectorMode_of_bits(_get_Mtvec_Mode(m)) {
        TrapVectorMode::TV_Direct => {Some(base)}
        TrapVectorMode::TV_Vector => {if {(_get_Mcause_IsInterrupt(c) == BitVector::<1>::new(0b1))} {
            Some(base.wrapped_add((_get_Mcause_Cause(c).zero_extend::<64>() << 2)))
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
pub fn prepare_trap_vector(core_ctx: &mut Core, p: Privilege, cause: Mcause) -> BitVector<64> {
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
pub fn trap_handler(core_ctx: &mut Core, del_priv: Privilege, intr: bool, c: BitVector<8>, pc: BitVector<64>, info: Option<BitVector<64>>) -> BitVector<64> {
    match del_priv {
        Privilege::Machine => {{
            core_ctx.mcause.bits = core_ctx.mcause.bits.set_subrange::<63, 64, 1>(bool_to_bits(intr));
            core_ctx.mcause.bits = core_ctx.mcause.bits.set_subrange::<0, 63, 63>(c.zero_extend::<63>());
            core_ctx.mstatus.bits = {
                let var_1 = {
                    let var_2 = core_ctx.mstatus;
                    _get_Mstatus_MIE(var_2)
                };
                core_ctx.mstatus.bits.set_subrange::<7, 8, 1>(var_1)
            };
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<3, 4, 1>(BitVector::<1>::new(0b0));
            core_ctx.mstatus.bits = {
                let var_3 = {
                    let var_4 = core_ctx.cur_privilege;
                    privLevel_to_bits(var_4)
                };
                core_ctx.mstatus.bits.set_subrange::<11, 13, 2>(var_3)
            };
            core_ctx.mtval = tval(info);
            core_ctx.mepc = pc;
            core_ctx.cur_privilege = del_priv;
            {
                let var_5 = core_ctx.mcause;
                prepare_trap_vector(core_ctx, del_priv, var_5)
            }
        }}
        Privilege::Supervisor => {{
            assert!(haveSupMode(()), "no supervisor mode present for delegation");
            core_ctx.scause.bits = core_ctx.scause.bits.set_subrange::<63, 64, 1>(bool_to_bits(intr));
            core_ctx.scause.bits = core_ctx.scause.bits.set_subrange::<0, 63, 63>(c.zero_extend::<63>());
            core_ctx.mstatus.bits = {
                let var_6 = {
                    let var_7 = core_ctx.mstatus;
                    _get_Mstatus_SIE(var_7)
                };
                core_ctx.mstatus.bits.set_subrange::<5, 6, 1>(var_6)
            };
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<1, 2, 1>(BitVector::<1>::new(0b0));
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<8, 9, 1>(match core_ctx.cur_privilege {
                Privilege::User => {BitVector::<1>::new(0b0)}
                Privilege::Supervisor => {BitVector::<1>::new(0b1)}
                Privilege::Machine => {panic!("todo_process_panic_type")}
                _ => {panic!("Unreachable code")}
            });
            core_ctx.stval = tval(info);
            core_ctx.sepc = pc;
            core_ctx.cur_privilege = del_priv;
            {
                let var_8 = core_ctx.scause;
                prepare_trap_vector(core_ctx, del_priv, var_8)
            }
        }}
        Privilege::User => {{
            core_ctx.ucause.bits = core_ctx.ucause.bits.set_subrange::<63, 64, 1>(bool_to_bits(intr));
            core_ctx.ucause.bits = core_ctx.ucause.bits.set_subrange::<0, 63, 63>(c.zero_extend::<63>());
            core_ctx.mstatus.bits = {
                let var_9 = {
                    let var_10 = core_ctx.mstatus;
                    _get_Mstatus_UIE(var_10)
                };
                core_ctx.mstatus.bits.set_subrange::<4, 5, 1>(var_9)
            };
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<0, 1, 1>(BitVector::<1>::new(0b0));
            core_ctx.utval = tval(info);
            core_ctx.uepc = pc;
            core_ctx.cur_privilege = del_priv;
            {
                let var_11 = core_ctx.ucause;
                prepare_trap_vector(core_ctx, del_priv, var_11)
            }
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// exception_delegatee
/// 
/// Generated from the Sail sources at `tests/trap/arch.sail` L370-380.
pub fn exception_delegatee(core_ctx: &mut Core, e: ExceptionType, p: Privilege) -> Privilege {
    let idx = num_of_ExceptionType(e);
    let _super_ = {
        let var_1 = bitvector_access(core_ctx.medeleg.bits, idx);
        bit_to_bool(var_1)
    };
    let user = false;
    let deleg = if {(haveUsrMode(()) && user)} {
        Privilege::User
    } else if {(haveSupMode(()) && _super_)} {
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
    ITYPE((BitVector<12>, regidx, regidx, iop)),
    CSR((BitVector<12>, regidx, regidx, bool, csrop))
}

/// encdec_csrop_backwards
/// 
/// Generated from the Sail sources.
pub fn encdec_csrop_backwards(arg_hashtag_: BitVector<2>) -> csrop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b01))} => {csrop::CSRRW}
        b__1 if {(b__1 == BitVector::<2>::new(0b10))} => {csrop::CSRRS}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {csrop::CSRRC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_csrop_backwards_matches
/// 
/// Generated from the Sail sources.
pub fn encdec_csrop_backwards_matches(arg_hashtag_: BitVector<2>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b01))} => {true}
        b__1 if {(b__1 == BitVector::<2>::new(0b10))} => {true}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

pub type csrRW = BitVector<2>;

/// encdec_backwards
/// 
/// Generated from the Sail sources.
pub fn encdec_backwards(arg_hashtag_: BitVector<32>) -> ast {
    let head_exp_hashtag_ = arg_hashtag_;
    match match head_exp_hashtag_ {
        v__0 if {((v__0.subrange::<12, 15, 3>() == BitVector::<3>::new(0b000)) && (v__0.subrange::<0, 7, 7>() == BitVector::<7>::new(0b0010011)))} => {let imm: BitVector<12> = v__0.subrange::<20, 32, 12>();
        let rs1: regidx = v__0.subrange::<15, 20, 5>();
        let rd: regidx = v__0.subrange::<7, 12, 5>();
        let imm: BitVector<12> = v__0.subrange::<20, 32, 12>();
        Some(ast::ITYPE((imm, rs1, rd, iop::RISCV_ADDI)))}
        v__3 if {let mapping1_hashtag__var_1: BitVector<2> = v__3.subrange::<12, 14, 2>();
        let mapping0_hashtag__var_2: BitVector<1> = v__3.subrange::<14, 15, 1>();
        ((bool_bits_backwards_matches(mapping0_hashtag__var_2) && encdec_csrop_backwards_matches(mapping1_hashtag__var_1)) && (v__3.subrange::<0, 7, 7>() == BitVector::<7>::new(0b1110011)))} => {let csr: BitVector<12> = v__3.subrange::<20, 32, 12>();
        let rs1: BitVector<5> = v__3.subrange::<15, 20, 5>();
        let rd: BitVector<5> = v__3.subrange::<7, 12, 5>();
        let mapping1_hashtag_: BitVector<2> = v__3.subrange::<12, 14, 2>();
        let mapping0_hashtag_: BitVector<1> = v__3.subrange::<14, 15, 1>();
        let csr: BitVector<12> = v__3.subrange::<20, 32, 12>();
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
pub fn is_CSR_defined(csr: BitVector<12>, p: Privilege) -> bool {
    match csr {
        b__0 if {(b__0 == BitVector::<12>::new(0b001101000000))} => {(p == Privilege::Machine)}
        b__1 if {(b__1 == BitVector::<12>::new(0b000101000000))} => {((p == Privilege::Machine) || (p == Privilege::Supervisor))}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}
