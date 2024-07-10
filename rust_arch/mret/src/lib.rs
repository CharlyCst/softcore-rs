mod sail;

use sail_prelude::BitVector;

#[derive(Debug)]
pub struct VirtContext {
    mepc: BitVector<64>,
    sepc: BitVector<64>,
    mstatus: BitVector<64>,
    next_pc: BitVector<64>,
    pc: BitVector<64>,
    cur_privilege: sail::Privilege,
}

#[cfg(test)]
mod tests {
    use super::*;
    use sail::Privilege;

    #[test]
    fn simple_mret() {
        let mepc = 0x1000;
        let mut ctx = VirtContext {
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
    use sail::Privilege;

    #[kani::proof]
    pub fn mret() {
        let mepc = kani::any::<u64>() & (!0b11);
        let mut ctx = VirtContext {
            mepc: BitVector::new(mepc),
            sepc: BitVector::new(0x2000),
            mstatus: BitVector::new(0x0),
            next_pc: BitVector::new(0x4004),
            pc: BitVector::new(0x4000),
            cur_privilege: Privilege::Machine,
        };

        sail::execute_MRET(&mut ctx);
        assert_eq!(ctx.next_pc.bits(), mepc);
    }
}
