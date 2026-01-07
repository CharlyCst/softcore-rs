//! Softcore RISC-V 64
//!
//! This library is a wrapper around a Rust translation of the official [RISC-V executable
//! specification][1]. The software core can be used to test the behavior of the hardware, for
//! instance to check if a memory access is allowed, or the register state after taking a trap.
//! This is especially helpful to test or verify low-level software, such as kernels, hypervisors,
//! or firmware.
//!
//! The raw translation is exposed in the [raw] module. A more polished (and slightly more stable)
//! interface is exposed through the [Core] methods.
//!
//! [1]: https://github.com/riscv/sail-riscv

mod arch_prelude;
pub mod config;
pub mod registers;

/// The raw translation of the official RISC-V executable specification.
///
/// The [RISC-V executable specification][1] is written in [Sail][2], a domain specific language to
/// specify Instruction Set Architectures (ISA). The translation is automated using a custom
/// Sail-to-Rust back-end for the Sail compiler.
///
/// [1]: https://github.com/riscv/sail-riscv
/// [2]: https://github.com/rems-project/sail
#[rustfmt::skip]
pub mod raw;

pub use raw::{Core, ExceptionType, ExecutionResult, Privilege, ast};
use raw::{cregidx, regidx};
use registers::GeneralRegister;
use registers::*;
pub use softcore_prelude as prelude;
use softcore_prelude::{BitVector, bv};

// ———————————————————————— Initialization Constants ———————————————————————— //

const DEFAULT_PMP_CFG: raw::Pmpcfg_ent = raw::Pmpcfg_ent { bits: bv(0) };
const DEFAULT_HPM_EVENT: raw::HpmEvent = raw::HpmEvent { bits: bv(0) };
const DEFAULT_TLB_ENTRY: Option<raw::TLB_Entry> = None;
const ZEROES: BitVector<64> = bv(0);

// ————————————————————————————— Trap Handling —————————————————————————————— //

/// Wether or not a trap occured after executing an instruction.
#[derive(Debug, Clone, Copy)]
pub enum Trap {
    /// The instruction trapped, contains the address of the trap handler.
    Some(u64),
    /// The instruction did not trap.
    None,
}

impl Trap {
    pub fn trapped(self) -> bool {
        match self {
            Trap::Some(_) => true,
            Trap::None => false,
        }
    }
}

// —————————————————————————— Core implementation ——————————————————————————— //

impl Core {
    /// Reset the core, initializing registers with specified reset values.
    ///
    /// This does not reset all registers and CSRs of the core, it only performs the minimal reset
    /// required by the specification.
    ///
    /// This function should be called on a fresh core to ensure the core starts in a sensible
    /// state.
    pub fn reset(&mut self) {
        raw::_reset_all_registers(self);
        raw::reset_sys(self, ());
    }

    /// Execute one instruction
    pub fn execute(&mut self, instr: ast) -> Trap {
        let exec_res = raw::execute(self, instr);
        self.process_execution_result(exec_res, instr)
    }

    /// Handle the necessary steps post instruction execution, such as jumping to the trap handler
    /// if necessary.
    fn process_execution_result(&mut self, exec_res: ExecutionResult, instr: ast) -> Trap {
        match exec_res {
            ExecutionResult::Retire_Success(_) => Trap::None,
            ExecutionResult::Wait_For_Interrupt(_) => Trap::None,
            ExecutionResult::Illegal_Instruction(_) => {
                let instr_bits = raw::encdec_forwards(self, instr);
                raw::handle_illegal(self, instr_bits);
                Trap::Some(self.nextPC.bits())
            }
            ExecutionResult::Trap((privilege, ctl, pc)) => {
                let pc = raw::exception_handler(self, privilege, ctl, pc);
                raw::set_next_pc(self, pc);
                Trap::Some(self.nextPC.bits())
            }
            ExecutionResult::Memory_Exception(_) => todo!("handle Memory_Exception"),
            ExecutionResult::Ext_CSR_Check_Failure(_) => todo!("handle Ext_CSR_Check_Failure"),
            ExecutionResult::Ext_ControlAddr_Check_Failure(_) => {
                todo!("handle Ext_ControlAddr_Check_Failure")
            }
            ExecutionResult::Ext_DataAddr_Check_Failure(_) => {
                todo!("handle Ext_DataAddr_Check_Failure")
            }
            ExecutionResult::Ext_XRET_Priv_Failure(_) => todo!("handle Ext_XRET_Priv_Failure"),
        }
    }

    /// Get the value of a general purpose register.
    pub fn get(&mut self, reg: GeneralRegister) -> u64 {
        let reg = match reg {
            raw::regidx::Regidx(reg) => reg.bits() as i128,
        };
        raw::rX(self, raw::regno::Regno(reg)).bits()
    }

    /// Set the value of a general purpose register.
    pub fn set(&mut self, reg: GeneralRegister, value: u64) {
        let reg = match reg {
            raw::regidx::Regidx(reg) => reg.bits() as i128,
        };
        raw::wX(self, raw::regno::Regno(reg), bv(value));
    }

