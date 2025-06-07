pub mod config;
pub mod raw;

pub use raw::{Privilege, SailVirtCtx};
use softcore_prelude::BitVector;

const DEFAULT_PMP_CFG: raw::Pmpcfg_ent = raw::Pmpcfg_ent {
    bits: BitVector::new(0),
};
const DEFAULT_HPM_EVENT: raw::HpmEvent = raw::HpmEvent {
    bits: BitVector::new(0),
};
const DEFAULT_TLB_ENTRY: Option<raw::TLB_Entry> = None;
const ZEROES: BitVector<64> = BitVector::new(0);

impl SailVirtCtx {
    /// Return the current privilege mode.
    pub fn mode(&self) -> Privilege {
        self.cur_privilege
    }

    /// Set the privilege mode mode
    pub fn set_mode(&mut self, mode: Privilege) {
        self.cur_privilege = mode
    }

    /// Return the `pmpaddr<index>` register.
    pub fn get_pmpaddr(&self, index: usize) -> u64 {
        self.pmpaddr_n[index].bits()
    }

    /// Set the `pmpaddr<index>` register to the given value.
    pub fn set_pmpaddr(&mut self, index: usize, val: u64) {
        raw::pmpWriteAddrReg(self, index, BitVector::new(val));
    }

    /// Set the `pmpcfg<index>` register to the given value.
    pub fn set_pmpcfg(&mut self, index: usize, val: u64) {
        raw::pmpWriteCfgReg(self, index, BitVector::new(val));
    }

    /// Check if an 8 byte access is allowed with the current mode and PMP configuration.
    ///
    /// Return None is the check succeed, or an error otherwise.
    pub fn pmp_check(
        &mut self,
        addr: u64,
        access_kind: raw::AccessType<()>,
    ) -> Option<raw::ExceptionType> {
        let addr = raw::physaddr::Physaddr(BitVector::new(addr));
        let width = 8;
        raw::pmpCheck::<8>(self, addr, width, access_kind, self.cur_privilege)
    }
}

