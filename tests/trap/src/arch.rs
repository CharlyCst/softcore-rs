#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

pub type priv_level = BitVector<2>;

pub type regidx = BitVector<5>;

pub type cregidx = BitVector<3>;

pub type csreg = BitVector<12>;

pub type Medeleg = BitField<64>;

pub type Mcause = BitField<64>;

pub type Mstatus = BitField<64>;

pub type Mtvec = BitField<64>;

pub type exc_code = BitVector<8>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct sync_exception {
    pub trap: ExceptionType,
    pub excinfo: Option<xlenbits>,
}

pub type tv_mode = BitVector<2>;

pub type csrRW = BitVector<2>;

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
}

pub fn bit_to_bool(sail_ctx: &mut SailVirtCtx, b: bool) -> bool {
    match b {
        true => {true}
        false => {false}
        _ => {panic!("Unreachable code")}
    }
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

pub fn _operator_biggerequal_u_<const N: usize>(sail_ctx: &mut SailVirtCtx, x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.as_usize() >= y.as_usize())
}

pub fn _operator_smaller_u_<const N: usize>(sail_ctx: &mut SailVirtCtx, x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.as_usize() < y.as_usize())
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine,
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

pub fn haveUsrMode(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> bool {
    true
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_internal_error(()),
}

pub fn _get_Mstatus_MIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<3, 4, 1>()
}

pub fn _get_Mstatus_SIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<1, 2, 1>()
}

pub fn _get_Mstatus_UIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<0, 1, 1>()
}

pub fn rX(sail_ctx: &mut SailVirtCtx, r: BitVector<5>) -> BitVector<64> {
    match r {
        b__0 if {(b__0 == BitVector::<5>::new(0b00000))} => {BitVector::<64>::new(BitVector::<4>::new(0b0000).bits())}
        _ => {sail_ctx.Xs[r.as_usize()]}
        _ => {panic!("Unreachable code")}
    }
}

pub fn wX(sail_ctx: &mut SailVirtCtx, r: BitVector<5>, v: BitVector<64>) {
    if {(r != BitVector::<5>::new(0b00000))} {
        sail_ctx.Xs[r.as_usize()] = v
    } else {
        ()
    }
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
    E_SAMO_Page_Fault(()),
}