    /// Get the value of a CSR identified by its CSR index.
    ///
    /// This function returns [None] if the CSR can not be read by the current privilege level or
    /// is not implemented given the core configuration.
    pub fn get_csr(&mut self, csr: u64) -> Option<u64> {
        let csr = bv(csr);
        if raw::check_CSR(self, csr, self.cur_privilege, false) {
            Some(raw::read_CSR(self, csr).bits())
        } else {
            None
        }
    }

    /// Set the value of a CSR identified by its CSR index.
    ///
    /// This function returns [None] if the CSR can not be written by the current privilege level
    /// or is not implemented given the core configuration. Otherwise the new CSR value is
    /// returned.
    pub fn set_csr(&mut self, csr: u64, value: u64) -> Option<u64> {
        let csr = bv(csr);
        if raw::check_CSR(self, csr, self.cur_privilege, true) {
            Some(raw::write_CSR(self, csr, bv(value)).bits())
        } else {
            None
        }
    }

    /// Atomic Read and Write CSR
    ///
    /// This function has the same effect as executing the `CSRRW` instruction, except for moving
    /// the PC on success and trapping on failure.
    pub fn csrrw(
        &mut self,
        rd: GeneralRegister,
        csr: u64,
        rs1: GeneralRegister,
    ) -> Result<(), raw::ExecutionResult> {
        let val = self.get(rs1);
        self.do_csr(val, csr, rd, raw::csrop::CSRRW, true)
    }

    /// Atomic Read and Set bits CSR
    ///
    /// This function has the same effect as executing the `CSRRS` instruction, except for moving
    /// the PC on success and trapping on failure.
    pub fn csrrs(
        &mut self,
        rd: GeneralRegister,
        csr: u64,
        rs1: GeneralRegister,
    ) -> Result<(), raw::ExecutionResult> {
        let val = self.get(rs1);
        self.do_csr(val, csr, rd, raw::csrop::CSRRS, rs1 != X0)
    }

    /// Atomic Read and Clear bits CSR
    ///
    /// This function has the same effect as executing the `CSRRc` instruction, except for moving
    /// the PC on success and trapping on failure.
    pub fn csrrc(
        &mut self,
        rd: GeneralRegister,
        csr: u64,
        rs1: GeneralRegister,
    ) -> Result<(), raw::ExecutionResult> {
        let val = self.get(rs1);
        self.do_csr(val, csr, rd, raw::csrop::CSRRC, rs1 != X0)
    }

    /// Atomic Read and Write immediate CSR
    ///
    /// This function has the same effect as executing the `CSRRWI` instruction, except for moving
    /// the PC on success and trapping on failure.
    pub fn csrrwi(
        &mut self,
        rd: GeneralRegister,
        csr: u64,
        uimm: u64,
    ) -> Result<(), raw::ExecutionResult> {
        let uimm = uimm & 0b11111; // The immediate is only 5 bits wide
        self.do_csr(uimm, csr, rd, raw::csrop::CSRRW, true)
    }

    /// Atomic Read and Set bits immediate CSR
    ///
    /// This function has the same effect as executing the `CSRRSI` instruction, except for moving
    /// the PC on success and trapping on failure.
    pub fn csrrsi(
        &mut self,
        rd: GeneralRegister,
        csr: u64,
        uimm: u64,
    ) -> Result<(), raw::ExecutionResult> {
        let uimm = uimm & 0b11111; // The immediate is only 5 bits wide
        self.do_csr(uimm, csr, rd, raw::csrop::CSRRS, uimm != 0)
    }

    /// Atomic Read and Clear bits immediate CSR
    ///
    /// This function has the same effect as executing the `CSRRCI` instruction, except for moving
    /// the PC on success and trapping on failure.
    pub fn csrrci(
        &mut self,
        rd: GeneralRegister,
        csr: u64,
        uimm: u64,
    ) -> Result<(), raw::ExecutionResult> {
        let uimm = uimm & 0b11111; // The immediate is only 5 bits wide
        self.do_csr(uimm, csr, rd, raw::csrop::CSRRC, uimm != 0)
    }

    /// Private helper functions to call the raw doCSR.
    ///
    /// Refer to the sail definition of `execute CSRReg` for how to use this function.
    fn do_csr(
        &mut self,
        val: u64,
        csr: u64,
        rd: GeneralRegister,
        op: raw::csrop,
        is_write: bool,
    ) -> Result<(), raw::ExecutionResult> {
        let csr = bv(csr);
        let val = bv(val);
        let res = raw::doCSR(self, csr, val, rd, op, is_write);
        match res {
            raw::ExecutionResult::Retire_Success(()) => Ok(()),
            _ => Err(res),
        }
    }

    /// Return the current privilege mode.
    pub fn mode(&self) -> Privilege {
        self.cur_privilege
    }

    /// Set the privilege mode
    pub fn set_mode(&mut self, mode: Privilege) {
        self.cur_privilege = mode
    }

    /// Decode an instruction
    pub fn decode_instr(&mut self, instr: u32) -> ast {
        raw::encdec_backwards(self, bv(instr as u64))
    }

    /// Encode and instruction
    pub fn encode_instr(&mut self, instr: ast) -> u32 {
        raw::encdec_forwards(self, instr).bits() as u32
    }

