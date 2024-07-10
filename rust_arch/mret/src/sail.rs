use sail_prelude::*;

fn haveUsrMode(TodoArgs: ()) -> bool {
    true
}

fn privLevel_to_bits(p: Privilege) -> BitVector<2> {
    match p.bits {
        User => 0b00,
        Supervisor => 0b01,
        Machine => 0b11,
    }
}

fn privLevel_of_bits(p: BitVector<2>) -> Privilege {
    match p.bits() {
        0b00 => User,
        0b01 => Supervisor,
        0b11 => Machine,
        _: TYPE_TODO => not_implemented("Invalid privilege level"),
    }
}

fn pc_alignment_mask() -> BitVector<64> {
    not_vec((0b10 as u64))
}

fn _get_Mstatus_MPIE(v: Mstatus) -> BitVector<1> {
    v.subrange::<7, 8, 1>()
}

fn _get_Mstatus_MPP(v: Mstatus) -> BitVector<2> {
    v.subrange::<11, 13, 2>()
}

fn set_next_pc(pc: BitVector<64>) {
    nextPC = pc
}

fn handle_illegal(TodoArgs: ()) {
    ()
}

fn get_xret_target(p: Privilege) -> BitVector<64> {
    match p {
        Machine => mepc,
        Supervisor => sepc,
        User => uepc,
    }
}

fn prepare_xret_target(p: Privilege) -> BitVector<64> {
    get_xret_target(p)
}

fn exception_handler(cur_priv: Privilege, pc: BitVector<64>) -> BitVector<64> {
    let prev_priv = cur_privilege;
    mstatus.bits[(3)..=(3)] = _get_Mstatus_MPIE(mstatus);
    mstatus.bits[(7)..=(7)] = 0b1;
    cur_privilege = privLevel_of_bits(_get_Mstatus_MPP(mstatus));
    mstatus.bits[(11)..=(12)] = privLevel_to_bits(if haveUsrMode(()) {
        User
    } else {
        Machine
    });
    if neq_anything(cur_privilege, Machine) {
        mstatus.bits[(17)..=(17)] = 0b0
    } else {
        ()
    };
    (prepare_xret_target(Machine) & pc_alignment_mask(()))
}

fn ext_check_xret_priv(TodoArgs: Privilege) -> bool {
    true
}

fn ext_fail_xret_priv(TodoArgs: ()) {
    ()
}

fn execute_MRET(TodoArgsApp: TodoUnsupportedUnionSignature) {
    if neq_anything(cur_privilege, Machine) {
        {
            handle_illegal(());
            RETIRE_FAIL
        }
    } else if not(ext_check_xret_priv(Machine)) {
        {
            ext_fail_xret_priv(());
            RETIRE_FAIL
        }
    } else {
        {
        set_next_pc(exception_handler(cur_privilege, PC));
        RETIRE_SUCCESS
    }
    }
}
