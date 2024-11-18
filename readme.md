# Virt-sail: A Sail to Rust Transpiler

> **Warning:** The project is currently under development. The generated code does not provide any guarantees regarding completeness or correctness.
 
**Virt-sail** transpiles the official Sail RISC-V specification into Rust. Sail is an ISA description language that contains formal specifications for RISC-V and other hardware architectures.

### Installation

To install Virt-sail, simply run:

```bash
make install
```

It is important to choose the version 0.17.1 [link](https://ocaml.org/p/libsail/0.17.1) of libsail, otherwise the code won't compile.

### Current semantic violations

> Throwing an error implies a direct panic in rust. Virt-sail doesn't support the try catch semantic currently.

> Currently we don't make a difference between bitvector and vector of bits (according to the documentation: "Note that a generic vector of bits and a bitvector are not the same type, despite us being able to write them using the same syntax. This means you cannot write a function that is polymorphic over both generic vectors and bitvectors). This is because we typically want these types to have very different representations in our various Sail backends, and we don’t want to be forced into a compilation strategy that requires monomorphising such functions.")


### Example Usage

You can generate a few examples by running:

```bash
make basic
```

This will generate a file out.rs.


### Example of rust transpiled code

```rust
const xlen: usize = 64;

const xlen_bytes: usize = 8;

type xlenbits = BitVector<xlen>;

type priv_level = BitVector<2>;

type regidx = BitVector<5>;

type cregidx = BitVector<3>;

type csreg = BitVector<12>;

type Mstatus = BitVector<64>;

struct SailVirtCtx {
    PC: xlenbits,
    nextPC: xlenbits,
    mepc: xlenbits,
    sepc: xlenbits,
    uepc: xlenbits,
    mstatus: Mstatus,
    cur_privilege: Privilege,
    Xs: [xlenbits;32],
}

fn EXTZ(sail_ctx: &mut SailVirtCtx, m: implicit<TodoNexpTypeVar>, v: BitVector<TodoNexpTypeVar>) -> BitVector<TodoNexpTypeVar> {
    sail_zero_extend(sail_ctx, v, m)
}

fn not(sail_ctx: &mut SailVirtCtx, p: atom_bool<TodoBoolType>) -> atom_bool<TodoBoolType> {
    not_bool(sail_ctx, p)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Privilege {
    User,
    Supervisor,
    Machine,
}

fn haveUsrMode(sail_ctx: &mut SailVirtCtx) -> bool {
    true
}

fn privLevel_to_bits(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<2> {
    match p {
        Privilege::User => BitVector::new(0b00),
        Privilege::Supervisor => BitVector::new(0b01),
        Privilege::Machine => BitVector::new(0b11),
    }
}

fn privLevel_of_bits(sail_ctx: &mut SailVirtCtx, p: BitVector<2>) -> Privilege {
    match p.bits() {
        0b00 => Privilege::User,
        0b01 => Privilege::Supervisor,
        0b11 => Privilege::Machine,
        _ => not_implemented(sail_ctx),
    }
}

fn pc_alignment_mask(sail_ctx: &mut SailVirtCtx) -> BitVector<64> {
    not_vec(sail_ctx, (BitVector::new(0b10) as u64))
}

fn _get_Mstatus_MPIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<7, 8, 1>(sail_ctx)
}

fn _get_Mstatus_MPP(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<2> {
    v.subrange::<11, 13, 2>(sail_ctx)
}

fn set_next_pc(sail_ctx: &mut SailVirtCtx, pc: BitVector<64>) {
    sail_ctx.nextPC = pc
}

fn handle_illegal(sail_ctx: &mut SailVirtCtx) {
    
}

fn get_xret_target(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<64> {
    match p {
        Privilege::Machine => sail_ctx.mepc,
        Privilege::Supervisor => sail_ctx.sepc,
        Privilege::User => sail_ctx.uepc,
    }
}

fn prepare_xret_target(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<64> {
    get_xret_target(sail_ctx, p)
}

fn exception_handler(sail_ctx: &mut SailVirtCtx, cur_priv: Privilege, pc: BitVector<64>) -> BitVector<64> {
    let prev_priv = sail_ctx.cur_privilege;
    sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<3, 4, 1>(_get_Mstatus_MPIE(sail_ctx, sail_ctx.mstatus));
    sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<7, 8, 1>(BitVector::new(0b1));
    sail_ctx.cur_privilege = privLevel_of_bits(sail_ctx, _get_Mstatus_MPP(sail_ctx, sail_ctx.mstatus));
    sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<11, 13, 2>(privLevel_to_bits(sail_ctx, if haveUsrMode(sail_ctx) {
        Privilege::User
    } else {
        Privilege::Machine
    }));
    if (sail_ctx.cur_privilege != Privilege::Machine) {
        sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<17, 18, 1>(BitVector::new(0b0))
    } else {
        
    };
    (prepare_xret_target(sail_ctx, Privilege::Machine) & pc_alignment_mask(sail_ctx))
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Retired {
    RETIRE_SUCCESS,
    RETIRE_FAIL,
}

fn ext_check_xret_priv(sail_ctx: &mut SailVirtCtx, p: Privilege) -> bool {
    true
}

fn ext_fail_xret_priv(sail_ctx: &mut SailVirtCtx) {
    
}

fn execute_MRET(sail_ctx: &mut SailVirtCtx) {
    if (sail_ctx.cur_privilege != Privilege::Machine) {
        {
            handle_illegal(sail_ctx);
            Retired::RETIRE_FAIL
        }
    } else if not(sail_ctx, ext_check_xret_priv(sail_ctx, Privilege::Machine)) {
        {
            ext_fail_xret_priv(sail_ctx);
            Retired::RETIRE_FAIL
        }
    } else {
        {
            set_next_pc(sail_ctx, exception_handler(sail_ctx, sail_ctx.cur_privilege, sail_ctx.PC));
            Retired::RETIRE_SUCCESS
        }
    }
}
```