    /// Return true if the CSR is defined (and enabled) on the core
    pub fn is_csr_defined(&mut self, csr_id: usize) -> bool {
        raw::is_CSR_defined(self, bv(csr_id as u64))
    }

    /// Dispatch pending interrupt
    ///
    /// This function looks for pending and enabled interrupts and perform the dispatch for the
    /// interrupt with highest priority.
    pub fn dispatch_interrupt(&mut self) {
        if let Some((int, target_priv)) = raw::dispatchInterrupt(self, self.cur_privilege) {
            raw::handle_interrupt(self, int, target_priv);
        }
    }

    /// Inject an exception, triggerring the appropriate trap handler
    ///
    /// The target privilege mode depends on the current execution mode and the *deleg CSR
    /// registers.
    /// The `tval` is the trap value, which depends on the exception type. Memory access fault will
    /// usually provide the faulting address.
    pub fn inject_exception(&mut self, exception: ExceptionType, tval: u64) {
        let current_level = self.cur_privilege;
        let target_level = raw::exception_delegatee(self, exception, current_level);
        raw::trap_handler(
            self,
            target_level,
            false,
            raw::exceptionType_to_bits(exception),
            self.PC,
            Some(bv(tval)),
            None,
        );
    }

    /// Return the `pmpaddr<index>` register.
    pub fn get_pmpaddr(&self, index: usize) -> u64 {
        self.pmpaddr_n[index].bits()
    }

    /// Set the `pmpaddr<index>` register to the given value.
    pub fn set_pmpaddr(&mut self, index: usize, val: u64) {
        raw::pmpWriteAddrReg(self, index as i128, bv(val));
    }

    /// Set the `pmpcfg<index>` register to the given value.
    pub fn set_pmpcfg(&mut self, index: usize, val: u64) {
        raw::pmpWriteCfgReg(self, index as i128, bv(val));
    }

    /// Check if an 8 byte access is allowed with the current mode and PMP configuration.
    ///
    /// Return None is the check succeed, or an error otherwise.
    pub fn pmp_check(
        &mut self,
        addr: u64,
        access_kind: raw::AccessType<()>,
    ) -> Option<raw::ExceptionType> {
        let addr = raw::physaddr::Physaddr(bv(addr));
        let width = 8;
        raw::pmpCheck(self, addr, width, access_kind, self.cur_privilege)
    }
}

/// Returns a fresh core instance with the provided configuration.
///
/// IMPORTANT: The freshtly created core is not guaranteed to be in a valid state. Call
/// [Core::reset] or update CSRs manually to ensure the core enters a valid starting state.
pub const fn new_core(config: raw::Config) -> Core {
    Core {
        PC: bv(0),
        nextPC: bv(0),
        x1: bv(0),
        x2: bv(0),
        x3: bv(0),
        x4: bv(0),
        x5: bv(0),
        x6: bv(0),
        x7: bv(0),
        x8: bv(0),
        x9: bv(0),
        x10: bv(0),
        x11: bv(0),
        x12: bv(0),
        x13: bv(0),
        x14: bv(0),
        x15: bv(0),
        x16: bv(0),
        x17: bv(0),
        x18: bv(0),
        x19: bv(0),
        x20: bv(0),
        x21: bv(0),
        x22: bv(0),
        x23: bv(0),
        x24: bv(0),
        x25: bv(0),
        x26: bv(0),
        x27: bv(0),
        x28: bv(0),
        x29: bv(0),
        x30: bv(0),
        x31: bv(0),
        cur_privilege: raw::Privilege::Machine,
        cur_inst: bv(0),
        misa: raw::Misa { bits: bv(0) },
        mstatus: raw::Mstatus { bits: bv(0) },
        menvcfg: raw::MEnvcfg { bits: bv(0) },
        senvcfg: raw::SEnvcfg { bits: bv(0) },
        mie: raw::Minterrupts { bits: bv(0) },
        mip: raw::Minterrupts { bits: bv(0) },
        medeleg: raw::Medeleg { bits: bv(0) },
        mideleg: raw::Minterrupts { bits: bv(0) },
        mtvec: raw::Mtvec { bits: bv(0) },
        mcause: raw::Mcause { bits: bv(0) },
        mepc: bv(0),
        mtval: bv(0),
        mscratch: bv(0),
        scounteren: raw::Counteren { bits: bv(0) },
        mcounteren: raw::Counteren { bits: bv(0) },
        mcountinhibit: raw::Counterin { bits: bv(0) },
        mcycle: bv(0),
        mtime: bv(0),
        minstret: bv(0),
        minstret_increment: false,
        mvendorid: bv(0),
        mimpid: bv(0),
        marchid: bv(0),
        mhartid: bv(0),
        mconfigptr: bv(0),
        stvec: raw::Mtvec { bits: bv(0) },
        sscratch: bv(0),
        sepc: bv(0),
        scause: raw::Mcause { bits: bv(0) },
        stval: bv(0),
        tselect: bv(0),
        vstart: bv(0),
        vl: bv(0),
        vtype: raw::Vtype { bits: bv(0) },
        pmpcfg_n: [DEFAULT_PMP_CFG; 64],
        pmpaddr_n: [ZEROES; 64],
        vr0: bv(0),
        vr1: bv(0),
        vr2: bv(0),
        vr3: bv(0),
        vr4: bv(0),
        vr5: bv(0),
        vr6: bv(0),
        vr7: bv(0),
        vr8: bv(0),
        vr9: bv(0),
        vr10: bv(0),
        vr11: bv(0),
        vr12: bv(0),
        vr13: bv(0),
        vr14: bv(0),
        vr15: bv(0),
        vr16: bv(0),
        vr17: bv(0),
        vr18: bv(0),
        vr19: bv(0),
        vr20: bv(0),
        vr21: bv(0),
        vr22: bv(0),
        vr23: bv(0),
        vr24: bv(0),
        vr25: bv(0),
        vr26: bv(0),
        vr27: bv(0),
        vr28: bv(0),
        vr29: bv(0),
        vr30: bv(0),
        vr31: bv(0),
        vcsr: raw::Vcsr { bits: bv(0) },
        mhpmevent: [DEFAULT_HPM_EVENT; 32],
        mhpmcounter: [ZEROES; 32],
        float_result: bv(0),
        float_fflags: bv(0),
        f0: bv(0),
        f1: bv(0),
        f2: bv(0),
        f3: bv(0),
        f4: bv(0),
        f5: bv(0),
        f6: bv(0),
        f7: bv(0),
        f8: bv(0),
        f9: bv(0),
        f10: bv(0),
        f11: bv(0),
        f12: bv(0),
        f13: bv(0),
        f14: bv(0),
        f15: bv(0),
        f16: bv(0),
        f17: bv(0),
        f18: bv(0),
        f19: bv(0),
        f20: bv(0),
        f21: bv(0),
        f22: bv(0),
        f23: bv(0),
        f24: bv(0),
        f25: bv(0),
        f26: bv(0),
        f27: bv(0),
        f28: bv(0),
        f29: bv(0),
        f30: bv(0),
        f31: bv(0),
        fcsr: raw::Fcsr { bits: bv(0) },
        mcyclecfg: raw::CountSmcntrpmf { bits: bv(0) },
        minstretcfg: raw::CountSmcntrpmf { bits: bv(0) },
        mtimecmp: bv(0),
        stimecmp: bv(0),
        htif_tohost: bv(0),
        htif_done: false,
        htif_exit_code: bv(0),
        htif_cmd_write: false,
        htif_payload_writes: bv(0),
        tlb: [DEFAULT_TLB_ENTRY; raw::num_tlb_entries as usize],
        satp: bv(0),
        hart_state: raw::HartState::HART_ACTIVE(()),
        config,
    }
}

