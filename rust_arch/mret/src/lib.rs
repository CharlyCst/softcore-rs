mod mirage;
mod sail;

use mirage::{VirtContext, VirtCsr};
use sail::Privilege;
use sail_prelude::BitVector;

#[derive(Debug)]
pub struct SailVirtContext {
    mepc: BitVector<64>,
    sepc: BitVector<64>,
    mstatus: BitVector<64>,
    next_pc: BitVector<64>,
    pc: BitVector<64>,
    cur_privilege: sail::Privilege,
}

impl SailVirtContext {
    pub fn from(ctx: &VirtContext) -> Self {
        SailVirtContext {
            mepc: BitVector::new(ctx.csr.mepc as u64),
            sepc: BitVector::new(ctx.csr.sepc as u64),
            mstatus: BitVector::new(ctx.csr.mstatus as u64),
            next_pc: BitVector::new(ctx.pc as u64),
            pc: BitVector::new(ctx.pc as u64),
            cur_privilege: match ctx.mode {
                mirage::Mode::U => Privilege::User,
                mirage::Mode::S => Privilege::Supervisor,
                mirage::Mode::M => Privilege::Machine,
            },
        }
    }

    pub fn into_virt_context(self) -> VirtContext {
        VirtContext {
            csr: VirtCsr {
                mepc: self.mepc.bits() as usize,
                sepc: self.sepc.bits() as usize,
                mstatus: self.mstatus.bits() as usize,
            },
            mode: match self.cur_privilege {
                Privilege::User => mirage::Mode::U,
                Privilege::Supervisor => mirage::Mode::S,
                Privilege::Machine => mirage::Mode::M,
            },
            pc: self.next_pc.bits() as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sail::Privilege;

    #[test]
    fn simple_mret() {
        let mepc = 0x1000;
        let mut ctx = SailVirtContext {
            mepc: BitVector::new(mepc),
            sepc: BitVector::new(0x2000),
            mstatus: BitVector::new(0x0),
            next_pc: BitVector::new(0x4004),
            pc: BitVector::new(0x4000),
            cur_privilege: Privilege::Machine,
        };

        sail::execute_MRET(&mut ctx);
        assert_eq!(ctx.next_pc.bits(), mepc);
        // panic!("{ctx:#x?}");
    }
}

#[cfg(kani)]
mod verification {
    use super::*;
    use mirage::{mstatus, Mode};

    // #[kani::proof]
    // pub fn test_mret() {
    //     let mepc = kani::any::<u64>() & (!0b11);
    //     let mut ctx = SailVirtContext {
    //         mepc: BitVector::new(mepc),
    //         sepc: BitVector::new(kani::any()),
    //         mstatus: BitVector::new(0x0),
    //         next_pc: BitVector::new(kani::any()),
    //         pc: BitVector::new(kani::any()),
    //         cur_privilege: Privilege::Machine,
    //     };

    //     sail::execute_MRET(&mut ctx);
    //     assert_eq!(ctx.next_pc.bits(), mepc);
    // }

    #[kani::proof]
    pub fn mret() {
        let mepc = kani::any::<usize>() & (!0b11);
        let mpp = match kani::any::<u8>() % 3 {
            0 => 0b00,
            1 => 0b01,
            2 => 0b11,
            _ => unreachable!(),
        };
        let mstatus = kani::any::<usize>() & !(0b11 << mstatus::MPP_OFFSET);
        let mut ctx = VirtContext {
            csr: VirtCsr {
                mepc,
                sepc: kani::any(),
                mstatus: mstatus | (mpp << mstatus::MPP_OFFSET),
            },
            mode: Mode::M,
            pc: kani::any(),
        };

        let mut sail_ctx = SailVirtContext::from(&ctx);
        // panic!("{sail_ctx:#x?}");
        sail::execute_MRET(&mut sail_ctx);
        ctx.mret();

        assert_eq!(ctx, sail_ctx.into_virt_context(), "mret equivalence");
    }
}
