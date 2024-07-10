mod sail;

use sail_prelude::BitVector;
use sail::Privilege;

#[derive(Debug)]
pub struct VirtContext {
    mepc: BitVector::<64>,
    sepc: BitVector::<64>,
    mstatus: BitVector::<64>,
    next_pc: BitVector::<64>,
    pc: BitVector::<64>,
    cur_privilege: sail::Privilege,
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn simple_mret() {
        let mut ctx = VirtContext {
            mepc: BitVector::new(0x1000),
            sepc: BitVector::new(0x2000),
            mstatus: BitVector::new(0x0),
            next_pc: BitVector::new(0x4004),
            pc: BitVector::new(0x4000),
            cur_privilege: Privilege::Machine
        };

        sail::execute_MRET(&mut ctx);
        panic!("{ctx:#x?}");
    }
}