// ———————————————————————————————— Helpers ————————————————————————————————— //

impl regidx {
    /// Creates a new regidx from a register index
    pub fn new(reg: u8) -> regidx {
        regidx::Regidx(bv(reg as u64))
    }

    /// Return the register index as bits.
    pub fn bits(self) -> u8 {
        let regidx::Regidx(bits) = self;
        bits.bits() as u8
    }
}

impl cregidx {
    /// Return the compressed register index as bits.
    ///
    /// Warning: this is not the same as the uncompressed register index.
    pub fn bits(self) -> u8 {
        let cregidx::Cregidx(bits) = self;
        bits.bits() as u8
    }

    /// Convert a compressed register index into an uncompressed register index.
    pub fn to_regidx(self) -> regidx {
        raw::creg2reg_idx(self)
    }
}

// ————————————————————————————————— Tests —————————————————————————————————— //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::raw::*;

    #[test]
    fn pmp_check() {
        let mut core = new_core(config::U74);
        let addr = 0x8000_0000;
        let access = AccessType::Read(());

        // Check the default access rights
        assert!(
            core.pmp_check(addr, access).is_none(),
            "M-mode can access all memory by default"
        );

        core.set_mode(Privilege::User);
        assert_eq!(
            core.pmp_check(addr, access),
            Some(ExceptionType::E_Load_Access_Fault(())),
            "U-mode has no access by default"
        );

        // Now let's add a PMP entry to allow reads from U-mode
        let pmp_addr = addr >> 2; // There is a shift of 2 in the pmpaddr registers
        core.set_pmpaddr(0, pmp_addr);
        core.set_pmpaddr(1, 2 * pmp_addr);
        core.set_pmpcfg(0, 0b0000_1001 << 8); // Entry 1, Read-only access, ToR matching mode
        assert!(
            core.pmp_check(addr, access).is_none(),
            "PMP allow read access"
        );
    }

    #[test]
    fn decoder() {
        let mut ctx = new_core(config::U74);
        let uimm0 = bv(0);

        // Load/Store

        assert_eq!(
            ctx.decode_instr(0xff87b703),
            ast::LOAD((
                bv(0xFFF - 7), // immediate is -8
                X15,
                X14,
                false,
                word_width::DOUBLE,
                false,
                false
            ))
        );

        // CSR instructions

        // csrrw x0, mstatus, x0
        assert_eq!(
            ctx.decode_instr(0x30001073),
            ast::CSRReg((bv(0x300), X0, X0, csrop::CSRRW))
        );
        // csrrs x0, mstatus, x0
        assert_eq!(
            ctx.decode_instr(0x30002073),
            ast::CSRReg((bv(0x300), X0, X0, csrop::CSRRS))
        );
        // csrrc x0, mstatus, x0
        assert_eq!(
            ctx.decode_instr(0x30003073),
            ast::CSRReg((bv(0x300), X0, X0, csrop::CSRRC))
        );
        // csrrwi x0, mstatus, 0
        assert_eq!(
            ctx.decode_instr(0x30005073),
            ast::CSRImm((bv(0x300), uimm0, X0, csrop::CSRRW))
        );
        // csrrsi x0, mstatus, 0
        assert_eq!(
            ctx.decode_instr(0x30006073),
            ast::CSRImm((bv(0x300), uimm0, X0, csrop::CSRRS))
        );
        // csrrci x0, mstatus, 0
        assert_eq!(
            ctx.decode_instr(0x30007073),
            ast::CSRImm((bv(0x300), uimm0, X0, csrop::CSRRC))
        );

        // Illegal
        assert_eq!(ctx.decode_instr(0x30001072), ast::ILLEGAL(bv(0x30001072)));
    }

    #[test]
    fn general_purpose_registers() {
        let mut ctx = new_core(config::U74);

        // Test X0 (ZERO) - should always be hardwired to 0
        assert_eq!(ctx.get(X0), 0, "X0 should be hardwired to 0");
        assert_eq!(ctx.get(ZERO), 0, "ZERO should be hardwired to 0");

        // Try to write to X0 - should remain 0
        ctx.set(X0, 0xDEADBEEF);
        assert_eq!(ctx.get(X0), 0, "X0 should remain 0 after write attempt");

        // Test some other registers using ABI names
        ctx.set(RA, 0x12345678);
        assert_eq!(ctx.get(RA), 0x12345678, "RA register should store value");
        assert_eq!(ctx.get(X1), 0x12345678, "X1 and RA should be the same");

        ctx.set(SP, 0x87654321);
        assert_eq!(ctx.get(SP), 0x87654321, "SP register should store value");
        assert_eq!(ctx.get(X2), 0x87654321, "X2 and SP should be the same");

        // Test function argument registers
        ctx.set(A0, 0xAAAAAAAA);
        ctx.set(A1, 0xBBBBBBBB);
        assert_eq!(ctx.get(A0), 0xAAAAAAAA, "A0 register should store value");
        assert_eq!(ctx.get(A1), 0xBBBBBBBB, "A1 register should store value");
        assert_eq!(ctx.get(X10), 0xAAAAAAAA, "X10 and A0 should be the same");
        assert_eq!(ctx.get(X11), 0xBBBBBBBB, "X11 and A1 should be the same");

        // Test saved registers
        ctx.set(S0, 0xCCCCCCCC);
        ctx.set(S1, 0xDDDDDDDD);
        assert_eq!(ctx.get(S0), 0xCCCCCCCC, "S0 register should store value");
        assert_eq!(ctx.get(S1), 0xDDDDDDDD, "S1 register should store value");
        assert_eq!(ctx.get(FP), 0xCCCCCCCC, "FP and S0 should be the same");
        assert_eq!(ctx.get(X8), 0xCCCCCCCC, "X8 and S0 should be the same");
        assert_eq!(ctx.get(X9), 0xDDDDDDDD, "X9 and S1 should be the same");

        // Test temporary registers
        ctx.set(T0, 0xEEEEEEEE);
        ctx.set(T6, 0xFFFFFFFF);
        assert_eq!(ctx.get(T0), 0xEEEEEEEE, "T0 register should store value");
        assert_eq!(ctx.get(T6), 0xFFFFFFFF, "T6 register should store value");
        assert_eq!(ctx.get(X5), 0xEEEEEEEE, "X5 and T0 should be the same");
        assert_eq!(ctx.get(X31), 0xFFFFFFFF, "X31 and T6 should be the same");

        // Verify X0 is still 0 after all the other operations
        assert_eq!(
            ctx.get(X0),
            0,
            "X0 should still be 0 after other register operations"
        );
    }

    #[test]
    fn csr_defined() {
        let mut core = new_core(config::U74);

        // Test standard machine-level CSRs that should exist
        assert!(core.is_csr_defined(0x300), "mstatus should be defined");
        assert!(core.is_csr_defined(0x301), "misa should be defined");
        assert!(core.is_csr_defined(0x304), "mie should be defined");
        assert!(core.is_csr_defined(0x305), "mtvec should be defined");
        assert!(core.is_csr_defined(0x341), "mepc should be defined");
        assert!(core.is_csr_defined(0x342), "mcause should be defined");
        assert!(core.is_csr_defined(0x343), "mtval should be defined");
        assert!(core.is_csr_defined(0x344), "mip should be defined");

        // Test PMP configuration registers
        // U74 core has 16 PMP entries, so pmpcfg0, pmpcfg2 should exist, but not pmpcfg4 or
        assert!(core.is_csr_defined(0x3A0), "pmpcfg0 should be defined");
        assert!(core.is_csr_defined(0x3A2), "pmpcfg2 should be defined");
        assert!(!core.is_csr_defined(0x3A4), "pmpcfg4 should not be defined");
        assert!(!core.is_csr_defined(0x3A6), "pmpcfg6 should not be defined");

        // Test that odd pmpcfg registers don't exist (RV64 uses even pmpcfg registers only)
        assert!(
            !core.is_csr_defined(0x3A1),
            "pmpcfg1 should not be defined on RV64"
        );
        assert!(
            !core.is_csr_defined(0x3A3),
            "pmpcfg3 should not be defined on RV64"
        );
        assert!(
            !core.is_csr_defined(0x3A5),
            "pmpcfg5 should not be defined on RV64"
        );

        // Test PMP address registers
        // U74 core has 16 PMP entries, so pmpaddr0-pmpaddr15 should exist
        assert!(core.is_csr_defined(0x3B0), "pmpaddr0 should be defined");
        assert!(core.is_csr_defined(0x3B5), "pmpaddr5 should be defined");
        assert!(core.is_csr_defined(0x3BF), "pmpaddr15 should be defined");

        // Test that PMP address registers beyond 16 don't exist on U74
        assert!(
            !core.is_csr_defined(0x3C0),
            "pmpaddr16 should not be defined on U74"
        );
        assert!(
            !core.is_csr_defined(0x3C8),
            "pmpaddr24 should not be defined on U74"
        );
        assert!(
            !core.is_csr_defined(0x3CF),
            "pmpaddr31 should not be defined on U74"
        );

        // Test some CSRs that definitely shouldn't exist
        assert!(
            !core.is_csr_defined(0x000),
            "CSR 0x000 should not be defined"
        );
        assert!(
            !core.is_csr_defined(0xFFF),
            "CSR 0xFFF should not be defined"
        );
        assert!(
            !core.is_csr_defined(0x200),
            "CSR 0x200 should not be defined"
        );
    }

    #[test]
    fn inject_exception() {
        let mut core = new_core(config::U74);

        // Set initial state
        core.set_mode(Privilege::User);
        core.PC = bv(0x1000);
        let initial_pc = core.PC.bits();

        assert_eq!(core.mode(), Privilege::User, "Initial mode should be User");

        // Inject a load access fault exception
        let fault_addr = 0x8000_0000;
        core.inject_exception(ExceptionType::E_Load_Access_Fault(()), fault_addr);

        // After exception, should be in Machine mode
        assert_eq!(
            core.mode(),
            Privilege::Machine,
            "Mode should be Machine after exception"
        );

        // Check that mepc was set to the PC at the time of the exception
        assert_eq!(
            core.mepc.bits(),
            initial_pc,
            "mepc should contain the PC when exception occurred"
        );

        // Check that mtval contains the fault address
        assert_eq!(
            core.mtval.bits(),
            fault_addr,
            "mtval should contain the fault address"
        );
    }

    #[test]
    fn csr_read_operations() {
        let mut core = new_core(config::U74);

        // Test reading standard CSRs - these should not panic
        let _mstatus = core.get_csr(0x300);
        let _misa = core.get_csr(0x301);
        let _mie = core.get_csr(0x304);
        let _mtvec = core.get_csr(0x305);
        let _mepc = core.get_csr(0x341);
        let _mcause = core.get_csr(0x342);
        let _mtval = core.get_csr(0x343);
        let _mip = core.get_csr(0x344);

        // Test that we can read and write mscratch
        let initial_value = core.get_csr(0x340);
        assert_eq!(initial_value, Some(0), "mscratch should be 0 initially");
    }

    #[test]
    fn csr_write_operations() {
        let mut core = new_core(config::U74);

        // Test CSRRW (read-write) operation
        let initial_value = 0x12345678;
        core.set(X1, initial_value);

        // Write to mscratch (a read-write register)
        let result = core.csrrw(X2, 0x340, X1);
        assert!(result.is_ok(), "csrrw should succeed for mscratch");

        // Read back the value
        let read_value = core.get_csr(0x340);
        assert_eq!(
            read_value,
            Some(initial_value),
            "mscratch should contain written value"
        );

        // X2 should contain the old value (0 for fresh core)
        assert_eq!(core.get(X2), 0, "rd should contain old CSR value");

        // Test writing to X0 (should not update rd)
        let new_value = 0x87654321;
        core.set(X3, new_value);
        let result = core.csrrw(X0, 0x340, X3);
        assert!(result.is_ok(), "csrrw with X0 as rd should succeed");

        // X0 should remain 0
        assert_eq!(core.get(X0), 0, "X0 should remain 0");

        // mscratch should have new value
        let read_value = core.get_csr(0x340);
        assert_eq!(
            read_value,
            Some(new_value),
            "mscratch should contain new value"
        );
    }

    #[test]
    fn csr_set_operations() {
        let mut core = new_core(config::U74);

        // Initialize mscratch with a known value
        core.set(X1, 0xFF00FF00);
        let _ = core.csrrw(X0, 0x340, X1);

        // Test CSRRS (read-set) operation
        let set_bits = 0x00FF00FF;
        core.set(X2, set_bits);

        let result = core.csrrs(X3, 0x340, X2);
        assert!(result.is_ok(), "csrrs should succeed for mscratch");

        // X3 should contain the old value
        assert_eq!(core.get(X3), 0xFF00FF00, "rd should contain old CSR value");

        // mscratch should have bits set
        let read_value = core.get_csr(0x340);
        assert_eq!(
            read_value,
            Some(0xFFFFFFFF),
            "mscratch should have bits set"
        );

        // Test CSRRS with X0 as rs1 (should only read, not modify)
        let result = core.csrrs(X4, 0x340, X0);
        assert!(result.is_ok(), "csrrs with X0 as rs1 should succeed");

        // X4 should contain current value
        assert_eq!(
            core.get(X4),
            0xFFFFFFFF,
            "rd should contain current CSR value"
        );

        // mscratch should be unchanged
        let read_value = core.get_csr(0x340);
        assert_eq!(read_value, Some(0xFFFFFFFF), "mscratch should be unchanged");
    }

    #[test]
    fn csr_clear_operations() {
        let mut core = new_core(config::U74);

        // Initialize mscratch with all bits set
        core.set(X1, 0xFFFFFFFF);
        let _ = core.csrrw(X0, 0x340, X1);

        // Test CSRRC (read-clear) operation
        let clear_bits = 0x0F0F0F0F;
        core.set(X2, clear_bits);

        let result = core.csrrc(X3, 0x340, X2);
        assert!(result.is_ok(), "csrrc should succeed for mscratch");

        // X3 should contain the old value
        assert_eq!(core.get(X3), 0xFFFFFFFF, "rd should contain old CSR value");

        // mscratch should have bits cleared
        let read_value = core.get_csr(0x340);
        assert_eq!(
            read_value,
            Some(0xF0F0F0F0),
            "mscratch should have bits cleared"
        );

        // Test CSRRC with X0 as rs1 (should only read, not modify)
        let result = core.csrrc(X4, 0x340, X0);
        assert!(result.is_ok(), "csrrc with X0 as rs1 should succeed");

        // X4 should contain current value
        assert_eq!(
            core.get(X4),
            0xF0F0F0F0,
            "rd should contain current CSR value"
        );

        // mscratch should be unchanged
        let read_value = core.get_csr(0x340);
        assert_eq!(read_value, Some(0xF0F0F0F0), "mscratch should be unchanged");
    }

    #[test]
    fn csr_immediate_operations() {
        let mut core = new_core(config::U74);

        // Test CSRRWI (read-write immediate)
        let result = core.csrrwi(X1, 0x340, 0x15);
        assert!(result.is_ok(), "csrrwi should succeed for mscratch");

        // X1 should contain old value (0)
        assert_eq!(core.get(X1), 0, "rd should contain old CSR value");

        // mscratch should have immediate value
        let read_value = core.get_csr(0x340);
        assert_eq!(
            read_value,
            Some(0x15),
            "mscratch should contain immediate value"
        );

        // Test CSRRSI (read-set immediate)
        let result = core.csrrsi(X2, 0x340, 0x0A);
        assert!(result.is_ok(), "csrrsi should succeed for mscratch");

        // X2 should contain old value
        assert_eq!(core.get(X2), 0x15, "rd should contain old CSR value");

        // mscratch should have bits set
        let read_value = core.get_csr(0x340);
        assert_eq!(read_value, Some(0x1F), "mscratch should have bits set");

        // Test CSRRCI (read-clear immediate)
        let result = core.csrrci(X3, 0x340, 0x05);
        assert!(result.is_ok(), "csrrci should succeed for mscratch");

        // X3 should contain old value
        assert_eq!(core.get(X3), 0x1F, "rd should contain old CSR value");

        // mscratch should have bits cleared
        let read_value = core.get_csr(0x340);
        assert_eq!(read_value, Some(0x1A), "mscratch should have bits cleared");

        // Test that immediate values are masked to 5 bits
        let result = core.csrrwi(X4, 0x340, 0xFF);
        assert!(result.is_ok(), "csrrwi with large immediate should succeed");

        // mscratch should only have lower 5 bits of immediate
        let read_value = core.get_csr(0x340);
        assert_eq!(
            read_value,
            Some(0x1F),
            "immediate should be masked to 5 bits"
        );

        // Test immediate operations with zero immediate (should not modify for set/clear)
        core.set(X5, 0x12345678);
        let _ = core.csrrw(X0, 0x340, X5);

        let result = core.csrrsi(X6, 0x340, 0);
        assert!(result.is_ok(), "csrrsi with zero immediate should succeed");
        assert_eq!(core.get(X6), 0x12345678, "rd should contain current value");

        let read_value = core.get_csr(0x340);
        assert_eq!(
            read_value,
            Some(0x12345678),
            "CSR should be unchanged with zero immediate"
        );

        let result = core.csrrci(X7, 0x340, 0);
        assert!(result.is_ok(), "csrrci with zero immediate should succeed");
        assert_eq!(core.get(X7), 0x12345678, "rd should contain current value");

        let read_value = core.get_csr(0x340);
        assert_eq!(
            read_value,
            Some(0x12345678),
            "CSR should be unchanged with zero immediate"
        );
    }

    #[test]
    fn execute_addi() {
        let mut core = new_core(config::U74);

        // Test basic ADDI: addi a1, x0, 0x42
        assert_eq!(core.get(A1), 0, "A1 should start at 0");
        let result = core.execute(ast::ITYPE((bv(0x42), X0, A1, iop::ADDI)));
        assert!(matches!(result, Trap::None));
        assert_eq!(core.get(A1), 0x42, "A1 should contain immediate value");

        // Test ADDI with register source: addi a1, a1, 0x20
        let result = core.execute(ast::ITYPE((bv(0x20), A1, A1, iop::ADDI)));
        assert!(matches!(result, Trap::None));
        assert_eq!(
            core.get(A1),
            0x62,
            "A1 should contain sum of previous value and immediate"
        );

        // Test ADDI with negative immediate (sign extension)
        core.set(T0, 100);
        let result = core.execute(ast::ITYPE((bv((-10i64) as u64), T0, T1, iop::ADDI)));
        assert!(matches!(result, Trap::None));
        assert_eq!(
            core.get(T1),
            90,
            "T1 should contain result of adding negative immediate"
        );

        // Test ADDI with X0 as destination (should be ignored)
        let result = core.execute(ast::ITYPE((bv(0xFF), T0, X0, iop::ADDI)));
        assert!(matches!(result, Trap::None));
        assert_eq!(core.get(X0), 0, "X0 should remain hardwired to 0");

        // Test overflow behavior
        core.set(T2, u64::MAX);
        let result = core.execute(ast::ITYPE((bv(1), T2, T3, iop::ADDI)));
        assert!(matches!(result, Trap::None));
        assert_eq!(core.get(T3), 0, "Addition should wrap around on overflow");
    }

    #[test]
    fn execute_jalr() {
        let mut core = new_core(config::U74);

        // Set up initial PC and register state
        core.PC = bv(0x1000);
        core.nextPC = bv(0x1004);
        core.set(T0, 0x3000); // Target address base

        let initial_pc = core.PC.bits();

        // Test JALR: jalr ra, t0, 8
        // This should jump to (t0 + 8) & ~1 and store PC+4 in ra
        let result = core.execute(ast::JALR((bv(8), T0, RA)));

        assert!(matches!(result, Trap::None));

        // Check that return address was stored (old PC + 4)
        assert_eq!(
            core.get(RA),
            initial_pc + 4,
            "Return address should be old PC + 4"
        );

        // Check that nextPC was updated to target address (t0 + offset) with LSB cleared
        let expected_target = (0x3000 + 8) & !1; // JALR clears the LSB
        assert_eq!(
            core.nextPC.bits(),
            expected_target,
            "nextPC should be updated to target address"
        );
    }

    #[test]
    fn execute_mul() {
        let mut core = new_core(config::U74);

        // Test MUL instruction: mul t0, t1, t2
        core.set(T1, 123);
        core.set(T2, 456);

        let mul_op = raw::mul_op {
            high: false,
            signed_rs1: true,
            signed_rs2: true,
        };
        let result = core.execute(ast::MUL((T2, T1, T0, mul_op)));

        assert!(matches!(result, Trap::None));

        // Check that the multiplication result is correct
        let expected = 123u64.wrapping_mul(456u64);
        assert_eq!(core.get(T0), expected, "MUL result should be correct");

        // Test with negative numbers (signed multiplication)
        core.set(T1, (-5i64) as u64); // -5 in two's complement
        core.set(T2, 3);

        let result = core.execute(ast::MUL((T2, T1, T0, mul_op)));
        assert!(matches!(result, Trap::None));

        let expected = ((-5i64).wrapping_mul(3i64)) as u64;
        assert_eq!(
            core.get(T0),
            expected,
            "MUL with negative numbers should work"
        );
    }

    #[test]
    fn execute_div() {
        let mut core = new_core(config::U74);

        // Test DIV instruction: div t0, t1, t2 (signed division)
        core.set(T1, 100);
        core.set(T2, 7);

        let result = core.execute(ast::DIV((T2, T1, T0, true))); // true = signed division

        assert!(matches!(result, Trap::None));

        // Check signed division result
        let expected = (100i64 / 7i64) as u64;
        assert_eq!(core.get(T0), expected, "DIV result should be correct");

        // Test division by zero (should return -1 according to RISC-V spec)
        core.set(T1, 42);
        core.set(T2, 0);

        let result = core.execute(ast::DIV((T2, T1, T0, true)));
        assert!(matches!(result, Trap::None));

        assert_eq!(
            core.get(T0),
            (-1i64) as u64,
            "Division by zero should return -1"
        );

        // Test signed division with negative numbers
        core.set(T1, (-20i64) as u64);
        core.set(T2, 3);

        let result = core.execute(ast::DIV((T2, T1, T0, true)));
        assert!(matches!(result, Trap::None));

        let expected = ((-20i64) / 3i64) as u64;
        assert_eq!(
            core.get(T0),
            expected,
            "Signed division should work correctly"
        );

        // Test unsigned division
        core.set(T1, 100);
        core.set(T2, 7);

        let result = core.execute(ast::DIV((T2, T1, T0, false))); // false = unsigned division
        assert!(matches!(result, Trap::None));

        let expected = 100u64 / 7u64;
        assert_eq!(
            core.get(T0),
            expected,
            "Unsigned division should work correctly"
        );
    }
}