pub fn num_of_ExceptionType(sail_ctx: &mut SailVirtCtx, e: ExceptionType) -> usize {
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

pub fn exceptionType_to_bits(sail_ctx: &mut SailVirtCtx, e: ExceptionType) -> BitVector<8> {
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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ctl_result {
    CTL_TRAP(sync_exception),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TrapVectorMode {
    TV_Direct,
    TV_Vector,
    TV_Reserved,
}

pub fn set_next_pc(sail_ctx: &mut SailVirtCtx, pc: BitVector<64>) {
    sail_ctx.nextPC = pc
}

pub fn tval(sail_ctx: &mut SailVirtCtx, excinfo: Option<BitVector<64>>) -> BitVector<64> {
    match excinfo {
        Some(e) => {e}
        None => {BitVector::<64>::new(BitVector::<1>::new(0b0).bits())}
        _ => {panic!("Unreachable code")}
    }
}

pub fn trap_handler(sail_ctx: &mut SailVirtCtx, del_priv: Privilege, intr: bool, c: BitVector<8>, pc: BitVector<64>, info: Option<BitVector<64>>) -> BitVector<64> {
    match del_priv {
        Privilege::Machine => {{
            sail_ctx.mcause = {
                let var_1 = bool_to_bits(sail_ctx, intr);
                sail_ctx.mcause.set_subrange::<63, 64, 1>(var_1)
            };
            sail_ctx.mcause = {
                let var_2 = BitVector::<63>::new(c.bits());
                sail_ctx.mcause.set_subrange::<0, 63, 63>(var_2)
            };
            sail_ctx.mstatus = {
                let var_3 = _get_Mstatus_MIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.set_subrange::<7, 8, 1>(var_3)
            };
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<3, 4, 1>(BitVector::<1>::new(0b0));
            sail_ctx.mstatus = {
                let var_4 = privLevel_to_bits(sail_ctx, sail_ctx.cur_privilege);
                sail_ctx.mstatus.set_subrange::<11, 13, 2>(var_4)
            };
            sail_ctx.mtval = tval(sail_ctx, info);
            todo!("E_var")
        }}
        Privilege::Supervisor => {{
            assert!(haveSupMode(sail_ctx, ()), "Process message");
            sail_ctx.scause = {
                let var_5 = bool_to_bits(sail_ctx, intr);
                sail_ctx.scause.set_subrange::<63, 64, 1>(var_5)
            };
            sail_ctx.scause = {
                let var_6 = BitVector::<63>::new(c.bits());
                sail_ctx.scause.set_subrange::<0, 63, 63>(var_6)
            };
            sail_ctx.mstatus = {
                let var_7 = _get_Mstatus_SIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.set_subrange::<5, 6, 1>(var_7)
            };
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<1, 2, 1>(BitVector::<1>::new(0b0));
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<8, 9, 1>(match sail_ctx.cur_privilege {
                Privilege::User => {BitVector::<1>::new(0b0)}
                Privilege::Supervisor => {BitVector::<1>::new(0b1)}
                Privilege::Machine => {panic!("todo_process_panic_type")}
                _ => {panic!("Unreachable code")}
            });
            todo!("E_var")
        }}
        Privilege::User => {{
            sail_ctx.ucause = {
                let var_8 = bool_to_bits(sail_ctx, intr);
                sail_ctx.ucause.set_subrange::<63, 64, 1>(var_8)
            };
            sail_ctx.ucause = {
                let var_9 = BitVector::<63>::new(c.bits());
                sail_ctx.ucause.set_subrange::<0, 63, 63>(var_9)
            };
            sail_ctx.mstatus = {
                let var_10 = _get_Mstatus_UIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.set_subrange::<4, 5, 1>(var_10)
            };
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<0, 1, 1>(BitVector::<1>::new(0b0));
            todo!("E_var")
        }}
        _ => {panic!("Unreachable code")}
    }
}

pub fn exception_delegatee(sail_ctx: &mut SailVirtCtx, e: ExceptionType, p: Privilege) -> Privilege {
    let idx = num_of_ExceptionType(sail_ctx, e);
    let _super_ = {
        let var_3 = bitvector_access(sail_ctx.medeleg.bits, idx);
        bit_to_bool(sail_ctx, var_3)
    };
    let user = false;
    let deleg = if {(haveUsrMode(sail_ctx, ()) && user)} {
        Privilege::User
    } else if {(haveSupMode(sail_ctx, ()) && _super_)} {
        Privilege::Supervisor
    } else {
        Privilege::Machine
    };
    if {{
        let var_1 = privLevel_to_bits(sail_ctx, deleg);
        let var_2 = privLevel_to_bits(sail_ctx, p);
        _operator_smaller_u_(sail_ctx, var_1, var_2)
    }} {
        p
    } else {
        deleg
    }
}

pub fn exception_handler(sail_ctx: &mut SailVirtCtx, cur_priv: Privilege, ctl: ctl_result, pc: BitVector<64>) -> BitVector<64> {
    match (cur_priv, ctl) {
        (_, ctl_result::CTL_TRAP(e)) => {{
            let del_priv = {
                let var_7 = e.trap;
                let var_8 = cur_priv;
                exception_delegatee(sail_ctx, var_7, var_8)
            };
            {
                let var_1 = del_priv;
                let var_2 = false;
                let var_3 = {
                    let var_6 = e.trap;
                    exceptionType_to_bits(sail_ctx, var_6)
                };
                let var_4 = pc;
                let var_5 = e.excinfo;
                trap_handler(sail_ctx, var_1, var_2, var_3, var_4, var_5)
            }
        }}
        _ => {panic!("Unreachable code")}
    }
}

pub fn handle_illegal(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    let t: sync_exception = sync_exception {
        trap: ExceptionType::E_Illegal_Instr(()),
        excinfo: None
    };
    {
        let var_1 = {
            let var_2 = sail_ctx.cur_privilege;
            let var_3 = ctl_result::CTL_TRAP(t);
            let var_4 = sail_ctx.PC;
            exception_handler(sail_ctx, var_2, var_3, var_4)
        };
        set_next_pc(sail_ctx, var_1)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    RISCV_ADDI,
    RISCV_SLTI,
    RISCV_SLTIU,
    RISCV_XORI,
    RISCV_ORI,
    RISCV_ANDI,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum csrop {
    CSRRW,
    CSRRS,
    CSRRC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ITYPE((BitVector<12>, regidx, regidx, iop)),
    CSR((BitVector<12>, regidx, regidx, bool, csrop)),
}

pub fn csrAccess(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>) -> BitVector<2> {
    csr.subrange::<10, 12, 2>()
}

pub fn csrPriv(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>) -> BitVector<2> {
    csr.subrange::<8, 10, 2>()
}

pub fn is_CSR_defined(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>, p: Privilege) -> bool {
    match csr {
        b__0 if {(b__0 == BitVector::<12>::new(0b001101000000))} => {(p == Privilege::Machine)}
        b__1 if {(b__1 == BitVector::<12>::new(0b000101000000))} => {((p == Privilege::Machine) || (p == Privilege::Supervisor))}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

pub fn check_CSR_access(sail_ctx: &mut SailVirtCtx, csrrw: BitVector<2>, csrpr: BitVector<2>, p: Privilege, isWrite: bool) -> bool {
    (!(((isWrite == true) && (csrrw == BitVector::<2>::new(0b11)))) && {
        let var_1 = privLevel_to_bits(sail_ctx, p);
        let var_2 = csrpr;
        _operator_biggerequal_u_(sail_ctx, var_1, var_2)
    })
}

pub fn check_CSR(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>, p: Privilege, isWrite: bool) -> bool {
    (is_CSR_defined(sail_ctx, csr, p) && {
        let var_1 = csrAccess(sail_ctx, csr);
        let var_2 = csrPriv(sail_ctx, csr);
        let var_3 = p;
        let var_4 = isWrite;
        check_CSR_access(sail_ctx, var_1, var_2, var_3, var_4)
    })
}

pub fn readCSR(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>) -> BitVector<64> {
    let res: xlenbits = match (csr, 64) {
        (b__0, _) if {(b__0 == BitVector::<12>::new(0b001101000000))} => {sail_ctx.mscratch}
        (b__1, _) if {(b__1 == BitVector::<12>::new(0b000101000000))} => {sail_ctx.sscratch}
        _ => {BitVector::<64>::new(BitVector::<4>::new(0b0000).bits())}
        _ => {panic!("Unreachable code")}
    };
    res
}

pub fn writeCSR(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>, value: BitVector<64>) {
    let res: Option<xlenbits> = match (csr, 64) {
        (b__0, _) if {(b__0 == BitVector::<12>::new(0b001101000000))} => {{
            sail_ctx.mscratch = value;
            Some(sail_ctx.mscratch)
        }}
        (b__1, _) if {(b__1 == BitVector::<12>::new(0b000101000000))} => {{
            sail_ctx.sscratch = value;
            Some(sail_ctx.sscratch)
        }}
        _ => {None}
        _ => {panic!("Unreachable code")}
    };
    ()
}

pub fn execute_ITYPE(sail_ctx: &mut SailVirtCtx, imm: BitVector<12>, rs1: regidx, rd: regidx) -> Retired {
    let rs1_val = rX(sail_ctx, rs1);
    let imm_ext: xlenbits = BitVector::<64>::new(imm.bits());
    let result = rs1_val.wrapped_add(imm_ext);
    wX(sail_ctx, rd, result);
    Retired::RETIRE_SUCCESS
}

pub fn execute_CSR(sail_ctx: &mut SailVirtCtx, csr: BitVector<12>, rs1: regidx, rd: regidx, is_imm: bool, op: csrop) -> Retired {
    let rs1_val: xlenbits = if {is_imm} {
        BitVector::<64>::new(rs1.bits())
    } else {
        rX(sail_ctx, rs1)
    };
    let isWrite: bool = match op {
        csrop::CSRRW => {true}
        _ => {if {is_imm} {
            (rs1_val.as_usize() != 0)
        } else {
            (rs1.as_usize() != 0)
        }}
        _ => {panic!("Unreachable code")}
    };
    if {!(check_CSR(sail_ctx, csr, sail_ctx.cur_privilege, isWrite))} {
        handle_illegal(sail_ctx, ());
        Retired::RETIRE_FAIL
    } else {
        let csr_val = readCSR(sail_ctx, csr);
        if {isWrite} {
            let new_val: xlenbits = match op {
                csrop::CSRRW => {rs1_val}
                csrop::CSRRS => {(csr_val | rs1_val)}
                csrop::CSRRC => {(csr_val & !(rs1_val))}
                _ => {panic!("Unreachable code")}
            };
            writeCSR(sail_ctx, csr, new_val)
        } else {
            ()
        };
        wX(sail_ctx, rd, csr_val);
        Retired::RETIRE_SUCCESS
    }
}