pub const fn new_ctx(config: raw::SailConfig) -> SailVirtCtx {
    SailVirtCtx {
        PC: BitVector::new(0),
        nextPC: BitVector::new(0),
        x1: BitVector::new(0),
        x2: BitVector::new(0),
        x3: BitVector::new(0),
        x4: BitVector::new(0),
        x5: BitVector::new(0),
        x6: BitVector::new(0),
        x7: BitVector::new(0),
        x8: BitVector::new(0),
        x9: BitVector::new(0),
        x10: BitVector::new(0),
        x11: BitVector::new(0),
        x12: BitVector::new(0),
        x13: BitVector::new(0),
        x14: BitVector::new(0),
        x15: BitVector::new(0),
        x16: BitVector::new(0),
        x17: BitVector::new(0),
        x18: BitVector::new(0),
        x19: BitVector::new(0),
        x20: BitVector::new(0),
        x21: BitVector::new(0),
        x22: BitVector::new(0),
        x23: BitVector::new(0),
        x24: BitVector::new(0),
        x25: BitVector::new(0),
        x26: BitVector::new(0),
        x27: BitVector::new(0),
        x28: BitVector::new(0),
        x29: BitVector::new(0),
        x30: BitVector::new(0),
        x31: BitVector::new(0),
        cur_privilege: raw::Privilege::Machine,
        cur_inst: BitVector::new(0),
        misa: raw::Misa {
            bits: BitVector::new(0),
        },
        mstatus: raw::Mstatus {
            bits: BitVector::new(0),
        },
        menvcfg: raw::MEnvcfg {
            bits: BitVector::new(0),
        },
        senvcfg: raw::SEnvcfg {
            bits: BitVector::new(0),
        },
        mie: raw::Minterrupts {
            bits: BitVector::new(0),
        },
        mip: raw::Minterrupts {
            bits: BitVector::new(0),
        },
        medeleg: raw::Medeleg {
            bits: BitVector::new(0),
        },
        mideleg: raw::Minterrupts {
            bits: BitVector::new(0),
        },
        mtvec: raw::Mtvec {
            bits: BitVector::new(0),
        },
        mcause: raw::Mcause {
            bits: BitVector::new(0),
        },
        mepc: BitVector::new(0),
        mtval: BitVector::new(0),
        mscratch: BitVector::new(0),
        scounteren: raw::Counteren {
            bits: BitVector::new(0),
        },
        mcounteren: raw::Counteren {
            bits: BitVector::new(0),
        },
        mcountinhibit: raw::Counterin {
            bits: BitVector::new(0),
        },
        mcycle: BitVector::new(0),
        mtime: BitVector::new(0),
        minstret: BitVector::new(0),
        minstret_increment: false,
        mvendorid: BitVector::new(0),
        mimpid: BitVector::new(0),
        marchid: BitVector::new(0),
        mhartid: BitVector::new(0),
        mconfigptr: BitVector::new(0),
        stvec: raw::Mtvec {
            bits: BitVector::new(0),
        },
        sscratch: BitVector::new(0),
        sepc: BitVector::new(0),
        scause: raw::Mcause {
            bits: BitVector::new(0),
        },
        stval: BitVector::new(0),
        tselect: BitVector::new(0),
        vstart: BitVector::new(0),
        vl: BitVector::new(0),
        vtype: raw::Vtype {
            bits: BitVector::new(0),
        },
        pmpcfg_n: [DEFAULT_PMP_CFG; 64],
        pmpaddr_n: [ZEROES; 64],
        vr0: BitVector::new(0),
        vr1: BitVector::new(0),
        vr2: BitVector::new(0),
        vr3: BitVector::new(0),
        vr4: BitVector::new(0),
        vr5: BitVector::new(0),
        vr6: BitVector::new(0),
        vr7: BitVector::new(0),
        vr8: BitVector::new(0),
        vr9: BitVector::new(0),
        vr10: BitVector::new(0),
        vr11: BitVector::new(0),
        vr12: BitVector::new(0),
        vr13: BitVector::new(0),
        vr14: BitVector::new(0),
        vr15: BitVector::new(0),
        vr16: BitVector::new(0),
        vr17: BitVector::new(0),
        vr18: BitVector::new(0),
        vr19: BitVector::new(0),
        vr20: BitVector::new(0),
        vr21: BitVector::new(0),
        vr22: BitVector::new(0),
        vr23: BitVector::new(0),
        vr24: BitVector::new(0),
        vr25: BitVector::new(0),
        vr26: BitVector::new(0),
        vr27: BitVector::new(0),
        vr28: BitVector::new(0),
        vr29: BitVector::new(0),
        vr30: BitVector::new(0),
        vr31: BitVector::new(0),
        vcsr: raw::Vcsr {
            bits: BitVector::new(0),
        },
        mhpmevent: [DEFAULT_HPM_EVENT; 32],
        mhpmcounter: [ZEROES; 32],
        float_result: BitVector::new(0),
        float_fflags: BitVector::new(0),
        f0: BitVector::new(0),
        f1: BitVector::new(0),
        f2: BitVector::new(0),
        f3: BitVector::new(0),
        f4: BitVector::new(0),
        f5: BitVector::new(0),
        f6: BitVector::new(0),
        f7: BitVector::new(0),
        f8: BitVector::new(0),
        f9: BitVector::new(0),
        f10: BitVector::new(0),
        f11: BitVector::new(0),
        f12: BitVector::new(0),
        f13: BitVector::new(0),
        f14: BitVector::new(0),
        f15: BitVector::new(0),
        f16: BitVector::new(0),
        f17: BitVector::new(0),
        f18: BitVector::new(0),
        f19: BitVector::new(0),
        f20: BitVector::new(0),
        f21: BitVector::new(0),
        f22: BitVector::new(0),
        f23: BitVector::new(0),
        f24: BitVector::new(0),
        f25: BitVector::new(0),
        f26: BitVector::new(0),
        f27: BitVector::new(0),
        f28: BitVector::new(0),
        f29: BitVector::new(0),
        f30: BitVector::new(0),
        f31: BitVector::new(0),
        fcsr: raw::Fcsr {
            bits: BitVector::new(0),
        },
        mcyclecfg: raw::CountSmcntrpmf {
            bits: BitVector::new(0),
        },
        minstretcfg: raw::CountSmcntrpmf {
            bits: BitVector::new(0),
        },
        mtimecmp: BitVector::new(0),
        stimecmp: BitVector::new(0),
        htif_tohost: BitVector::new(0),
        htif_done: false,
        htif_exit_code: BitVector::new(0),
        htif_cmd_write: false,
        htif_payload_writes: BitVector::new(0),
        tlb: [DEFAULT_TLB_ENTRY; raw::num_tlb_entries],
        satp: BitVector::new(0),
        hart_state: raw::HartState::HART_ACTIVE(()),
        config,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pmp_check() {
        let mut ctx = new_ctx(config::U74);
        let addr = 0x8000_0000;
        let access = raw::AccessType::Read(());

        // Check the default access rights
        assert!(
            ctx.pmp_check(addr, access).is_none(),
            "M-mode can access all memory by default"
        );

        ctx.set_mode(Privilege::User);
        assert_eq!(
            ctx.pmp_check(addr, access),
            Some(ExceptionType::E_Load_Access_Fault(())),
            "U-mode has no access by default"
        );

        // Now let's add a PMP entry to allow reads from U-mode
        let pmp_addr = addr >> 2; // There is a shift of 2 in the pmpaddr registers
        ctx.set_pmpaddr(0, pmp_addr);
        ctx.set_pmpaddr(1, 2 * pmp_addr);
        ctx.set_pmpcfg(0, 0b0000_1001 << 8); // Entry 1, Read-only access, ToR matching mode
        assert!(
            ctx.pmp_check(addr, access).is_none(),
            "PMP allow read access"
        );
    }
}
