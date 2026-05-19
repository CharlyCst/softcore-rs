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
    pub x1: regtype,
    pub x2: regtype,
    pub x3: regtype,
    pub x4: regtype,
    pub x5: regtype,
    pub x6: regtype,
    pub x7: regtype,
    pub x8: regtype,
    pub x9: regtype,
    pub x10: regtype,
    pub x11: regtype,
    pub x12: regtype,
    pub x13: regtype,
    pub x14: regtype,
    pub x15: regtype,
    pub x16: regtype,
    pub x17: regtype,
    pub x18: regtype,
    pub x19: regtype,
    pub x20: regtype,
    pub x21: regtype,
    pub x22: regtype,
    pub x23: regtype,
    pub x24: regtype,
    pub x25: regtype,
    pub x26: regtype,
    pub x27: regtype,
    pub x28: regtype,
    pub x29: regtype,
    pub x30: regtype,
    pub x31: regtype,
    pub cur_privilege: Privilege,
    pub cur_inst: xlenbits,
    pub misa: Misa,
    pub mstatus: Mstatus,
    pub menvcfg: MEnvcfg,
    pub senvcfg: SEnvcfg,
    pub mie: Minterrupts,
    pub mip: Minterrupts,
    pub medeleg: Medeleg,
    pub mideleg: Minterrupts,
    pub mtvec: Mtvec,
    pub mcause: Mcause,
    pub mepc: xlenbits,
    pub mtval: xlenbits,
    pub mscratch: xlenbits,
    pub scounteren: Counteren,
    pub mcounteren: Counteren,
    pub mcountinhibit: Counterin,
    pub mcycle: BitDynamic,
    pub mtime: BitDynamic,
    pub minstret: BitDynamic,
    pub minstret_increment: bool,
    pub mvendorid: BitDynamic,
    pub mimpid: xlenbits,
    pub marchid: xlenbits,
    pub mhartid: xlenbits,
    pub mconfigptr: xlenbits,
    pub stvec: Mtvec,
    pub sscratch: xlenbits,
    pub sepc: xlenbits,
    pub scause: Mcause,
    pub stval: xlenbits,
    pub tselect: xlenbits,
    pub vstart: BitDynamic,
    pub vl: xlenbits,
    pub vtype: Vtype,
    pub pmpcfg_n: [Pmpcfg_ent; (64 as usize)],
    pub pmpaddr_n: [xlenbits; (64 as usize)],
    pub vr0: vregtype,
    pub vr1: vregtype,
    pub vr2: vregtype,
    pub vr3: vregtype,
    pub vr4: vregtype,
    pub vr5: vregtype,
    pub vr6: vregtype,
    pub vr7: vregtype,
    pub vr8: vregtype,
    pub vr9: vregtype,
    pub vr10: vregtype,
    pub vr11: vregtype,
    pub vr12: vregtype,
    pub vr13: vregtype,
    pub vr14: vregtype,
    pub vr15: vregtype,
    pub vr16: vregtype,
    pub vr17: vregtype,
    pub vr18: vregtype,
    pub vr19: vregtype,
    pub vr20: vregtype,
    pub vr21: vregtype,
    pub vr22: vregtype,
    pub vr23: vregtype,
    pub vr24: vregtype,
    pub vr25: vregtype,
    pub vr26: vregtype,
    pub vr27: vregtype,
    pub vr28: vregtype,
    pub vr29: vregtype,
    pub vr30: vregtype,
    pub vr31: vregtype,
    pub vcsr: Vcsr,
    pub mhpmevent: [HpmEvent; (32 as usize)],
    pub mhpmcounter: [BitDynamic; (32 as usize)],
    pub mcyclecfg: CountSmcntrpmf,
    pub minstretcfg: CountSmcntrpmf,
    pub mtimecmp: BitDynamic,
    pub stimecmp: BitDynamic,
    pub htif_tohost: BitDynamic,
    pub htif_done: bool,
    pub htif_exit_code: BitDynamic,
    pub htif_cmd_write: bool,
    pub htif_payload_writes: BitDynamic,
    pub tlb: [Option<TLB_Entry>; (num_tlb_entries as usize)],
    pub satp: xlenbits,
    pub hart_state: HartState,
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {
    pub base: ConfigBase,
    pub extensions: ConfigExtensions,
    pub memory: ConfigMemory,
    pub platform: ConfigPlatform,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigBase {
    pub mtval_has_illegal_instruction_bits: bool,
    pub writable_fiom: bool,
    pub writable_hpm_counters: BitDynamic,
    pub writable_misa: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigExtensions {
    pub A: ConfigA,
    pub B: ConfigB,
    pub FD: ConfigFD,
    pub M: ConfigM,
    pub S: ConfigS,
    pub Smcntrpmf: ConfigSmcntrpmf,
    pub Sscofpmf: ConfigSscofpmf,
    pub Sstc: ConfigSstc,
    pub Sv32: ConfigSv32,
    pub Sv39: ConfigSv39,
    pub Sv48: ConfigSv48,
    pub Sv57: ConfigSv57,
    pub Svbare: ConfigSvbare,
    pub Svinval: ConfigSvinval,
    pub U: ConfigU,
    pub V: ConfigV,
    pub Zaamo: ConfigZaamo,
    pub Zabha: ConfigZabha,
    pub Zalrsc: ConfigZalrsc,
    pub Zba: ConfigZba,
    pub Zbb: ConfigZbb,
    pub Zbc: ConfigZbc,
    pub Zbkb: ConfigZbkb,
    pub Zbkc: ConfigZbkc,
    pub Zbkx: ConfigZbkx,
    pub Zbs: ConfigZbs,
    pub Zca: ConfigZca,
    pub Zcb: ConfigZcb,
    pub Zcd: ConfigZcd,
    pub Zcf: ConfigZcf,
    pub Zcmop: ConfigZcmop,
    pub Zfa: ConfigZfa,
    pub Zfh: ConfigZfh,
    pub Zfhmin: ConfigZfhmin,
    pub Zfinx: ConfigZfinx,
    pub Zhinx: ConfigZhinx,
    pub Zicbom: ConfigZicbom,
    pub Zicboz: ConfigZicboz,
    pub Zicntr: ConfigZicntr,
    pub Zicond: ConfigZicond,
    pub Zifencei: ConfigZifencei,
    pub Zihpm: ConfigZihpm,
    pub Zimop: ConfigZimop,
    pub Zknd: ConfigZknd,
    pub Zkne: ConfigZkne,
    pub Zknh: ConfigZknh,
    pub Zkr: ConfigZkr,
    pub Zksed: ConfigZksed,
    pub Zksh: ConfigZksh,
    pub Zmmul: ConfigZmmul,
    pub Zvbb: ConfigZvbb,
    pub Zvbc: ConfigZvbc,
    pub Zvkb: ConfigZvkb,
    pub Zvknha: ConfigZvknha,
    pub Zvknhb: ConfigZvknhb,
    pub Zvksh: ConfigZvksh,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigA {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigB {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigFD {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigM {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigS {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSmcntrpmf {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSscofpmf {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSstc {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSv32 {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSv39 {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSv48 {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSv57 {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSvbare {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigSvinval {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigU {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigV {
    pub elen_exp: i128,
    pub supported: bool,
    pub vl_use_ceil: bool,
    pub vlen_exp: i128,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZaamo {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZabha {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZalrsc {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZba {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZbb {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZbc {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZbkb {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZbkc {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZbkx {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZbs {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZca {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZcb {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZcd {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZcf {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZcmop {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZfa {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZfh {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZfhmin {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZfinx {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZhinx {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZicbom {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZicboz {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZicntr {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZicond {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZifencei {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZihpm {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZimop {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZknd {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZkne {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZknh {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZkr {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZksed {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZksh {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZmmul {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZvbb {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZvbc {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZvkb {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZvknha {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZvknhb {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigZvksh {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigMemory {
    pub pmp: ConfigPmp,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigPmp {
    pub count: i128,
    pub grain: i128,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigPlatform {
    pub archid: i128,
    pub cache_block_size_exp: i128,
    pub hartid: i128,
    pub impid: i128,
    pub vendorid: i128,
}

/// Initialize all registers.
///
/// This function should be called before using a fresh core, otherwise the core might not be in a valid state.
pub fn _reset_all_registers(core_ctx: &mut Core) {
    core_ctx.misa = _reset_misa();
    core_ctx.mstatus = _reset_mstatus(core_ctx);
    core_ctx.menvcfg = _reset_menvcfg(core_ctx);
    core_ctx.senvcfg = _reset_senvcfg(core_ctx);
    core_ctx.mvendorid = _reset_mvendorid(core_ctx);
    core_ctx.mimpid = _reset_mimpid(core_ctx);
    core_ctx.marchid = _reset_marchid(core_ctx);
    core_ctx.mhartid = _reset_mhartid(core_ctx);
    core_ctx.mconfigptr = _reset_mconfigptr();
    core_ctx.tlb = _reset_tlb();
    core_ctx.hart_state = _reset_hart_state()
}

/// Initialize the misa register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L80-83.
pub fn _reset_misa() -> Misa {
    _update_Misa_MXL(Mk_Misa(zeros(64)), architecture_forwards(Architecture::RV64))
}

/// Initialize the mstatus register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L275-284.
pub fn _reset_mstatus(core_ctx: &mut Core) -> Mstatus {
    let mxl: BitDynamic = architecture_forwards(Architecture::RV64);
    {
        let var_1: Mstatus = {
            let var_3: BitDynamic = if {((64 != 32) && hartSupports(core_ctx, extension::Ext_S))} {
                mxl
            } else {
                zeros(2)
            };
            _update_Mstatus_SXL(Mk_Mstatus(zeros(64)), var_3)
        };
        let var_2: BitDynamic = if {((64 != 32) && hartSupports(core_ctx, extension::Ext_U))} {
            mxl
        } else {
            zeros(2)
        };
        _update_Mstatus_UXL(var_1, var_2)
    }
}

/// Initialize the menvcfg register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L371.
pub fn _reset_menvcfg(core_ctx: &mut Core) -> MEnvcfg {
    legalize_menvcfg(core_ctx, Mk_MEnvcfg(zeros(64)), zeros(64))
}

/// Initialize the senvcfg register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L372.
pub fn _reset_senvcfg(core_ctx: &mut Core) -> SEnvcfg {
    legalize_senvcfg(core_ctx, Mk_SEnvcfg(zeros(64)), zeros(64))
}

/// Initialize the mvendorid register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L652.
pub fn _reset_mvendorid(core_ctx: &mut Core) -> BitDynamic {
    to_bits(32, (core_ctx.config.platform.vendorid as i128))
}

/// Initialize the mimpid register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L653.
pub fn _reset_mimpid(core_ctx: &mut Core) -> xlenbits {
    to_bits(64, (core_ctx.config.platform.impid as i128))
}

/// Initialize the marchid register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L654.
pub fn _reset_marchid(core_ctx: &mut Core) -> xlenbits {
    to_bits(64, (core_ctx.config.platform.archid as i128))
}

/// Initialize the mhartid register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L655.
pub fn _reset_mhartid(core_ctx: &mut Core) -> xlenbits {
    to_bits(64, (core_ctx.config.platform.hartid as i128))
}

/// Initialize the mconfigptr register.
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L657.
pub fn _reset_mconfigptr() -> xlenbits {
    zeros(64)
}

/// Initialize the tlb register.
///
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L73.
pub fn _reset_tlb() -> [Option<TLB_Entry>; (num_tlb_entries as usize)] {
    [None; (64 as usize)]
}

/// Initialize the hart_state register.
///
/// Generated from the Sail sources at `riscv_step.sail` L11.
pub fn _reset_hart_state() -> HartState {
    HartState::HART_ACTIVE(())
}

/// neq_bool
///
/// Generated from the Sail sources at `sail/lib/flow.sail` L81.
pub fn neq_bool(x: bool, y: bool) -> bool {
    !((x == y))
}

/// __id
///
/// Generated from the Sail sources at `sail/lib/flow.sail` L107.
pub fn __id(x: i128) -> i128 {
    x
}

pub type xlenbits = BitDynamic;

/// virtaddr
///
/// Generated from the Sail sources at `prelude_mem_addrtype.sail` L17.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum virtaddr {
    Virtaddr(xlenbits)
}

/// sub_virtaddr_xlenbits
///
/// Generated from the Sail sources at `prelude_mem_addrtype.sail` L23.
pub fn sub_virtaddr_xlenbits(virtaddr::Virtaddr(addr): virtaddr, offset: BitDynamic) -> virtaddr {
    virtaddr::Virtaddr(sub_vec(addr, offset))
}

/// hex_bits_forwards_matches
///
/// Generated from the Sail sources at `sail/lib/hex_bits.sail` L63.
pub const fn hex_bits_forwards_matches(bv: BitDynamic) -> bool {
    true
}

/// hex_bits_backwards
///
/// Generated from the Sail sources at `sail/lib/hex_bits.sail` L65.
pub fn hex_bits_backwards(n: i128, str: &'static str) -> BitDynamic {
    parse_hex_bits(n, str)
}

/// get_config_print_platform
///
/// Generated from the Sail sources at `prelude.sail` L75.
pub const fn get_config_print_platform(unit_arg: ()) -> bool {
    false
}

/// zeros
///
/// Generated from the Sail sources at `prelude.sail` L84.
pub const fn zeros(n: i128) -> BitDynamic {
    sail_zeros(n)
}

/// ones
///
/// Generated from the Sail sources at `prelude.sail` L87.
pub const fn ones(n: i128) -> BitDynamic {
    sail_ones(n)
}

/// bool_bit_backwards
///
/// Generated from the Sail sources.
pub fn bool_bit_backwards(arg_hashtag_: bool) -> bool {
    match arg_hashtag_ {
        true => {true}
        false => {false}
        _ => {panic!("Unreachable code")}
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

/// bool_not_bits_forwards
///
/// Generated from the Sail sources.
pub fn bool_not_bits_forwards(arg_hashtag_: bool) -> BitDynamic {
    match arg_hashtag_ {
        true => {BitDynamic::new(1, 0b0)}
        false => {BitDynamic::new(1, 0b1)}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_not_bits_backwards
///
/// Generated from the Sail sources.
pub fn bool_not_bits_backwards(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(1, 0b0))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// bool_not_bits_backwards_matches
///
/// Generated from the Sail sources.
pub fn bool_not_bits_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(1, 0b0))} => {true}
        b__1 if {(b__1 == BitDynamic::new(1, 0b1))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// bit_to_bool
///
/// Generated from the Sail sources at `prelude.sail` L111.
pub fn bit_to_bool(x: bool) -> bool {
    bool_bit_backwards(x)
}

/// bool_to_bits
///
/// Generated from the Sail sources at `prelude.sail` L112.
pub fn bool_to_bits(x: bool) -> BitDynamic {
    bool_bits_forwards(x)
}

/// to_bits
///
/// Generated from the Sail sources at `prelude.sail` L117.
pub fn to_bits(l: i128, n: i128) -> BitDynamic {
    get_slice_int(l, n, 0)
}

/// (operator <_s)
///
/// Generated from the Sail sources at `prelude.sail` L137.
pub fn _operator_smaller_s_(x: BitDynamic, y: BitDynamic) -> bool {
    (x.signed() < y.signed())
}

/// (operator >=_s)
///
/// Generated from the Sail sources at `prelude.sail` L140.
pub fn _operator_biggerequal_s_(x: BitDynamic, y: BitDynamic) -> bool {
    (x.signed() >= y.signed())
}

/// (operator <_u)
///
/// Generated from the Sail sources at `prelude.sail` L141.
pub fn _operator_smaller_u_(x: BitDynamic, y: BitDynamic) -> bool {
    (x.unsigned() < y.unsigned())
}

/// (operator >=_u)
///
/// Generated from the Sail sources at `prelude.sail` L144.
pub fn _operator_biggerequal_u_(x: BitDynamic, y: BitDynamic) -> bool {
    (x.unsigned() >= y.unsigned())
}

/// shift_bits_right_arith
///
/// Generated from the Sail sources at `prelude.sail` L166-167.
pub fn shift_bits_right_arith(value: BitDynamic, shift: BitDynamic) -> BitDynamic {
    shift_right_arith(value, shift.unsigned())
}

/// rotate_bits_left
///
/// Generated from the Sail sources at `prelude.sail` L177-178.
pub fn rotate_bits_left(v: BitDynamic, n: BitDynamic) -> BitDynamic {
    (shift_bits_left(v, n) | shift_bits_right(v, sub_vec(to_bits(bitvector_length(n), bitvector_length(v)), n)))
}

/// rotatel
///
/// Generated from the Sail sources at `prelude.sail` L185-186.
pub fn rotatel(v: BitDynamic, n: i128) -> BitDynamic {
    ((v << n) | (v >> (bitvector_length(v) - n)))
}

/// log2
///
/// Generated from the Sail sources at `prelude.sail` L207-218.
pub fn log2(n: i128) -> i128 {
    let result: i128 = match n {
        l__789 if {(l__789 == 1)} => {0}
        l__790 if {(l__790 == 2)} => {1}
        l__791 if {(l__791 == 4)} => {2}
        l__792 if {(l__792 == 8)} => {3}
        l__793 if {(l__793 == 16)} => {4}
        l__794 if {(l__794 == 32)} => {5}
        _ => {6}
        _ => {panic!("Unreachable code")}
    };
    result
}

/// max_mem_access
///
/// Generated from the Sail sources at `prelude.sail` L233.
pub const max_mem_access: i128 = 4096;

pub type mem_access_width = i128;

/// exception
///
/// Generated from the Sail sources at `riscv_errors.sail` L11-14.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_not_implemented(&'static str),
    Error_internal_error(())
}

/// log2_xlen
///
/// Generated from the Sail sources at `riscv_xlen.sail` L17.
pub const log2_xlen: i128 = 6;

/// log2_xlen_bytes
///
/// Generated from the Sail sources at `riscv_xlen.sail` L18.
pub const log2_xlen_bytes: i128 = 3;

/// xlen_bytes
///
/// Generated from the Sail sources at `riscv_xlen.sail` L19.
pub const xlen_bytes: i128 = 8;

/// xlen
///
/// Generated from the Sail sources at `riscv_xlen.sail` L20.
pub const xlen: i128 = 64;

/// asidlen
///
/// Generated from the Sail sources at `riscv_xlen.sail` L22.
pub const asidlen: i128 = 16;

pub type asidbits = BitDynamic;

pub type flenbits = BitDynamic;

/// flen_bytes
///
/// Generated from the Sail sources at `riscv_flen.sail` L15.
pub const flen_bytes: i128 = 8;

/// flen
///
/// Generated from the Sail sources at `riscv_flen.sail` L16.
pub const flen: i128 = 64;

/// get_elen_pow
///
/// Generated from the Sail sources at `riscv_vlen.sail` L9.
pub fn get_elen_pow(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    core_ctx.config.extensions.V.elen_exp
}

/// vlen_exp
///
/// Generated from the Sail sources at `riscv_vlen.sail` L16.
pub fn vlen_exp(core_ctx: &mut Core) -> i128 {
    core_ctx.config.extensions.V.vlen_exp
}

/// vlen
///
/// Generated from the Sail sources at `riscv_vlen.sail` L19.
pub fn vlen(core_ctx: &mut Core) -> i128 {
    i128::pow(2, ((vlen_exp(core_ctx) as u32) as u32))
}

/// get_vlen_pow
///
/// Generated from the Sail sources at `riscv_vlen.sail` L22.
pub fn get_vlen_pow(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    vlen_exp(core_ctx)
}

/// get_vlen
///
/// Generated from the Sail sources at `riscv_vlen.sail` L23.
pub fn get_vlen(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    i128::pow(2, (vlen_exp(core_ctx) as u32))
}

pub type physaddrbits = BitDynamic;

/// physaddrbits_len
///
/// Generated from the Sail sources at `prelude_mem_addrtype.sail` L14.
pub const physaddrbits_len: i128 = 64;

/// physaddr
///
/// Generated from the Sail sources at `prelude_mem_addrtype.sail` L16.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum physaddr {
    Physaddr(physaddrbits)
}

/// bits_of_virtaddr
///
/// Generated from the Sail sources at `prelude_mem_addrtype.sail` L21.
pub fn bits_of_virtaddr(virtaddr::Virtaddr(vaddr): virtaddr) -> BitDynamic {
    vaddr
}

pub type mem_meta = ();

/// default_meta
///
/// Generated from the Sail sources at `prelude_mem_metadata.sail` L15.
pub const default_meta: mem_meta = ();

/// result
///
/// Generated from the Sail sources at `sail/lib/result.sail` L60-63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum result<A, B> {
    Ok(A),
    Err(B)
}

/// sail_branch_announce
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/common.sail` L92.
pub const fn sail_branch_announce(_: i128, _: BitDynamic) {
    ()
}

/// Access_variety
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write_v1.sail` L58-62.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_variety {
    AV_plain,
    AV_exclusive,
    AV_atomic_rmw
}

/// Access_strength
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write_v1.sail` L67-71.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_strength {
    AS_normal,
    AS_rel_or_acq,
    AS_acq_rcpc
}

/// Explicit_access_kind
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write_v1.sail` L76-79.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Explicit_access_kind {
    pub variety: Access_variety,
    pub strength: Access_strength,
}

/// Access_kind
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write_v1.sail` L84-89.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_kind<ARCH_AK> {
    AK_explicit(Explicit_access_kind),
    AK_ifetch(()),
    AK_ttw(()),
    AK_arch(ARCH_AK)
}

/// Mem_read_request
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write_v1.sail` L94-105.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mem_read_request<const N: i128, const VASIZE: i128, PA, TS, ARCH_AK> {
    pub access_kind: Access_kind::<ARCH_AK>,
    pub va: Option<BitDynamic>,
    pub pa: PA,
    pub translation: TS,
    pub size: i128,
    pub tag: bool,
}

/// __monomorphize_reads
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write_v1.sail` L137.
pub const __monomorphize_reads: bool = false;

/// __monomorphize_writes
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write_v1.sail` L138.
pub const __monomorphize_writes: bool = false;

/// write_kind
///
/// Generated from the Sail sources at `prelude_mem.sail` L19-26.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum write_kind {
    Write_plain,
    Write_RISCV_release,
    Write_RISCV_strong_release,
    Write_RISCV_conditional,
    Write_RISCV_conditional_release,
    Write_RISCV_conditional_strong_release
}

/// read_kind
///
/// Generated from the Sail sources at `prelude_mem.sail` L28-36.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum read_kind {
    Read_plain,
    Read_ifetch,
    Read_RISCV_acquire,
    Read_RISCV_strong_acquire,
    Read_RISCV_reserved,
    Read_RISCV_reserved_acquire,
    Read_RISCV_reserved_strong_acquire
}

/// barrier_kind
///
/// Generated from the Sail sources at `prelude_mem.sail` L38-50.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum barrier_kind {
    Barrier_RISCV_rw_rw,
    Barrier_RISCV_r_rw,
    Barrier_RISCV_r_r,
    Barrier_RISCV_rw_w,
    Barrier_RISCV_w_w,
    Barrier_RISCV_w_rw,
    Barrier_RISCV_rw_r,
    Barrier_RISCV_r_w,
    Barrier_RISCV_w_r,
    Barrier_RISCV_tso,
    Barrier_RISCV_i
}

/// RISCV_strong_access
///
/// Generated from the Sail sources at `prelude_mem.sail` L55-57.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct RISCV_strong_access {
    pub variety: Access_variety,
}

/// sail_barrier
///
/// Generated from the Sail sources at `sail/lib/concurrency_interface/barrier.sail` L58.
pub const fn sail_barrier(_: barrier_kind) {
    ()
}

/// extension
///
/// Generated from the Sail sources at `riscv_extensions.sail` L10.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum extension {
    Ext_M,
    Ext_A,
    Ext_F,
    Ext_D,
    Ext_B,
    Ext_V,
    Ext_S,
    Ext_U,
    Ext_Zicbom,
    Ext_Zicboz,
    Ext_Zicntr,
    Ext_Zicond,
    Ext_Zifencei,
    Ext_Zihpm,
    Ext_Zimop,
    Ext_Zmmul,
    Ext_Zaamo,
    Ext_Zabha,
    Ext_Zalrsc,
    Ext_Zfa,
    Ext_Zfh,
    Ext_Zfhmin,
    Ext_Zfinx,
    Ext_Zdinx,
    Ext_Zca,
    Ext_Zcb,
    Ext_Zcd,
    Ext_Zcf,
    Ext_Zcmop,
    Ext_C,
    Ext_Zba,
    Ext_Zbb,
    Ext_Zbc,
    Ext_Zbkb,
    Ext_Zbkc,
    Ext_Zbkx,
    Ext_Zbs,
    Ext_Zknd,
    Ext_Zkne,
    Ext_Zknh,
    Ext_Zkr,
    Ext_Zksed,
    Ext_Zksh,
    Ext_Zhinx,
    Ext_Zvbb,
    Ext_Zvkb,
    Ext_Zvbc,
    Ext_Zvknha,
    Ext_Zvknhb,
    Ext_Zvksh,
    Ext_Sscofpmf,
    Ext_Sstc,
    Ext_Svinval,
    Ext_Svnapot,
    Ext_Svpbmt,
    Ext_Svbare,
    Ext_Sv32,
    Ext_Sv39,
    Ext_Sv48,
    Ext_Sv57,
    Ext_Smcntrpmf
}

/// hartSupports
///
/// Generated from the Sail sources at `riscv_extensions.sail` L32.
pub fn hartSupports(core_ctx: &mut Core, merge_hashtag_var: extension) -> bool {
    match merge_hashtag_var {
        extension::Ext_M => {core_ctx.config.extensions.M.supported}
        extension::Ext_A => {core_ctx.config.extensions.A.supported}
        extension::Ext_F => {core_ctx.config.extensions.FD.supported}
        extension::Ext_D => {core_ctx.config.extensions.FD.supported}
        extension::Ext_B => {core_ctx.config.extensions.B.supported}
        extension::Ext_V => {core_ctx.config.extensions.V.supported}
        extension::Ext_S => {core_ctx.config.extensions.S.supported}
        extension::Ext_U => {core_ctx.config.extensions.U.supported}
        extension::Ext_Zicbom => {core_ctx.config.extensions.Zicbom.supported}
        extension::Ext_Zicboz => {core_ctx.config.extensions.Zicboz.supported}
        extension::Ext_Zicntr => {core_ctx.config.extensions.Zicntr.supported}
        extension::Ext_Zicond => {core_ctx.config.extensions.Zicond.supported}
        extension::Ext_Zifencei => {core_ctx.config.extensions.Zifencei.supported}
        extension::Ext_Zihpm => {core_ctx.config.extensions.Zihpm.supported}
        extension::Ext_Zimop => {core_ctx.config.extensions.Zimop.supported}
        extension::Ext_Zmmul => {core_ctx.config.extensions.Zmmul.supported}
        extension::Ext_Zaamo => {core_ctx.config.extensions.Zaamo.supported}
        extension::Ext_Zabha => {core_ctx.config.extensions.Zabha.supported}
        extension::Ext_Zalrsc => {core_ctx.config.extensions.Zalrsc.supported}
        extension::Ext_Zfa => {core_ctx.config.extensions.Zfa.supported}
        extension::Ext_Zfh => {core_ctx.config.extensions.Zfh.supported}
        extension::Ext_Zfhmin => {core_ctx.config.extensions.Zfhmin.supported}
        extension::Ext_Zfinx => {core_ctx.config.extensions.Zfinx.supported}
        extension::Ext_Zdinx => {core_ctx.config.extensions.Zfinx.supported}
        extension::Ext_Zca => {core_ctx.config.extensions.Zca.supported}
        extension::Ext_Zcb => {core_ctx.config.extensions.Zcb.supported}
        extension::Ext_Zcd => {core_ctx.config.extensions.Zcd.supported}
        extension::Ext_Zcf => {false}
        extension::Ext_Zcmop => {core_ctx.config.extensions.Zcmop.supported}
        extension::Ext_C => {(hartSupports(core_ctx, extension::Ext_Zca) && ((hartSupports(core_ctx, extension::Ext_Zcf) || (!(hartSupports(core_ctx, extension::Ext_F)) || (64 != 32))) && (hartSupports(core_ctx, extension::Ext_Zcd) || !(hartSupports(core_ctx, extension::Ext_D)))))}
        extension::Ext_Zba => {core_ctx.config.extensions.Zba.supported}
        extension::Ext_Zbb => {core_ctx.config.extensions.Zbb.supported}
        extension::Ext_Zbc => {core_ctx.config.extensions.Zbc.supported}
        extension::Ext_Zbkb => {core_ctx.config.extensions.Zbkb.supported}
        extension::Ext_Zbkc => {core_ctx.config.extensions.Zbkc.supported}
        extension::Ext_Zbkx => {core_ctx.config.extensions.Zbkx.supported}
        extension::Ext_Zbs => {core_ctx.config.extensions.Zbs.supported}
        extension::Ext_Zknd => {core_ctx.config.extensions.Zknd.supported}
        extension::Ext_Zkne => {core_ctx.config.extensions.Zkne.supported}
        extension::Ext_Zknh => {core_ctx.config.extensions.Zknh.supported}
        extension::Ext_Zkr => {core_ctx.config.extensions.Zkr.supported}
        extension::Ext_Zksed => {core_ctx.config.extensions.Zksed.supported}
        extension::Ext_Zksh => {core_ctx.config.extensions.Zksh.supported}
        extension::Ext_Zhinx => {core_ctx.config.extensions.Zhinx.supported}
        extension::Ext_Zvbb => {core_ctx.config.extensions.Zvbb.supported}
        extension::Ext_Zvkb => {core_ctx.config.extensions.Zvkb.supported}
        extension::Ext_Zvbc => {core_ctx.config.extensions.Zvbc.supported}
        extension::Ext_Zvknha => {core_ctx.config.extensions.Zvknha.supported}
        extension::Ext_Zvknhb => {core_ctx.config.extensions.Zvknhb.supported}
        extension::Ext_Zvksh => {core_ctx.config.extensions.Zvksh.supported}
        extension::Ext_Sscofpmf => {core_ctx.config.extensions.Sscofpmf.supported}
        extension::Ext_Sstc => {core_ctx.config.extensions.Sstc.supported}
        extension::Ext_Svinval => {core_ctx.config.extensions.Svinval.supported}
        extension::Ext_Svnapot => {false}
        extension::Ext_Svpbmt => {false}
        extension::Ext_Svbare => {core_ctx.config.extensions.Svbare.supported}
        extension::Ext_Sv32 => {false}
        extension::Ext_Sv39 => {(core_ctx.config.extensions.Sv39.supported as bool)}
        extension::Ext_Sv48 => {(core_ctx.config.extensions.Sv48.supported as bool)}
        extension::Ext_Sv57 => {(core_ctx.config.extensions.Sv57.supported as bool)}
        extension::Ext_Smcntrpmf => {core_ctx.config.extensions.Smcntrpmf.supported}
        _ => {panic!("Unreachable code")}
    }
}

pub type exc_code = BitDynamic;

pub type ext_ptw = ();

/// init_ext_ptw
///
/// Generated from the Sail sources at `riscv_types_ext.sail` L42.
pub const init_ext_ptw: ext_ptw = ();

pub type ext_ptw_fail = ();

pub type ext_ptw_error = ();

pub type ext_exc_type = ();

/// ext_exc_type_to_bits
///
/// Generated from the Sail sources at `riscv_types_ext.sail` L61.
pub fn ext_exc_type_to_bits(unit_arg: ()) -> BitDynamic {
    BitDynamic::new(8, 0b00011000)
}

/// num_of_ext_exc_type
///
/// Generated from the Sail sources at `riscv_types_ext.sail` L65.
pub const fn num_of_ext_exc_type(unit_arg: ()) -> i128 {
    24
}

/// ext_exc_type_to_str
///
/// Generated from the Sail sources at `riscv_types_ext.sail` L69.
pub const fn ext_exc_type_to_str(unit_arg: ()) -> &'static str {
    "extension-exception"
}

/// xlen_max_unsigned
///
/// Generated from the Sail sources at `riscv_types.sail` L10.
pub const xlen_max_unsigned: i128 = 18446744073709551615;

/// xlen_max_signed
///
/// Generated from the Sail sources at `riscv_types.sail` L11.
pub const xlen_max_signed: i128 = 9223372036854775807;

/// xlen_min_signed
///
/// Generated from the Sail sources at `riscv_types.sail` L12.
pub const xlen_min_signed: i128 = -9223372036854775808;

pub type half = BitDynamic;

pub type word = BitDynamic;

pub type instbits = BitDynamic;

/// pagesize_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L21.
pub const pagesize_bits: i128 = 12;

/// regidx
///
/// Generated from the Sail sources at `riscv_types.sail` L25.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum regidx {
    Regidx(BitDynamic)
}

/// cregidx
///
/// Generated from the Sail sources at `riscv_types.sail` L26.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum cregidx {
    Cregidx(BitDynamic)
}

pub type csreg = BitDynamic;

/// regno
///
/// Generated from the Sail sources at `riscv_types.sail` L34.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum regno {
    Regno(i128)
}

/// regidx_to_regno
///
/// Generated from the Sail sources at `riscv_types.sail` L36.
pub fn regidx_to_regno(regidx::Regidx(b): regidx) -> regno {
    regno::Regno(b.unsigned())
}

/// regno_to_regidx
///
/// Generated from the Sail sources at `riscv_types.sail` L37.
pub fn regno_to_regidx(regno::Regno(b): regno) -> regidx {
    regidx::Regidx(to_bits(5, b))
}

/// creg2reg_idx
///
/// Generated from the Sail sources at `riscv_types.sail` L41.
pub fn creg2reg_idx(cregidx::Cregidx(i): cregidx) -> regidx {
    regidx::Regidx(bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b01)), BitDynamic::from(i)))
}

/// zreg
///
/// Generated from the Sail sources at `riscv_types.sail` L44.
pub const zreg: regidx = regidx::Regidx(BitDynamic::new(5, 0b00000));

/// ra
///
/// Generated from the Sail sources at `riscv_types.sail` L45.
pub const ra: regidx = regidx::Regidx(BitDynamic::new(5, 0b00001));

/// sp
///
/// Generated from the Sail sources at `riscv_types.sail` L46.
pub const sp: regidx = regidx::Regidx(BitDynamic::new(5, 0b00010));

/// Architecture
///
/// Generated from the Sail sources at `riscv_types.sail` L50.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Architecture {
    RV32,
    RV64,
    RV128
}

pub type arch_xlen = BitDynamic;

/// architecture_forwards
///
/// Generated from the Sail sources.
pub fn architecture_forwards(arg_hashtag_: Architecture) -> BitDynamic {
    match arg_hashtag_ {
        Architecture::RV32 => {BitDynamic::new(2, 0b01)}
        Architecture::RV64 => {BitDynamic::new(2, 0b10)}
        Architecture::RV128 => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// architecture_backwards
///
/// Generated from the Sail sources.
pub fn architecture_backwards(arg_hashtag_: BitDynamic) -> Architecture {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b01))} => {Architecture::RV32}
        b__1 if {(b__1 == BitDynamic::new(2, 0b10))} => {Architecture::RV64}
        b__2 if {(b__2 == BitDynamic::new(2, 0b11))} => {Architecture::RV128}
        _ => {panic!("{}, l {}: {}", "riscv_types.sail", 57, "architecture(0b00) is invalid")}
        _ => {panic!("Unreachable code")}
    }
}

pub type priv_level = BitDynamic;

/// Privilege
///
/// Generated from the Sail sources at `riscv_types.sail` L63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine
}

/// privLevel_bits_forwards
///
/// Generated from the Sail sources.
pub fn privLevel_bits_forwards(arg_hashtag_: Privilege) -> BitDynamic {
    match arg_hashtag_ {
        Privilege::User => {BitDynamic::new(2, 0b00)}
        Privilege::Supervisor => {BitDynamic::new(2, 0b01)}
        Privilege::Machine => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// privLevel_bits_backwards
///
/// Generated from the Sail sources.
pub fn privLevel_bits_backwards(arg_hashtag_: BitDynamic) -> Privilege {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {Privilege::User}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {Privilege::Supervisor}
        b__2 if {(b__2 == BitDynamic::new(2, 0b11))} => {Privilege::Machine}
        _ => {panic!("{}, l {}: {}", "riscv_types.sail", 69, format!("{}{}", "Invalid privilege level: ", bits_str(BitDynamic::new(2, 0b10))))}
        _ => {panic!("Unreachable code")}
    }
}

/// privLevel_to_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L72.
pub fn privLevel_to_bits(p: Privilege) -> BitDynamic {
    privLevel_bits_forwards(p)
}

/// privLevel_of_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L73.
pub fn privLevel_of_bits(b: BitDynamic) -> Privilege {
    privLevel_bits_backwards(b)
}

/// privLevel_to_str
///
/// Generated from the Sail sources at `riscv_types.sail` L75-80.
pub fn privLevel_to_str(p: Privilege) -> &'static str {
    match p {
        Privilege::User => {"U"}
        Privilege::Supervisor => {"S"}
        Privilege::Machine => {"M"}
        _ => {panic!("Unreachable code")}
    }
}

/// AccessType
///
/// Generated from the Sail sources at `riscv_types.sail` L86-91.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum AccessType<A> {
    Read(A),
    Write(A),
    ReadWrite((A, A)),
    InstructionFetch(())
}

/// csr_name_map_forwards
///
/// Generated from the Sail sources.
pub fn csr_name_map_forwards(arg_hashtag_: BitDynamic) -> &'static str {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(12, 0b001100000001))} => {"misa"}
        b__1 if {(b__1 == BitDynamic::new(12, 0b001100000000))} => {"mstatus"}
        b__2 if {(b__2 == BitDynamic::new(12, 0b001100001010))} => {"menvcfg"}
        b__3 if {(b__3 == BitDynamic::new(12, 0b001100011010))} => {"menvcfgh"}
        b__4 if {(b__4 == BitDynamic::new(12, 0b000100001010))} => {"senvcfg"}
        b__5 if {(b__5 == BitDynamic::new(12, 0b001100000100))} => {"mie"}
        b__6 if {(b__6 == BitDynamic::new(12, 0b001101000100))} => {"mip"}
        b__7 if {(b__7 == BitDynamic::new(12, 0b001100000010))} => {"medeleg"}
        b__8 if {(b__8 == BitDynamic::new(12, 0b001100010010))} => {"medelegh"}
        b__9 if {(b__9 == BitDynamic::new(12, 0b001100000011))} => {"mideleg"}
        b__10 if {(b__10 == BitDynamic::new(12, 0b001101000010))} => {"mcause"}
        b__11 if {(b__11 == BitDynamic::new(12, 0b001101000011))} => {"mtval"}
        b__12 if {(b__12 == BitDynamic::new(12, 0b001101000000))} => {"mscratch"}
        b__13 if {(b__13 == BitDynamic::new(12, 0b000100000110))} => {"scounteren"}
        b__14 if {(b__14 == BitDynamic::new(12, 0b001100000110))} => {"mcounteren"}
        b__15 if {(b__15 == BitDynamic::new(12, 0b001100100000))} => {"mcountinhibit"}
        b__16 if {(b__16 == BitDynamic::new(12, 0b111100010001))} => {"mvendorid"}
        b__17 if {(b__17 == BitDynamic::new(12, 0b111100010010))} => {"marchid"}
        b__18 if {(b__18 == BitDynamic::new(12, 0b111100010011))} => {"mimpid"}
        b__19 if {(b__19 == BitDynamic::new(12, 0b111100010100))} => {"mhartid"}
        b__20 if {(b__20 == BitDynamic::new(12, 0b111100010101))} => {"mconfigptr"}
        b__21 if {(b__21 == BitDynamic::new(12, 0b000100000000))} => {"sstatus"}
        b__22 if {(b__22 == BitDynamic::new(12, 0b000101000100))} => {"sip"}
        b__23 if {(b__23 == BitDynamic::new(12, 0b000100000100))} => {"sie"}
        b__24 if {(b__24 == BitDynamic::new(12, 0b000101000000))} => {"sscratch"}
        b__25 if {(b__25 == BitDynamic::new(12, 0b000101000010))} => {"scause"}
        b__26 if {(b__26 == BitDynamic::new(12, 0b000101000011))} => {"stval"}
        b__27 if {(b__27 == BitDynamic::new(12, 0b011110100000))} => {"tselect"}
        b__28 if {(b__28 == BitDynamic::new(12, 0b011110100001))} => {"tdata1"}
        b__29 if {(b__29 == BitDynamic::new(12, 0b011110100010))} => {"tdata2"}
        b__30 if {(b__30 == BitDynamic::new(12, 0b011110100011))} => {"tdata3"}
        b__31 if {(b__31 == BitDynamic::new(12, 0b001110100000))} => {"pmpcfg0"}
        b__32 if {(b__32 == BitDynamic::new(12, 0b001110100001))} => {"pmpcfg1"}
        b__33 if {(b__33 == BitDynamic::new(12, 0b001110100010))} => {"pmpcfg2"}
        b__34 if {(b__34 == BitDynamic::new(12, 0b001110100011))} => {"pmpcfg3"}
        b__35 if {(b__35 == BitDynamic::new(12, 0b001110100100))} => {"pmpcfg4"}
        b__36 if {(b__36 == BitDynamic::new(12, 0b001110100101))} => {"pmpcfg5"}
        b__37 if {(b__37 == BitDynamic::new(12, 0b001110100110))} => {"pmpcfg6"}
        b__38 if {(b__38 == BitDynamic::new(12, 0b001110100111))} => {"pmpcfg7"}
        b__39 if {(b__39 == BitDynamic::new(12, 0b001110101000))} => {"pmpcfg8"}
        b__40 if {(b__40 == BitDynamic::new(12, 0b001110101001))} => {"pmpcfg9"}
        b__41 if {(b__41 == BitDynamic::new(12, 0b001110101010))} => {"pmpcfg10"}
        b__42 if {(b__42 == BitDynamic::new(12, 0b001110101011))} => {"pmpcfg11"}
        b__43 if {(b__43 == BitDynamic::new(12, 0b001110101100))} => {"pmpcfg12"}
        b__44 if {(b__44 == BitDynamic::new(12, 0b001110101101))} => {"pmpcfg13"}
        b__45 if {(b__45 == BitDynamic::new(12, 0b001110101110))} => {"pmpcfg14"}
        b__46 if {(b__46 == BitDynamic::new(12, 0b001110101111))} => {"pmpcfg15"}
        b__47 if {(b__47 == BitDynamic::new(12, 0b001110110000))} => {"pmpaddr0"}
        b__48 if {(b__48 == BitDynamic::new(12, 0b001110110001))} => {"pmpaddr1"}
        b__49 if {(b__49 == BitDynamic::new(12, 0b001110110010))} => {"pmpaddr2"}
        b__50 if {(b__50 == BitDynamic::new(12, 0b001110110011))} => {"pmpaddr3"}
        b__51 if {(b__51 == BitDynamic::new(12, 0b001110110100))} => {"pmpaddr4"}
        b__52 if {(b__52 == BitDynamic::new(12, 0b001110110101))} => {"pmpaddr5"}
        b__53 if {(b__53 == BitDynamic::new(12, 0b001110110110))} => {"pmpaddr6"}
        b__54 if {(b__54 == BitDynamic::new(12, 0b001110110111))} => {"pmpaddr7"}
        b__55 if {(b__55 == BitDynamic::new(12, 0b001110111000))} => {"pmpaddr8"}
        b__56 if {(b__56 == BitDynamic::new(12, 0b001110111001))} => {"pmpaddr9"}
        b__57 if {(b__57 == BitDynamic::new(12, 0b001110111010))} => {"pmpaddr10"}
        b__58 if {(b__58 == BitDynamic::new(12, 0b001110111011))} => {"pmpaddr11"}
        b__59 if {(b__59 == BitDynamic::new(12, 0b001110111100))} => {"pmpaddr12"}
        b__60 if {(b__60 == BitDynamic::new(12, 0b001110111101))} => {"pmpaddr13"}
        b__61 if {(b__61 == BitDynamic::new(12, 0b001110111110))} => {"pmpaddr14"}
        b__62 if {(b__62 == BitDynamic::new(12, 0b001110111111))} => {"pmpaddr15"}
        b__63 if {(b__63 == BitDynamic::new(12, 0b001111000000))} => {"pmpaddr16"}
        b__64 if {(b__64 == BitDynamic::new(12, 0b001111000001))} => {"pmpaddr17"}
        b__65 if {(b__65 == BitDynamic::new(12, 0b001111000010))} => {"pmpaddr18"}
        b__66 if {(b__66 == BitDynamic::new(12, 0b001111000011))} => {"pmpaddr19"}
        b__67 if {(b__67 == BitDynamic::new(12, 0b001111000100))} => {"pmpaddr20"}
        b__68 if {(b__68 == BitDynamic::new(12, 0b001111000101))} => {"pmpaddr21"}
        b__69 if {(b__69 == BitDynamic::new(12, 0b001111000110))} => {"pmpaddr22"}
        b__70 if {(b__70 == BitDynamic::new(12, 0b001111000111))} => {"pmpaddr23"}
        b__71 if {(b__71 == BitDynamic::new(12, 0b001111001000))} => {"pmpaddr24"}
        b__72 if {(b__72 == BitDynamic::new(12, 0b001111001001))} => {"pmpaddr25"}
        b__73 if {(b__73 == BitDynamic::new(12, 0b001111001010))} => {"pmpaddr26"}
        b__74 if {(b__74 == BitDynamic::new(12, 0b001111001011))} => {"pmpaddr27"}
        b__75 if {(b__75 == BitDynamic::new(12, 0b001111001100))} => {"pmpaddr28"}
        b__76 if {(b__76 == BitDynamic::new(12, 0b001111001101))} => {"pmpaddr29"}
        b__77 if {(b__77 == BitDynamic::new(12, 0b001111001110))} => {"pmpaddr30"}
        b__78 if {(b__78 == BitDynamic::new(12, 0b001111001111))} => {"pmpaddr31"}
        b__79 if {(b__79 == BitDynamic::new(12, 0b001111010000))} => {"pmpaddr32"}
        b__80 if {(b__80 == BitDynamic::new(12, 0b001111010001))} => {"pmpaddr33"}
        b__81 if {(b__81 == BitDynamic::new(12, 0b001111010010))} => {"pmpaddr34"}
        b__82 if {(b__82 == BitDynamic::new(12, 0b001111010011))} => {"pmpaddr35"}
        b__83 if {(b__83 == BitDynamic::new(12, 0b001111010100))} => {"pmpaddr36"}
        b__84 if {(b__84 == BitDynamic::new(12, 0b001111010101))} => {"pmpaddr37"}
        b__85 if {(b__85 == BitDynamic::new(12, 0b001111010110))} => {"pmpaddr38"}
        b__86 if {(b__86 == BitDynamic::new(12, 0b001111010111))} => {"pmpaddr39"}
        b__87 if {(b__87 == BitDynamic::new(12, 0b001111011000))} => {"pmpaddr40"}
        b__88 if {(b__88 == BitDynamic::new(12, 0b001111011001))} => {"pmpaddr41"}
        b__89 if {(b__89 == BitDynamic::new(12, 0b001111011010))} => {"pmpaddr42"}
        b__90 if {(b__90 == BitDynamic::new(12, 0b001111011011))} => {"pmpaddr43"}
        b__91 if {(b__91 == BitDynamic::new(12, 0b001111011100))} => {"pmpaddr44"}
        b__92 if {(b__92 == BitDynamic::new(12, 0b001111011101))} => {"pmpaddr45"}
        b__93 if {(b__93 == BitDynamic::new(12, 0b001111011110))} => {"pmpaddr46"}
        b__94 if {(b__94 == BitDynamic::new(12, 0b001111011111))} => {"pmpaddr47"}
        b__95 if {(b__95 == BitDynamic::new(12, 0b001111100000))} => {"pmpaddr48"}
        b__96 if {(b__96 == BitDynamic::new(12, 0b001111100001))} => {"pmpaddr49"}
        b__97 if {(b__97 == BitDynamic::new(12, 0b001111100010))} => {"pmpaddr50"}
        b__98 if {(b__98 == BitDynamic::new(12, 0b001111100011))} => {"pmpaddr51"}
        b__99 if {(b__99 == BitDynamic::new(12, 0b001111100100))} => {"pmpaddr52"}
        b__100 if {(b__100 == BitDynamic::new(12, 0b001111100101))} => {"pmpaddr53"}
        b__101 if {(b__101 == BitDynamic::new(12, 0b001111100110))} => {"pmpaddr54"}
        b__102 if {(b__102 == BitDynamic::new(12, 0b001111100111))} => {"pmpaddr55"}
        b__103 if {(b__103 == BitDynamic::new(12, 0b001111101000))} => {"pmpaddr56"}
        b__104 if {(b__104 == BitDynamic::new(12, 0b001111101001))} => {"pmpaddr57"}
        b__105 if {(b__105 == BitDynamic::new(12, 0b001111101010))} => {"pmpaddr58"}
        b__106 if {(b__106 == BitDynamic::new(12, 0b001111101011))} => {"pmpaddr59"}
        b__107 if {(b__107 == BitDynamic::new(12, 0b001111101100))} => {"pmpaddr60"}
        b__108 if {(b__108 == BitDynamic::new(12, 0b001111101101))} => {"pmpaddr61"}
        b__109 if {(b__109 == BitDynamic::new(12, 0b001111101110))} => {"pmpaddr62"}
        b__110 if {(b__110 == BitDynamic::new(12, 0b001111101111))} => {"pmpaddr63"}
        b__111 if {(b__111 == BitDynamic::new(12, 0b000000001000))} => {"vstart"}
        b__112 if {(b__112 == BitDynamic::new(12, 0b000000001001))} => {"vxsat"}
        b__113 if {(b__113 == BitDynamic::new(12, 0b000000001010))} => {"vxrm"}
        b__114 if {(b__114 == BitDynamic::new(12, 0b000000001111))} => {"vcsr"}
        b__115 if {(b__115 == BitDynamic::new(12, 0b110000100000))} => {"vl"}
        b__116 if {(b__116 == BitDynamic::new(12, 0b110000100001))} => {"vtype"}
        b__117 if {(b__117 == BitDynamic::new(12, 0b110000100010))} => {"vlenb"}
        b__118 if {(b__118 == BitDynamic::new(12, 0b000100000101))} => {"stvec"}
        b__119 if {(b__119 == BitDynamic::new(12, 0b000101000001))} => {"sepc"}
        b__120 if {(b__120 == BitDynamic::new(12, 0b001100000101))} => {"mtvec"}
        b__121 if {(b__121 == BitDynamic::new(12, 0b001101000001))} => {"mepc"}
        b__122 if {(b__122 == BitDynamic::new(12, 0b110000000011))} => {"hpmcounter3"}
        b__123 if {(b__123 == BitDynamic::new(12, 0b110000000100))} => {"hpmcounter4"}
        b__124 if {(b__124 == BitDynamic::new(12, 0b110000000101))} => {"hpmcounter5"}
        b__125 if {(b__125 == BitDynamic::new(12, 0b110000000110))} => {"hpmcounter6"}
        b__126 if {(b__126 == BitDynamic::new(12, 0b110000000111))} => {"hpmcounter7"}
        b__127 if {(b__127 == BitDynamic::new(12, 0b110000001000))} => {"hpmcounter8"}
        b__128 if {(b__128 == BitDynamic::new(12, 0b110000001001))} => {"hpmcounter9"}
        b__129 if {(b__129 == BitDynamic::new(12, 0b110000001010))} => {"hpmcounter10"}
        b__130 if {(b__130 == BitDynamic::new(12, 0b110000001011))} => {"hpmcounter11"}
        b__131 if {(b__131 == BitDynamic::new(12, 0b110000001100))} => {"hpmcounter12"}
        b__132 if {(b__132 == BitDynamic::new(12, 0b110000001101))} => {"hpmcounter13"}
        b__133 if {(b__133 == BitDynamic::new(12, 0b110000001110))} => {"hpmcounter14"}
        b__134 if {(b__134 == BitDynamic::new(12, 0b110000001111))} => {"hpmcounter15"}
        b__135 if {(b__135 == BitDynamic::new(12, 0b110000010000))} => {"hpmcounter16"}
        b__136 if {(b__136 == BitDynamic::new(12, 0b110000010001))} => {"hpmcounter17"}
        b__137 if {(b__137 == BitDynamic::new(12, 0b110000010010))} => {"hpmcounter18"}
        b__138 if {(b__138 == BitDynamic::new(12, 0b110000010011))} => {"hpmcounter19"}
        b__139 if {(b__139 == BitDynamic::new(12, 0b110000010100))} => {"hpmcounter20"}
        b__140 if {(b__140 == BitDynamic::new(12, 0b110000010101))} => {"hpmcounter21"}
        b__141 if {(b__141 == BitDynamic::new(12, 0b110000010110))} => {"hpmcounter22"}
        b__142 if {(b__142 == BitDynamic::new(12, 0b110000010111))} => {"hpmcounter23"}
        b__143 if {(b__143 == BitDynamic::new(12, 0b110000011000))} => {"hpmcounter24"}
        b__144 if {(b__144 == BitDynamic::new(12, 0b110000011001))} => {"hpmcounter25"}
        b__145 if {(b__145 == BitDynamic::new(12, 0b110000011010))} => {"hpmcounter26"}
        b__146 if {(b__146 == BitDynamic::new(12, 0b110000011011))} => {"hpmcounter27"}
        b__147 if {(b__147 == BitDynamic::new(12, 0b110000011100))} => {"hpmcounter28"}
        b__148 if {(b__148 == BitDynamic::new(12, 0b110000011101))} => {"hpmcounter29"}
        b__149 if {(b__149 == BitDynamic::new(12, 0b110000011110))} => {"hpmcounter30"}
        b__150 if {(b__150 == BitDynamic::new(12, 0b110000011111))} => {"hpmcounter31"}
        b__151 if {(b__151 == BitDynamic::new(12, 0b110010000011))} => {"hpmcounter3h"}
        b__152 if {(b__152 == BitDynamic::new(12, 0b110010000100))} => {"hpmcounter4h"}
        b__153 if {(b__153 == BitDynamic::new(12, 0b110010000101))} => {"hpmcounter5h"}
        b__154 if {(b__154 == BitDynamic::new(12, 0b110010000110))} => {"hpmcounter6h"}
        b__155 if {(b__155 == BitDynamic::new(12, 0b110010000111))} => {"hpmcounter7h"}
        b__156 if {(b__156 == BitDynamic::new(12, 0b110010001000))} => {"hpmcounter8h"}
        b__157 if {(b__157 == BitDynamic::new(12, 0b110010001001))} => {"hpmcounter9h"}
        b__158 if {(b__158 == BitDynamic::new(12, 0b110010001010))} => {"hpmcounter10h"}
        b__159 if {(b__159 == BitDynamic::new(12, 0b110010001011))} => {"hpmcounter11h"}
        b__160 if {(b__160 == BitDynamic::new(12, 0b110010001100))} => {"hpmcounter12h"}
        b__161 if {(b__161 == BitDynamic::new(12, 0b110010001101))} => {"hpmcounter13h"}
        b__162 if {(b__162 == BitDynamic::new(12, 0b110010001110))} => {"hpmcounter14h"}
        b__163 if {(b__163 == BitDynamic::new(12, 0b110010001111))} => {"hpmcounter15h"}
        b__164 if {(b__164 == BitDynamic::new(12, 0b110010010000))} => {"hpmcounter16h"}
        b__165 if {(b__165 == BitDynamic::new(12, 0b110010010001))} => {"hpmcounter17h"}
        b__166 if {(b__166 == BitDynamic::new(12, 0b110010010010))} => {"hpmcounter18h"}
        b__167 if {(b__167 == BitDynamic::new(12, 0b110010010011))} => {"hpmcounter19h"}
        b__168 if {(b__168 == BitDynamic::new(12, 0b110010010100))} => {"hpmcounter20h"}
        b__169 if {(b__169 == BitDynamic::new(12, 0b110010010101))} => {"hpmcounter21h"}
        b__170 if {(b__170 == BitDynamic::new(12, 0b110010010110))} => {"hpmcounter22h"}
        b__171 if {(b__171 == BitDynamic::new(12, 0b110010010111))} => {"hpmcounter23h"}
        b__172 if {(b__172 == BitDynamic::new(12, 0b110010011000))} => {"hpmcounter24h"}
        b__173 if {(b__173 == BitDynamic::new(12, 0b110010011001))} => {"hpmcounter25h"}
        b__174 if {(b__174 == BitDynamic::new(12, 0b110010011010))} => {"hpmcounter26h"}
        b__175 if {(b__175 == BitDynamic::new(12, 0b110010011011))} => {"hpmcounter27h"}
        b__176 if {(b__176 == BitDynamic::new(12, 0b110010011100))} => {"hpmcounter28h"}
        b__177 if {(b__177 == BitDynamic::new(12, 0b110010011101))} => {"hpmcounter29h"}
        b__178 if {(b__178 == BitDynamic::new(12, 0b110010011110))} => {"hpmcounter30h"}
        b__179 if {(b__179 == BitDynamic::new(12, 0b110010011111))} => {"hpmcounter31h"}
        b__180 if {(b__180 == BitDynamic::new(12, 0b001100100011))} => {"mhpmevent3"}
        b__181 if {(b__181 == BitDynamic::new(12, 0b001100100100))} => {"mhpmevent4"}
        b__182 if {(b__182 == BitDynamic::new(12, 0b001100100101))} => {"mhpmevent5"}
        b__183 if {(b__183 == BitDynamic::new(12, 0b001100100110))} => {"mhpmevent6"}
        b__184 if {(b__184 == BitDynamic::new(12, 0b001100100111))} => {"mhpmevent7"}
        b__185 if {(b__185 == BitDynamic::new(12, 0b001100101000))} => {"mhpmevent8"}
        b__186 if {(b__186 == BitDynamic::new(12, 0b001100101001))} => {"mhpmevent9"}
        b__187 if {(b__187 == BitDynamic::new(12, 0b001100101010))} => {"mhpmevent10"}
        b__188 if {(b__188 == BitDynamic::new(12, 0b001100101011))} => {"mhpmevent11"}
        b__189 if {(b__189 == BitDynamic::new(12, 0b001100101100))} => {"mhpmevent12"}
        b__190 if {(b__190 == BitDynamic::new(12, 0b001100101101))} => {"mhpmevent13"}
        b__191 if {(b__191 == BitDynamic::new(12, 0b001100101110))} => {"mhpmevent14"}
        b__192 if {(b__192 == BitDynamic::new(12, 0b001100101111))} => {"mhpmevent15"}
        b__193 if {(b__193 == BitDynamic::new(12, 0b001100110000))} => {"mhpmevent16"}
        b__194 if {(b__194 == BitDynamic::new(12, 0b001100110001))} => {"mhpmevent17"}
        b__195 if {(b__195 == BitDynamic::new(12, 0b001100110010))} => {"mhpmevent18"}
        b__196 if {(b__196 == BitDynamic::new(12, 0b001100110011))} => {"mhpmevent19"}
        b__197 if {(b__197 == BitDynamic::new(12, 0b001100110100))} => {"mhpmevent20"}
        b__198 if {(b__198 == BitDynamic::new(12, 0b001100110101))} => {"mhpmevent21"}
        b__199 if {(b__199 == BitDynamic::new(12, 0b001100110110))} => {"mhpmevent22"}
        b__200 if {(b__200 == BitDynamic::new(12, 0b001100110111))} => {"mhpmevent23"}
        b__201 if {(b__201 == BitDynamic::new(12, 0b001100111000))} => {"mhpmevent24"}
        b__202 if {(b__202 == BitDynamic::new(12, 0b001100111001))} => {"mhpmevent25"}
        b__203 if {(b__203 == BitDynamic::new(12, 0b001100111010))} => {"mhpmevent26"}
        b__204 if {(b__204 == BitDynamic::new(12, 0b001100111011))} => {"mhpmevent27"}
        b__205 if {(b__205 == BitDynamic::new(12, 0b001100111100))} => {"mhpmevent28"}
        b__206 if {(b__206 == BitDynamic::new(12, 0b001100111101))} => {"mhpmevent29"}
        b__207 if {(b__207 == BitDynamic::new(12, 0b001100111110))} => {"mhpmevent30"}
        b__208 if {(b__208 == BitDynamic::new(12, 0b001100111111))} => {"mhpmevent31"}
        b__209 if {(b__209 == BitDynamic::new(12, 0b101100000011))} => {"mhpmcounter3"}
        b__210 if {(b__210 == BitDynamic::new(12, 0b101100000100))} => {"mhpmcounter4"}
        b__211 if {(b__211 == BitDynamic::new(12, 0b101100000101))} => {"mhpmcounter5"}
        b__212 if {(b__212 == BitDynamic::new(12, 0b101100000110))} => {"mhpmcounter6"}
        b__213 if {(b__213 == BitDynamic::new(12, 0b101100000111))} => {"mhpmcounter7"}
        b__214 if {(b__214 == BitDynamic::new(12, 0b101100001000))} => {"mhpmcounter8"}
        b__215 if {(b__215 == BitDynamic::new(12, 0b101100001001))} => {"mhpmcounter9"}
        b__216 if {(b__216 == BitDynamic::new(12, 0b101100001010))} => {"mhpmcounter10"}
        b__217 if {(b__217 == BitDynamic::new(12, 0b101100001011))} => {"mhpmcounter11"}
        b__218 if {(b__218 == BitDynamic::new(12, 0b101100001100))} => {"mhpmcounter12"}
        b__219 if {(b__219 == BitDynamic::new(12, 0b101100001101))} => {"mhpmcounter13"}
        b__220 if {(b__220 == BitDynamic::new(12, 0b101100001110))} => {"mhpmcounter14"}
        b__221 if {(b__221 == BitDynamic::new(12, 0b101100001111))} => {"mhpmcounter15"}
        b__222 if {(b__222 == BitDynamic::new(12, 0b101100010000))} => {"mhpmcounter16"}
        b__223 if {(b__223 == BitDynamic::new(12, 0b101100010001))} => {"mhpmcounter17"}
        b__224 if {(b__224 == BitDynamic::new(12, 0b101100010010))} => {"mhpmcounter18"}
        b__225 if {(b__225 == BitDynamic::new(12, 0b101100010011))} => {"mhpmcounter19"}
        b__226 if {(b__226 == BitDynamic::new(12, 0b101100010100))} => {"mhpmcounter20"}
        b__227 if {(b__227 == BitDynamic::new(12, 0b101100010101))} => {"mhpmcounter21"}
        b__228 if {(b__228 == BitDynamic::new(12, 0b101100010110))} => {"mhpmcounter22"}
        b__229 if {(b__229 == BitDynamic::new(12, 0b101100010111))} => {"mhpmcounter23"}
        b__230 if {(b__230 == BitDynamic::new(12, 0b101100011000))} => {"mhpmcounter24"}
        b__231 if {(b__231 == BitDynamic::new(12, 0b101100011001))} => {"mhpmcounter25"}
        b__232 if {(b__232 == BitDynamic::new(12, 0b101100011010))} => {"mhpmcounter26"}
        b__233 if {(b__233 == BitDynamic::new(12, 0b101100011011))} => {"mhpmcounter27"}
        b__234 if {(b__234 == BitDynamic::new(12, 0b101100011100))} => {"mhpmcounter28"}
        b__235 if {(b__235 == BitDynamic::new(12, 0b101100011101))} => {"mhpmcounter29"}
        b__236 if {(b__236 == BitDynamic::new(12, 0b101100011110))} => {"mhpmcounter30"}
        b__237 if {(b__237 == BitDynamic::new(12, 0b101100011111))} => {"mhpmcounter31"}
        b__238 if {(b__238 == BitDynamic::new(12, 0b101110000011))} => {"mhpmcounter3h"}
        b__239 if {(b__239 == BitDynamic::new(12, 0b101110000100))} => {"mhpmcounter4h"}
        b__240 if {(b__240 == BitDynamic::new(12, 0b101110000101))} => {"mhpmcounter5h"}
        b__241 if {(b__241 == BitDynamic::new(12, 0b101110000110))} => {"mhpmcounter6h"}
        b__242 if {(b__242 == BitDynamic::new(12, 0b101110000111))} => {"mhpmcounter7h"}
        b__243 if {(b__243 == BitDynamic::new(12, 0b101110001000))} => {"mhpmcounter8h"}
        b__244 if {(b__244 == BitDynamic::new(12, 0b101110001001))} => {"mhpmcounter9h"}
        b__245 if {(b__245 == BitDynamic::new(12, 0b101110001010))} => {"mhpmcounter10h"}
        b__246 if {(b__246 == BitDynamic::new(12, 0b101110001011))} => {"mhpmcounter11h"}
        b__247 if {(b__247 == BitDynamic::new(12, 0b101110001100))} => {"mhpmcounter12h"}
        b__248 if {(b__248 == BitDynamic::new(12, 0b101110001101))} => {"mhpmcounter13h"}
        b__249 if {(b__249 == BitDynamic::new(12, 0b101110001110))} => {"mhpmcounter14h"}
        b__250 if {(b__250 == BitDynamic::new(12, 0b101110001111))} => {"mhpmcounter15h"}
        b__251 if {(b__251 == BitDynamic::new(12, 0b101110010000))} => {"mhpmcounter16h"}
        b__252 if {(b__252 == BitDynamic::new(12, 0b101110010001))} => {"mhpmcounter17h"}
        b__253 if {(b__253 == BitDynamic::new(12, 0b101110010010))} => {"mhpmcounter18h"}
        b__254 if {(b__254 == BitDynamic::new(12, 0b101110010011))} => {"mhpmcounter19h"}
        b__255 if {(b__255 == BitDynamic::new(12, 0b101110010100))} => {"mhpmcounter20h"}
        b__256 if {(b__256 == BitDynamic::new(12, 0b101110010101))} => {"mhpmcounter21h"}
        b__257 if {(b__257 == BitDynamic::new(12, 0b101110010110))} => {"mhpmcounter22h"}
        b__258 if {(b__258 == BitDynamic::new(12, 0b101110010111))} => {"mhpmcounter23h"}
        b__259 if {(b__259 == BitDynamic::new(12, 0b101110011000))} => {"mhpmcounter24h"}
        b__260 if {(b__260 == BitDynamic::new(12, 0b101110011001))} => {"mhpmcounter25h"}
        b__261 if {(b__261 == BitDynamic::new(12, 0b101110011010))} => {"mhpmcounter26h"}
        b__262 if {(b__262 == BitDynamic::new(12, 0b101110011011))} => {"mhpmcounter27h"}
        b__263 if {(b__263 == BitDynamic::new(12, 0b101110011100))} => {"mhpmcounter28h"}
        b__264 if {(b__264 == BitDynamic::new(12, 0b101110011101))} => {"mhpmcounter29h"}
        b__265 if {(b__265 == BitDynamic::new(12, 0b101110011110))} => {"mhpmcounter30h"}
        b__266 if {(b__266 == BitDynamic::new(12, 0b101110011111))} => {"mhpmcounter31h"}
        b__267 if {(b__267 == BitDynamic::new(12, 0b101110000011))} => {"mhpmcounter3h"}
        b__268 if {(b__268 == BitDynamic::new(12, 0b101110000100))} => {"mhpmcounter4h"}
        b__269 if {(b__269 == BitDynamic::new(12, 0b101110000101))} => {"mhpmcounter5h"}
        b__270 if {(b__270 == BitDynamic::new(12, 0b101110000110))} => {"mhpmcounter6h"}
        b__271 if {(b__271 == BitDynamic::new(12, 0b101110000111))} => {"mhpmcounter7h"}
        b__272 if {(b__272 == BitDynamic::new(12, 0b101110001000))} => {"mhpmcounter8h"}
        b__273 if {(b__273 == BitDynamic::new(12, 0b101110001001))} => {"mhpmcounter9h"}
        b__274 if {(b__274 == BitDynamic::new(12, 0b101110001010))} => {"mhpmcounter10h"}
        b__275 if {(b__275 == BitDynamic::new(12, 0b101110001011))} => {"mhpmcounter11h"}
        b__276 if {(b__276 == BitDynamic::new(12, 0b101110001100))} => {"mhpmcounter12h"}
        b__277 if {(b__277 == BitDynamic::new(12, 0b101110001101))} => {"mhpmcounter13h"}
        b__278 if {(b__278 == BitDynamic::new(12, 0b101110001110))} => {"mhpmcounter14h"}
        b__279 if {(b__279 == BitDynamic::new(12, 0b101110001111))} => {"mhpmcounter15h"}
        b__280 if {(b__280 == BitDynamic::new(12, 0b101110010000))} => {"mhpmcounter16h"}
        b__281 if {(b__281 == BitDynamic::new(12, 0b101110010001))} => {"mhpmcounter17h"}
        b__282 if {(b__282 == BitDynamic::new(12, 0b101110010010))} => {"mhpmcounter18h"}
        b__283 if {(b__283 == BitDynamic::new(12, 0b101110010011))} => {"mhpmcounter19h"}
        b__284 if {(b__284 == BitDynamic::new(12, 0b101110010100))} => {"mhpmcounter20h"}
        b__285 if {(b__285 == BitDynamic::new(12, 0b101110010101))} => {"mhpmcounter21h"}
        b__286 if {(b__286 == BitDynamic::new(12, 0b101110010110))} => {"mhpmcounter22h"}
        b__287 if {(b__287 == BitDynamic::new(12, 0b101110010111))} => {"mhpmcounter23h"}
        b__288 if {(b__288 == BitDynamic::new(12, 0b101110011000))} => {"mhpmcounter24h"}
        b__289 if {(b__289 == BitDynamic::new(12, 0b101110011001))} => {"mhpmcounter25h"}
        b__290 if {(b__290 == BitDynamic::new(12, 0b101110011010))} => {"mhpmcounter26h"}
        b__291 if {(b__291 == BitDynamic::new(12, 0b101110011011))} => {"mhpmcounter27h"}
        b__292 if {(b__292 == BitDynamic::new(12, 0b101110011100))} => {"mhpmcounter28h"}
        b__293 if {(b__293 == BitDynamic::new(12, 0b101110011101))} => {"mhpmcounter29h"}
        b__294 if {(b__294 == BitDynamic::new(12, 0b101110011110))} => {"mhpmcounter30h"}
        b__295 if {(b__295 == BitDynamic::new(12, 0b101110011111))} => {"mhpmcounter31h"}
        b__296 if {(b__296 == BitDynamic::new(12, 0b110110100000))} => {"scountovf"}
        b__297 if {(b__297 == BitDynamic::new(12, 0b000000010101))} => {"seed"}
        b__298 if {(b__298 == BitDynamic::new(12, 0b110000000000))} => {"cycle"}
        b__299 if {(b__299 == BitDynamic::new(12, 0b110000000001))} => {"time"}
        b__300 if {(b__300 == BitDynamic::new(12, 0b110000000010))} => {"instret"}
        b__301 if {(b__301 == BitDynamic::new(12, 0b110010000000))} => {"cycleh"}
        b__302 if {(b__302 == BitDynamic::new(12, 0b110010000001))} => {"timeh"}
        b__303 if {(b__303 == BitDynamic::new(12, 0b110010000010))} => {"instreth"}
        b__304 if {(b__304 == BitDynamic::new(12, 0b101100000000))} => {"mcycle"}
        b__305 if {(b__305 == BitDynamic::new(12, 0b101100000010))} => {"minstret"}
        b__306 if {(b__306 == BitDynamic::new(12, 0b101110000000))} => {"mcycleh"}
        b__307 if {(b__307 == BitDynamic::new(12, 0b101110000010))} => {"minstreth"}
        b__308 if {(b__308 == BitDynamic::new(12, 0b000000000001))} => {"fflags"}
        b__309 if {(b__309 == BitDynamic::new(12, 0b000000000010))} => {"frm"}
        b__310 if {(b__310 == BitDynamic::new(12, 0b000000000011))} => {"fcsr"}
        b__311 if {(b__311 == BitDynamic::new(12, 0b001100100001))} => {"mcyclecfg"}
        b__312 if {(b__312 == BitDynamic::new(12, 0b011100100001))} => {"mcyclecfgh"}
        b__313 if {(b__313 == BitDynamic::new(12, 0b001100100010))} => {"minstretcfg"}
        b__314 if {(b__314 == BitDynamic::new(12, 0b011100100010))} => {"minstretcfgh"}
        b__315 if {(b__315 == BitDynamic::new(12, 0b000101001101))} => {"stimecmp"}
        b__316 if {(b__316 == BitDynamic::new(12, 0b000101011101))} => {"stimecmph"}
        b__317 if {(b__317 == BitDynamic::new(12, 0b000110000000))} => {"satp"}
        reg => {hex_bits_12_forwards(reg)}
        _ => {panic!("Unreachable code")}
    }
}

/// ExceptionType
///
/// Generated from the Sail sources at `riscv_types.sail` L125-145.
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
    E_Extension(ext_exc_type)
}

/// exceptionType_to_str
///
/// Generated from the Sail sources at `riscv_types.sail` L197-218.
pub fn exceptionType_to_str(e: ExceptionType) -> &'static str {
    match e {
        ExceptionType::E_Fetch_Addr_Align(()) => {"misaligned-fetch"}
        ExceptionType::E_Fetch_Access_Fault(()) => {"fetch-access-fault"}
        ExceptionType::E_Illegal_Instr(()) => {"illegal-instruction"}
        ExceptionType::E_Breakpoint(()) => {"breakpoint"}
        ExceptionType::E_Load_Addr_Align(()) => {"misaligned-load"}
        ExceptionType::E_Load_Access_Fault(()) => {"load-access-fault"}
        ExceptionType::E_SAMO_Addr_Align(()) => {"misaligned-store/amo"}
        ExceptionType::E_SAMO_Access_Fault(()) => {"store/amo-access-fault"}
        ExceptionType::E_U_EnvCall(()) => {"u-call"}
        ExceptionType::E_S_EnvCall(()) => {"s-call"}
        ExceptionType::E_Reserved_10(()) => {"reserved-0"}
        ExceptionType::E_M_EnvCall(()) => {"m-call"}
        ExceptionType::E_Fetch_Page_Fault(()) => {"fetch-page-fault"}
        ExceptionType::E_Load_Page_Fault(()) => {"load-page-fault"}
        ExceptionType::E_Reserved_14(()) => {"reserved-1"}
        ExceptionType::E_SAMO_Page_Fault(()) => {"store/amo-page-fault"}
        ExceptionType::E_Extension(e) => {"extension-exception"}
        _ => {panic!("Unreachable code")}
    }
}

/// amoop
///
/// Generated from the Sail sources at `riscv_types.sail` L279-280.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum amoop {
    AMOSWAP,
    AMOADD,
    AMOXOR,
    AMOAND,
    AMOOR,
    AMOMIN,
    AMOMAX,
    AMOMINU,
    AMOMAXU
}

/// bop
///
/// Generated from the Sail sources at `riscv_types.sail` L271.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum bop {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU
}

/// cbop_zicbom
///
/// Generated from the Sail sources at `riscv_types.sail` L283.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum cbop_zicbom {
    CBO_CLEAN,
    CBO_FLUSH,
    CBO_INVAL
}

/// csrop
///
/// Generated from the Sail sources at `riscv_types.sail` L281.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum csrop {
    CSRRW,
    CSRRS,
    CSRRC
}

/// f_bin_f_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L90.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_f_op_D {
    FSGNJ_D,
    FSGNJN_D,
    FSGNJX_D,
    FMIN_D,
    FMAX_D
}

/// f_bin_f_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L52.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_f_op_H {
    FSGNJ_H,
    FSGNJN_H,
    FSGNJX_H,
    FMIN_H,
    FMAX_H
}

/// f_bin_rm_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L80.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_rm_op_D {
    FADD_D,
    FSUB_D,
    FMUL_D,
    FDIV_D
}

/// f_bin_rm_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L38.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_rm_op_H {
    FADD_H,
    FSUB_H,
    FMUL_H,
    FDIV_H
}

/// f_bin_rm_op_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L60.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_rm_op_S {
    FADD_S,
    FSUB_S,
    FMUL_S,
    FDIV_S
}

/// f_bin_op_f_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L74.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_op_f_S {
    FSGNJ_S,
    FSGNJN_S,
    FSGNJX_S,
    FMIN_S,
    FMAX_S
}

/// f_bin_op_x_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L76.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_op_x_S {
    FEQ_S,
    FLT_S,
    FLE_S
}

/// f_bin_x_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L92.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_x_op_D {
    FEQ_D,
    FLT_D,
    FLE_D
}

/// f_bin_x_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L54.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_x_op_H {
    FEQ_H,
    FLT_H,
    FLE_H
}

/// f_madd_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L78.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_madd_op_D {
    FMADD_D,
    FMSUB_D,
    FNMSUB_D,
    FNMADD_D
}

/// f_madd_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L36.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_madd_op_H {
    FMADD_H,
    FMSUB_H,
    FNMSUB_H,
    FNMADD_H
}

/// f_madd_op_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L58.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_madd_op_S {
    FMADD_S,
    FMSUB_S,
    FNMSUB_S,
    FNMADD_S
}

/// f_un_f_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L97.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_f_op_D {
    FMV_D_X
}

/// f_un_f_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L50.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_f_op_H {
    FMV_H_X
}

/// f_un_rm_ff_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L82.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_ff_op_D {
    FSQRT_D,
    FCVT_S_D,
    FCVT_D_S
}

/// f_un_rm_ff_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L40.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_ff_op_H {
    FSQRT_H,
    FCVT_H_S,
    FCVT_H_D,
    FCVT_S_H,
    FCVT_D_H
}

/// f_un_rm_fx_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L84-85.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_fx_op_D {
    FCVT_W_D,
    FCVT_WU_D,
    FCVT_L_D,
    FCVT_LU_D
}

/// f_un_rm_fx_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L42-43.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_fx_op_H {
    FCVT_W_H,
    FCVT_WU_H,
    FCVT_L_H,
    FCVT_LU_H
}

/// f_un_rm_fx_op_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L64-65.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_fx_op_S {
    FCVT_W_S,
    FCVT_WU_S,
    FCVT_L_S,
    FCVT_LU_S
}

/// f_un_rm_xf_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L87-88.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_xf_op_D {
    FCVT_D_W,
    FCVT_D_WU,
    FCVT_D_L,
    FCVT_D_LU
}

/// f_un_rm_xf_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L45-46.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_xf_op_H {
    FCVT_H_W,
    FCVT_H_WU,
    FCVT_H_L,
    FCVT_H_LU
}

/// f_un_rm_xf_op_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L67-68.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_xf_op_S {
    FCVT_S_W,
    FCVT_S_WU,
    FCVT_S_L,
    FCVT_S_LU
}

/// f_un_op_f_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L70.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_op_f_S {
    FMV_W_X
}

/// f_un_op_x_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L72.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_op_x_S {
    FCLASS_S,
    FMV_X_W
}

/// f_un_x_op_D
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L94-95.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_x_op_D {
    FCLASS_D,
    FMV_X_D
}

/// f_un_x_op_H
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L48.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_x_op_H {
    FCLASS_H,
    FMV_X_H
}

/// rounding_mode
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L56.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rounding_mode {
    RM_RNE,
    RM_RTZ,
    RM_RDN,
    RM_RUP,
    RM_RMM,
    RM_DYN
}

/// fvfmafunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L138.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvfmafunct6 {
    VF_VMADD,
    VF_VNMADD,
    VF_VMSUB,
    VF_VNMSUB,
    VF_VMACC,
    VF_VNMACC,
    VF_VMSAC,
    VF_VNMSAC
}

/// fvfmfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L146.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvfmfunct6 {
    VFM_VMFEQ,
    VFM_VMFLE,
    VFM_VMFLT,
    VFM_VMFNE,
    VFM_VMFGT,
    VFM_VMFGE
}

/// fvffunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L135-136.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvffunct6 {
    VF_VADD,
    VF_VSUB,
    VF_VMIN,
    VF_VMAX,
    VF_VSGNJ,
    VF_VSGNJN,
    VF_VSGNJX,
    VF_VDIV,
    VF_VRDIV,
    VF_VMUL,
    VF_VRSUB,
    VF_VSLIDE1UP,
    VF_VSLIDE1DOWN
}

/// fvvmafunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L115.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvvmafunct6 {
    FVV_VMADD,
    FVV_VNMADD,
    FVV_VMSUB,
    FVV_VNMSUB,
    FVV_VMACC,
    FVV_VNMACC,
    FVV_VMSAC,
    FVV_VNMSAC
}

/// fvvmfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L123.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvvmfunct6 {
    FVVM_VMFEQ,
    FVVM_VMFLE,
    FVVM_VMFLT,
    FVVM_VMFNE
}

/// fvvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L112-113.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvvfunct6 {
    FVV_VADD,
    FVV_VSUB,
    FVV_VMIN,
    FVV_VMAX,
    FVV_VSGNJ,
    FVV_VSGNJN,
    FVV_VSGNJX,
    FVV_VDIV,
    FVV_VMUL
}

/// iop
///
/// Generated from the Sail sources at `riscv_types.sail` L272.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI
}

/// mmfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L55.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum mmfunct6 {
    MM_VMAND,
    MM_VMNAND,
    MM_VMANDN,
    MM_VMXOR,
    MM_VMOR,
    MM_VMNOR,
    MM_VMORN,
    MM_VMXNOR
}

/// mul_op
///
/// Generated from the Sail sources at `riscv_types.sail` L325-329.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct mul_op {
    pub high: bool,
    pub signed_rs1: bool,
    pub signed_rs2: bool,
}

/// mvvmafunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L89.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum mvvmafunct6 {
    MVV_VMACC,
    MVV_VNMSAC,
    MVV_VMADD,
    MVV_VNMSUB
}

/// mvvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L86-87.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum mvvfunct6 {
    MVV_VAADDU,
    MVV_VAADD,
    MVV_VASUBU,
    MVV_VASUB,
    MVV_VMUL,
    MVV_VMULH,
    MVV_VMULHU,
    MVV_VMULHSU,
    MVV_VDIVU,
    MVV_VDIV,
    MVV_VREMU,
    MVV_VREM
}

/// mvxmafunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L104.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum mvxmafunct6 {
    MVX_VMACC,
    MVX_VNMSAC,
    MVX_VMADD,
    MVX_VNMSUB
}

/// mvxfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L101-102.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum mvxfunct6 {
    MVX_VAADDU,
    MVX_VAADD,
    MVX_VASUBU,
    MVX_VASUB,
    MVX_VSLIDE1UP,
    MVX_VSLIDE1DOWN,
    MVX_VMUL,
    MVX_VMULH,
    MVX_VMULHU,
    MVX_VMULHSU,
    MVX_VDIVU,
    MVX_VDIV,
    MVX_VREMU,
    MVX_VREM
}

/// nisfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L59.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nisfunct6 {
    NIS_VNSRL,
    NIS_VNSRA
}

/// nifunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L57.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nifunct6 {
    NI_VNCLIPU,
    NI_VNCLIP
}

/// nvsfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L49.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nvsfunct6 {
    NVS_VNSRL,
    NVS_VNSRA
}

/// nvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L47.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nvfunct6 {
    NV_VNCLIPU,
    NV_VNCLIP
}

/// nxsfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L53.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nxsfunct6 {
    NXS_VNSRL,
    NXS_VNSRA
}

/// nxfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L51.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nxfunct6 {
    NX_VNCLIPU,
    NX_VNCLIP
}

/// rmvvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L91-92.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rmvvfunct6 {
    MVV_VREDSUM,
    MVV_VREDAND,
    MVV_VREDOR,
    MVV_VREDXOR,
    MVV_VREDMINU,
    MVV_VREDMIN,
    MVV_VREDMAXU,
    MVV_VREDMAX
}

/// rop
///
/// Generated from the Sail sources at `riscv_types.sail` L274-275.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rop {
    ADD,
    SUB,
    SLL,
    SLT,
    SLTU,
    XOR,
    SRL,
    SRA,
    OR,
    AND
}

/// ropw
///
/// Generated from the Sail sources at `riscv_types.sail` L277.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ropw {
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW
}

/// sop
///
/// Generated from the Sail sources at `riscv_types.sail` L273.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum sop {
    SLLI,
    SRLI,
    SRAI
}

/// sopw
///
/// Generated from the Sail sources at `riscv_types.sail` L278.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum sopw {
    SLLIW,
    SRLIW,
    SRAIW
}

/// word_width
///
/// Generated from the Sail sources at `riscv_types.sail` L93.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum word_width {
    BYTE,
    HALF,
    WORD,
    DOUBLE
}

/// uop
///
/// Generated from the Sail sources at `riscv_types.sail` L270.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum uop {
    LUI,
    AUIPC
}

/// vext2funct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L69.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vext2funct6 {
    VEXT2_ZVF2,
    VEXT2_SVF2
}

/// vext4funct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L71.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vext4funct6 {
    VEXT4_ZVF4,
    VEXT4_SVF4
}

/// vext8funct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L73.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vext8funct6 {
    VEXT8_ZVF8,
    VEXT8_SVF8
}

/// vfnunary0
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L130-131.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vfnunary0 {
    FNV_CVT_XU_F,
    FNV_CVT_X_F,
    FNV_CVT_F_XU,
    FNV_CVT_F_X,
    FNV_CVT_F_F,
    FNV_CVT_ROD_F_F,
    FNV_CVT_RTZ_XU_F,
    FNV_CVT_RTZ_X_F
}

/// vfunary0
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L125.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vfunary0 {
    FV_CVT_XU_F,
    FV_CVT_X_F,
    FV_CVT_F_XU,
    FV_CVT_F_X,
    FV_CVT_RTZ_XU_F,
    FV_CVT_RTZ_X_F
}

/// vfunary1
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L133.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vfunary1 {
    FVV_VSQRT,
    FVV_VRSQRT7,
    FVV_VREC7,
    FVV_VCLASS
}

/// vicmpfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L45.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vicmpfunct6 {
    VICMP_VMSEQ,
    VICMP_VMSNE,
    VICMP_VMSLEU,
    VICMP_VMSLE,
    VICMP_VMSGTU,
    VICMP_VMSGT
}

/// vimcfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L38.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vimcfunct6 {
    VIMC_VMADC
}

/// vimsfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L40.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vimsfunct6 {
    VIMS_VADC
}

/// vimfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L36.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vimfunct6 {
    VIM_VMADC
}

/// visgfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L84.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum visgfunct6 {
    VI_VSLIDEUP,
    VI_VSLIDEDOWN,
    VI_VRGATHER
}

/// vifunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L79-80.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vifunct6 {
    VI_VADD,
    VI_VRSUB,
    VI_VAND,
    VI_VOR,
    VI_VXOR,
    VI_VSADDU,
    VI_VSADD,
    VI_VSLL,
    VI_VSRL,
    VI_VSRA,
    VI_VSSRL,
    VI_VSSRA
}

/// vlewidth
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L110.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vlewidth {
    VLE8,
    VLE16,
    VLE32,
    VLE64
}

/// vmlsop
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L148.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vmlsop {
    VLM,
    VSM
}

/// vregidx
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L9.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregidx {
    Vregidx(BitDynamic)
}

/// zvkfunct6
///
/// Generated from the Sail sources at `riscv_zvk_utils.sail` L32.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum zvkfunct6 {
    ZVK_VSHA2CH,
    ZVK_VSHA2CL
}

/// vvcmpfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L22.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvcmpfunct6 {
    VVCMP_VMSEQ,
    VVCMP_VMSNE,
    VVCMP_VMSLTU,
    VVCMP_VMSLT,
    VVCMP_VMSLEU,
    VVCMP_VMSLE
}

/// vvmcfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L26.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvmcfunct6 {
    VVMC_VMADC,
    VVMC_VMSBC
}

/// vvmsfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L28.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvmsfunct6 {
    VVMS_VADC,
    VVMS_VSBC
}

/// vvmfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L24.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvmfunct6 {
    VVM_VMADC,
    VVM_VMSBC
}

/// vvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L18-20.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvfunct6 {
    VV_VADD,
    VV_VSUB,
    VV_VMINU,
    VV_VMIN,
    VV_VMAXU,
    VV_VMAX,
    VV_VAND,
    VV_VOR,
    VV_VXOR,
    VV_VRGATHER,
    VV_VRGATHEREI16,
    VV_VSADDU,
    VV_VSADD,
    VV_VSSUBU,
    VV_VSSUB,
    VV_VSLL,
    VV_VSMUL,
    VV_VSRL,
    VV_VSRA,
    VV_VSSRL,
    VV_VSSRA
}

/// vxcmpfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L42-43.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxcmpfunct6 {
    VXCMP_VMSEQ,
    VXCMP_VMSNE,
    VXCMP_VMSLTU,
    VXCMP_VMSLT,
    VXCMP_VMSLEU,
    VXCMP_VMSLE,
    VXCMP_VMSGTU,
    VXCMP_VMSGT
}

/// vxmcfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L32.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxmcfunct6 {
    VXMC_VMADC,
    VXMC_VMSBC
}

/// vxmsfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L34.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxmsfunct6 {
    VXMS_VADC,
    VXMS_VSBC
}

/// vxmfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L30.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxmfunct6 {
    VXM_VMADC,
    VXM_VMSBC
}

/// vxsgfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L82.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxsgfunct6 {
    VX_VSLIDEUP,
    VX_VSLIDEDOWN,
    VX_VRGATHER
}

/// vxfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L75-77.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxfunct6 {
    VX_VADD,
    VX_VSUB,
    VX_VRSUB,
    VX_VMINU,
    VX_VMIN,
    VX_VMAXU,
    VX_VMAX,
    VX_VAND,
    VX_VOR,
    VX_VXOR,
    VX_VSADDU,
    VX_VSADD,
    VX_VSSUBU,
    VX_VSSUB,
    VX_VSLL,
    VX_VSMUL,
    VX_VSRL,
    VX_VSRA,
    VX_VSSRL,
    VX_VSSRA
}

/// brop_zba
///
/// Generated from the Sail sources at `riscv_types.sail` L285.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum brop_zba {
    SH1ADD,
    SH2ADD,
    SH3ADD
}

/// bropw_zba
///
/// Generated from the Sail sources at `riscv_types.sail` L293.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum bropw_zba {
    ADDUW,
    SH1ADDUW,
    SH2ADDUW,
    SH3ADDUW
}

/// extop_zbb
///
/// Generated from the Sail sources at `riscv_types.sail` L299.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum extop_zbb {
    SEXTB,
    SEXTH,
    ZEXTH
}

/// brop_zbb
///
/// Generated from the Sail sources at `riscv_types.sail` L287.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum brop_zbb {
    ANDN,
    ORN,
    XNOR,
    MAX,
    MAXU,
    MIN,
    MINU,
    ROL,
    ROR
}

/// bropw_zbb
///
/// Generated from the Sail sources at `riscv_types.sail` L295.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum bropw_zbb {
    ROLW,
    RORW
}

/// brop_zbkb
///
/// Generated from the Sail sources at `riscv_types.sail` L289.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum brop_zbkb {
    PACK,
    PACKH
}

/// biop_zbs
///
/// Generated from the Sail sources at `riscv_types.sail` L297.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum biop_zbs {
    BCLRI,
    BEXTI,
    BINVI,
    BSETI
}

/// brop_zbs
///
/// Generated from the Sail sources at `riscv_types.sail` L291.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum brop_zbs {
    BCLR,
    BEXT,
    BINV,
    BSET
}

/// zicondop
///
/// Generated from the Sail sources at `riscv_types.sail` L301.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum zicondop {
    CZERO_EQZ,
    CZERO_NEZ
}

/// f_un_rm_ff_op_S
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L62.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_ff_op_S {
    FSQRT_S
}

/// ast
///
/// Generated from the Sail sources at `riscv_insts_begin.sail` L14.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ILLEGAL(word),
    C_ILLEGAL(half),
    UTYPE((BitDynamic, regidx, uop)),
    JAL((BitDynamic, regidx)),
    JALR((BitDynamic, regidx, regidx)),
    BTYPE((BitDynamic, regidx, regidx, bop)),
    ITYPE((BitDynamic, regidx, regidx, iop)),
    SHIFTIOP((BitDynamic, regidx, regidx, sop)),
    RTYPE((regidx, regidx, regidx, rop)),
    LOAD((BitDynamic, regidx, regidx, bool, word_width, bool, bool)),
    STORE((BitDynamic, regidx, regidx, word_width, bool, bool)),
    ADDIW((BitDynamic, regidx, regidx)),
    RTYPEW((regidx, regidx, regidx, ropw)),
    SHIFTIWOP((BitDynamic, regidx, regidx, sopw)),
    ECALL(()),
    MRET(()),
    SRET(()),
    EBREAK(()),
    LOADRES((bool, bool, regidx, word_width, regidx)),
    STORECON((bool, bool, regidx, regidx, word_width, regidx)),
    AMO((amoop, bool, bool, regidx, regidx, word_width, regidx)),
    C_NOP(()),
    C_ADDI4SPN((cregidx, BitDynamic)),
    C_LW((BitDynamic, cregidx, cregidx)),
    C_LD((BitDynamic, cregidx, cregidx)),
    C_SW((BitDynamic, cregidx, cregidx)),
    C_SD((BitDynamic, cregidx, cregidx)),
    C_ADDI((BitDynamic, regidx)),
    C_JAL(BitDynamic),
    C_ADDIW((BitDynamic, regidx)),
    C_LI((BitDynamic, regidx)),
    C_ADDI16SP(BitDynamic),
    C_LUI((BitDynamic, regidx)),
    C_SRLI((BitDynamic, cregidx)),
    C_SRAI((BitDynamic, cregidx)),
    C_ANDI((BitDynamic, cregidx)),
    C_SUB((cregidx, cregidx)),
    C_XOR((cregidx, cregidx)),
    C_OR((cregidx, cregidx)),
    C_AND((cregidx, cregidx)),
    C_SUBW((cregidx, cregidx)),
    C_ADDW((cregidx, cregidx)),
    C_J(BitDynamic),
    C_BEQZ((BitDynamic, cregidx)),
    C_BNEZ((BitDynamic, cregidx)),
    C_SLLI((BitDynamic, regidx)),
    C_LWSP((BitDynamic, regidx)),
    C_LDSP((BitDynamic, regidx)),
    C_SWSP((BitDynamic, regidx)),
    C_SDSP((BitDynamic, regidx)),
    C_JR(regidx),
    C_JALR(regidx),
    C_MV((regidx, regidx)),
    C_EBREAK(()),
    C_ADD((regidx, regidx)),
    MUL((regidx, regidx, regidx, mul_op)),
    DIV((regidx, regidx, regidx, bool)),
    REM((regidx, regidx, regidx, bool)),
    MULW((regidx, regidx, regidx)),
    DIVW((regidx, regidx, regidx, bool)),
    REMW((regidx, regidx, regidx, bool)),
    CSRReg((csreg, regidx, regidx, csrop)),
    CSRImm((csreg, BitDynamic, regidx, csrop)),
    C_NOP_HINT(BitDynamic),
    C_ADDI_HINT(regidx),
    C_LI_HINT(BitDynamic),
    C_LUI_HINT(BitDynamic),
    C_MV_HINT(regidx),
    C_ADD_HINT(regidx),
    C_SLLI_HINT((BitDynamic, regidx)),
    C_SRLI_HINT(cregidx),
    C_SRAI_HINT(cregidx),
    C_FLW((BitDynamic, cregidx, cregidx)),
    C_FSW((BitDynamic, cregidx, cregidx)),
    C_FLD((BitDynamic, cregidx, cregidx)),
    C_FSD((BitDynamic, cregidx, cregidx)),
    SINVAL_VMA((regidx, regidx)),
    SLLIUW((BitDynamic, regidx, regidx)),
    ZBA_RTYPEUW((regidx, regidx, regidx, bropw_zba)),
    ZBA_RTYPE((regidx, regidx, regidx, brop_zba)),
    RORIW((BitDynamic, regidx, regidx)),
    RORI((BitDynamic, regidx, regidx)),
    ZBB_RTYPEW((regidx, regidx, regidx, bropw_zbb)),
    ZBB_RTYPE((regidx, regidx, regidx, brop_zbb)),
    ZBB_EXTOP((regidx, regidx, extop_zbb)),
    REV8((regidx, regidx)),
    ORCB((regidx, regidx)),
    CPOP((regidx, regidx)),
    CPOPW((regidx, regidx)),
    CLZ((regidx, regidx)),
    CLZW((regidx, regidx)),
    CTZ((regidx, regidx)),
    CTZW((regidx, regidx)),
    CLMUL((regidx, regidx, regidx)),
    CLMULH((regidx, regidx, regidx)),
    CLMULR((regidx, regidx, regidx)),
    ZBS_IOP((BitDynamic, regidx, regidx, biop_zbs)),
    ZBS_RTYPE((regidx, regidx, regidx, brop_zbs)),
    C_LBU((BitDynamic, cregidx, cregidx)),
    C_LHU((BitDynamic, cregidx, cregidx)),
    C_LH((BitDynamic, cregidx, cregidx)),
    C_SB((BitDynamic, cregidx, cregidx)),
    C_SH((BitDynamic, cregidx, cregidx)),
    C_ZEXT_B(cregidx),
    C_SEXT_B(cregidx),
    C_ZEXT_H(cregidx),
    C_SEXT_H(cregidx),
    C_ZEXT_W(cregidx),
    C_NOT(cregidx),
    C_MUL((cregidx, cregidx)),
    AES32ESMI((BitDynamic, regidx, regidx, regidx)),
    AES32ESI((BitDynamic, regidx, regidx, regidx)),
    AES32DSMI((BitDynamic, regidx, regidx, regidx)),
    AES32DSI((BitDynamic, regidx, regidx, regidx)),
    AES64KS1I((BitDynamic, regidx, regidx)),
    AES64KS2((regidx, regidx, regidx)),
    AES64IM((regidx, regidx)),
    AES64ESM((regidx, regidx, regidx)),
    AES64ES((regidx, regidx, regidx)),
    AES64DSM((regidx, regidx, regidx)),
    AES64DS((regidx, regidx, regidx)),
    ZBKB_RTYPE((regidx, regidx, regidx, brop_zbkb)),
    ZBKB_PACKW((regidx, regidx, regidx)),
    BREV8((regidx, regidx)),
    XPERM8((regidx, regidx, regidx)),
    XPERM4((regidx, regidx, regidx)),
    ZICOND_RTYPE((regidx, regidx, regidx, zicondop)),
    VSETVLI((BitDynamic, BitDynamic, BitDynamic, BitDynamic, regidx, regidx)),
    VSETVL((regidx, regidx, regidx)),
    VSETIVLI((BitDynamic, BitDynamic, BitDynamic, BitDynamic, BitDynamic, regidx)),
    VVTYPE((vvfunct6, BitDynamic, vregidx, vregidx, vregidx)),
    NVSTYPE((nvsfunct6, BitDynamic, vregidx, vregidx, vregidx)),
    NVTYPE((nvfunct6, BitDynamic, vregidx, vregidx, vregidx)),
    MASKTYPEV((vregidx, vregidx, vregidx)),
    MOVETYPEV((vregidx, vregidx)),
    VXTYPE((vxfunct6, BitDynamic, vregidx, regidx, vregidx)),
    NXSTYPE((nxsfunct6, BitDynamic, vregidx, regidx, vregidx)),
    NXTYPE((nxfunct6, BitDynamic, vregidx, regidx, vregidx)),
    VXSG((vxsgfunct6, BitDynamic, vregidx, regidx, vregidx)),
    MASKTYPEX((vregidx, regidx, vregidx)),
    MOVETYPEX((regidx, vregidx)),
    VITYPE((vifunct6, BitDynamic, vregidx, BitDynamic, vregidx)),
    NISTYPE((nisfunct6, BitDynamic, vregidx, BitDynamic, vregidx)),
    NITYPE((nifunct6, BitDynamic, vregidx, BitDynamic, vregidx)),
    VISG((visgfunct6, BitDynamic, vregidx, BitDynamic, vregidx)),
    MASKTYPEI((vregidx, BitDynamic, vregidx)),
    MOVETYPEI((vregidx, BitDynamic)),
    VMVRTYPE((vregidx, BitDynamic, vregidx)),
    MVVTYPE((mvvfunct6, BitDynamic, vregidx, vregidx, vregidx)),
    MVVMATYPE((mvvmafunct6, BitDynamic, vregidx, vregidx, vregidx)),
    VEXT2TYPE((vext2funct6, BitDynamic, vregidx, vregidx)),
    VEXT4TYPE((vext4funct6, BitDynamic, vregidx, vregidx)),
    VEXT8TYPE((vext8funct6, BitDynamic, vregidx, vregidx)),
    VMVXS((vregidx, regidx)),
    MVVCOMPRESS((vregidx, vregidx, vregidx)),
    MVXTYPE((mvxfunct6, BitDynamic, vregidx, regidx, vregidx)),
    MVXMATYPE((mvxmafunct6, BitDynamic, vregidx, regidx, vregidx)),
    VMVSX((regidx, vregidx)),
    FVVTYPE((fvvfunct6, BitDynamic, vregidx, vregidx, vregidx)),
    FVVMATYPE((fvvmafunct6, BitDynamic, vregidx, vregidx, vregidx)),
    VFUNARY0((BitDynamic, vregidx, vfunary0, vregidx)),
    VFNUNARY0((BitDynamic, vregidx, vfnunary0, vregidx)),
    VFUNARY1((BitDynamic, vregidx, vfunary1, vregidx)),
    VLSEGTYPE((BitDynamic, BitDynamic, regidx, vlewidth, vregidx)),
    VLSEGFFTYPE((BitDynamic, BitDynamic, regidx, vlewidth, vregidx)),
    VSSEGTYPE((BitDynamic, BitDynamic, regidx, vlewidth, vregidx)),
    VLSSEGTYPE((BitDynamic, BitDynamic, regidx, regidx, vlewidth, vregidx)),
    VSSSEGTYPE((BitDynamic, BitDynamic, regidx, regidx, vlewidth, vregidx)),
    VLUXSEGTYPE((BitDynamic, BitDynamic, vregidx, regidx, vlewidth, vregidx)),
    VLOXSEGTYPE((BitDynamic, BitDynamic, vregidx, regidx, vlewidth, vregidx)),
    VSUXSEGTYPE((BitDynamic, BitDynamic, vregidx, regidx, vlewidth, vregidx)),
    VSOXSEGTYPE((BitDynamic, BitDynamic, vregidx, regidx, vlewidth, vregidx)),
    VLRETYPE((BitDynamic, regidx, vlewidth, vregidx)),
    VSRETYPE((BitDynamic, regidx, vregidx)),
    VMTYPE((regidx, vregidx, vmlsop)),
    MMTYPE((mmfunct6, vregidx, vregidx, vregidx)),
    VCPOP_M((BitDynamic, vregidx, regidx)),
    VFIRST_M((BitDynamic, vregidx, regidx)),
    VMSBF_M((BitDynamic, vregidx, vregidx)),
    VMSIF_M((BitDynamic, vregidx, vregidx)),
    VMSOF_M((BitDynamic, vregidx, vregidx)),
    VIOTA_M((BitDynamic, vregidx, vregidx)),
    VID_V((BitDynamic, vregidx)),
    VVMTYPE((vvmfunct6, vregidx, vregidx, vregidx)),
    VVMCTYPE((vvmcfunct6, vregidx, vregidx, vregidx)),
    VVMSTYPE((vvmsfunct6, vregidx, vregidx, vregidx)),
    VVCMPTYPE((vvcmpfunct6, BitDynamic, vregidx, vregidx, vregidx)),
    VXMTYPE((vxmfunct6, vregidx, regidx, vregidx)),
    VXMCTYPE((vxmcfunct6, vregidx, regidx, vregidx)),
    VXMSTYPE((vxmsfunct6, vregidx, regidx, vregidx)),
    VXCMPTYPE((vxcmpfunct6, BitDynamic, vregidx, regidx, vregidx)),
    VIMTYPE((vimfunct6, vregidx, BitDynamic, vregidx)),
    VIMCTYPE((vimcfunct6, vregidx, BitDynamic, vregidx)),
    VIMSTYPE((vimsfunct6, vregidx, BitDynamic, vregidx)),
    VICMPTYPE((vicmpfunct6, BitDynamic, vregidx, BitDynamic, vregidx)),
    FVVMTYPE((fvvmfunct6, BitDynamic, vregidx, vregidx, vregidx)),
    RMVVTYPE((rmvvfunct6, BitDynamic, vregidx, vregidx, vregidx)),
    ZICBOM((cbop_zicbom, regidx)),
    ZICBOZ(regidx),
    VANDN_VV((BitDynamic, vregidx, vregidx, vregidx)),
    VANDN_VX((BitDynamic, vregidx, regidx, vregidx)),
    VBREV_V((BitDynamic, vregidx, vregidx)),
    VBREV8_V((BitDynamic, vregidx, vregidx)),
    VREV8_V((BitDynamic, vregidx, vregidx)),
    VCLZ_V((BitDynamic, vregidx, vregidx)),
    VCTZ_V((BitDynamic, vregidx, vregidx)),
    VCPOP_V((BitDynamic, vregidx, vregidx)),
    VROL_VV((BitDynamic, vregidx, vregidx, vregidx)),
    VROL_VX((BitDynamic, vregidx, regidx, vregidx)),
    VROR_VV((BitDynamic, vregidx, vregidx, vregidx)),
    VROR_VX((BitDynamic, vregidx, regidx, vregidx)),
    VROR_VI((BitDynamic, vregidx, BitDynamic, vregidx)),
    VCLMUL_VV((BitDynamic, vregidx, vregidx, vregidx)),
    VCLMUL_VX((BitDynamic, vregidx, regidx, vregidx)),
    VCLMULH_VV((BitDynamic, vregidx, vregidx, vregidx)),
    VCLMULH_VX((BitDynamic, vregidx, regidx, vregidx)),
    VSHA2MS_VV((vregidx, vregidx, vregidx)),
    ZVKSHA2TYPE((zvkfunct6, vregidx, vregidx, vregidx)),
    VSM3ME_VV((vregidx, vregidx, vregidx)),
    VSM3C_VI((vregidx, BitDynamic, vregidx)),
    ZIMOP_MOP_R((BitDynamic, regidx, regidx)),
    ZIMOP_MOP_RR((BitDynamic, regidx, regidx, regidx)),
    ZCMOP(BitDynamic)
}

/// PTW_Error
///
/// Generated from the Sail sources at `riscv_vmem_ptw.sail` L17-25.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PTW_Error {
    PTW_Invalid_Addr(()),
    PTW_Access(()),
    PTW_Invalid_PTE(()),
    PTW_No_Permission(()),
    PTW_Misaligned(()),
    PTW_PTE_Update(()),
    PTW_Ext_Error(ext_ptw_error)
}

/// InterruptType
///
/// Generated from the Sail sources at `riscv_types.sail` L97-107.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum InterruptType {
    I_U_Software,
    I_S_Software,
    I_M_Software,
    I_U_Timer,
    I_S_Timer,
    I_M_Timer,
    I_U_External,
    I_S_External,
    I_M_External
}

/// interruptType_to_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L110-121.
pub fn interruptType_to_bits(i: InterruptType) -> BitDynamic {
    match i {
        InterruptType::I_U_Software => {BitDynamic::new(8, 0b00000000)}
        InterruptType::I_S_Software => {BitDynamic::new(8, 0b00000001)}
        InterruptType::I_M_Software => {BitDynamic::new(8, 0b00000011)}
        InterruptType::I_U_Timer => {BitDynamic::new(8, 0b00000100)}
        InterruptType::I_S_Timer => {BitDynamic::new(8, 0b00000101)}
        InterruptType::I_M_Timer => {BitDynamic::new(8, 0b00000111)}
        InterruptType::I_U_External => {BitDynamic::new(8, 0b00001000)}
        InterruptType::I_S_External => {BitDynamic::new(8, 0b00001001)}
        InterruptType::I_M_External => {BitDynamic::new(8, 0b00001011)}
        _ => {panic!("Unreachable code")}
    }
}

/// exceptionType_to_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L148-169.
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
        ExceptionType::E_Extension(e) => {ext_exc_type_to_bits(e)}
        _ => {panic!("Unreachable code")}
    }
}

/// num_of_ExceptionType
///
/// Generated from the Sail sources at `riscv_types.sail` L172-194.
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
        ExceptionType::E_Extension(e) => {24}
        _ => {panic!("Unreachable code")}
    }
}

pub type tv_mode = BitDynamic;

/// TrapVectorMode
///
/// Generated from the Sail sources at `riscv_types.sail` L225.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TrapVectorMode {
    TV_Direct,
    TV_Vector,
    TV_Reserved
}

/// trapVectorMode_of_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L228-233.
pub fn trapVectorMode_of_bits(m: BitDynamic) -> TrapVectorMode {
    match m {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {TrapVectorMode::TV_Direct}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {TrapVectorMode::TV_Vector}
        _ => {TrapVectorMode::TV_Reserved}
        _ => {panic!("Unreachable code")}
    }
}

pub type ext_status = BitDynamic;

/// ExtStatus
///
/// Generated from the Sail sources at `riscv_types.sail` L238.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExtStatus {
    Off,
    Initial,
    Clean,
    Dirty
}

/// extStatus_bits_forwards
///
/// Generated from the Sail sources.
pub fn extStatus_bits_forwards(arg_hashtag_: ExtStatus) -> BitDynamic {
    match arg_hashtag_ {
        ExtStatus::Off => {BitDynamic::new(2, 0b00)}
        ExtStatus::Initial => {BitDynamic::new(2, 0b01)}
        ExtStatus::Clean => {BitDynamic::new(2, 0b10)}
        ExtStatus::Dirty => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// extStatus_bits_backwards
///
/// Generated from the Sail sources.
pub fn extStatus_bits_backwards(arg_hashtag_: BitDynamic) -> ExtStatus {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {ExtStatus::Off}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {ExtStatus::Initial}
        b__2 if {(b__2 == BitDynamic::new(2, 0b10))} => {ExtStatus::Clean}
        _ => {ExtStatus::Dirty}
        _ => {panic!("Unreachable code")}
    }
}

/// extStatus_to_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L247.
pub fn extStatus_to_bits(e: ExtStatus) -> BitDynamic {
    extStatus_bits_forwards(e)
}

/// extStatus_of_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L248.
pub fn extStatus_of_bits(b: BitDynamic) -> ExtStatus {
    extStatus_bits_backwards(b)
}

pub type satp_mode = BitDynamic;

/// SATPMode
///
/// Generated from the Sail sources at `riscv_types.sail` L253.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SATPMode {
    Bare,
    Sv32,
    Sv39,
    Sv48,
    Sv57
}

/// satpMode_of_bits
///
/// Generated from the Sail sources at `riscv_types.sail` L255-263.
pub fn satpMode_of_bits(a: Architecture, m: BitDynamic) -> Option<SATPMode> {
    match (a, m) {
        (_, b__0) if {(b__0 == BitDynamic::new(4, 0b0000))} => {Some(SATPMode::Bare)}
        (Architecture::RV32, b__1) if {(b__1 == BitDynamic::new(4, 0b0001))} => {Some(SATPMode::Sv32)}
        (Architecture::RV64, b__2) if {(b__2 == BitDynamic::new(4, 0b1000))} => {Some(SATPMode::Sv39)}
        (Architecture::RV64, b__3) if {(b__3 == BitDynamic::new(4, 0b1001))} => {Some(SATPMode::Sv48)}
        (Architecture::RV64, b__4) if {(b__4 == BitDynamic::new(4, 0b1010))} => {Some(SATPMode::Sv57)}
        (_, _) => {None}
        _ => {panic!("Unreachable code")}
    }
}

pub type csrRW = BitDynamic;

/// size_enc_backwards
///
/// Generated from the Sail sources.
pub fn size_enc_backwards(arg_hashtag_: BitDynamic) -> word_width {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {word_width::BYTE}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {word_width::HALF}
        b__2 if {(b__2 == BitDynamic::new(2, 0b10))} => {word_width::WORD}
        _ => {word_width::DOUBLE}
        _ => {panic!("Unreachable code")}
    }
}

/// size_enc_backwards_matches
///
/// Generated from the Sail sources.
pub fn size_enc_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {true}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {true}
        b__2 if {(b__2 == BitDynamic::new(2, 0b10))} => {true}
        b__3 if {(b__3 == BitDynamic::new(2, 0b11))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// size_bytes_forwards
///
/// Generated from the Sail sources.
pub fn size_bytes_forwards(arg_hashtag_: word_width) -> i128 {
    match arg_hashtag_ {
        word_width::BYTE => {1}
        word_width::HALF => {2}
        word_width::WORD => {4}
        word_width::DOUBLE => {8}
        _ => {panic!("Unreachable code")}
    }
}

pub type level_range<const V: i128> = i128;

pub type ext_access_type = ();

/// Data
///
/// Generated from the Sail sources at `riscv_vmem_types.sail` L36.
pub const Data: ext_access_type = ();

/// default_write_acc
///
/// Generated from the Sail sources at `riscv_vmem_types.sail` L38.
pub const default_write_acc: ext_access_type = Data;

/// xreg_write_callback
///
/// Generated from the Sail sources at `riscv_callbacks.sail` L23.
pub const fn xreg_write_callback(_: regidx, missing_arg_0: BitDynamic) {
    ()
}

/// csr_full_read_callback
///
/// Generated from the Sail sources at `riscv_callbacks.sail` L29.
pub const fn csr_full_read_callback(_: &'static str, missing_arg_0: BitDynamic, missing_arg_1: BitDynamic) {
    ()
}

/// trap_callback
///
/// Generated from the Sail sources at `riscv_callbacks.sail` L32.
pub const fn trap_callback(unit_arg: ()) {
    ()
}

/// csr_name_map_backwards
///
/// Generated from the Sail sources.
pub fn csr_name_map_backwards(arg_hashtag_: &'static str) -> BitDynamic {
    let head_exp_hashtag_: &'static str = arg_hashtag_;
    match match head_exp_hashtag_ {
        "misa" => {Some(BitDynamic::new(12, 0b001100000001))}
        "mstatus" => {Some(BitDynamic::new(12, 0b001100000000))}
        "menvcfg" => {Some(BitDynamic::new(12, 0b001100001010))}
        "menvcfgh" => {Some(BitDynamic::new(12, 0b001100011010))}
        "senvcfg" => {Some(BitDynamic::new(12, 0b000100001010))}
        "mie" => {Some(BitDynamic::new(12, 0b001100000100))}
        "mip" => {Some(BitDynamic::new(12, 0b001101000100))}
        "medeleg" => {Some(BitDynamic::new(12, 0b001100000010))}
        "medelegh" => {Some(BitDynamic::new(12, 0b001100010010))}
        "mideleg" => {Some(BitDynamic::new(12, 0b001100000011))}
        "mcause" => {Some(BitDynamic::new(12, 0b001101000010))}
        "mtval" => {Some(BitDynamic::new(12, 0b001101000011))}
        "mscratch" => {Some(BitDynamic::new(12, 0b001101000000))}
        "scounteren" => {Some(BitDynamic::new(12, 0b000100000110))}
        "mcounteren" => {Some(BitDynamic::new(12, 0b001100000110))}
        "mcountinhibit" => {Some(BitDynamic::new(12, 0b001100100000))}
        "mvendorid" => {Some(BitDynamic::new(12, 0b111100010001))}
        "marchid" => {Some(BitDynamic::new(12, 0b111100010010))}
        "mimpid" => {Some(BitDynamic::new(12, 0b111100010011))}
        "mhartid" => {Some(BitDynamic::new(12, 0b111100010100))}
        "mconfigptr" => {Some(BitDynamic::new(12, 0b111100010101))}
        "sstatus" => {Some(BitDynamic::new(12, 0b000100000000))}
        "sip" => {Some(BitDynamic::new(12, 0b000101000100))}
        "sie" => {Some(BitDynamic::new(12, 0b000100000100))}
        "sscratch" => {Some(BitDynamic::new(12, 0b000101000000))}
        "scause" => {Some(BitDynamic::new(12, 0b000101000010))}
        "stval" => {Some(BitDynamic::new(12, 0b000101000011))}
        "tselect" => {Some(BitDynamic::new(12, 0b011110100000))}
        "tdata1" => {Some(BitDynamic::new(12, 0b011110100001))}
        "tdata2" => {Some(BitDynamic::new(12, 0b011110100010))}
        "tdata3" => {Some(BitDynamic::new(12, 0b011110100011))}
        "pmpcfg0" => {Some(BitDynamic::new(12, 0b001110100000))}
        "pmpcfg1" => {Some(BitDynamic::new(12, 0b001110100001))}
        "pmpcfg2" => {Some(BitDynamic::new(12, 0b001110100010))}
        "pmpcfg3" => {Some(BitDynamic::new(12, 0b001110100011))}
        "pmpcfg4" => {Some(BitDynamic::new(12, 0b001110100100))}
        "pmpcfg5" => {Some(BitDynamic::new(12, 0b001110100101))}
        "pmpcfg6" => {Some(BitDynamic::new(12, 0b001110100110))}
        "pmpcfg7" => {Some(BitDynamic::new(12, 0b001110100111))}
        "pmpcfg8" => {Some(BitDynamic::new(12, 0b001110101000))}
        "pmpcfg9" => {Some(BitDynamic::new(12, 0b001110101001))}
        "pmpcfg10" => {Some(BitDynamic::new(12, 0b001110101010))}
        "pmpcfg11" => {Some(BitDynamic::new(12, 0b001110101011))}
        "pmpcfg12" => {Some(BitDynamic::new(12, 0b001110101100))}
        "pmpcfg13" => {Some(BitDynamic::new(12, 0b001110101101))}
        "pmpcfg14" => {Some(BitDynamic::new(12, 0b001110101110))}
        "pmpcfg15" => {Some(BitDynamic::new(12, 0b001110101111))}
        "pmpaddr0" => {Some(BitDynamic::new(12, 0b001110110000))}
        "pmpaddr1" => {Some(BitDynamic::new(12, 0b001110110001))}
        "pmpaddr2" => {Some(BitDynamic::new(12, 0b001110110010))}
        "pmpaddr3" => {Some(BitDynamic::new(12, 0b001110110011))}
        "pmpaddr4" => {Some(BitDynamic::new(12, 0b001110110100))}
        "pmpaddr5" => {Some(BitDynamic::new(12, 0b001110110101))}
        "pmpaddr6" => {Some(BitDynamic::new(12, 0b001110110110))}
        "pmpaddr7" => {Some(BitDynamic::new(12, 0b001110110111))}
        "pmpaddr8" => {Some(BitDynamic::new(12, 0b001110111000))}
        "pmpaddr9" => {Some(BitDynamic::new(12, 0b001110111001))}
        "pmpaddr10" => {Some(BitDynamic::new(12, 0b001110111010))}
        "pmpaddr11" => {Some(BitDynamic::new(12, 0b001110111011))}
        "pmpaddr12" => {Some(BitDynamic::new(12, 0b001110111100))}
        "pmpaddr13" => {Some(BitDynamic::new(12, 0b001110111101))}
        "pmpaddr14" => {Some(BitDynamic::new(12, 0b001110111110))}
        "pmpaddr15" => {Some(BitDynamic::new(12, 0b001110111111))}
        "pmpaddr16" => {Some(BitDynamic::new(12, 0b001111000000))}
        "pmpaddr17" => {Some(BitDynamic::new(12, 0b001111000001))}
        "pmpaddr18" => {Some(BitDynamic::new(12, 0b001111000010))}
        "pmpaddr19" => {Some(BitDynamic::new(12, 0b001111000011))}
        "pmpaddr20" => {Some(BitDynamic::new(12, 0b001111000100))}
        "pmpaddr21" => {Some(BitDynamic::new(12, 0b001111000101))}
        "pmpaddr22" => {Some(BitDynamic::new(12, 0b001111000110))}
        "pmpaddr23" => {Some(BitDynamic::new(12, 0b001111000111))}
        "pmpaddr24" => {Some(BitDynamic::new(12, 0b001111001000))}
        "pmpaddr25" => {Some(BitDynamic::new(12, 0b001111001001))}
        "pmpaddr26" => {Some(BitDynamic::new(12, 0b001111001010))}
        "pmpaddr27" => {Some(BitDynamic::new(12, 0b001111001011))}
        "pmpaddr28" => {Some(BitDynamic::new(12, 0b001111001100))}
        "pmpaddr29" => {Some(BitDynamic::new(12, 0b001111001101))}
        "pmpaddr30" => {Some(BitDynamic::new(12, 0b001111001110))}
        "pmpaddr31" => {Some(BitDynamic::new(12, 0b001111001111))}
        "pmpaddr32" => {Some(BitDynamic::new(12, 0b001111010000))}
        "pmpaddr33" => {Some(BitDynamic::new(12, 0b001111010001))}
        "pmpaddr34" => {Some(BitDynamic::new(12, 0b001111010010))}
        "pmpaddr35" => {Some(BitDynamic::new(12, 0b001111010011))}
        "pmpaddr36" => {Some(BitDynamic::new(12, 0b001111010100))}
        "pmpaddr37" => {Some(BitDynamic::new(12, 0b001111010101))}
        "pmpaddr38" => {Some(BitDynamic::new(12, 0b001111010110))}
        "pmpaddr39" => {Some(BitDynamic::new(12, 0b001111010111))}
        "pmpaddr40" => {Some(BitDynamic::new(12, 0b001111011000))}
        "pmpaddr41" => {Some(BitDynamic::new(12, 0b001111011001))}
        "pmpaddr42" => {Some(BitDynamic::new(12, 0b001111011010))}
        "pmpaddr43" => {Some(BitDynamic::new(12, 0b001111011011))}
        "pmpaddr44" => {Some(BitDynamic::new(12, 0b001111011100))}
        "pmpaddr45" => {Some(BitDynamic::new(12, 0b001111011101))}
        "pmpaddr46" => {Some(BitDynamic::new(12, 0b001111011110))}
        "pmpaddr47" => {Some(BitDynamic::new(12, 0b001111011111))}
        "pmpaddr48" => {Some(BitDynamic::new(12, 0b001111100000))}
        "pmpaddr49" => {Some(BitDynamic::new(12, 0b001111100001))}
        "pmpaddr50" => {Some(BitDynamic::new(12, 0b001111100010))}
        "pmpaddr51" => {Some(BitDynamic::new(12, 0b001111100011))}
        "pmpaddr52" => {Some(BitDynamic::new(12, 0b001111100100))}
        "pmpaddr53" => {Some(BitDynamic::new(12, 0b001111100101))}
        "pmpaddr54" => {Some(BitDynamic::new(12, 0b001111100110))}
        "pmpaddr55" => {Some(BitDynamic::new(12, 0b001111100111))}
        "pmpaddr56" => {Some(BitDynamic::new(12, 0b001111101000))}
        "pmpaddr57" => {Some(BitDynamic::new(12, 0b001111101001))}
        "pmpaddr58" => {Some(BitDynamic::new(12, 0b001111101010))}
        "pmpaddr59" => {Some(BitDynamic::new(12, 0b001111101011))}
        "pmpaddr60" => {Some(BitDynamic::new(12, 0b001111101100))}
        "pmpaddr61" => {Some(BitDynamic::new(12, 0b001111101101))}
        "pmpaddr62" => {Some(BitDynamic::new(12, 0b001111101110))}
        "pmpaddr63" => {Some(BitDynamic::new(12, 0b001111101111))}
        "vstart" => {Some(BitDynamic::new(12, 0b000000001000))}
        "vxsat" => {Some(BitDynamic::new(12, 0b000000001001))}
        "vxrm" => {Some(BitDynamic::new(12, 0b000000001010))}
        "vcsr" => {Some(BitDynamic::new(12, 0b000000001111))}
        "vl" => {Some(BitDynamic::new(12, 0b110000100000))}
        "vtype" => {Some(BitDynamic::new(12, 0b110000100001))}
        "vlenb" => {Some(BitDynamic::new(12, 0b110000100010))}
        "stvec" => {Some(BitDynamic::new(12, 0b000100000101))}
        "sepc" => {Some(BitDynamic::new(12, 0b000101000001))}
        "mtvec" => {Some(BitDynamic::new(12, 0b001100000101))}
        "mepc" => {Some(BitDynamic::new(12, 0b001101000001))}
        "hpmcounter3" => {Some(BitDynamic::new(12, 0b110000000011))}
        "hpmcounter4" => {Some(BitDynamic::new(12, 0b110000000100))}
        "hpmcounter5" => {Some(BitDynamic::new(12, 0b110000000101))}
        "hpmcounter6" => {Some(BitDynamic::new(12, 0b110000000110))}
        "hpmcounter7" => {Some(BitDynamic::new(12, 0b110000000111))}
        "hpmcounter8" => {Some(BitDynamic::new(12, 0b110000001000))}
        "hpmcounter9" => {Some(BitDynamic::new(12, 0b110000001001))}
        "hpmcounter10" => {Some(BitDynamic::new(12, 0b110000001010))}
        "hpmcounter11" => {Some(BitDynamic::new(12, 0b110000001011))}
        "hpmcounter12" => {Some(BitDynamic::new(12, 0b110000001100))}
        "hpmcounter13" => {Some(BitDynamic::new(12, 0b110000001101))}
        "hpmcounter14" => {Some(BitDynamic::new(12, 0b110000001110))}
        "hpmcounter15" => {Some(BitDynamic::new(12, 0b110000001111))}
        "hpmcounter16" => {Some(BitDynamic::new(12, 0b110000010000))}
        "hpmcounter17" => {Some(BitDynamic::new(12, 0b110000010001))}
        "hpmcounter18" => {Some(BitDynamic::new(12, 0b110000010010))}
        "hpmcounter19" => {Some(BitDynamic::new(12, 0b110000010011))}
        "hpmcounter20" => {Some(BitDynamic::new(12, 0b110000010100))}
        "hpmcounter21" => {Some(BitDynamic::new(12, 0b110000010101))}
        "hpmcounter22" => {Some(BitDynamic::new(12, 0b110000010110))}
        "hpmcounter23" => {Some(BitDynamic::new(12, 0b110000010111))}
        "hpmcounter24" => {Some(BitDynamic::new(12, 0b110000011000))}
        "hpmcounter25" => {Some(BitDynamic::new(12, 0b110000011001))}
        "hpmcounter26" => {Some(BitDynamic::new(12, 0b110000011010))}
        "hpmcounter27" => {Some(BitDynamic::new(12, 0b110000011011))}
        "hpmcounter28" => {Some(BitDynamic::new(12, 0b110000011100))}
        "hpmcounter29" => {Some(BitDynamic::new(12, 0b110000011101))}
        "hpmcounter30" => {Some(BitDynamic::new(12, 0b110000011110))}
        "hpmcounter31" => {Some(BitDynamic::new(12, 0b110000011111))}
        "hpmcounter3h" => {Some(BitDynamic::new(12, 0b110010000011))}
        "hpmcounter4h" => {Some(BitDynamic::new(12, 0b110010000100))}
        "hpmcounter5h" => {Some(BitDynamic::new(12, 0b110010000101))}
        "hpmcounter6h" => {Some(BitDynamic::new(12, 0b110010000110))}
        "hpmcounter7h" => {Some(BitDynamic::new(12, 0b110010000111))}
        "hpmcounter8h" => {Some(BitDynamic::new(12, 0b110010001000))}
        "hpmcounter9h" => {Some(BitDynamic::new(12, 0b110010001001))}
        "hpmcounter10h" => {Some(BitDynamic::new(12, 0b110010001010))}
        "hpmcounter11h" => {Some(BitDynamic::new(12, 0b110010001011))}
        "hpmcounter12h" => {Some(BitDynamic::new(12, 0b110010001100))}
        "hpmcounter13h" => {Some(BitDynamic::new(12, 0b110010001101))}
        "hpmcounter14h" => {Some(BitDynamic::new(12, 0b110010001110))}
        "hpmcounter15h" => {Some(BitDynamic::new(12, 0b110010001111))}
        "hpmcounter16h" => {Some(BitDynamic::new(12, 0b110010010000))}
        "hpmcounter17h" => {Some(BitDynamic::new(12, 0b110010010001))}
        "hpmcounter18h" => {Some(BitDynamic::new(12, 0b110010010010))}
        "hpmcounter19h" => {Some(BitDynamic::new(12, 0b110010010011))}
        "hpmcounter20h" => {Some(BitDynamic::new(12, 0b110010010100))}
        "hpmcounter21h" => {Some(BitDynamic::new(12, 0b110010010101))}
        "hpmcounter22h" => {Some(BitDynamic::new(12, 0b110010010110))}
        "hpmcounter23h" => {Some(BitDynamic::new(12, 0b110010010111))}
        "hpmcounter24h" => {Some(BitDynamic::new(12, 0b110010011000))}
        "hpmcounter25h" => {Some(BitDynamic::new(12, 0b110010011001))}
        "hpmcounter26h" => {Some(BitDynamic::new(12, 0b110010011010))}
        "hpmcounter27h" => {Some(BitDynamic::new(12, 0b110010011011))}
        "hpmcounter28h" => {Some(BitDynamic::new(12, 0b110010011100))}
        "hpmcounter29h" => {Some(BitDynamic::new(12, 0b110010011101))}
        "hpmcounter30h" => {Some(BitDynamic::new(12, 0b110010011110))}
        "hpmcounter31h" => {Some(BitDynamic::new(12, 0b110010011111))}
        "mhpmevent3" => {Some(BitDynamic::new(12, 0b001100100011))}
        "mhpmevent4" => {Some(BitDynamic::new(12, 0b001100100100))}
        "mhpmevent5" => {Some(BitDynamic::new(12, 0b001100100101))}
        "mhpmevent6" => {Some(BitDynamic::new(12, 0b001100100110))}
        "mhpmevent7" => {Some(BitDynamic::new(12, 0b001100100111))}
        "mhpmevent8" => {Some(BitDynamic::new(12, 0b001100101000))}
        "mhpmevent9" => {Some(BitDynamic::new(12, 0b001100101001))}
        "mhpmevent10" => {Some(BitDynamic::new(12, 0b001100101010))}
        "mhpmevent11" => {Some(BitDynamic::new(12, 0b001100101011))}
        "mhpmevent12" => {Some(BitDynamic::new(12, 0b001100101100))}
        "mhpmevent13" => {Some(BitDynamic::new(12, 0b001100101101))}
        "mhpmevent14" => {Some(BitDynamic::new(12, 0b001100101110))}
        "mhpmevent15" => {Some(BitDynamic::new(12, 0b001100101111))}
        "mhpmevent16" => {Some(BitDynamic::new(12, 0b001100110000))}
        "mhpmevent17" => {Some(BitDynamic::new(12, 0b001100110001))}
        "mhpmevent18" => {Some(BitDynamic::new(12, 0b001100110010))}
        "mhpmevent19" => {Some(BitDynamic::new(12, 0b001100110011))}
        "mhpmevent20" => {Some(BitDynamic::new(12, 0b001100110100))}
        "mhpmevent21" => {Some(BitDynamic::new(12, 0b001100110101))}
        "mhpmevent22" => {Some(BitDynamic::new(12, 0b001100110110))}
        "mhpmevent23" => {Some(BitDynamic::new(12, 0b001100110111))}
        "mhpmevent24" => {Some(BitDynamic::new(12, 0b001100111000))}
        "mhpmevent25" => {Some(BitDynamic::new(12, 0b001100111001))}
        "mhpmevent26" => {Some(BitDynamic::new(12, 0b001100111010))}
        "mhpmevent27" => {Some(BitDynamic::new(12, 0b001100111011))}
        "mhpmevent28" => {Some(BitDynamic::new(12, 0b001100111100))}
        "mhpmevent29" => {Some(BitDynamic::new(12, 0b001100111101))}
        "mhpmevent30" => {Some(BitDynamic::new(12, 0b001100111110))}
        "mhpmevent31" => {Some(BitDynamic::new(12, 0b001100111111))}
        "mhpmcounter3" => {Some(BitDynamic::new(12, 0b101100000011))}
        "mhpmcounter4" => {Some(BitDynamic::new(12, 0b101100000100))}
        "mhpmcounter5" => {Some(BitDynamic::new(12, 0b101100000101))}
        "mhpmcounter6" => {Some(BitDynamic::new(12, 0b101100000110))}
        "mhpmcounter7" => {Some(BitDynamic::new(12, 0b101100000111))}
        "mhpmcounter8" => {Some(BitDynamic::new(12, 0b101100001000))}
        "mhpmcounter9" => {Some(BitDynamic::new(12, 0b101100001001))}
        "mhpmcounter10" => {Some(BitDynamic::new(12, 0b101100001010))}
        "mhpmcounter11" => {Some(BitDynamic::new(12, 0b101100001011))}
        "mhpmcounter12" => {Some(BitDynamic::new(12, 0b101100001100))}
        "mhpmcounter13" => {Some(BitDynamic::new(12, 0b101100001101))}
        "mhpmcounter14" => {Some(BitDynamic::new(12, 0b101100001110))}
        "mhpmcounter15" => {Some(BitDynamic::new(12, 0b101100001111))}
        "mhpmcounter16" => {Some(BitDynamic::new(12, 0b101100010000))}
        "mhpmcounter17" => {Some(BitDynamic::new(12, 0b101100010001))}
        "mhpmcounter18" => {Some(BitDynamic::new(12, 0b101100010010))}
        "mhpmcounter19" => {Some(BitDynamic::new(12, 0b101100010011))}
        "mhpmcounter20" => {Some(BitDynamic::new(12, 0b101100010100))}
        "mhpmcounter21" => {Some(BitDynamic::new(12, 0b101100010101))}
        "mhpmcounter22" => {Some(BitDynamic::new(12, 0b101100010110))}
        "mhpmcounter23" => {Some(BitDynamic::new(12, 0b101100010111))}
        "mhpmcounter24" => {Some(BitDynamic::new(12, 0b101100011000))}
        "mhpmcounter25" => {Some(BitDynamic::new(12, 0b101100011001))}
        "mhpmcounter26" => {Some(BitDynamic::new(12, 0b101100011010))}
        "mhpmcounter27" => {Some(BitDynamic::new(12, 0b101100011011))}
        "mhpmcounter28" => {Some(BitDynamic::new(12, 0b101100011100))}
        "mhpmcounter29" => {Some(BitDynamic::new(12, 0b101100011101))}
        "mhpmcounter30" => {Some(BitDynamic::new(12, 0b101100011110))}
        "mhpmcounter31" => {Some(BitDynamic::new(12, 0b101100011111))}
        "mhpmcounter3h" => {Some(BitDynamic::new(12, 0b101110000011))}
        "mhpmcounter4h" => {Some(BitDynamic::new(12, 0b101110000100))}
        "mhpmcounter5h" => {Some(BitDynamic::new(12, 0b101110000101))}
        "mhpmcounter6h" => {Some(BitDynamic::new(12, 0b101110000110))}
        "mhpmcounter7h" => {Some(BitDynamic::new(12, 0b101110000111))}
        "mhpmcounter8h" => {Some(BitDynamic::new(12, 0b101110001000))}
        "mhpmcounter9h" => {Some(BitDynamic::new(12, 0b101110001001))}
        "mhpmcounter10h" => {Some(BitDynamic::new(12, 0b101110001010))}
        "mhpmcounter11h" => {Some(BitDynamic::new(12, 0b101110001011))}
        "mhpmcounter12h" => {Some(BitDynamic::new(12, 0b101110001100))}
        "mhpmcounter13h" => {Some(BitDynamic::new(12, 0b101110001101))}
        "mhpmcounter14h" => {Some(BitDynamic::new(12, 0b101110001110))}
        "mhpmcounter15h" => {Some(BitDynamic::new(12, 0b101110001111))}
        "mhpmcounter16h" => {Some(BitDynamic::new(12, 0b101110010000))}
        "mhpmcounter17h" => {Some(BitDynamic::new(12, 0b101110010001))}
        "mhpmcounter18h" => {Some(BitDynamic::new(12, 0b101110010010))}
        "mhpmcounter19h" => {Some(BitDynamic::new(12, 0b101110010011))}
        "mhpmcounter20h" => {Some(BitDynamic::new(12, 0b101110010100))}
        "mhpmcounter21h" => {Some(BitDynamic::new(12, 0b101110010101))}
        "mhpmcounter22h" => {Some(BitDynamic::new(12, 0b101110010110))}
        "mhpmcounter23h" => {Some(BitDynamic::new(12, 0b101110010111))}
        "mhpmcounter24h" => {Some(BitDynamic::new(12, 0b101110011000))}
        "mhpmcounter25h" => {Some(BitDynamic::new(12, 0b101110011001))}
        "mhpmcounter26h" => {Some(BitDynamic::new(12, 0b101110011010))}
        "mhpmcounter27h" => {Some(BitDynamic::new(12, 0b101110011011))}
        "mhpmcounter28h" => {Some(BitDynamic::new(12, 0b101110011100))}
        "mhpmcounter29h" => {Some(BitDynamic::new(12, 0b101110011101))}
        "mhpmcounter30h" => {Some(BitDynamic::new(12, 0b101110011110))}
        "mhpmcounter31h" => {Some(BitDynamic::new(12, 0b101110011111))}
        "mhpmcounter3h" => {Some(BitDynamic::new(12, 0b101110000011))}
        "mhpmcounter4h" => {Some(BitDynamic::new(12, 0b101110000100))}
        "mhpmcounter5h" => {Some(BitDynamic::new(12, 0b101110000101))}
        "mhpmcounter6h" => {Some(BitDynamic::new(12, 0b101110000110))}
        "mhpmcounter7h" => {Some(BitDynamic::new(12, 0b101110000111))}
        "mhpmcounter8h" => {Some(BitDynamic::new(12, 0b101110001000))}
        "mhpmcounter9h" => {Some(BitDynamic::new(12, 0b101110001001))}
        "mhpmcounter10h" => {Some(BitDynamic::new(12, 0b101110001010))}
        "mhpmcounter11h" => {Some(BitDynamic::new(12, 0b101110001011))}
        "mhpmcounter12h" => {Some(BitDynamic::new(12, 0b101110001100))}
        "mhpmcounter13h" => {Some(BitDynamic::new(12, 0b101110001101))}
        "mhpmcounter14h" => {Some(BitDynamic::new(12, 0b101110001110))}
        "mhpmcounter15h" => {Some(BitDynamic::new(12, 0b101110001111))}
        "mhpmcounter16h" => {Some(BitDynamic::new(12, 0b101110010000))}
        "mhpmcounter17h" => {Some(BitDynamic::new(12, 0b101110010001))}
        "mhpmcounter18h" => {Some(BitDynamic::new(12, 0b101110010010))}
        "mhpmcounter19h" => {Some(BitDynamic::new(12, 0b101110010011))}
        "mhpmcounter20h" => {Some(BitDynamic::new(12, 0b101110010100))}
        "mhpmcounter21h" => {Some(BitDynamic::new(12, 0b101110010101))}
        "mhpmcounter22h" => {Some(BitDynamic::new(12, 0b101110010110))}
        "mhpmcounter23h" => {Some(BitDynamic::new(12, 0b101110010111))}
        "mhpmcounter24h" => {Some(BitDynamic::new(12, 0b101110011000))}
        "mhpmcounter25h" => {Some(BitDynamic::new(12, 0b101110011001))}
        "mhpmcounter26h" => {Some(BitDynamic::new(12, 0b101110011010))}
        "mhpmcounter27h" => {Some(BitDynamic::new(12, 0b101110011011))}
        "mhpmcounter28h" => {Some(BitDynamic::new(12, 0b101110011100))}
        "mhpmcounter29h" => {Some(BitDynamic::new(12, 0b101110011101))}
        "mhpmcounter30h" => {Some(BitDynamic::new(12, 0b101110011110))}
        "mhpmcounter31h" => {Some(BitDynamic::new(12, 0b101110011111))}
        "scountovf" => {Some(BitDynamic::new(12, 0b110110100000))}
        "seed" => {Some(BitDynamic::new(12, 0b000000010101))}
        "cycle" => {Some(BitDynamic::new(12, 0b110000000000))}
        "time" => {Some(BitDynamic::new(12, 0b110000000001))}
        "instret" => {Some(BitDynamic::new(12, 0b110000000010))}
        "cycleh" => {Some(BitDynamic::new(12, 0b110010000000))}
        "timeh" => {Some(BitDynamic::new(12, 0b110010000001))}
        "instreth" => {Some(BitDynamic::new(12, 0b110010000010))}
        "mcycle" => {Some(BitDynamic::new(12, 0b101100000000))}
        "minstret" => {Some(BitDynamic::new(12, 0b101100000010))}
        "mcycleh" => {Some(BitDynamic::new(12, 0b101110000000))}
        "minstreth" => {Some(BitDynamic::new(12, 0b101110000010))}
        "fflags" => {Some(BitDynamic::new(12, 0b000000000001))}
        "frm" => {Some(BitDynamic::new(12, 0b000000000010))}
        "fcsr" => {Some(BitDynamic::new(12, 0b000000000011))}
        "mcyclecfg" => {Some(BitDynamic::new(12, 0b001100100001))}
        "mcyclecfgh" => {Some(BitDynamic::new(12, 0b011100100001))}
        "minstretcfg" => {Some(BitDynamic::new(12, 0b001100100010))}
        "minstretcfgh" => {Some(BitDynamic::new(12, 0b011100100010))}
        "stimecmp" => {Some(BitDynamic::new(12, 0b000101001101))}
        "stimecmph" => {Some(BitDynamic::new(12, 0b000101011101))}
        "satp" => {Some(BitDynamic::new(12, 0b000110000000))}
        mapping0_hashtag_ if {hex_bits_12_backwards_matches(mapping0_hashtag_)} => {match hex_bits_12_backwards(mapping0_hashtag_) {
            reg => {Some(reg)}
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

/// csr_id_read_callback
///
/// Generated from the Sail sources at `riscv_callbacks.sail` L47-50.
pub fn csr_id_read_callback(csr: BitDynamic, value: BitDynamic) {
    let name: &'static str = csr_name_map_forwards(csr);

}

pub type regtype = xlenbits;

/// zero_reg
///
/// Generated from the Sail sources at `riscv_reg_type.sail` L13.
pub const zero_reg: regtype = zeros(64);

/// regval_from_reg
///
/// Generated from the Sail sources at `riscv_reg_type.sail` L22.
pub fn regval_from_reg(r: BitDynamic) -> BitDynamic {
    r
}

/// regval_into_reg
///
/// Generated from the Sail sources at `riscv_reg_type.sail` L25.
pub fn regval_into_reg(v: BitDynamic) -> BitDynamic {
    v
}

pub type fregtype = flenbits;

/// zero_freg
///
/// Generated from the Sail sources at `riscv_freg_type.sail` L15.
pub const zero_freg: fregtype = zeros(64);

/// rX
///
/// Generated from the Sail sources at `riscv_regs.sail` L48-86.
pub fn rX(core_ctx: &mut Core, regno::Regno(r): regno) -> BitDynamic {
    let v: regtype = match r {
        l__569 if {(l__569 == 0)} => {zero_reg}
        l__570 if {(l__570 == 1)} => {core_ctx.x1}
        l__571 if {(l__571 == 2)} => {core_ctx.x2}
        l__572 if {(l__572 == 3)} => {core_ctx.x3}
        l__573 if {(l__573 == 4)} => {core_ctx.x4}
        l__574 if {(l__574 == 5)} => {core_ctx.x5}
        l__575 if {(l__575 == 6)} => {core_ctx.x6}
        l__576 if {(l__576 == 7)} => {core_ctx.x7}
        l__577 if {(l__577 == 8)} => {core_ctx.x8}
        l__578 if {(l__578 == 9)} => {core_ctx.x9}
        l__579 if {(l__579 == 10)} => {core_ctx.x10}
        l__580 if {(l__580 == 11)} => {core_ctx.x11}
        l__581 if {(l__581 == 12)} => {core_ctx.x12}
        l__582 if {(l__582 == 13)} => {core_ctx.x13}
        l__583 if {(l__583 == 14)} => {core_ctx.x14}
        l__584 if {(l__584 == 15)} => {core_ctx.x15}
        l__585 if {(l__585 == 16)} => {core_ctx.x16}
        l__586 if {(l__586 == 17)} => {core_ctx.x17}
        l__587 if {(l__587 == 18)} => {core_ctx.x18}
        l__588 if {(l__588 == 19)} => {core_ctx.x19}
        l__589 if {(l__589 == 20)} => {core_ctx.x20}
        l__590 if {(l__590 == 21)} => {core_ctx.x21}
        l__591 if {(l__591 == 22)} => {core_ctx.x22}
        l__592 if {(l__592 == 23)} => {core_ctx.x23}
        l__593 if {(l__593 == 24)} => {core_ctx.x24}
        l__594 if {(l__594 == 25)} => {core_ctx.x25}
        l__595 if {(l__595 == 26)} => {core_ctx.x26}
        l__596 if {(l__596 == 27)} => {core_ctx.x27}
        l__597 if {(l__597 == 28)} => {core_ctx.x28}
        l__598 if {(l__598 == 29)} => {core_ctx.x29}
        l__599 if {(l__599 == 30)} => {core_ctx.x30}
        l__600 if {(l__600 == 31)} => {core_ctx.x31}
        _ => {{
            assert!(false, "invalid register number");
            panic!("exit")
        }}
        _ => {panic!("Unreachable code")}
    };
    regval_from_reg(v)
}

/// wX
///
/// Generated from the Sail sources at `riscv_regs.sail` L88-126.
pub fn wX(core_ctx: &mut Core, regno::Regno(r): regno, in_v: BitDynamic) {
    let v: BitDynamic = regval_into_reg(in_v);
    match r {
        l__537 if {(l__537 == 0)} => {()}
        l__538 if {(l__538 == 1)} => {core_ctx.x1 = v}
        l__539 if {(l__539 == 2)} => {core_ctx.x2 = v}
        l__540 if {(l__540 == 3)} => {core_ctx.x3 = v}
        l__541 if {(l__541 == 4)} => {core_ctx.x4 = v}
        l__542 if {(l__542 == 5)} => {core_ctx.x5 = v}
        l__543 if {(l__543 == 6)} => {core_ctx.x6 = v}
        l__544 if {(l__544 == 7)} => {core_ctx.x7 = v}
        l__545 if {(l__545 == 8)} => {core_ctx.x8 = v}
        l__546 if {(l__546 == 9)} => {core_ctx.x9 = v}
        l__547 if {(l__547 == 10)} => {core_ctx.x10 = v}
        l__548 if {(l__548 == 11)} => {core_ctx.x11 = v}
        l__549 if {(l__549 == 12)} => {core_ctx.x12 = v}
        l__550 if {(l__550 == 13)} => {core_ctx.x13 = v}
        l__551 if {(l__551 == 14)} => {core_ctx.x14 = v}
        l__552 if {(l__552 == 15)} => {core_ctx.x15 = v}
        l__553 if {(l__553 == 16)} => {core_ctx.x16 = v}
        l__554 if {(l__554 == 17)} => {core_ctx.x17 = v}
        l__555 if {(l__555 == 18)} => {core_ctx.x18 = v}
        l__556 if {(l__556 == 19)} => {core_ctx.x19 = v}
        l__557 if {(l__557 == 20)} => {core_ctx.x20 = v}
        l__558 if {(l__558 == 21)} => {core_ctx.x21 = v}
        l__559 if {(l__559 == 22)} => {core_ctx.x22 = v}
        l__560 if {(l__560 == 23)} => {core_ctx.x23 = v}
        l__561 if {(l__561 == 24)} => {core_ctx.x24 = v}
        l__562 if {(l__562 == 25)} => {core_ctx.x25 = v}
        l__563 if {(l__563 == 26)} => {core_ctx.x26 = v}
        l__564 if {(l__564 == 27)} => {core_ctx.x27 = v}
        l__565 if {(l__565 == 28)} => {core_ctx.x28 = v}
        l__566 if {(l__566 == 29)} => {core_ctx.x29 = v}
        l__567 if {(l__567 == 30)} => {core_ctx.x30 = v}
        l__568 if {(l__568 == 31)} => {core_ctx.x31 = v}
        _ => {assert!(false, "invalid register number")}
        _ => {panic!("Unreachable code")}
    };
    if {(r != 0)} {
        ()
    } else {
        ()
    }
}

/// rX_bits
///
/// Generated from the Sail sources at `riscv_regs.sail` L128.
pub fn rX_bits(core_ctx: &mut Core, i: regidx) -> BitDynamic {
    rX(core_ctx, regidx_to_regno(i))
}

/// wX_bits
///
/// Generated from the Sail sources at `riscv_regs.sail` L130-132.
pub fn wX_bits(core_ctx: &mut Core, i: regidx, data: BitDynamic) {
    wX(core_ctx, regidx_to_regno(i), data)
}

/// encdec_reg_forwards
///
/// Generated from the Sail sources.
pub fn encdec_reg_forwards(arg_hashtag_: regidx) -> BitDynamic {
    match arg_hashtag_ {
        regidx::Regidx(r) => {r}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_reg_backwards
///
/// Generated from the Sail sources.
pub fn encdec_reg_backwards(arg_hashtag_: BitDynamic) -> regidx {
    match arg_hashtag_ {
        r => {regidx::Regidx(r)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_reg_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_reg_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        r => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// get_arch_pc
///
/// Generated from the Sail sources at `riscv_pc_access.sail` L18.
pub fn get_arch_pc(core_ctx: &mut Core, unit_arg: ()) -> BitDynamic {
    core_ctx.PC
}

/// get_next_pc
///
/// Generated from the Sail sources at `riscv_pc_access.sail` L21.
pub fn get_next_pc(core_ctx: &mut Core, unit_arg: ()) -> BitDynamic {
    core_ctx.nextPC
}

/// set_next_pc
///
/// Generated from the Sail sources at `riscv_pc_access.sail` L24-27.
pub fn set_next_pc(core_ctx: &mut Core, pc: BitDynamic) {
    core_ctx.nextPC = pc
}

/// Misa
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L49-78.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Misa {
    pub bits: BitDynamic,
}

/// Mk_Misa
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Misa(v: BitDynamic) -> Misa {
    Misa {
        bits: v
    }
}

/// CountSmcntrpmf
///
/// Generated from the Sail sources at `riscv_smcntrpmf.sail` L3-9.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CountSmcntrpmf {
    pub bits: BitDynamic,
}

/// Counteren
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L581-586.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Counteren {
    pub bits: BitDynamic,
}

/// Counterin
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L614-618.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Counterin {
    pub bits: BitDynamic,
}

/// HpmEvent
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L165-175.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HpmEvent {
    pub bits: BitDynamic,
}

/// MEnvcfg
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L315-332.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MEnvcfg {
    pub bits: BitDynamic,
}

/// Mcause
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L518-521.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mcause {
    pub bits: BitDynamic,
}

/// Medeleg
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L449-464.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Medeleg {
    pub bits: BitDynamic,
}

/// Minterrupts
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L404-413.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Minterrupts {
    pub bits: BitDynamic,
}

/// Mstatus
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L168-204.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitDynamic,
}

/// Mtvec
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L503-506.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mtvec {
    pub bits: BitDynamic,
}

/// PTE_Ext
///
/// Generated from the Sail sources at `riscv_vmem_pte.sail` L25-31.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PTE_Ext {
    pub bits: BitDynamic,
}

/// PTE_Flags
///
/// Generated from the Sail sources at `riscv_vmem_pte.sail` L55-64.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PTE_Flags {
    pub bits: BitDynamic,
}

/// Pmpcfg_ent
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L33-41.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Pmpcfg_ent {
    pub bits: BitDynamic,
}

/// SEnvcfg
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L334-345.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SEnvcfg {
    pub bits: BitDynamic,
}

/// Satp32
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L850-854.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Satp32 {
    pub bits: BitDynamic,
}

/// Satp64
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L844-848.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Satp64 {
    pub bits: BitDynamic,
}

/// Sinterrupts
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L746-752.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Sinterrupts {
    pub bits: BitDynamic,
}

/// Sstatus
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L680-694.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Sstatus {
    pub bits: BitDynamic,
}

/// Vcsr
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L225-228.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Vcsr {
    pub bits: BitDynamic,
}

/// Vtype
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L923-930.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Vtype {
    pub bits: BitDynamic,
}

/// htif_cmd
///
/// Generated from the Sail sources at `riscv_platform.sail` L283-287.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct htif_cmd {
    pub bits: BitDynamic,
}

/// _get_Misa_A
///
/// Generated from the Sail sources.
pub fn _get_Misa_A(v: Misa) -> BitDynamic {
    v.bits.subrange::<0, 1, 1>()
}

/// _update_Misa_A
///
/// Generated from the Sail sources.
pub fn _update_Misa_A(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 0, 0, x)
    }
}

/// _update_Pmpcfg_ent_A
///
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_A(v: Pmpcfg_ent, x: BitDynamic) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 4, 3, x)
    }
}

/// _get_Pmpcfg_ent_A
///
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_A(v: Pmpcfg_ent) -> BitDynamic {
    v.bits.subrange::<3, 5, 2>()
}

/// _get_Misa_B
///
/// Generated from the Sail sources.
pub fn _get_Misa_B(v: Misa) -> BitDynamic {
    v.bits.subrange::<1, 2, 1>()
}

/// _update_Misa_B
///
/// Generated from the Sail sources.
pub fn _update_Misa_B(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _get_Misa_C
///
/// Generated from the Sail sources.
pub fn _get_Misa_C(v: Misa) -> BitDynamic {
    v.bits.subrange::<2, 3, 1>()
}

/// _update_Misa_C
///
/// Generated from the Sail sources.
pub fn _update_Misa_C(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 2, 2, x)
    }
}

/// _get_Misa_D
///
/// Generated from the Sail sources.
pub fn _get_Misa_D(v: Misa) -> BitDynamic {
    v.bits.subrange::<3, 4, 1>()
}

/// _update_Misa_D
///
/// Generated from the Sail sources.
pub fn _update_Misa_D(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 3, 3, x)
    }
}

/// _get_Misa_F
///
/// Generated from the Sail sources.
pub fn _get_Misa_F(v: Misa) -> BitDynamic {
    v.bits.subrange::<5, 6, 1>()
}

/// _update_Misa_F
///
/// Generated from the Sail sources.
pub fn _update_Misa_F(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _update_Misa_I
///
/// Generated from the Sail sources.
pub fn _update_Misa_I(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 8, 8, x)
    }
}

/// _update_Pmpcfg_ent_L
///
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_L(v: Pmpcfg_ent, x: BitDynamic) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _get_Pmpcfg_ent_L
///
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_L(v: Pmpcfg_ent) -> BitDynamic {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_Misa_M
///
/// Generated from the Sail sources.
pub fn _get_Misa_M(v: Misa) -> BitDynamic {
    v.bits.subrange::<12, 13, 1>()
}

/// _update_Misa_M
///
/// Generated from the Sail sources.
pub fn _update_Misa_M(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 12, 12, x)
    }
}

/// _get_Misa_MXL
///
/// Generated from the Sail sources.
pub fn _get_Misa_MXL(v: Misa) -> BitDynamic {
    v.bits.subrange::<62, 64, 2>()
}

/// _update_Misa_MXL
///
/// Generated from the Sail sources.
pub fn _update_Misa_MXL(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 63, 62, x)
    }
}

/// _update_Pmpcfg_ent_R
///
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_R(v: Pmpcfg_ent, x: BitDynamic) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 0, 0, x)
    }
}

/// _get_Pmpcfg_ent_R
///
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_R(v: Pmpcfg_ent) -> BitDynamic {
    v.bits.subrange::<0, 1, 1>()
}

/// _get_Misa_S
///
/// Generated from the Sail sources.
pub fn _get_Misa_S(v: Misa) -> BitDynamic {
    v.bits.subrange::<18, 19, 1>()
}

/// _update_Misa_S
///
/// Generated from the Sail sources.
pub fn _update_Misa_S(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 18, 18, x)
    }
}

/// _get_Misa_U
///
/// Generated from the Sail sources.
pub fn _get_Misa_U(v: Misa) -> BitDynamic {
    v.bits.subrange::<20, 21, 1>()
}

/// _update_Misa_U
///
/// Generated from the Sail sources.
pub fn _update_Misa_U(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 20, 20, x)
    }
}

/// _get_Misa_V
///
/// Generated from the Sail sources.
pub fn _get_Misa_V(v: Misa) -> BitDynamic {
    v.bits.subrange::<21, 22, 1>()
}

/// _update_Misa_V
///
/// Generated from the Sail sources.
pub fn _update_Misa_V(v: Misa, x: BitDynamic) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 21, 21, x)
    }
}

/// _update_Pmpcfg_ent_W
///
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_W(v: Pmpcfg_ent, x: BitDynamic) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _get_Pmpcfg_ent_W
///
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_W(v: Pmpcfg_ent) -> BitDynamic {
    v.bits.subrange::<1, 2, 1>()
}

/// _update_Pmpcfg_ent_X
///
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_X(v: Pmpcfg_ent, x: BitDynamic) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 2, 2, x)
    }
}

/// _get_Pmpcfg_ent_X
///
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_X(v: Pmpcfg_ent) -> BitDynamic {
    v.bits.subrange::<2, 3, 1>()
}

/// sys_enable_writable_misa
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L86.
pub fn sys_enable_writable_misa(core_ctx: &mut Core, unit_arg: ()) -> bool {
    core_ctx.config.base.writable_misa
}

/// sys_enable_writable_fiom
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L90.
pub fn sys_enable_writable_fiom(core_ctx: &mut Core, unit_arg: ()) -> bool {
    core_ctx.config.base.writable_fiom
}

/// sys_pmp_count
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L93.
pub fn sys_pmp_count(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    core_ctx.config.memory.pmp.count
}

/// sys_pmp_grain
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L97.
pub fn sys_pmp_grain(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    core_ctx.config.memory.pmp.grain
}

/// sys_writable_hpm_counters
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L100.
pub fn sys_writable_hpm_counters(core_ctx: &mut Core, unit_arg: ()) -> BitDynamic {
    core_ctx.config.base.writable_hpm_counters
}

/// ext_veto_disable_C
///
/// Generated from the Sail sources at `riscv_misa_ext.sail` L9.
pub const fn ext_veto_disable_C(unit_arg: ()) -> bool {
    false
}

/// legalize_misa
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L110-130.
pub fn legalize_misa(core_ctx: &mut Core, m: Misa, v: BitDynamic) -> Misa {
    let v: Misa = Mk_Misa(v);
    if {(!(sys_enable_writable_misa(core_ctx, ())) || ((_get_Misa_C(v) == BitDynamic::new(1, 0b0)) && (bitvector_access(core_ctx.nextPC, 1) == true)))} {
        m
    } else {
        {
            let var_1: Misa = {
                let var_3: Misa = {
                    let var_5: Misa = {
                        let var_7: Misa = {
                            let var_9: Misa = {
                                let var_10: Misa = {
                                    let var_12: Misa = {
                                        let var_14: Misa = {
                                            let var_16: Misa = {
                                                let var_18: BitDynamic = if {hartSupports(core_ctx, extension::Ext_A)} {
                                                    _get_Misa_A(v)
                                                } else {
                                                    BitDynamic::new(1, 0b0)
                                                };
                                                _update_Misa_A(m, var_18)
                                            };
                                            let var_17: BitDynamic = if {hartSupports(core_ctx, extension::Ext_B)} {
                                                _get_Misa_B(v)
                                            } else {
                                                BitDynamic::new(1, 0b0)
                                            };
                                            _update_Misa_B(var_16, var_17)
                                        };
                                        let var_15: BitDynamic = if {hartSupports(core_ctx, extension::Ext_C)} {
                                            _get_Misa_C(v)
                                        } else {
                                            BitDynamic::new(1, 0b0)
                                        };
                                        _update_Misa_C(var_14, var_15)
                                    };
                                    let var_13: BitDynamic = if {(hartSupports(core_ctx, extension::Ext_D) && (_get_Misa_F(v) == BitDynamic::new(1, 0b1)))} {
                                        _get_Misa_D(v)
                                    } else {
                                        BitDynamic::new(1, 0b0)
                                    };
                                    _update_Misa_D(var_12, var_13)
                                };
                                let var_11: BitDynamic = if {hartSupports(core_ctx, extension::Ext_F)} {
                                    _get_Misa_F(v)
                                } else {
                                    BitDynamic::new(1, 0b0)
                                };
                                _update_Misa_F(var_10, var_11)
                            };
                            _update_Misa_I(var_9, BitDynamic::new(1, 0b1))
                        };
                        let var_8: BitDynamic = if {hartSupports(core_ctx, extension::Ext_M)} {
                            _get_Misa_M(v)
                        } else {
                            BitDynamic::new(1, 0b0)
                        };
                        _update_Misa_M(var_7, var_8)
                    };
                    let var_6: BitDynamic = if {(hartSupports(core_ctx, extension::Ext_S) && (_get_Misa_U(v) == BitDynamic::new(1, 0b1)))} {
                        _get_Misa_S(v)
                    } else {
                        BitDynamic::new(1, 0b0)
                    };
                    _update_Misa_S(var_5, var_6)
                };
                let var_4: BitDynamic = if {hartSupports(core_ctx, extension::Ext_U)} {
                    _get_Misa_U(v)
                } else {
                    BitDynamic::new(1, 0b0)
                };
                _update_Misa_U(var_3, var_4)
            };
            let var_2: BitDynamic = if {(hartSupports(core_ctx, extension::Ext_V) && ((_get_Misa_F(v) == BitDynamic::new(1, 0b1)) && (_get_Misa_D(v) == BitDynamic::new(1, 0b1))))} {
                _get_Misa_V(v)
            } else {
                BitDynamic::new(1, 0b0)
            };
            _update_Misa_V(var_1, var_2)
        }
    }
}

/// Mk_Mstatus
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Mstatus(v: BitDynamic) -> Mstatus {
    Mstatus {
        bits: v
    }
}

/// _update_Mstatus_SXL
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_SXL(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 35, 34, x)
    }
}

/// _update_Mstatus_UXL
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_UXL(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 33, 32, x)
    }
}

/// _get_Mstatus_FS
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_FS(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<13, 15, 2>()
}

/// _get_Mstatus_VS
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_VS(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<9, 11, 2>()
}

/// currentlyEnabled
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L103.
pub fn currentlyEnabled(core_ctx: &mut Core, merge_hashtag_var: extension) -> bool {
    match merge_hashtag_var {
        extension::Ext_Sstc => {hartSupports(core_ctx, extension::Ext_Sstc)}
        extension::Ext_U => {(hartSupports(core_ctx, extension::Ext_U) && ({
            let var_1: Misa = core_ctx.misa;
            _get_Misa_U(var_1)
        } == BitDynamic::new(1, 0b1)))}
        extension::Ext_S => {(hartSupports(core_ctx, extension::Ext_S) && ({
            let var_2: Misa = core_ctx.misa;
            _get_Misa_S(var_2)
        } == BitDynamic::new(1, 0b1)))}
        extension::Ext_Svbare => {currentlyEnabled(core_ctx, extension::Ext_S)}
        extension::Ext_Sv32 => {(hartSupports(core_ctx, extension::Ext_Sv32) && currentlyEnabled(core_ctx, extension::Ext_S))}
        extension::Ext_Sv39 => {(hartSupports(core_ctx, extension::Ext_Sv39) && currentlyEnabled(core_ctx, extension::Ext_S))}
        extension::Ext_Sv48 => {(hartSupports(core_ctx, extension::Ext_Sv48) && currentlyEnabled(core_ctx, extension::Ext_S))}
        extension::Ext_Sv57 => {(hartSupports(core_ctx, extension::Ext_Sv57) && currentlyEnabled(core_ctx, extension::Ext_S))}
        extension::Ext_V => {(hartSupports(core_ctx, extension::Ext_V) && (({
            let var_4: Misa = core_ctx.misa;
            _get_Misa_V(var_4)
        } == BitDynamic::new(1, 0b1)) && ({
            let var_3: Mstatus = core_ctx.mstatus;
            _get_Mstatus_VS(var_3)
        } != BitDynamic::new(2, 0b00))))}
        extension::Ext_Zihpm => {(hartSupports(core_ctx, extension::Ext_Zihpm) && currentlyEnabled(core_ctx, extension::Ext_Zicntr))}
        extension::Ext_Sscofpmf => {(hartSupports(core_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(core_ctx, extension::Ext_Zihpm))}
        extension::Ext_Zkr => {hartSupports(core_ctx, extension::Ext_Zkr)}
        extension::Ext_Zicntr => {hartSupports(core_ctx, extension::Ext_Zicntr)}
        extension::Ext_F => {(hartSupports(core_ctx, extension::Ext_F) && (({
            let var_6: Misa = core_ctx.misa;
            _get_Misa_F(var_6)
        } == BitDynamic::new(1, 0b1)) && ({
            let var_5: Mstatus = core_ctx.mstatus;
            _get_Mstatus_FS(var_5)
        } != BitDynamic::new(2, 0b00))))}
        extension::Ext_D => {(hartSupports(core_ctx, extension::Ext_D) && (({
            let var_8: Misa = core_ctx.misa;
            _get_Misa_D(var_8)
        } == BitDynamic::new(1, 0b1)) && ({
            let var_7: Mstatus = core_ctx.mstatus;
            _get_Mstatus_FS(var_7)
        } != BitDynamic::new(2, 0b00))))}
        extension::Ext_Zfinx => {hartSupports(core_ctx, extension::Ext_Zfinx)}
        extension::Ext_Smcntrpmf => {(hartSupports(core_ctx, extension::Ext_Smcntrpmf) && currentlyEnabled(core_ctx, extension::Ext_Zicntr))}
        extension::Ext_Svnapot => {false}
        extension::Ext_Svpbmt => {false}
        extension::Ext_C => {(hartSupports(core_ctx, extension::Ext_C) && ({
            let var_9: Misa = core_ctx.misa;
            _get_Misa_C(var_9)
        } == BitDynamic::new(1, 0b1)))}
        extension::Ext_Zca => {(hartSupports(core_ctx, extension::Ext_Zca) && (currentlyEnabled(core_ctx, extension::Ext_C) || !(hartSupports(core_ctx, extension::Ext_C))))}
        extension::Ext_Zifencei => {hartSupports(core_ctx, extension::Ext_Zifencei)}
        extension::Ext_A => {(hartSupports(core_ctx, extension::Ext_A) && ({
            let var_10: Misa = core_ctx.misa;
            _get_Misa_A(var_10)
        } == BitDynamic::new(1, 0b1)))}
        extension::Ext_Zabha => {(hartSupports(core_ctx, extension::Ext_Zabha) && currentlyEnabled(core_ctx, extension::Ext_Zaamo))}
        extension::Ext_Zalrsc => {(hartSupports(core_ctx, extension::Ext_Zalrsc) || currentlyEnabled(core_ctx, extension::Ext_A))}
        extension::Ext_Zaamo => {(hartSupports(core_ctx, extension::Ext_Zaamo) || currentlyEnabled(core_ctx, extension::Ext_A))}
        extension::Ext_M => {(hartSupports(core_ctx, extension::Ext_M) && ({
            let var_11: Misa = core_ctx.misa;
            _get_Misa_M(var_11)
        } == BitDynamic::new(1, 0b1)))}
        extension::Ext_Zmmul => {(hartSupports(core_ctx, extension::Ext_Zmmul) || currentlyEnabled(core_ctx, extension::Ext_M))}
        extension::Ext_Zfh => {(hartSupports(core_ctx, extension::Ext_Zfh) && currentlyEnabled(core_ctx, extension::Ext_F))}
        extension::Ext_Zfhmin => {((hartSupports(core_ctx, extension::Ext_Zfhmin) && currentlyEnabled(core_ctx, extension::Ext_F)) || currentlyEnabled(core_ctx, extension::Ext_Zfh))}
        extension::Ext_Zcf => {(hartSupports(core_ctx, extension::Ext_Zcf) && (currentlyEnabled(core_ctx, extension::Ext_F) && (currentlyEnabled(core_ctx, extension::Ext_Zca) && (currentlyEnabled(core_ctx, extension::Ext_C) || !(hartSupports(core_ctx, extension::Ext_C))))))}
        extension::Ext_Zdinx => {hartSupports(core_ctx, extension::Ext_Zdinx)}
        extension::Ext_Zcd => {(hartSupports(core_ctx, extension::Ext_Zcd) && (currentlyEnabled(core_ctx, extension::Ext_D) && (currentlyEnabled(core_ctx, extension::Ext_Zca) && (currentlyEnabled(core_ctx, extension::Ext_C) || !(hartSupports(core_ctx, extension::Ext_C))))))}
        extension::Ext_Svinval => {hartSupports(core_ctx, extension::Ext_Svinval)}
        extension::Ext_B => {(hartSupports(core_ctx, extension::Ext_B) && ({
            let var_12: Misa = core_ctx.misa;
            _get_Misa_B(var_12)
        } == BitDynamic::new(1, 0b1)))}
        extension::Ext_Zba => {(hartSupports(core_ctx, extension::Ext_Zba) || currentlyEnabled(core_ctx, extension::Ext_B))}
        extension::Ext_Zbb => {(hartSupports(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_B))}
        extension::Ext_Zbkb => {hartSupports(core_ctx, extension::Ext_Zbkb)}
        extension::Ext_Zbc => {hartSupports(core_ctx, extension::Ext_Zbc)}
        extension::Ext_Zbkc => {hartSupports(core_ctx, extension::Ext_Zbkc)}
        extension::Ext_Zbs => {(hartSupports(core_ctx, extension::Ext_Zbs) || currentlyEnabled(core_ctx, extension::Ext_B))}
        extension::Ext_Zcb => {(hartSupports(core_ctx, extension::Ext_Zcb) && currentlyEnabled(core_ctx, extension::Ext_Zca))}
        extension::Ext_Zhinx => {(hartSupports(core_ctx, extension::Ext_Zhinx) && currentlyEnabled(core_ctx, extension::Ext_Zfinx))}
        extension::Ext_Zfa => {(hartSupports(core_ctx, extension::Ext_Zfa) && currentlyEnabled(core_ctx, extension::Ext_F))}
        extension::Ext_Zknh => {hartSupports(core_ctx, extension::Ext_Zknh)}
        extension::Ext_Zkne => {hartSupports(core_ctx, extension::Ext_Zkne)}
        extension::Ext_Zknd => {hartSupports(core_ctx, extension::Ext_Zknd)}
        extension::Ext_Zksh => {hartSupports(core_ctx, extension::Ext_Zksh)}
        extension::Ext_Zksed => {hartSupports(core_ctx, extension::Ext_Zksed)}
        extension::Ext_Zbkx => {hartSupports(core_ctx, extension::Ext_Zbkx)}
        extension::Ext_Zicond => {hartSupports(core_ctx, extension::Ext_Zicond)}
        extension::Ext_Zicbom => {hartSupports(core_ctx, extension::Ext_Zicbom)}
        extension::Ext_Zicboz => {hartSupports(core_ctx, extension::Ext_Zicboz)}
        extension::Ext_Zvbb => {(hartSupports(core_ctx, extension::Ext_Zvbb) && currentlyEnabled(core_ctx, extension::Ext_V))}
        extension::Ext_Zvkb => {((hartSupports(core_ctx, extension::Ext_Zvkb) || currentlyEnabled(core_ctx, extension::Ext_Zvbb)) && currentlyEnabled(core_ctx, extension::Ext_V))}
        extension::Ext_Zvbc => {(hartSupports(core_ctx, extension::Ext_Zvbc) && currentlyEnabled(core_ctx, extension::Ext_V))}
        extension::Ext_Zvknha => {(hartSupports(core_ctx, extension::Ext_Zvknha) && currentlyEnabled(core_ctx, extension::Ext_V))}
        extension::Ext_Zvknhb => {(hartSupports(core_ctx, extension::Ext_Zvknhb) && currentlyEnabled(core_ctx, extension::Ext_V))}
        extension::Ext_Zvksh => {(hartSupports(core_ctx, extension::Ext_Zvksh) && currentlyEnabled(core_ctx, extension::Ext_V))}
        extension::Ext_Zimop => {hartSupports(core_ctx, extension::Ext_Zimop)}
        extension::Ext_Zcmop => {(hartSupports(core_ctx, extension::Ext_Zcmop) && currentlyEnabled(core_ctx, extension::Ext_Zca))}
        _ => {panic!("Unreachable code")}
    }
}

/// virtual_memory_supported
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L149-151.
pub fn virtual_memory_supported(core_ctx: &mut Core, unit_arg: ()) -> bool {
    (currentlyEnabled(core_ctx, extension::Ext_Sv32) || (currentlyEnabled(core_ctx, extension::Ext_Sv39) || (currentlyEnabled(core_ctx, extension::Ext_Sv48) || currentlyEnabled(core_ctx, extension::Ext_Sv57))))
}

/// lowest_supported_privLevel
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L157-158.
pub fn lowest_supported_privLevel(core_ctx: &mut Core, unit_arg: ()) -> Privilege {
    if {currentlyEnabled(core_ctx, extension::Ext_U)} {
        Privilege::User
    } else {
        Privilege::Machine
    }
}

/// have_privLevel
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L160-166.
pub fn have_privLevel(core_ctx: &mut Core, _priv_: BitDynamic) -> bool {
    match _priv_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {currentlyEnabled(core_ctx, extension::Ext_U)}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__2 if {(b__2 == BitDynamic::new(2, 0b10))} => {false}
        _ => {true}
        _ => {panic!("Unreachable code")}
    }
}

/// _update_Mstatus_FS
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_FS(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 14, 13, x)
    }
}

/// _update_Sstatus_FS
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_FS(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 14, 13, x)
    }
}

/// _get_Sstatus_FS
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_FS(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<13, 15, 2>()
}

/// _get_Mstatus_MIE
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_MIE(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<3, 4, 1>()
}

/// _update_Mstatus_MIE
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_MIE(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 3, 3, x)
    }
}

/// _get_Mstatus_MPIE
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPIE(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<7, 8, 1>()
}

/// _update_Mstatus_MPIE
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_MPIE(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _get_Mstatus_MPP
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPP(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<11, 13, 2>()
}

/// _update_Mstatus_MPP
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_MPP(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 12, 11, x)
    }
}

/// _get_Mstatus_MPRV
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPRV(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<17, 18, 1>()
}

/// _update_Mstatus_MPRV
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_MPRV(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 17, 17, x)
    }
}

/// _get_Mstatus_MXR
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_MXR(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<19, 20, 1>()
}

/// _update_Mstatus_MXR
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_MXR(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 19, 19, x)
    }
}

/// _update_Sstatus_MXR
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_MXR(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 19, 19, x)
    }
}

/// _get_Sstatus_MXR
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_MXR(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<19, 20, 1>()
}

/// _get_Mstatus_SD
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_SD(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<63, 64, 1>()
}

/// _update_Mstatus_SD
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_SD(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 63, 63, x)
    }
}

/// _update_Sstatus_SD
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_SD(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 63, 63, x)
    }
}

/// _get_Mstatus_SIE
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_SIE(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<1, 2, 1>()
}

/// _update_Mstatus_SIE
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_SIE(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _update_Sstatus_SIE
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_SIE(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _get_Sstatus_SIE
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_SIE(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<1, 2, 1>()
}

/// _get_Mstatus_SPIE
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_SPIE(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<5, 6, 1>()
}

/// _update_Mstatus_SPIE
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_SPIE(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _update_Sstatus_SPIE
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_SPIE(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _get_Sstatus_SPIE
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_SPIE(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<5, 6, 1>()
}

/// _get_Mstatus_SPP
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_SPP(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<8, 9, 1>()
}

/// _update_Mstatus_SPP
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_SPP(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 8, 8, x)
    }
}

/// _update_Sstatus_SPP
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_SPP(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 8, 8, x)
    }
}

/// _get_Sstatus_SPP
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_SPP(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<8, 9, 1>()
}

/// _get_Mstatus_SUM
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_SUM(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<18, 19, 1>()
}

/// _update_Mstatus_SUM
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_SUM(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 18, 18, x)
    }
}

/// _update_Sstatus_SUM
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_SUM(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 18, 18, x)
    }
}

/// _get_Sstatus_SUM
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_SUM(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<18, 19, 1>()
}

/// _get_Mstatus_SXL
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_SXL(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<34, 36, 2>()
}

/// _get_Mstatus_TSR
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_TSR(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<22, 23, 1>()
}

/// _update_Mstatus_TSR
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_TSR(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 22, 22, x)
    }
}

/// _get_Mstatus_TVM
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_TVM(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<20, 21, 1>()
}

/// _update_Mstatus_TVM
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_TVM(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 20, 20, x)
    }
}

/// _get_Mstatus_TW
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_TW(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<21, 22, 1>()
}

/// _update_Mstatus_TW
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_TW(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 21, 21, x)
    }
}

/// _get_Mstatus_UXL
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_UXL(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<32, 34, 2>()
}

/// _update_Sstatus_UXL
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_UXL(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 33, 32, x)
    }
}

/// _get_Sstatus_UXL
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_UXL(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<32, 34, 2>()
}

/// _update_Mstatus_VS
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_VS(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 10, 9, x)
    }
}

/// _update_Sstatus_VS
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_VS(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 10, 9, x)
    }
}

/// _get_Sstatus_VS
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_VS(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<9, 11, 2>()
}

/// _get_Mstatus_XS
///
/// Generated from the Sail sources.
pub fn _get_Mstatus_XS(v: Mstatus) -> BitDynamic {
    v.bits.subrange::<15, 17, 2>()
}

/// _update_Mstatus_XS
///
/// Generated from the Sail sources.
pub fn _update_Mstatus_XS(v: Mstatus, x: BitDynamic) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 16, 15, x)
    }
}

/// _update_Sstatus_XS
///
/// Generated from the Sail sources.
pub fn _update_Sstatus_XS(v: Sstatus, x: BitDynamic) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 16, 15, x)
    }
}

/// _get_Sstatus_XS
///
/// Generated from the Sail sources.
pub fn _get_Sstatus_XS(v: Sstatus) -> BitDynamic {
    v.bits.subrange::<15, 17, 2>()
}

/// effectivePrivilege
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L206-209.
pub fn effectivePrivilege(t: AccessType::<()>, m: Mstatus, _priv_: Privilege) -> Privilege {
    if {((t != AccessType::InstructionFetch(())) && (_get_Mstatus_MPRV(m) == BitDynamic::new(1, 0b1)))} {
        privLevel_of_bits(_get_Mstatus_MPP(m))
    } else {
        _priv_
    }
}

/// get_mstatus_SXL
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L211-215.
pub fn get_mstatus_SXL(m: Mstatus) -> BitDynamic {
    _get_Mstatus_SXL(m)
}

/// get_mstatus_UXL
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L217-221.
pub fn get_mstatus_UXL(m: Mstatus) -> BitDynamic {
    _get_Mstatus_UXL(m)
}

/// legalize_mstatus
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L223-273.
pub fn legalize_mstatus(core_ctx: &mut Core, o: Mstatus, v: BitDynamic) -> Mstatus {
    let v: Mstatus = Mk_Mstatus(v);
    let o: Mstatus = {
        let var_1: Mstatus = {
            let var_3: Mstatus = {
                let var_4: Mstatus = {
                    let var_6: Mstatus = {
                        let var_7: Mstatus = {
                            let var_8: Mstatus = {
                                let var_10: Mstatus = {
                                    let var_13: Mstatus = {
                                        let var_15: Mstatus = {
                                            let var_16: Mstatus = {
                                                let var_18: Mstatus = {
                                                    let var_20: Mstatus = {
                                                        let var_22: Mstatus = {
                                                            let var_24: Mstatus = {
                                                                let var_26: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                                                                    _get_Mstatus_TSR(v)
                                                                } else {
                                                                    BitDynamic::new(1, 0b0)
                                                                };
                                                                _update_Mstatus_TSR(o, var_26)
                                                            };
                                                            let var_25: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_U)} {
                                                                _get_Mstatus_TW(v)
                                                            } else {
                                                                BitDynamic::new(1, 0b0)
                                                            };
                                                            _update_Mstatus_TW(var_24, var_25)
                                                        };
                                                        let var_23: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                                                            _get_Mstatus_TVM(v)
                                                        } else {
                                                            BitDynamic::new(1, 0b0)
                                                        };
                                                        _update_Mstatus_TVM(var_22, var_23)
                                                    };
                                                    let var_21: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                                                        _get_Mstatus_MXR(v)
                                                    } else {
                                                        BitDynamic::new(1, 0b0)
                                                    };
                                                    _update_Mstatus_MXR(var_20, var_21)
                                                };
                                                let var_19: BitDynamic = if {virtual_memory_supported(core_ctx, ())} {
                                                    _get_Mstatus_SUM(v)
                                                } else {
                                                    BitDynamic::new(1, 0b0)
                                                };
                                                _update_Mstatus_SUM(var_18, var_19)
                                            };
                                            let var_17: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_U)} {
                                                _get_Mstatus_MPRV(v)
                                            } else {
                                                BitDynamic::new(1, 0b0)
                                            };
                                            _update_Mstatus_MPRV(var_16, var_17)
                                        };
                                        _update_Mstatus_XS(var_15, extStatus_to_bits(ExtStatus::Off))
                                    };
                                    let var_14: BitDynamic = if {hartSupports(core_ctx, extension::Ext_Zfinx)} {
                                        extStatus_to_bits(ExtStatus::Off)
                                    } else {
                                        _get_Mstatus_FS(v)
                                    };
                                    _update_Mstatus_FS(var_13, var_14)
                                };
                                let var_11: BitDynamic = if {have_privLevel(core_ctx, _get_Mstatus_MPP(v))} {
                                    _get_Mstatus_MPP(v)
                                } else {
                                    {
                                        let var_12: Privilege = lowest_supported_privLevel(core_ctx, ());
                                        privLevel_to_bits(var_12)
                                    }
                                };
                                _update_Mstatus_MPP(var_10, var_11)
                            };
                            let var_9: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                                _get_Mstatus_SPP(v)
                            } else {
                                BitDynamic::new(1, 0b0)
                            };
                            _update_Mstatus_SPP(var_8, var_9)
                        };
                        _update_Mstatus_VS(var_7, _get_Mstatus_VS(v))
                    };
                    _update_Mstatus_MPIE(var_6, _get_Mstatus_MPIE(v))
                };
                let var_5: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                    _get_Mstatus_SPIE(v)
                } else {
                    BitDynamic::new(1, 0b0)
                };
                _update_Mstatus_SPIE(var_4, var_5)
            };
            _update_Mstatus_MIE(var_3, _get_Mstatus_MIE(v))
        };
        let var_2: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
            _get_Mstatus_SIE(v)
        } else {
            BitDynamic::new(1, 0b0)
        };
        _update_Mstatus_SIE(var_1, var_2)
    };
    let dirty: bool = ((extStatus_of_bits(_get_Mstatus_FS(o)) == ExtStatus::Dirty) || ((extStatus_of_bits(_get_Mstatus_XS(o)) == ExtStatus::Dirty) || (extStatus_of_bits(_get_Mstatus_VS(o)) == ExtStatus::Dirty)));
    _update_Mstatus_SD(o, bool_to_bits(dirty))
}

/// cur_architecture
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L300-308.
pub fn cur_architecture(core_ctx: &mut Core, unit_arg: ()) -> Architecture {
    let a: arch_xlen = match core_ctx.cur_privilege {
        Privilege::Machine => {{
            let var_1: Misa = core_ctx.misa;
            _get_Misa_MXL(var_1)
        }}
        Privilege::Supervisor => {{
            let var_2: Mstatus = core_ctx.mstatus;
            get_mstatus_SXL(var_2)
        }}
        Privilege::User => {{
            let var_3: Mstatus = core_ctx.mstatus;
            get_mstatus_UXL(var_3)
        }}
        _ => {panic!("Unreachable code")}
    };
    architecture_backwards(a)
}

/// in32BitMode
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L310-312.
pub fn in32BitMode(core_ctx: &mut Core, unit_arg: ()) -> bool {
    (cur_architecture(core_ctx, ()) == Architecture::RV32)
}

/// Mk_MEnvcfg
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_MEnvcfg(v: BitDynamic) -> MEnvcfg {
    MEnvcfg {
        bits: v
    }
}

/// _get_MEnvcfg_CBCFE
///
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_CBCFE(v: MEnvcfg) -> BitDynamic {
    v.bits.subrange::<6, 7, 1>()
}

/// _update_MEnvcfg_CBCFE
///
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_CBCFE(v: MEnvcfg, x: BitDynamic) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 6, 6, x)
    }
}

/// _update_SEnvcfg_CBCFE
///
/// Generated from the Sail sources.
pub fn _update_SEnvcfg_CBCFE(v: SEnvcfg, x: BitDynamic) -> SEnvcfg {
    SEnvcfg {
        bits: update_subrange_bits(v.bits, 6, 6, x)
    }
}

/// _get_SEnvcfg_CBCFE
///
/// Generated from the Sail sources.
pub fn _get_SEnvcfg_CBCFE(v: SEnvcfg) -> BitDynamic {
    v.bits.subrange::<6, 7, 1>()
}

/// _get_MEnvcfg_CBIE
///
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_CBIE(v: MEnvcfg) -> BitDynamic {
    v.bits.subrange::<4, 6, 2>()
}

/// _update_MEnvcfg_CBIE
///
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_CBIE(v: MEnvcfg, x: BitDynamic) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 5, 4, x)
    }
}

/// _update_SEnvcfg_CBIE
///
/// Generated from the Sail sources.
pub fn _update_SEnvcfg_CBIE(v: SEnvcfg, x: BitDynamic) -> SEnvcfg {
    SEnvcfg {
        bits: update_subrange_bits(v.bits, 5, 4, x)
    }
}

/// _get_SEnvcfg_CBIE
///
/// Generated from the Sail sources.
pub fn _get_SEnvcfg_CBIE(v: SEnvcfg) -> BitDynamic {
    v.bits.subrange::<4, 6, 2>()
}

/// _get_MEnvcfg_CBZE
///
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_CBZE(v: MEnvcfg) -> BitDynamic {
    v.bits.subrange::<7, 8, 1>()
}

/// _update_MEnvcfg_CBZE
///
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_CBZE(v: MEnvcfg, x: BitDynamic) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _update_SEnvcfg_CBZE
///
/// Generated from the Sail sources.
pub fn _update_SEnvcfg_CBZE(v: SEnvcfg, x: BitDynamic) -> SEnvcfg {
    SEnvcfg {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _get_SEnvcfg_CBZE
///
/// Generated from the Sail sources.
pub fn _get_SEnvcfg_CBZE(v: SEnvcfg) -> BitDynamic {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_MEnvcfg_FIOM
///
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_FIOM(v: MEnvcfg) -> BitDynamic {
    v.bits.subrange::<0, 1, 1>()
}

/// _update_MEnvcfg_FIOM
///
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_FIOM(v: MEnvcfg, x: BitDynamic) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 0, 0, x)
    }
}

/// _update_SEnvcfg_FIOM
///
/// Generated from the Sail sources.
pub fn _update_SEnvcfg_FIOM(v: SEnvcfg, x: BitDynamic) -> SEnvcfg {
    SEnvcfg {
        bits: update_subrange_bits(v.bits, 0, 0, x)
    }
}

/// _get_SEnvcfg_FIOM
///
/// Generated from the Sail sources.
pub fn _get_SEnvcfg_FIOM(v: SEnvcfg) -> BitDynamic {
    v.bits.subrange::<0, 1, 1>()
}

/// _get_MEnvcfg_STCE
///
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_STCE(v: MEnvcfg) -> BitDynamic {
    v.bits.subrange::<63, 64, 1>()
}

/// _update_MEnvcfg_STCE
///
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_STCE(v: MEnvcfg, x: BitDynamic) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 63, 63, x)
    }
}

/// Mk_SEnvcfg
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_SEnvcfg(v: BitDynamic) -> SEnvcfg {
    SEnvcfg {
        bits: v
    }
}

/// legalize_menvcfg
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L347-357.
pub fn legalize_menvcfg(core_ctx: &mut Core, o: MEnvcfg, v: BitDynamic) -> MEnvcfg {
    let v: MEnvcfg = Mk_MEnvcfg(v);
    {
        let var_1: MEnvcfg = {
            let var_3: MEnvcfg = {
                let var_5: MEnvcfg = {
                    let var_7: MEnvcfg = {
                        let var_9: BitDynamic = if {sys_enable_writable_fiom(core_ctx, ())} {
                            _get_MEnvcfg_FIOM(v)
                        } else {
                            BitDynamic::new(1, 0b0)
                        };
                        _update_MEnvcfg_FIOM(o, var_9)
                    };
                    let var_8: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Zicboz)} {
                        _get_MEnvcfg_CBZE(v)
                    } else {
                        BitDynamic::new(1, 0b0)
                    };
                    _update_MEnvcfg_CBZE(var_7, var_8)
                };
                let var_6: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} {
                    _get_MEnvcfg_CBCFE(v)
                } else {
                    BitDynamic::new(1, 0b0)
                };
                _update_MEnvcfg_CBCFE(var_5, var_6)
            };
            let var_4: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} {
                if {(_get_MEnvcfg_CBIE(v) != BitDynamic::new(2, 0b10))} {
                    _get_MEnvcfg_CBIE(v)
                } else {
                    BitDynamic::new(2, 0b00)
                }
            } else {
                BitDynamic::new(2, 0b00)
            };
            _update_MEnvcfg_CBIE(var_3, var_4)
        };
        let var_2: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Sstc)} {
            _get_MEnvcfg_STCE(v)
        } else {
            BitDynamic::new(1, 0b0)
        };
        _update_MEnvcfg_STCE(var_1, var_2)
    }
}

/// legalize_senvcfg
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L359-368.
pub fn legalize_senvcfg(core_ctx: &mut Core, o: SEnvcfg, v: BitDynamic) -> SEnvcfg {
    let v: SEnvcfg = Mk_SEnvcfg(v);
    {
        let var_1: SEnvcfg = {
            let var_3: SEnvcfg = {
                let var_5: SEnvcfg = {
                    let var_7: BitDynamic = if {sys_enable_writable_fiom(core_ctx, ())} {
                        _get_SEnvcfg_FIOM(v)
                    } else {
                        BitDynamic::new(1, 0b0)
                    };
                    _update_SEnvcfg_FIOM(o, var_7)
                };
                let var_6: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Zicboz)} {
                    _get_SEnvcfg_CBZE(v)
                } else {
                    BitDynamic::new(1, 0b0)
                };
                _update_SEnvcfg_CBZE(var_5, var_6)
            };
            let var_4: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} {
                _get_SEnvcfg_CBCFE(v)
            } else {
                BitDynamic::new(1, 0b0)
            };
            _update_SEnvcfg_CBCFE(var_3, var_4)
        };
        let var_2: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} {
            if {(_get_SEnvcfg_CBIE(v) != BitDynamic::new(2, 0b10))} {
                _get_SEnvcfg_CBIE(v)
            } else {
                BitDynamic::new(2, 0b00)
            }
        } else {
            BitDynamic::new(2, 0b00)
        };
        _update_SEnvcfg_CBIE(var_1, var_2)
    }
}

/// is_fiom_active
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L394-400.
pub fn is_fiom_active(core_ctx: &mut Core, unit_arg: ()) -> bool {
    match core_ctx.cur_privilege {
        Privilege::Machine => {false}
        Privilege::Supervisor => {({
            let var_1: MEnvcfg = core_ctx.menvcfg;
            _get_MEnvcfg_FIOM(var_1)
        } == BitDynamic::new(1, 0b1))}
        Privilege::User => {(({
            let var_3: MEnvcfg = core_ctx.menvcfg;
            _get_MEnvcfg_FIOM(var_3)
        } | {
            let var_2: SEnvcfg = core_ctx.senvcfg;
            _get_SEnvcfg_FIOM(var_2)
        }) == BitDynamic::new(1, 0b1))}
        _ => {panic!("Unreachable code")}
    }
}

/// Mk_Minterrupts
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Minterrupts(v: BitDynamic) -> Minterrupts {
    Minterrupts {
        bits: v
    }
}

/// _get_Minterrupts_MEI
///
/// Generated from the Sail sources.
pub fn _get_Minterrupts_MEI(v: Minterrupts) -> BitDynamic {
    v.bits.subrange::<11, 12, 1>()
}

/// _update_Minterrupts_MEI
///
/// Generated from the Sail sources.
pub fn _update_Minterrupts_MEI(v: Minterrupts, x: BitDynamic) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 11, 11, x)
    }
}

/// _get_Minterrupts_MSI
///
/// Generated from the Sail sources.
pub fn _get_Minterrupts_MSI(v: Minterrupts) -> BitDynamic {
    v.bits.subrange::<3, 4, 1>()
}

/// _update_Minterrupts_MSI
///
/// Generated from the Sail sources.
pub fn _update_Minterrupts_MSI(v: Minterrupts, x: BitDynamic) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 3, 3, x)
    }
}

/// _get_Minterrupts_MTI
///
/// Generated from the Sail sources.
pub fn _get_Minterrupts_MTI(v: Minterrupts) -> BitDynamic {
    v.bits.subrange::<7, 8, 1>()
}

/// _update_Minterrupts_MTI
///
/// Generated from the Sail sources.
pub fn _update_Minterrupts_MTI(v: Minterrupts, x: BitDynamic) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _get_Minterrupts_SEI
///
/// Generated from the Sail sources.
pub fn _get_Minterrupts_SEI(v: Minterrupts) -> BitDynamic {
    v.bits.subrange::<9, 10, 1>()
}

/// _update_Minterrupts_SEI
///
/// Generated from the Sail sources.
pub fn _update_Minterrupts_SEI(v: Minterrupts, x: BitDynamic) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 9, 9, x)
    }
}

/// _update_Sinterrupts_SEI
///
/// Generated from the Sail sources.
pub fn _update_Sinterrupts_SEI(v: Sinterrupts, x: BitDynamic) -> Sinterrupts {
    Sinterrupts {
        bits: update_subrange_bits(v.bits, 9, 9, x)
    }
}

/// _get_Sinterrupts_SEI
///
/// Generated from the Sail sources.
pub fn _get_Sinterrupts_SEI(v: Sinterrupts) -> BitDynamic {
    v.bits.subrange::<9, 10, 1>()
}

/// _get_Minterrupts_SSI
///
/// Generated from the Sail sources.
pub fn _get_Minterrupts_SSI(v: Minterrupts) -> BitDynamic {
    v.bits.subrange::<1, 2, 1>()
}

/// _update_Minterrupts_SSI
///
/// Generated from the Sail sources.
pub fn _update_Minterrupts_SSI(v: Minterrupts, x: BitDynamic) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _update_Sinterrupts_SSI
///
/// Generated from the Sail sources.
pub fn _update_Sinterrupts_SSI(v: Sinterrupts, x: BitDynamic) -> Sinterrupts {
    Sinterrupts {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _get_Sinterrupts_SSI
///
/// Generated from the Sail sources.
pub fn _get_Sinterrupts_SSI(v: Sinterrupts) -> BitDynamic {
    v.bits.subrange::<1, 2, 1>()
}

/// _get_Minterrupts_STI
///
/// Generated from the Sail sources.
pub fn _get_Minterrupts_STI(v: Minterrupts) -> BitDynamic {
    v.bits.subrange::<5, 6, 1>()
}

/// _update_Minterrupts_STI
///
/// Generated from the Sail sources.
pub fn _update_Minterrupts_STI(v: Minterrupts, x: BitDynamic) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _update_Sinterrupts_STI
///
/// Generated from the Sail sources.
pub fn _update_Sinterrupts_STI(v: Sinterrupts, x: BitDynamic) -> Sinterrupts {
    Sinterrupts {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _get_Sinterrupts_STI
///
/// Generated from the Sail sources.
pub fn _get_Sinterrupts_STI(v: Sinterrupts) -> BitDynamic {
    v.bits.subrange::<5, 6, 1>()
}

/// legalize_mip
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L415-427.
pub fn legalize_mip(core_ctx: &mut Core, o: Minterrupts, v: BitDynamic) -> Minterrupts {
    let v: Minterrupts = Mk_Minterrupts(v);
    {
        let var_1: Minterrupts = {
            let var_4: Minterrupts = {
                let var_6: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                    _get_Minterrupts_SEI(v)
                } else {
                    BitDynamic::new(1, 0b0)
                };
                _update_Minterrupts_SEI(o, var_6)
            };
            let var_5: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                _get_Minterrupts_SSI(v)
            } else {
                BitDynamic::new(1, 0b0)
            };
            _update_Minterrupts_SSI(var_4, var_5)
        };
        let var_2: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
            if {(currentlyEnabled(core_ctx, extension::Ext_Sstc) && ({
                let var_3: MEnvcfg = core_ctx.menvcfg;
                _get_MEnvcfg_STCE(var_3)
            } == BitDynamic::new(1, 0b1)))} {
                _get_Minterrupts_STI(o)
            } else {
                _get_Minterrupts_STI(v)
            }
        } else {
            BitDynamic::new(1, 0b0)
        };
        _update_Minterrupts_STI(var_1, var_2)
    }
}

/// legalize_mie
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L429-439.
pub fn legalize_mie(core_ctx: &mut Core, o: Minterrupts, v: BitDynamic) -> Minterrupts {
    let v: Minterrupts = Mk_Minterrupts(v);
    {
        let var_1: Minterrupts = {
            let var_3: Minterrupts = {
                let var_5: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                    _get_Minterrupts_SEI(v)
                } else {
                    BitDynamic::new(1, 0b0)
                };
                _update_Minterrupts_SEI(_update_Minterrupts_MSI(_update_Minterrupts_MTI(_update_Minterrupts_MEI(o, _get_Minterrupts_MEI(v)), _get_Minterrupts_MTI(v)), _get_Minterrupts_MSI(v)), var_5)
            };
            let var_4: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                _get_Minterrupts_STI(v)
            } else {
                BitDynamic::new(1, 0b0)
            };
            _update_Minterrupts_STI(var_3, var_4)
        };
        let var_2: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
            _get_Minterrupts_SSI(v)
        } else {
            BitDynamic::new(1, 0b0)
        };
        _update_Minterrupts_SSI(var_1, var_2)
    }
}

/// legalize_mideleg
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L441-445.
pub fn legalize_mideleg(o: Minterrupts, v: BitDynamic) -> Minterrupts {
    _update_Minterrupts_MSI(_update_Minterrupts_MTI(_update_Minterrupts_MEI(Mk_Minterrupts(v), BitDynamic::new(1, 0b0)), BitDynamic::new(1, 0b0)), BitDynamic::new(1, 0b0))
}

/// Mk_Medeleg
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Medeleg(v: BitDynamic) -> Medeleg {
    Medeleg {
        bits: v
    }
}

/// _update_Medeleg_MEnvCall
///
/// Generated from the Sail sources.
pub fn _update_Medeleg_MEnvCall(v: Medeleg, x: BitDynamic) -> Medeleg {
    Medeleg {
        bits: update_subrange_bits(v.bits, 11, 11, x)
    }
}

/// legalize_medeleg
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L466-469.
pub fn legalize_medeleg(o: Medeleg, v: BitDynamic) -> Medeleg {
    _update_Medeleg_MEnvCall(Mk_Medeleg(v), BitDynamic::new(1, 0b0))
}

/// Mk_Mtvec
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Mtvec(v: BitDynamic) -> Mtvec {
    Mtvec {
        bits: v
    }
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

/// _update_Mtvec_Mode
///
/// Generated from the Sail sources.
pub fn _update_Mtvec_Mode(v: Mtvec, x: BitDynamic) -> Mtvec {
    Mtvec {
        bits: update_subrange_bits(v.bits, 1, 0, x)
    }
}

/// _get_Satp32_Mode
///
/// Generated from the Sail sources.
pub fn _get_Satp32_Mode(v: Satp32) -> BitDynamic {
    v.bits.subrange::<31, 32, 1>()
}

/// _get_Satp64_Mode
///
/// Generated from the Sail sources.
pub fn _get_Satp64_Mode(v: Satp64) -> BitDynamic {
    v.bits.subrange::<60, 64, 4>()
}

/// legalize_tvec
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L509-516.
pub fn legalize_tvec(o: Mtvec, v: BitDynamic) -> Mtvec {
    let v: Mtvec = Mk_Mtvec(v);
    match trapVectorMode_of_bits(_get_Mtvec_Mode(v)) {
        TrapVectorMode::TV_Direct => {v}
        TrapVectorMode::TV_Vector => {v}
        _ => {_update_Mtvec_Mode(v, _get_Mtvec_Mode(o))}
        _ => {panic!("Unreachable code")}
    }
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

/// tvec_addr
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L529-538.
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

/// legalize_xepc
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L547-552.
pub fn legalize_xepc(core_ctx: &mut Core, v: BitDynamic) -> BitDynamic {
    if {hartSupports(core_ctx, extension::Ext_C)} {
        bitvector_update(v, 0, false)
    } else {
        update_subrange_bits(v, 1, 0, zeros(2))
    }
}

/// align_pc
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L556-560.
pub fn align_pc(core_ctx: &mut Core, addr: BitDynamic) -> BitDynamic {
    if {({
        let var_1: Misa = core_ctx.misa;
        _get_Misa_C(var_1)
    } == BitDynamic::new(1, 0b1))} {
        bitvector_update(addr, 0, false)
    } else {
        update_subrange_bits(addr, 1, 0, zeros(2))
    }
}

/// Mk_Counteren
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Counteren(v: BitDynamic) -> Counteren {
    Counteren {
        bits: v
    }
}

/// _get_Counteren_TM
///
/// Generated from the Sail sources.
pub fn _get_Counteren_TM(v: Counteren) -> BitDynamic {
    v.bits.subrange::<1, 2, 1>()
}

/// legalize_scounteren
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L589-592.
pub fn legalize_scounteren(core_ctx: &mut Core, c: Counteren, v: BitDynamic) -> Counteren {
    let supported_counters: BitDynamic = bitvector_concat(BitDynamic::from(subrange_bits(sys_writable_hpm_counters(core_ctx, ()), 31, 3)), BitDynamic::from(BitDynamic::new(3, 0b111)));
    Mk_Counteren((v.subrange::<0, 32, 32>() & supported_counters))
}

/// legalize_mcounteren
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L601-604.
pub fn legalize_mcounteren(core_ctx: &mut Core, c: Counteren, v: BitDynamic) -> Counteren {
    let supported_counters: BitDynamic = bitvector_concat(BitDynamic::from(subrange_bits(sys_writable_hpm_counters(core_ctx, ()), 31, 3)), BitDynamic::from(BitDynamic::new(3, 0b111)));
    Mk_Counteren((v.subrange::<0, 32, 32>() & supported_counters))
}

/// Mk_Counterin
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Counterin(v: BitDynamic) -> Counterin {
    Counterin {
        bits: v
    }
}

/// legalize_mcountinhibit
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L620-624.
pub fn legalize_mcountinhibit(core_ctx: &mut Core, c: Counterin, v: BitDynamic) -> Counterin {
    let supported_counters: BitDynamic = bitvector_concat(BitDynamic::from(subrange_bits(sys_writable_hpm_counters(core_ctx, ()), 31, 3)), BitDynamic::from(BitDynamic::new(3, 0b101)));
    Mk_Counterin((v.subrange::<0, 32, 32>() & supported_counters))
}

/// Mk_Sstatus
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Sstatus(v: BitDynamic) -> Sstatus {
    Sstatus {
        bits: v
    }
}

/// lower_mstatus
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L697-714.
pub fn lower_mstatus(m: Mstatus) -> Sstatus {
    let s: Sstatus = Mk_Sstatus(zeros(64));
    _update_Sstatus_SIE(_update_Sstatus_SPIE(_update_Sstatus_SPP(_update_Sstatus_VS(_update_Sstatus_FS(_update_Sstatus_XS(_update_Sstatus_SUM(_update_Sstatus_MXR(_update_Sstatus_UXL(_update_Sstatus_SD(s, _get_Mstatus_SD(m)), _get_Mstatus_UXL(m)), _get_Mstatus_MXR(m)), _get_Mstatus_SUM(m)), _get_Mstatus_XS(m)), _get_Mstatus_FS(m)), _get_Mstatus_VS(m)), _get_Mstatus_SPP(m)), _get_Mstatus_SPIE(m)), _get_Mstatus_SIE(m))
}

/// lift_sstatus
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L716-734.
pub fn lift_sstatus(m: Mstatus, s: Sstatus) -> Mstatus {
    let dirty: bool = ((extStatus_of_bits(_get_Sstatus_FS(s)) == ExtStatus::Dirty) || ((extStatus_of_bits(_get_Sstatus_XS(s)) == ExtStatus::Dirty) || (extStatus_of_bits(_get_Sstatus_VS(s)) == ExtStatus::Dirty)));
    _update_Mstatus_SIE(_update_Mstatus_SPIE(_update_Mstatus_SPP(_update_Mstatus_VS(_update_Mstatus_FS(_update_Mstatus_XS(_update_Mstatus_SUM(_update_Mstatus_MXR(_update_Mstatus_UXL(_update_Mstatus_SD(m, bool_to_bits(dirty)), _get_Sstatus_UXL(s)), _get_Sstatus_MXR(s)), _get_Sstatus_SUM(s)), _get_Sstatus_XS(s)), _get_Sstatus_FS(s)), _get_Sstatus_VS(s)), _get_Sstatus_SPP(s)), _get_Sstatus_SPIE(s)), _get_Sstatus_SIE(s))
}

/// legalize_sstatus
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L736-738.
pub fn legalize_sstatus(core_ctx: &mut Core, m: Mstatus, v: BitDynamic) -> Mstatus {
    legalize_mstatus(core_ctx, m, lift_sstatus(m, Mk_Sstatus(v.zero_extend_dyn(64))).bits)
}

/// Mk_Sinterrupts
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Sinterrupts(v: BitDynamic) -> Sinterrupts {
    Sinterrupts {
        bits: v
    }
}

/// lower_mip
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L756-764.
pub fn lower_mip(m: Minterrupts, d: Minterrupts) -> Sinterrupts {
    let s: Sinterrupts = Mk_Sinterrupts(zeros(64));
    _update_Sinterrupts_SSI(_update_Sinterrupts_STI(_update_Sinterrupts_SEI(s, (_get_Minterrupts_SEI(m) & _get_Minterrupts_SEI(d))), (_get_Minterrupts_STI(m) & _get_Minterrupts_STI(d))), (_get_Minterrupts_SSI(m) & _get_Minterrupts_SSI(d)))
}

/// lower_mie
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L767-775.
pub fn lower_mie(m: Minterrupts, d: Minterrupts) -> Sinterrupts {
    let s: Sinterrupts = Mk_Sinterrupts(zeros(64));
    _update_Sinterrupts_SSI(_update_Sinterrupts_STI(_update_Sinterrupts_SEI(s, (_get_Minterrupts_SEI(m) & _get_Minterrupts_SEI(d))), (_get_Minterrupts_STI(m) & _get_Minterrupts_STI(d))), (_get_Minterrupts_SSI(m) & _get_Minterrupts_SSI(d)))
}

/// lift_sip
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L778-782.
pub fn lift_sip(o: Minterrupts, d: Minterrupts, s: Sinterrupts) -> Minterrupts {
    let m: Minterrupts = o;
    let m: Minterrupts = if {(_get_Minterrupts_SSI(d) == BitDynamic::new(1, 0b1))} {
        _update_Minterrupts_SSI(m, _get_Sinterrupts_SSI(s))
    } else {
        m
    };
    m
}

/// legalize_sip
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L784-786.
pub fn legalize_sip(m: Minterrupts, d: Minterrupts, v: BitDynamic) -> Minterrupts {
    lift_sip(m, d, Mk_Sinterrupts(v))
}

/// lift_sie
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L796-803.
pub fn lift_sie(o: Minterrupts, d: Minterrupts, s: Sinterrupts) -> Minterrupts {
    let m: Minterrupts = o;
    {
        let var_1: Minterrupts = {
            let var_3: Minterrupts = {
                let var_5: BitDynamic = if {(_get_Minterrupts_SEI(d) == BitDynamic::new(1, 0b1))} {
                    _get_Sinterrupts_SEI(s)
                } else {
                    _get_Minterrupts_SEI(m)
                };
                _update_Minterrupts_SEI(m, var_5)
            };
            let var_4: BitDynamic = if {(_get_Minterrupts_STI(d) == BitDynamic::new(1, 0b1))} {
                _get_Sinterrupts_STI(s)
            } else {
                _get_Minterrupts_STI(m)
            };
            _update_Minterrupts_STI(var_3, var_4)
        };
        let var_2: BitDynamic = if {(_get_Minterrupts_SSI(d) == BitDynamic::new(1, 0b1))} {
            _get_Sinterrupts_SSI(s)
        } else {
            _get_Minterrupts_SSI(m)
        };
        _update_Minterrupts_SSI(var_1, var_2)
    }
}

/// legalize_sie
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L805-807.
pub fn legalize_sie(m: Minterrupts, d: Minterrupts, v: BitDynamic) -> Minterrupts {
    lift_sie(m, d, Mk_Sinterrupts(v))
}

/// Mk_Satp64
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Satp64(v: BitDynamic) -> Satp64 {
    Satp64 {
        bits: v
    }
}

/// Mk_Satp32
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Satp32(v: BitDynamic) -> Satp32 {
    Satp32 {
        bits: v
    }
}

/// legalize_satp
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L858-888.
pub fn legalize_satp(core_ctx: &mut Core, arch: Architecture, prev_value: BitDynamic, written_value: BitDynamic) -> BitDynamic {
    let s: Satp64 = Mk_Satp64(written_value);
    match satpMode_of_bits(arch, _get_Satp64_Mode(s)) {
        None => {prev_value}
        Some(Sv_mode) => {match Sv_mode {
            SATPMode::Bare if {currentlyEnabled(core_ctx, extension::Ext_Svbare)} => {s.bits}
            SATPMode::Sv39 if {currentlyEnabled(core_ctx, extension::Ext_Sv39)} => {s.bits}
            SATPMode::Sv48 if {currentlyEnabled(core_ctx, extension::Ext_Sv48)} => {s.bits}
            SATPMode::Sv57 if {currentlyEnabled(core_ctx, extension::Ext_Sv57)} => {s.bits}
            _ => {prev_value}
            _ => {panic!("Unreachable code")}
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// get_vlenb
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L919-921.
pub fn get_vlenb(core_ctx: &mut Core, unit_arg: ()) -> BitDynamic {
    to_bits(64, (i128::pow(2, (get_vlen_pow(core_ctx, ()) as u32)) / 8))
}

/// _get_Vtype_vill
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vill(v: Vtype) -> BitDynamic {
    v.bits.subrange::<63, 64, 1>()
}

/// _get_Vtype_vlmul
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vlmul(v: Vtype) -> BitDynamic {
    v.bits.subrange::<0, 3, 3>()
}

/// _get_Vtype_vma
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vma(v: Vtype) -> BitDynamic {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_Vtype_vsew
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vsew(v: Vtype) -> BitDynamic {
    v.bits.subrange::<3, 6, 3>()
}

/// _get_Vtype_vta
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vta(v: Vtype) -> BitDynamic {
    v.bits.subrange::<6, 7, 1>()
}

/// get_sew_pow
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L936-945.
pub fn get_sew_pow(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    let SEW_pow: i128 = match {
        let var_1: Vtype = core_ctx.vtype;
        _get_Vtype_vsew(var_1)
    } {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {3}
        b__1 if {(b__1 == BitDynamic::new(3, 0b001))} => {4}
        b__2 if {(b__2 == BitDynamic::new(3, 0b010))} => {5}
        b__3 if {(b__3 == BitDynamic::new(3, 0b011))} => {6}
        _ => {{
            assert!(false, "invalid vsew field in vtype");
            panic!("exit")
        }}
        _ => {panic!("Unreachable code")}
    };
    SEW_pow
}

/// get_sew
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L948-958.
pub fn get_sew(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    let res: i128 = match get_sew_pow(core_ctx, ()) {
        l__533 if {(l__533 == 3)} => {8}
        l__534 if {(l__534 == 4)} => {16}
        l__535 if {(l__535 == 5)} => {32}
        l__536 if {(l__536 == 6)} => {64}
        _ => {{
            panic!("{}, l {}: {}", "riscv_sys_regs.sail", 954, "invalid SEW");
            8
        }}
        _ => {panic!("Unreachable code")}
    };
    assert!((res <= i128::pow(2, (vlen_exp(core_ctx) as u32))), "riscv_sys_regs.sail:956.28-956.29");
    res
}

/// get_lmul_pow
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L976-987.
pub fn get_lmul_pow(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    match {
        let var_1: Vtype = core_ctx.vtype;
        _get_Vtype_vlmul(var_1)
    } {
        b__0 if {(b__0 == BitDynamic::new(3, 0b101))} => {-3}
        b__1 if {(b__1 == BitDynamic::new(3, 0b110))} => {-2}
        b__2 if {(b__2 == BitDynamic::new(3, 0b111))} => {-1}
        b__3 if {(b__3 == BitDynamic::new(3, 0b000))} => {0}
        b__4 if {(b__4 == BitDynamic::new(3, 0b001))} => {1}
        b__5 if {(b__5 == BitDynamic::new(3, 0b010))} => {2}
        b__6 if {(b__6 == BitDynamic::new(3, 0b011))} => {3}
        _ => {{
            assert!(false, "invalid vlmul field in vtype");
            panic!("exit")
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// agtype
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L989.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum agtype {
    UNDISTURBED,
    AGNOSTIC
}

/// decode_agtype
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L992-997.
pub fn decode_agtype(ag: BitDynamic) -> agtype {
    match ag {
        b__0 if {(b__0 == BitDynamic::new(1, 0b0))} => {agtype::UNDISTURBED}
        _ => {agtype::AGNOSTIC}
        _ => {panic!("Unreachable code")}
    }
}

/// get_vtype_vma
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L1000.
pub fn get_vtype_vma(core_ctx: &mut Core, unit_arg: ()) -> agtype {
    let var_1: BitDynamic = {
        let var_2: Vtype = core_ctx.vtype;
        _get_Vtype_vma(var_2)
    };
    decode_agtype(var_1)
}

/// get_vtype_vta
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L1003.
pub fn get_vtype_vta(core_ctx: &mut Core, unit_arg: ()) -> agtype {
    let var_1: BitDynamic = {
        let var_2: Vtype = core_ctx.vtype;
        _get_Vtype_vta(var_2)
    };
    decode_agtype(var_1)
}

/// PmpAddrMatchType
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L11.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PmpAddrMatchType {
    OFF,
    TOR,
    NA4,
    NAPOT
}

/// pmpAddrMatchType_of_bits
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L14-21.
pub fn pmpAddrMatchType_of_bits(bs: BitDynamic) -> PmpAddrMatchType {
    match bs {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {PmpAddrMatchType::OFF}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {PmpAddrMatchType::TOR}
        b__2 if {(b__2 == BitDynamic::new(2, 0b10))} => {PmpAddrMatchType::NA4}
        _ => {PmpAddrMatchType::NAPOT}
        _ => {panic!("Unreachable code")}
    }
}

/// pmpAddrMatchType_to_bits
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L24-31.
pub fn pmpAddrMatchType_to_bits(bs: PmpAddrMatchType) -> BitDynamic {
    match bs {
        PmpAddrMatchType::OFF => {BitDynamic::new(2, 0b00)}
        PmpAddrMatchType::TOR => {BitDynamic::new(2, 0b01)}
        PmpAddrMatchType::NA4 => {BitDynamic::new(2, 0b10)}
        PmpAddrMatchType::NAPOT => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// Mk_Pmpcfg_ent
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_Pmpcfg_ent(v: BitDynamic) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: v
    }
}

/// pmpReadCfgReg
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L48-67.
pub fn pmpReadCfgReg(core_ctx: &mut Core, n: i128) -> BitDynamic {
    assert!((((n as usize) % (2 as usize)) == 0), "Unexpected pmp config reg read");
    bitvector_concat(BitDynamic::from(core_ctx.pmpcfg_n[(((n * 4) + 7) as usize)].bits), BitDynamic::from(bitvector_concat(BitDynamic::from(core_ctx.pmpcfg_n[(((n * 4) + 6) as usize)].bits), BitDynamic::from(bitvector_concat(BitDynamic::from(core_ctx.pmpcfg_n[(((n * 4) + 5) as usize)].bits), BitDynamic::from(bitvector_concat(BitDynamic::from(core_ctx.pmpcfg_n[(((n * 4) + 4) as usize)].bits), BitDynamic::from(bitvector_concat(BitDynamic::from(core_ctx.pmpcfg_n[(((n * 4) + 3) as usize)].bits), BitDynamic::from(bitvector_concat(BitDynamic::from(core_ctx.pmpcfg_n[(((n * 4) + 2) as usize)].bits), BitDynamic::from(bitvector_concat(BitDynamic::from(core_ctx.pmpcfg_n[(((n * 4) + 1) as usize)].bits), BitDynamic::from(core_ctx.pmpcfg_n[(((n * 4) + 0) as usize)].bits))))))))))))))
}

/// pmpReadAddrReg
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L69-89.
pub fn pmpReadAddrReg(core_ctx: &mut Core, n: i128) -> BitDynamic {
    let G: i128 = sys_pmp_grain(core_ctx, ());
    let match_type: BitDynamic = _get_Pmpcfg_ent_A(core_ctx.pmpcfg_n[(n as usize)]);
    let addr: BitDynamic = core_ctx.pmpaddr_n[(n as usize)];
    match bitvector_access(match_type, 1) {
        true if {(G >= 2)} => {{
            let mask: xlenbits = ones(min_int((G - 1), 64)).zero_extend_dyn(64);
            (addr | mask)
        }}
        false if {(G >= 1)} => {{
            let mask: xlenbits = ones(min_int(G, 64)).zero_extend_dyn(64);
            (addr & !(mask))
        }}
        _ => {addr}
        _ => {panic!("Unreachable code")}
    }
}

/// pmpLocked
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L92-93.
pub fn pmpLocked(cfg: Pmpcfg_ent) -> bool {
    (_get_Pmpcfg_ent_L(cfg) == BitDynamic::new(1, 0b1))
}

/// pmpTORLocked
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L95-96.
pub fn pmpTORLocked(cfg: Pmpcfg_ent) -> bool {
    ((_get_Pmpcfg_ent_L(cfg) == BitDynamic::new(1, 0b1)) && (pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(cfg)) == PmpAddrMatchType::TOR))
}

/// pmpWriteCfg
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L98-117.
pub fn pmpWriteCfg(core_ctx: &mut Core, n: i128, cfg: Pmpcfg_ent, v: BitDynamic) -> Pmpcfg_ent {
    if {pmpLocked(cfg)} {
        cfg
    } else {
        let cfg: Pmpcfg_ent = Mk_Pmpcfg_ent((v & BitDynamic::new(8, 0b10011111)));
        let cfg: Pmpcfg_ent = if {((_get_Pmpcfg_ent_W(cfg) == BitDynamic::new(1, 0b1)) && (_get_Pmpcfg_ent_R(cfg) == BitDynamic::new(1, 0b0)))} {
            _update_Pmpcfg_ent_R(_update_Pmpcfg_ent_W(_update_Pmpcfg_ent_X(cfg, BitDynamic::new(1, 0b0)), BitDynamic::new(1, 0b0)), BitDynamic::new(1, 0b0))
        } else {
            cfg
        };
        let cfg: Pmpcfg_ent = if {((sys_pmp_grain(core_ctx, ()) >= 1) && (pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(cfg)) == PmpAddrMatchType::NA4))} {
            _update_Pmpcfg_ent_A(cfg, pmpAddrMatchType_to_bits(PmpAddrMatchType::OFF))
        } else {
            cfg
        };
        cfg
    }
}

/// pmpWriteCfgReg
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L119-134.
pub fn pmpWriteCfgReg(core_ctx: &mut Core, n: i128, v: BitDynamic) {
    assert!((((n as usize) % (2 as usize)) == 0), "Unexpected pmp config reg write");
    for i in 0..=7 {
        let idx: i128 = ((n * 4) + i);
        core_ctx.pmpcfg_n[(idx as usize)] = pmpWriteCfg(core_ctx, idx, core_ctx.pmpcfg_n[(idx as usize)], subrange_bits(v, ((8 * i) + 7), (8 * i)))
    }
}

/// pmpWriteAddr
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L136-139.
pub fn pmpWriteAddr(locked: bool, tor_locked: bool, reg: BitDynamic, v: BitDynamic) -> BitDynamic {
    if {(locked || tor_locked)} {
        reg
    } else {
        v.subrange::<0, 54, 54>().zero_extend_dyn(64)
    }
}

/// pmpWriteAddrReg
///
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L141-148.
pub fn pmpWriteAddrReg(core_ctx: &mut Core, n: i128, v: BitDynamic) {
    core_ctx.pmpaddr_n[(n as usize)] = {
        let var_1: bool = if {((n + 1) < 64)} {
            pmpTORLocked(core_ctx.pmpcfg_n[((n + 1) as usize)])
        } else {
            false
        };
        pmpWriteAddr(pmpLocked(core_ctx.pmpcfg_n[(n as usize)]), var_1, core_ctx.pmpaddr_n[(n as usize)], v)
    }
}

/// pmpCheckRWX
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L13-19.
pub fn pmpCheckRWX(ent: Pmpcfg_ent, acc: AccessType::<()>) -> bool {
    match acc {
        AccessType::Read(_) => {(_get_Pmpcfg_ent_R(ent) == BitDynamic::new(1, 0b1))}
        AccessType::Write(_) => {(_get_Pmpcfg_ent_W(ent) == BitDynamic::new(1, 0b1))}
        AccessType::ReadWrite(_) => {((_get_Pmpcfg_ent_R(ent) == BitDynamic::new(1, 0b1)) && (_get_Pmpcfg_ent_W(ent) == BitDynamic::new(1, 0b1)))}
        AccessType::InstructionFetch(()) => {(_get_Pmpcfg_ent_X(ent) == BitDynamic::new(1, 0b1))}
        _ => {panic!("Unreachable code")}
    }
}

/// pmpAddrMatch
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L24.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum pmpAddrMatch {
    PMP_NoMatch,
    PMP_PartialMatch,
    PMP_Match
}

/// pmpRangeMatch
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L29-39.
pub fn pmpRangeMatch(begin: nat, end_: nat, addr: nat, width: nat) -> pmpAddrMatch {
    if {(((addr + width) <= begin) || (end_ <= addr))} {
        pmpAddrMatch::PMP_NoMatch
    } else if {((begin <= addr) && ((addr + width) <= end_))} {
        pmpAddrMatch::PMP_Match
    } else {
        pmpAddrMatch::PMP_PartialMatch
    }
}

/// pmpMatchAddr
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L41-80.
pub fn pmpMatchAddr(core_ctx: &mut Core, physaddr::Physaddr(addr): physaddr, width: BitDynamic, ent: Pmpcfg_ent, pmpaddr: BitDynamic, prev_pmpaddr: BitDynamic) -> pmpAddrMatch {
    let addr: i128 = addr.unsigned();
    let width: i128 = width.unsigned();
    match pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(ent)) {
        PmpAddrMatchType::OFF => {pmpAddrMatch::PMP_NoMatch}
        PmpAddrMatchType::TOR => {{
            if {_operator_biggerequal_u_(prev_pmpaddr, pmpaddr)} {
                pmpAddrMatch::PMP_NoMatch
            } else {
                pmpRangeMatch((((prev_pmpaddr.unsigned() as i128) * (4 as i128)) as nat), (((pmpaddr.unsigned() as i128) * (4 as i128)) as nat), (addr as nat), (width as nat))
            }
        }}
        PmpAddrMatchType::NA4 => {{
            assert!((sys_pmp_grain(core_ctx, ()) < 1), "NA4 cannot be selected when PMP grain G >= 1.");
            let begin: i128 = ((pmpaddr.unsigned() as i128) * (4 as i128));
            pmpRangeMatch((begin as nat), ((begin + 4) as nat), (addr as nat), (width as nat))
        }}
        PmpAddrMatchType::NAPOT => {{
            let mask: BitDynamic = (pmpaddr ^ (pmpaddr + 1));
            let begin_words: i128 = (pmpaddr & !(mask)).unsigned();
            let end_words: i128 = ((begin_words + mask.unsigned()) + 1);
            pmpRangeMatch(((begin_words * 4) as nat), ((end_words * 4) as nat), (addr as nat), (width as nat))
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// pmpMatch
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L82.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum pmpMatch {
    PMP_Success,
    PMP_Continue,
    PMP_Fail
}

/// pmpMatchEntry
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L84-93.
pub fn pmpMatchEntry(core_ctx: &mut Core, addr: physaddr, width: BitDynamic, acc: AccessType::<()>, _priv_: Privilege, ent: Pmpcfg_ent, pmpaddr: BitDynamic, prev_pmpaddr: BitDynamic) -> pmpMatch {
    match pmpMatchAddr(core_ctx, addr, width, ent, pmpaddr, prev_pmpaddr) {
        pmpAddrMatch::PMP_NoMatch => {pmpMatch::PMP_Continue}
        pmpAddrMatch::PMP_PartialMatch => {pmpMatch::PMP_Fail}
        pmpAddrMatch::PMP_Match => {if {(pmpCheckRWX(ent, acc) || ((_priv_ == Privilege::Machine) && !(pmpLocked(ent))))} {
            pmpMatch::PMP_Success
        } else {
            pmpMatch::PMP_Fail
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// accessToFault
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L97-103.
pub fn accessToFault(acc: AccessType::<()>) -> ExceptionType {
    match acc {
        AccessType::Read(_) => {ExceptionType::E_Load_Access_Fault(())}
        AccessType::Write(_) => {ExceptionType::E_SAMO_Access_Fault(())}
        AccessType::ReadWrite(_) => {ExceptionType::E_SAMO_Access_Fault(())}
        AccessType::InstructionFetch(()) => {ExceptionType::E_Fetch_Access_Fault(())}
        _ => {panic!("Unreachable code")}
    }
}

/// pmpCheck
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L105-118.
pub fn pmpCheck(core_ctx: &mut Core, addr: physaddr, width: i128, acc: AccessType::<()>, _priv_: Privilege) -> Option<ExceptionType> {
    let width: xlenbits = to_bits(64, width);
    for i in 0..=63 {
        let prev_pmpaddr: BitDynamic = if {(i > 0)} {
            pmpReadAddrReg(core_ctx, (i - 1))
        } else {
            zeros(64)
        };
        match {
            let var_1: BitDynamic = pmpReadAddrReg(core_ctx, i);
            pmpMatchEntry(core_ctx, addr, width, acc, _priv_, core_ctx.pmpcfg_n[(i as usize)], var_1, prev_pmpaddr)
        } {
            pmpMatch::PMP_Success => {{
                return None;
            }}
            pmpMatch::PMP_Fail => {{
                return Some(accessToFault(acc));
            }}
            pmpMatch::PMP_Continue => {()}
            _ => {panic!("Unreachable code")}
        }
    };
    if {(_priv_ == Privilege::Machine)} {
        None
    } else {
        Some(accessToFault(acc))
    }
}

/// reset_pmp
///
/// Generated from the Sail sources at `riscv_pmp_control.sail` L120-131.
pub fn reset_pmp(core_ctx: &mut Core, unit_arg: ()) {
    assert!(((sys_pmp_count(core_ctx, ()) == 0) || (((sys_pmp_count(core_ctx, ()) == 16) || ((sys_pmp_count(core_ctx, ()) == 64) as bool)) as bool)), "sys_pmp_count() must be 0, 16, or 64");
    for i in 0..=63 {
        core_ctx.pmpcfg_n[(i as usize)] = _update_Pmpcfg_ent_L(_update_Pmpcfg_ent_A(core_ctx.pmpcfg_n[(i as usize)], pmpAddrMatchType_to_bits(PmpAddrMatchType::OFF)), BitDynamic::new(1, 0b0))
    }
}

/// ext_check_CSR
///
/// Generated from the Sail sources at `riscv_ext_regs.sail` L18.
pub const fn ext_check_CSR(csrno: BitDynamic, p: Privilege, isWrite: bool) -> bool {
    true
}

/// Ext_FetchAddr_Check
///
/// Generated from the Sail sources at `riscv_addr_checks_common.sail` L17-20.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Ext_FetchAddr_Check<A> {
    Ext_FetchAddr_OK(virtaddr),
    Ext_FetchAddr_Error(A)
}

/// Ext_ControlAddr_Check
///
/// Generated from the Sail sources at `riscv_addr_checks_common.sail` L22-25.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Ext_ControlAddr_Check<A> {
    Ext_ControlAddr_OK(virtaddr),
    Ext_ControlAddr_Error(A)
}

/// Ext_DataAddr_Check
///
/// Generated from the Sail sources at `riscv_addr_checks_common.sail` L27-30.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Ext_DataAddr_Check<A> {
    Ext_DataAddr_OK(virtaddr),
    Ext_DataAddr_Error(A)
}

/// Ext_PhysAddr_Check
///
/// Generated from the Sail sources at `riscv_addr_checks_common.sail` L32-35.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Ext_PhysAddr_Check {
    Ext_PhysAddr_OK(()),
    Ext_PhysAddr_Error(ExceptionType)
}

pub type ext_fetch_addr_error = ();

pub type ext_control_addr_error = ();

/// ext_control_check_addr
///
/// Generated from the Sail sources at `riscv_addr_checks.sail` L40-41.
pub fn ext_control_check_addr(pc: BitDynamic) -> Ext_ControlAddr_Check::<()> {
    Ext_ControlAddr_Check::Ext_ControlAddr_OK(virtaddr::Virtaddr(pc))
}

/// ext_control_check_pc
///
/// Generated from the Sail sources at `riscv_addr_checks.sail` L44-45.
pub fn ext_control_check_pc(pc: BitDynamic) -> Ext_ControlAddr_Check::<()> {
    Ext_ControlAddr_Check::Ext_ControlAddr_OK(virtaddr::Virtaddr(pc))
}

pub type ext_data_addr_error = ();

/// ext_data_get_addr
///
/// Generated from the Sail sources at `riscv_addr_checks.sail` L57-60.
pub fn ext_data_get_addr(core_ctx: &mut Core, base: regidx, offset: BitDynamic, acc: AccessType::<()>, width: i128) -> Ext_DataAddr_Check::<()> {
    let addr: virtaddr = {
        let var_1: BitDynamic = rX_bits(core_ctx, base).wrapped_add(offset);
        virtaddr::Virtaddr(var_1)
    };
    Ext_DataAddr_Check::Ext_DataAddr_OK(addr)
}

pub type vreglenbits = BitDynamic;

pub type vregtype = vreglenbits;

/// wvvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L61.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wvvfunct6 {
    WVV_VADD,
    WVV_VSUB,
    WVV_VADDU,
    WVV_VSUBU,
    WVV_VWMUL,
    WVV_VWMULU,
    WVV_VWMULSU
}

/// wvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wvfunct6 {
    WV_VADD,
    WV_VSUB,
    WV_VADDU,
    WV_VSUBU
}

/// wvxfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L65.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wvxfunct6 {
    WVX_VADD,
    WVX_VSUB,
    WVX_VADDU,
    WVX_VSUBU,
    WVX_VWMUL,
    WVX_VWMULU,
    WVX_VWMULSU
}

/// wxfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L67.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wxfunct6 {
    WX_VADD,
    WX_VSUB,
    WX_VADDU,
    WX_VSUBU
}

/// rivvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L94.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rivvfunct6 {
    IVV_VWREDSUMU,
    IVV_VWREDSUM
}

/// rfvvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L96-97.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rfvvfunct6 {
    FVV_VFREDOSUM,
    FVV_VFREDUSUM,
    FVV_VFREDMAX,
    FVV_VFREDMIN,
    FVV_VFWREDOSUM,
    FVV_VFWREDUSUM
}

/// wmvvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L99.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wmvvfunct6 {
    WMVV_VWMACCU,
    WMVV_VWMACC,
    WMVV_VWMACCSU
}

/// wmvxfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L106.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wmvxfunct6 {
    WMVX_VWMACCU,
    WMVX_VWMACC,
    WMVX_VWMACCUS,
    WMVX_VWMACCSU
}

/// maskfunct3
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L108.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum maskfunct3 {
    VV_VMERGE,
    VI_VMERGE,
    VX_VMERGE
}

/// fwvvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L117.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvvfunct6 {
    FWVV_VADD,
    FWVV_VSUB,
    FWVV_VMUL
}

/// fwvvmafunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L119.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvvmafunct6 {
    FWVV_VMACC,
    FWVV_VNMACC,
    FWVV_VMSAC,
    FWVV_VNMSAC
}

/// fwvfunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L121.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvfunct6 {
    FWV_VADD,
    FWV_VSUB
}

/// vfwunary0
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L127-128.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vfwunary0 {
    FWV_CVT_XU_F,
    FWV_CVT_X_F,
    FWV_CVT_F_XU,
    FWV_CVT_F_X,
    FWV_CVT_F_F,
    FWV_CVT_RTZ_XU_F,
    FWV_CVT_RTZ_X_F
}

/// fwvffunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L140.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvffunct6 {
    FWVF_VADD,
    FWVF_VSUB,
    FWVF_VMUL
}

/// fwvfmafunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L142.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvfmafunct6 {
    FWVF_VMACC,
    FWVF_VNMACC,
    FWVF_VMSAC,
    FWVF_VNMSAC
}

/// fwffunct6
///
/// Generated from the Sail sources at `riscv_vreg_type.sail` L144.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwffunct6 {
    FWF_VADD,
    FWF_VSUB
}

/// vregno
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L10.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregno {
    Vregno(i128)
}

/// vregidx_to_vregno
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L11.
pub fn vregidx_to_vregno(vregidx::Vregidx(b): vregidx) -> vregno {
    vregno::Vregno(b.unsigned())
}

/// vregno_to_vregidx
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L12.
pub fn vregno_to_vregidx(vregno::Vregno(b): vregno) -> vregidx {
    vregidx::Vregidx(to_bits(5, b))
}

/// vregidx_offset
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L13.
pub fn vregidx_offset(vregidx::Vregidx(r): vregidx, o: BitDynamic) -> vregidx {
    vregidx::Vregidx(r.wrapped_add(o))
}

/// vregidx_bits
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L14.
pub fn vregidx_bits(vregidx::Vregidx(b): vregidx) -> BitDynamic {
    b
}

/// encdec_vreg_forwards
///
/// Generated from the Sail sources.
pub fn encdec_vreg_forwards(arg_hashtag_: vregidx) -> BitDynamic {
    match arg_hashtag_ {
        vregidx::Vregidx(r) => {r}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vreg_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vreg_backwards(arg_hashtag_: BitDynamic) -> vregidx {
    match arg_hashtag_ {
        r => {vregidx::Vregidx(r)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vreg_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vreg_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        r => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// vreg_write_callback
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L19.
pub const fn vreg_write_callback(_: vregidx, missing_arg_0: BitDynamic) {
    ()
}

/// zvreg
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L21.
pub const zvreg: vregidx = vregidx::Vregidx(BitDynamic::new(5, 0b00000));

/// dirty_v_context
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L94-99.
pub fn dirty_v_context(core_ctx: &mut Core, unit_arg: ()) {
    assert!(hartSupports(core_ctx, extension::Ext_V), "riscv_vext_regs.sail:95.28-95.29");
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(extStatus_to_bits(ExtStatus::Dirty), 10, 9);
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b1), 63, 63)
}

/// rV
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L101-136.
pub fn rV(core_ctx: &mut Core, vregno::Vregno(r): vregno) -> BitDynamic {
    match r {
        l__250 if {(l__250 == 0)} => {core_ctx.vr0}
        l__251 if {(l__251 == 1)} => {core_ctx.vr1}
        l__252 if {(l__252 == 2)} => {core_ctx.vr2}
        l__253 if {(l__253 == 3)} => {core_ctx.vr3}
        l__254 if {(l__254 == 4)} => {core_ctx.vr4}
        l__255 if {(l__255 == 5)} => {core_ctx.vr5}
        l__256 if {(l__256 == 6)} => {core_ctx.vr6}
        l__257 if {(l__257 == 7)} => {core_ctx.vr7}
        l__258 if {(l__258 == 8)} => {core_ctx.vr8}
        l__259 if {(l__259 == 9)} => {core_ctx.vr9}
        l__260 if {(l__260 == 10)} => {core_ctx.vr10}
        l__261 if {(l__261 == 11)} => {core_ctx.vr11}
        l__262 if {(l__262 == 12)} => {core_ctx.vr12}
        l__263 if {(l__263 == 13)} => {core_ctx.vr13}
        l__264 if {(l__264 == 14)} => {core_ctx.vr14}
        l__265 if {(l__265 == 15)} => {core_ctx.vr15}
        l__266 if {(l__266 == 16)} => {core_ctx.vr16}
        l__267 if {(l__267 == 17)} => {core_ctx.vr17}
        l__268 if {(l__268 == 18)} => {core_ctx.vr18}
        l__269 if {(l__269 == 19)} => {core_ctx.vr19}
        l__270 if {(l__270 == 20)} => {core_ctx.vr20}
        l__271 if {(l__271 == 21)} => {core_ctx.vr21}
        l__272 if {(l__272 == 22)} => {core_ctx.vr22}
        l__273 if {(l__273 == 23)} => {core_ctx.vr23}
        l__274 if {(l__274 == 24)} => {core_ctx.vr24}
        l__275 if {(l__275 == 25)} => {core_ctx.vr25}
        l__276 if {(l__276 == 26)} => {core_ctx.vr26}
        l__277 if {(l__277 == 27)} => {core_ctx.vr27}
        l__278 if {(l__278 == 28)} => {core_ctx.vr28}
        l__279 if {(l__279 == 29)} => {core_ctx.vr29}
        l__280 if {(l__280 == 30)} => {core_ctx.vr30}
        _ => {core_ctx.vr31}
        _ => {panic!("Unreachable code")}
    }
}

/// wV
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L138-177.
pub fn wV(core_ctx: &mut Core, vregno::Vregno(r): vregno, v: BitDynamic) {
    match r {
        l__219 if {(l__219 == 0)} => {core_ctx.vr0 = v}
        l__220 if {(l__220 == 1)} => {core_ctx.vr1 = v}
        l__221 if {(l__221 == 2)} => {core_ctx.vr2 = v}
        l__222 if {(l__222 == 3)} => {core_ctx.vr3 = v}
        l__223 if {(l__223 == 4)} => {core_ctx.vr4 = v}
        l__224 if {(l__224 == 5)} => {core_ctx.vr5 = v}
        l__225 if {(l__225 == 6)} => {core_ctx.vr6 = v}
        l__226 if {(l__226 == 7)} => {core_ctx.vr7 = v}
        l__227 if {(l__227 == 8)} => {core_ctx.vr8 = v}
        l__228 if {(l__228 == 9)} => {core_ctx.vr9 = v}
        l__229 if {(l__229 == 10)} => {core_ctx.vr10 = v}
        l__230 if {(l__230 == 11)} => {core_ctx.vr11 = v}
        l__231 if {(l__231 == 12)} => {core_ctx.vr12 = v}
        l__232 if {(l__232 == 13)} => {core_ctx.vr13 = v}
        l__233 if {(l__233 == 14)} => {core_ctx.vr14 = v}
        l__234 if {(l__234 == 15)} => {core_ctx.vr15 = v}
        l__235 if {(l__235 == 16)} => {core_ctx.vr16 = v}
        l__236 if {(l__236 == 17)} => {core_ctx.vr17 = v}
        l__237 if {(l__237 == 18)} => {core_ctx.vr18 = v}
        l__238 if {(l__238 == 19)} => {core_ctx.vr19 = v}
        l__239 if {(l__239 == 20)} => {core_ctx.vr20 = v}
        l__240 if {(l__240 == 21)} => {core_ctx.vr21 = v}
        l__241 if {(l__241 == 22)} => {core_ctx.vr22 = v}
        l__242 if {(l__242 == 23)} => {core_ctx.vr23 = v}
        l__243 if {(l__243 == 24)} => {core_ctx.vr24 = v}
        l__244 if {(l__244 == 25)} => {core_ctx.vr25 = v}
        l__245 if {(l__245 == 26)} => {core_ctx.vr26 = v}
        l__246 if {(l__246 == 27)} => {core_ctx.vr27 = v}
        l__247 if {(l__247 == 28)} => {core_ctx.vr28 = v}
        l__248 if {(l__248 == 29)} => {core_ctx.vr29 = v}
        l__249 if {(l__249 == 30)} => {core_ctx.vr30 = v}
        _ => {core_ctx.vr31 = v}
        _ => {panic!("Unreachable code")}
    };
    dirty_v_context(core_ctx, ())
}

/// rV_bits
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L179.
pub fn rV_bits(core_ctx: &mut Core, i: vregidx) -> BitDynamic {
    rV(core_ctx, vregidx_to_vregno(i))
}

/// wV_bits
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L181-183.
pub fn wV_bits(core_ctx: &mut Core, i: vregidx, data: BitDynamic) {
    wV(core_ctx, vregidx_to_vregno(i), data)
}

/// _get_Vcsr_vxrm
///
/// Generated from the Sail sources.
pub fn _get_Vcsr_vxrm(v: Vcsr) -> BitDynamic {
    v.bits.subrange::<1, 3, 2>()
}

/// _get_Vcsr_vxsat
///
/// Generated from the Sail sources.
pub fn _get_Vcsr_vxsat(v: Vcsr) -> BitDynamic {
    v.bits.subrange::<0, 1, 1>()
}

/// ext_write_vcsr
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L232-236.
pub fn ext_write_vcsr(core_ctx: &mut Core, vxrm_val: BitDynamic, vxsat_val: BitDynamic) {
    core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange(vxrm_val, 2, 1);
    core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange(vxsat_val, 0, 0);
    dirty_v_context(core_ctx, ())
}

/// get_num_elem
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L240-247.
pub fn get_num_elem(core_ctx: &mut Core, LMUL_pow: i128, SEW: i128) -> i128 {
    let LMUL_pow_reg: i128 = if {(LMUL_pow < 0)} {
        0
    } else {
        LMUL_pow
    };
    let num_elem: i128 = quot_round_zero((i128::pow(2, (LMUL_pow_reg as u32)) * get_vlen(core_ctx, ())), SEW);
    assert!(((num_elem <= i128::pow(2, (vlen_exp(core_ctx) as u32))) && (num_elem > 0)), "riscv_vext_regs.sail:245.48-245.49");
    num_elem
}

/// read_single_vreg
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L251-262.
pub fn read_single_vreg(core_ctx: &mut Core, num_elem: i128, SEW: i128, vrid: vregidx) -> Vec::<BitDynamic> {
    let bv: vregtype = rV_bits(core_ctx, vrid);
    let mut result: Vec::<BitDynamic> = vec![zeros(__id(SEW)); (__id(num_elem) as usize)];
    {
        assert!(((8 <= SEW) && (SEW <= 64)), "riscv_vext_regs.sail:255.29-255.30");
        for i in 0..=(num_elem - 1) {
            let start_index: i128 = (i * SEW);
            result[(i as usize)] = slice(bv, start_index, SEW)
        };
        result
    }
}

/// write_single_vreg
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L266-276.
pub fn write_single_vreg(core_ctx: &mut Core, num_elem: i128, SEW: i128, vrid: vregidx, v: Vec::<BitDynamic>) {
    let mut r: vregtype = zeros(i128::pow(2, (vlen_exp(core_ctx) as u32)));
    {
        assert!(((8 <= SEW) && (SEW <= 64)), "riscv_vext_regs.sail:269.29-269.30");
        for i in (0..=(num_elem - 1)).rev() {
            {
                r = (r << SEW);
                r = (r | v[(i as usize)].zero_extend_dyn(i128::pow(2, (vlen_exp(core_ctx) as u32))))
            }
        };
        wV_bits(core_ctx, vrid, r)
    }
}

/// read_vreg
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L280-314.
pub fn read_vreg(core_ctx: &mut Core, num_elem: i128, SEW: i128, LMUL_pow: i128, vrid: vregidx) -> Vec::<BitDynamic> {
    let vrid_val: i128 = vregidx_bits(vrid).unsigned();
    let mut result: Vec::<BitDynamic> = vec![zeros(__id(SEW)); (__id(num_elem) as usize)];
    {
        let LMUL_pow_reg: i128 = if {(LMUL_pow < 0)} {
            0
        } else {
            LMUL_pow
        };
        if {((vrid_val + i128::pow(2, (LMUL_pow_reg as u32))) > 32)} {
            assert!(false, "invalid register group: vrid overflow the largest number")
        } else if {(((vrid_val as usize) % (i128::pow(2, (LMUL_pow_reg as u32)) as usize)) != 0)} {
            assert!(false, "invalid register group: vrid is not a multiple of EMUL")
        } else {
            if {(LMUL_pow < 0)} {
                result = read_single_vreg(core_ctx, vector_length(&result), SEW, vrid)
            } else {
                let num_elem_single: i128 = quot_round_zero(get_vlen(core_ctx, ()), SEW);
                assert!((__id(num_elem_single) >= 0), "riscv_vext_regs.sail:297.34-297.35");
                for i_lmul in 0..=(i128::pow(2, (LMUL_pow_reg as u32)) - 1) {
                    let r_start_i: i128 = (i_lmul * __id(num_elem_single));
                    let r_end_i: i128 = ((r_start_i + __id(num_elem_single)) - 1);
                    let vrid_lmul: vregidx = vregidx_offset(vrid, to_bits(5, i_lmul));
                    let single_result: Vec::<BitDynamic> = read_single_vreg(core_ctx, __id(num_elem_single), SEW, vrid_lmul);
                    for r_i in r_start_i..=r_end_i {
                        let s_i: i128 = (r_i - r_start_i);
                        assert!(((0 <= r_i) && (r_i < num_elem)), "riscv_vext_regs.sail:305.42-305.43");
                        assert!(((0 <= s_i) && (s_i < __id(num_elem_single))), "riscv_vext_regs.sail:306.50-306.51");
                        result[(r_i as usize)] = single_result[(s_i as usize)]
                    }
                }
            }
        };
        result
    }
}

/// write_vreg
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L331-349.
pub fn write_vreg(core_ctx: &mut Core, num_elem: i128, SEW: i128, LMUL_pow: i128, vrid: vregidx, vec: Vec::<BitDynamic>) {
    let LMUL_pow_reg: i128 = if {(LMUL_pow < 0)} {
        0
    } else {
        LMUL_pow
    };
    let num_elem_single: i128 = quot_round_zero(get_vlen(core_ctx, ()), SEW);
    assert!((__id(num_elem_single) >= 0), "riscv_vext_regs.sail:335.30-335.31");
    for i_lmul in 0..=(i128::pow(2, (LMUL_pow_reg as u32)) - 1) {
        let mut single_vec: Vec::<BitDynamic> = vec![zeros(__id(SEW)); (__id(num_elem_single) as usize)];
        {
            let vrid_lmul: vregidx = vregidx_offset(vrid, to_bits(5, i_lmul));
            let r_start_i: i128 = (i_lmul * __id(num_elem_single));
            let r_end_i: i128 = ((r_start_i + __id(num_elem_single)) - 1);
            for r_i in r_start_i..=r_end_i {
                let s_i: i128 = (r_i - r_start_i);
                assert!(((0 <= r_i) && (r_i < num_elem)), "riscv_vext_regs.sail:343.38-343.39");
                assert!(((0 <= s_i) && (s_i < __id(num_elem_single))), "riscv_vext_regs.sail:344.46-344.47");
                single_vec[(s_i as usize)] = vec[(r_i as usize)]
            };
            write_single_vreg(core_ctx, __id(num_elem_single), SEW, vrid_lmul, single_vec)
        }
    }
}

/// read_vmask
///
/// Generated from the Sail sources at `riscv_vext_regs.sail` L374-387.
pub fn read_vmask(core_ctx: &mut Core, num_elem: i128, vm: BitDynamic, vrid: vregidx) -> BitDynamic {
    let vreg_val: vregtype = rV_bits(core_ctx, vrid);
    let mut result: BitDynamic = ones(__id(num_elem));
    {
        if {(vm == BitDynamic::new(1, 0b1))} {
            return result;
        } else {
            ()
        };
        for i in 0..=(num_elem - 1) {
            result = result.set_bit(i, bitvector_access(vreg_val, i))
        };
        result
    }
}

/// set_vstart
///
/// Generated from the Sail sources at `riscv_vext_control.sail` L15-20.
pub fn set_vstart(core_ctx: &mut Core, value: BitDynamic) {
    dirty_v_context(core_ctx, ());
    let vstart_length: i128 = get_vlen_pow(core_ctx, ());
    core_ctx.vstart = subrange_bits(value, (vstart_length - 1), 0).zero_extend_dyn(16)
}

pub type ext_exception = ();

/// ext_check_xret_priv
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L14.
pub const fn ext_check_xret_priv(p: Privilege) -> bool {
    true
}

/// handle_trap_extension
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L18.
pub const fn handle_trap_extension(p: Privilege, pc: BitDynamic, u: Option<()>) {
    ()
}

/// prepare_trap_vector
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L21-31.
pub fn prepare_trap_vector(core_ctx: &mut Core, p: Privilege, cause: Mcause) -> BitDynamic {
    let tvec: Mtvec = match p {
        Privilege::Machine => {core_ctx.mtvec}
        Privilege::Supervisor => {core_ctx.stvec}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sys_exceptions.sail", 25, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    };
    match tvec_addr(tvec, cause) {
        Some(epc) => {epc}
        None => {panic!("{}, l {}: {}", "riscv_sys_exceptions.sail", 29, "Invalid tvec mode")}
        _ => {panic!("Unreachable code")}
    }
}

/// get_xepc
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L41-46.
pub fn get_xepc(core_ctx: &mut Core, p: Privilege) -> BitDynamic {
    match p {
        Privilege::Machine => {{
            let var_1: BitDynamic = core_ctx.mepc;
            align_pc(core_ctx, var_1)
        }}
        Privilege::Supervisor => {{
            let var_2: BitDynamic = core_ctx.sepc;
            align_pc(core_ctx, var_2)
        }}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sys_exceptions.sail", 45, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

/// set_xepc
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L49-57.
pub fn set_xepc(core_ctx: &mut Core, p: Privilege, value: BitDynamic) -> BitDynamic {
    let target: BitDynamic = legalize_xepc(core_ctx, value);
    match p {
        Privilege::Machine => {core_ctx.mepc = target}
        Privilege::Supervisor => {core_ctx.sepc = target}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sys_exceptions.sail", 54, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    };
    target
}

/// prepare_xret_target
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L60-61.
pub fn prepare_xret_target(core_ctx: &mut Core, p: Privilege) -> BitDynamic {
    get_xepc(core_ctx, p)
}

/// get_mtvec
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L65-66.
pub fn get_mtvec(core_ctx: &mut Core, unit_arg: ()) -> BitDynamic {
    core_ctx.mtvec.bits
}

/// get_stvec
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L68-69.
pub fn get_stvec(core_ctx: &mut Core, unit_arg: ()) -> BitDynamic {
    core_ctx.stvec.bits
}

/// set_mtvec
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L71-74.
pub fn set_mtvec(core_ctx: &mut Core, value: BitDynamic) -> BitDynamic {
    core_ctx.mtvec = {
        let var_1: Mtvec = core_ctx.mtvec;
        legalize_tvec(var_1, value)
    };
    core_ctx.mtvec.bits
}

/// set_stvec
///
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L76-79.
pub fn set_stvec(core_ctx: &mut Core, value: BitDynamic) -> BitDynamic {
    core_ctx.stvec = {
        let var_1: Mtvec = core_ctx.stvec;
        legalize_tvec(var_1, value)
    };
    core_ctx.stvec.bits
}

/// sync_exception
///
/// Generated from the Sail sources at `riscv_sync_exception.sail` L11-15.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct sync_exception {
    pub trap: ExceptionType,
    pub excinfo: Option<xlenbits>,
    pub ext: Option<ext_exception>,
}

/// Mk_HpmEvent
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_HpmEvent(v: BitDynamic) -> HpmEvent {
    HpmEvent {
        bits: v
    }
}

/// _get_HpmEvent_MINH
///
/// Generated from the Sail sources.
pub fn _get_HpmEvent_MINH(v: HpmEvent) -> BitDynamic {
    v.bits.subrange::<62, 63, 1>()
}

/// _update_HpmEvent_MINH
///
/// Generated from the Sail sources.
pub fn _update_HpmEvent_MINH(v: HpmEvent, x: BitDynamic) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 62, 62, x)
    }
}

/// _update_CountSmcntrpmf_MINH
///
/// Generated from the Sail sources.
pub fn _update_CountSmcntrpmf_MINH(v: CountSmcntrpmf, x: BitDynamic) -> CountSmcntrpmf {
    CountSmcntrpmf {
        bits: update_subrange_bits(v.bits, 62, 62, x)
    }
}

/// _get_CountSmcntrpmf_MINH
///
/// Generated from the Sail sources.
pub fn _get_CountSmcntrpmf_MINH(v: CountSmcntrpmf) -> BitDynamic {
    v.bits.subrange::<62, 63, 1>()
}

/// _get_HpmEvent_OF
///
/// Generated from the Sail sources.
pub fn _get_HpmEvent_OF(v: HpmEvent) -> BitDynamic {
    v.bits.subrange::<63, 64, 1>()
}

/// _update_HpmEvent_OF
///
/// Generated from the Sail sources.
pub fn _update_HpmEvent_OF(v: HpmEvent, x: BitDynamic) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 63, 63, x)
    }
}

/// _get_HpmEvent_SINH
///
/// Generated from the Sail sources.
pub fn _get_HpmEvent_SINH(v: HpmEvent) -> BitDynamic {
    v.bits.subrange::<61, 62, 1>()
}

/// _update_HpmEvent_SINH
///
/// Generated from the Sail sources.
pub fn _update_HpmEvent_SINH(v: HpmEvent, x: BitDynamic) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 61, 61, x)
    }
}

/// _update_CountSmcntrpmf_SINH
///
/// Generated from the Sail sources.
pub fn _update_CountSmcntrpmf_SINH(v: CountSmcntrpmf, x: BitDynamic) -> CountSmcntrpmf {
    CountSmcntrpmf {
        bits: update_subrange_bits(v.bits, 61, 61, x)
    }
}

/// _get_CountSmcntrpmf_SINH
///
/// Generated from the Sail sources.
pub fn _get_CountSmcntrpmf_SINH(v: CountSmcntrpmf) -> BitDynamic {
    v.bits.subrange::<61, 62, 1>()
}

/// _get_HpmEvent_UINH
///
/// Generated from the Sail sources.
pub fn _get_HpmEvent_UINH(v: HpmEvent) -> BitDynamic {
    v.bits.subrange::<60, 61, 1>()
}

/// _update_HpmEvent_UINH
///
/// Generated from the Sail sources.
pub fn _update_HpmEvent_UINH(v: HpmEvent, x: BitDynamic) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 60, 60, x)
    }
}

/// _update_CountSmcntrpmf_UINH
///
/// Generated from the Sail sources.
pub fn _update_CountSmcntrpmf_UINH(v: CountSmcntrpmf, x: BitDynamic) -> CountSmcntrpmf {
    CountSmcntrpmf {
        bits: update_subrange_bits(v.bits, 60, 60, x)
    }
}

/// _get_CountSmcntrpmf_UINH
///
/// Generated from the Sail sources.
pub fn _get_CountSmcntrpmf_UINH(v: CountSmcntrpmf) -> BitDynamic {
    v.bits.subrange::<60, 61, 1>()
}

/// _update_HpmEvent_VSINH
///
/// Generated from the Sail sources.
pub fn _update_HpmEvent_VSINH(v: HpmEvent, x: BitDynamic) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 59, 59, x)
    }
}

/// _update_HpmEvent_VUINH
///
/// Generated from the Sail sources.
pub fn _update_HpmEvent_VUINH(v: HpmEvent, x: BitDynamic) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 58, 58, x)
    }
}

/// _get_HpmEvent_event
///
/// Generated from the Sail sources.
pub fn _get_HpmEvent_event(v: HpmEvent) -> BitDynamic {
    v.bits.subrange::<0, 32, 32>()
}

/// _update_HpmEvent_event
///
/// Generated from the Sail sources.
pub fn _update_HpmEvent_event(v: HpmEvent, x: BitDynamic) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 31, 0, x)
    }
}

pub type hpmidx = i128;

/// hpmidx_from_bits
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L189-193.
pub fn hpmidx_from_bits(b: BitDynamic) -> i128 {
    let index: i128 = b.unsigned();
    assert!((index >= 3), "unreachable HPM index");
    index
}

/// legalize_hpmevent
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L195-205.
pub fn legalize_hpmevent(core_ctx: &mut Core, v: HpmEvent) -> HpmEvent {
    {
        let var_1: HpmEvent = {
            let var_2: HpmEvent = {
                let var_3: HpmEvent = {
                    let var_4: HpmEvent = {
                        let var_6: HpmEvent = {
                            let var_8: HpmEvent = {
                                let var_10: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Sscofpmf)} {
                                    _get_HpmEvent_OF(v)
                                } else {
                                    BitDynamic::new(1, 0b0)
                                };
                                _update_HpmEvent_OF(Mk_HpmEvent(zeros(64)), var_10)
                            };
                            let var_9: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_Sscofpmf)} {
                                _get_HpmEvent_MINH(v)
                            } else {
                                BitDynamic::new(1, 0b0)
                            };
                            _update_HpmEvent_MINH(var_8, var_9)
                        };
                        let var_7: BitDynamic = if {(currentlyEnabled(core_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(core_ctx, extension::Ext_S))} {
                            _get_HpmEvent_SINH(v)
                        } else {
                            BitDynamic::new(1, 0b0)
                        };
                        _update_HpmEvent_SINH(var_6, var_7)
                    };
                    let var_5: BitDynamic = if {(currentlyEnabled(core_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(core_ctx, extension::Ext_U))} {
                        _get_HpmEvent_UINH(v)
                    } else {
                        BitDynamic::new(1, 0b0)
                    };
                    _update_HpmEvent_UINH(var_4, var_5)
                };
                _update_HpmEvent_VSINH(var_3, BitDynamic::new(1, 0b0))
            };
            _update_HpmEvent_VUINH(var_2, BitDynamic::new(1, 0b0))
        };
        _update_HpmEvent_event(var_1, _get_HpmEvent_event(v))
    }
}

/// read_mhpmcounter
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L207.
pub fn read_mhpmcounter(core_ctx: &mut Core, index: i128) -> BitDynamic {
    subrange_bits(core_ctx.mhpmcounter[(index as usize)], 63, 0)
}

/// read_mhpmcounterh
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L208.
pub fn read_mhpmcounterh(core_ctx: &mut Core, index: i128) -> BitDynamic {
    subrange_bits(core_ctx.mhpmcounter[(index as usize)], 63, 32)
}

/// read_mhpmevent
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L209.
pub fn read_mhpmevent(core_ctx: &mut Core, index: i128) -> BitDynamic {
    core_ctx.mhpmevent[(index as usize)].bits.subrange::<0, 64, 64>()
}

/// write_mhpmcounter
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L212-213.
pub fn write_mhpmcounter(core_ctx: &mut Core, index: i128, value: BitDynamic) {
    if {(bitvector_access(sys_writable_hpm_counters(core_ctx, ()), index) == true)} {
        core_ctx.mhpmcounter[(index as usize)] = core_ctx.mhpmcounter[(index as usize)].set_subrange(value, 63, 0)
    } else {
        ()
    }
}

/// write_mhpmcounterh
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L215-216.
pub fn write_mhpmcounterh(core_ctx: &mut Core, index: i128, value: BitDynamic) {
    if {(bitvector_access(sys_writable_hpm_counters(core_ctx, ()), index) == true)} {
        core_ctx.mhpmcounter[(index as usize)] = core_ctx.mhpmcounter[(index as usize)].set_subrange(value, 63, 32)
    } else {
        ()
    }
}

/// write_mhpmevent
///
/// Generated from the Sail sources at `riscv_zihpm.sail` L218-224.
pub fn write_mhpmevent(core_ctx: &mut Core, index: i128, value: BitDynamic) {
    if {(bitvector_access(sys_writable_hpm_counters(core_ctx, ()), index) == true)} {
        core_ctx.mhpmevent[(index as usize)] = legalize_hpmevent(core_ctx, Mk_HpmEvent(value))
    } else {
        ()
    }
}

/// read_mhpmeventh
///
/// Generated from the Sail sources at `riscv_sscofpmf.sail` L44.
pub fn read_mhpmeventh(core_ctx: &mut Core, index: i128) -> BitDynamic {
    core_ctx.mhpmevent[(index as usize)].bits.subrange::<32, 64, 32>()
}

/// write_mhpmeventh
///
/// Generated from the Sail sources at `riscv_sscofpmf.sail` L46-48.
pub fn write_mhpmeventh(core_ctx: &mut Core, index: i128, value: BitDynamic) {
    if {(bitvector_access(sys_writable_hpm_counters(core_ctx, ()), index) == true)} {
        core_ctx.mhpmevent[(index as usize)] = legalize_hpmevent(core_ctx, Mk_HpmEvent(bitvector_concat(BitDynamic::from(value), BitDynamic::from(core_ctx.mhpmevent[(index as usize)].bits.subrange::<0, 32, 32>()))))
    } else {
        ()
    }
}

/// get_scountovf
///
/// Generated from the Sail sources at `riscv_sscofpmf.sail` L60-76.
pub fn get_scountovf(core_ctx: &mut Core, _priv_: Privilege) -> BitDynamic {
    let overflow: BitDynamic = bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(31 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(30 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(29 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(28 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(27 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(26 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(25 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(24 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(23 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(22 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(21 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(20 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(19 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(18 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(17 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(16 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(15 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(14 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(13 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(12 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(11 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(10 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(9 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(8 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(7 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(6 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(5 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(4 as usize)])), BitDynamic::from(bitvector_concat(BitDynamic::from(_get_HpmEvent_OF(core_ctx.mhpmevent[(3 as usize)])), BitDynamic::from(BitDynamic::new(3, 0b000)))))))))))))))))))))))))))))))))))))))))))))))))))))))))));
    match _priv_ {
        Privilege::Machine => {overflow}
        Privilege::Supervisor => {(overflow & core_ctx.mcounteren.bits)}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sscofpmf.sail", 74, "scountovf not readable from User mode")}
        _ => {panic!("Unreachable code")}
    }
}

/// seed_opst
///
/// Generated from the Sail sources at `riscv_zkr_control.sail` L13-18.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum seed_opst {
    BIST,
    ES16,
    WAIT,
    DEAD
}

/// opst_code_forwards
///
/// Generated from the Sail sources.
pub fn opst_code_forwards(arg_hashtag_: seed_opst) -> BitDynamic {
    match arg_hashtag_ {
        seed_opst::BIST => {BitDynamic::new(2, 0b00)}
        seed_opst::WAIT => {BitDynamic::new(2, 0b01)}
        seed_opst::ES16 => {BitDynamic::new(2, 0b10)}
        seed_opst::DEAD => {BitDynamic::new(2, 0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// read_seed_csr
///
/// Generated from the Sail sources at `riscv_zkr_control.sail` L33-38.
pub fn read_seed_csr(unit_arg: ()) -> BitDynamic {
    let reserved_bits: BitDynamic = BitDynamic::new(6, 0b000000);
    let custom_bits: BitDynamic = BitDynamic::new(8, 0b00000000);
    let seed: BitDynamic = get_16_random_bits(());
    bitvector_concat(BitDynamic::from(opst_code_forwards(seed_opst::ES16)), BitDynamic::from(bitvector_concat(BitDynamic::from(reserved_bits), BitDynamic::from(bitvector_concat(BitDynamic::from(custom_bits), BitDynamic::from(seed)))))).zero_extend_dyn(64)
}

/// write_seed_csr
///
/// Generated from the Sail sources at `riscv_zkr_control.sail` L41.
pub fn write_seed_csr(unit_arg: ()) -> BitDynamic {
    zeros(64)
}

pub type bits_rm = BitDynamic;

pub type bits_fflags = BitDynamic;

pub type bits_H = BitDynamic;

pub type bits_S = BitDynamic;

pub type bits_D = BitDynamic;

pub type bits_W = BitDynamic;

pub type bits_WU = BitDynamic;

pub type bits_L = BitDynamic;

pub type bits_LU = BitDynamic;

/// fregno
///
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L56.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fregno {
    Fregno(i128)
}

/// encdec_freg_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_freg_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        r => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// dirty_fd_context
///
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L109-114.
pub fn dirty_fd_context(core_ctx: &mut Core, unit_arg: ()) {
    assert!(hartSupports(core_ctx, extension::Ext_F), "riscv_fdext_regs.sail:110.28-110.29");
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(extStatus_to_bits(ExtStatus::Dirty), 14, 13);
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b1), 63, 63)
}

/// dirty_fd_context_if_present
///
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L116-119.
pub fn dirty_fd_context_if_present(core_ctx: &mut Core, unit_arg: ()) {
    assert!({
        let var_1: bool = hartSupports(core_ctx, extension::Ext_F);
        let var_2: bool = hartSupports(core_ctx, extension::Ext_Zfinx);
        neq_bool(var_1, var_2)
    }, "riscv_fdext_regs.sail:117.55-117.56");
    if {hartSupports(core_ctx, extension::Ext_F)} {
        dirty_fd_context(core_ctx, ())
    } else {
        ()
    }
}

/// Mk_CountSmcntrpmf
///
/// Generated from the Sail sources at `` L1.
pub fn Mk_CountSmcntrpmf(v: BitDynamic) -> CountSmcntrpmf {
    CountSmcntrpmf {
        bits: v
    }
}

/// legalize_smcntrpmf
///
/// Generated from the Sail sources at `riscv_smcntrpmf.sail` L11-21.
pub fn legalize_smcntrpmf(core_ctx: &mut Core, c: CountSmcntrpmf, value: BitDynamic) -> CountSmcntrpmf {
    let v: CountSmcntrpmf = Mk_CountSmcntrpmf(value);
    {
        let var_1: CountSmcntrpmf = {
            let var_3: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                _get_CountSmcntrpmf_SINH(v)
            } else {
                BitDynamic::new(1, 0b0)
            };
            _update_CountSmcntrpmf_SINH(_update_CountSmcntrpmf_MINH(c, _get_CountSmcntrpmf_MINH(v)), var_3)
        };
        let var_2: BitDynamic = if {currentlyEnabled(core_ctx, extension::Ext_U)} {
            _get_CountSmcntrpmf_UINH(v)
        } else {
            BitDynamic::new(1, 0b0)
        };
        _update_CountSmcntrpmf_UINH(var_1, var_2)
    }
}

/// csrAccess
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L13.
pub fn csrAccess(csr: BitDynamic) -> BitDynamic {
    csr.subrange::<10, 12, 2>()
}

/// csrPriv
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L14.
pub fn csrPriv(csr: BitDynamic) -> BitDynamic {
    csr.subrange::<8, 10, 2>()
}

/// check_CSR_priv
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L17-18.
pub fn check_CSR_priv(csr: BitDynamic, p: Privilege) -> bool {
    _operator_biggerequal_u_(privLevel_to_bits(p), csrPriv(csr))
}

/// check_CSR_access
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L21-22.
pub fn check_CSR_access(csr: BitDynamic, isWrite: bool) -> bool {
    !((isWrite && (csrAccess(csr) == BitDynamic::new(2, 0b11))))
}

/// check_TVM_SATP
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L24-25.
pub fn check_TVM_SATP(core_ctx: &mut Core, csr: BitDynamic, p: Privilege) -> bool {
    !(((csr == BitDynamic::new(12, 0b000110000000)) && ((p == Privilege::Supervisor) && ({
        let var_1: Mstatus = core_ctx.mstatus;
        _get_Mstatus_TVM(var_1)
    } == BitDynamic::new(1, 0b1)))))
}

/// feature_enabled_for_priv
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L29-33.
pub fn feature_enabled_for_priv(core_ctx: &mut Core, p: Privilege, machine_enable_bit: bool, supervisor_enable_bit: bool) -> bool {
    match p {
        Privilege::Machine => {true}
        Privilege::Supervisor => {(machine_enable_bit == true)}
        Privilege::User => {((machine_enable_bit == true) && (!(currentlyEnabled(core_ctx, extension::Ext_S)) || (supervisor_enable_bit == true)))}
        _ => {panic!("Unreachable code")}
    }
}

/// check_Counteren
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L36-43.
pub fn check_Counteren(core_ctx: &mut Core, csr: BitDynamic, p: Privilege) -> bool {
    if {(_operator_smaller_u_(csr, BitDynamic::new(12, 0b110000000000)) || _operator_smaller_u_(BitDynamic::new(12, 0b110000011111), csr))} {
        return true;
    } else {
        ()
    };
    let index: i128 = csr.subrange::<0, 5, 5>().unsigned();
    {
        let var_1: bool = bitvector_access(core_ctx.mcounteren.bits, index);
        let var_2: bool = bitvector_access(core_ctx.scounteren.bits, index);
        feature_enabled_for_priv(core_ctx, p, var_1, var_2)
    }
}

/// check_Stimecmp
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L46-51.
pub fn check_Stimecmp(core_ctx: &mut Core, csr: BitDynamic, p: Privilege) -> bool {
    if {((csr != BitDynamic::new(12, 0b000101001101)) && (csr != BitDynamic::new(12, 0b000101011101)))} {
        return true;
    } else {
        ()
    };
    ((p == Privilege::Machine) || ((p == Privilege::Supervisor) && (({
        let var_2: Counteren = core_ctx.mcounteren;
        _get_Counteren_TM(var_2)
    } == BitDynamic::new(1, 0b1)) && ({
        let var_1: MEnvcfg = core_ctx.menvcfg;
        _get_MEnvcfg_STCE(var_1)
    } == BitDynamic::new(1, 0b1)))))
}

/// check_seed_CSR
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L56-69.
pub fn check_seed_CSR(csr: BitDynamic, p: Privilege, isWrite: bool) -> bool {
    if {!((csr == BitDynamic::new(12, 0b000000010101)))} {
        true
    } else if {!(isWrite)} {
        false
    } else {
        match p {
            Privilege::Machine => {true}
            Privilege::Supervisor => {false}
            Privilege::User => {false}
            _ => {panic!("Unreachable code")}
        }
    }
}

/// is_CSR_defined
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L133.
pub fn is_CSR_defined(core_ctx: &mut Core, merge_hashtag_var: BitDynamic) -> bool {
    match merge_hashtag_var {
        b__0 if {(b__0 == BitDynamic::new(12, 0b001100000001))} => {true}
        b__1 if {(b__1 == BitDynamic::new(12, 0b001100000000))} => {true}
        b__2 if {(b__2 == BitDynamic::new(12, 0b001100010000))} => {false}
        b__3 if {(b__3 == BitDynamic::new(12, 0b001100001010))} => {currentlyEnabled(core_ctx, extension::Ext_U)}
        b__4 if {(b__4 == BitDynamic::new(12, 0b001100011010))} => {false}
        b__5 if {(b__5 == BitDynamic::new(12, 0b000100001010))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__6 if {(b__6 == BitDynamic::new(12, 0b001100000100))} => {true}
        b__7 if {(b__7 == BitDynamic::new(12, 0b001101000100))} => {true}
        b__8 if {(b__8 == BitDynamic::new(12, 0b001100000010))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__9 if {(b__9 == BitDynamic::new(12, 0b001100010010))} => {false}
        b__10 if {(b__10 == BitDynamic::new(12, 0b001100000011))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__11 if {(b__11 == BitDynamic::new(12, 0b001101000010))} => {true}
        b__12 if {(b__12 == BitDynamic::new(12, 0b001101000011))} => {true}
        b__13 if {(b__13 == BitDynamic::new(12, 0b001101000000))} => {true}
        b__14 if {(b__14 == BitDynamic::new(12, 0b000100000110))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__15 if {(b__15 == BitDynamic::new(12, 0b001100000110))} => {currentlyEnabled(core_ctx, extension::Ext_U)}
        b__16 if {(b__16 == BitDynamic::new(12, 0b001100100000))} => {true}
        b__17 if {(b__17 == BitDynamic::new(12, 0b111100010001))} => {true}
        b__18 if {(b__18 == BitDynamic::new(12, 0b111100010010))} => {true}
        b__19 if {(b__19 == BitDynamic::new(12, 0b111100010011))} => {true}
        b__20 if {(b__20 == BitDynamic::new(12, 0b111100010100))} => {true}
        b__21 if {(b__21 == BitDynamic::new(12, 0b111100010101))} => {true}
        b__22 if {(b__22 == BitDynamic::new(12, 0b000100000000))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__23 if {(b__23 == BitDynamic::new(12, 0b000101000100))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__24 if {(b__24 == BitDynamic::new(12, 0b000100000100))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__25 if {(b__25 == BitDynamic::new(12, 0b000101000000))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__26 if {(b__26 == BitDynamic::new(12, 0b000101000010))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__27 if {(b__27 == BitDynamic::new(12, 0b000101000011))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__28 if {(b__28 == BitDynamic::new(12, 0b011110100000))} => {true}
        v__3688 if {(v__3688.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111010))} => {let idx: BitDynamic = v__3688.subrange::<0, 4, 4>();
        ((sys_pmp_count(core_ctx, ()) > (4 * idx.unsigned())) && (bitvector_access(idx, 0) == false))}
        v__3690 if {(v__3690.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111011))} => {let idx: BitDynamic = v__3690.subrange::<0, 4, 4>();
        (sys_pmp_count(core_ctx, ()) > bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b00)), BitDynamic::from(idx)).unsigned())}
        v__3692 if {(v__3692.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111100))} => {let idx: BitDynamic = v__3692.subrange::<0, 4, 4>();
        (sys_pmp_count(core_ctx, ()) > bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b01)), BitDynamic::from(idx)).unsigned())}
        v__3694 if {(v__3694.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111101))} => {let idx: BitDynamic = v__3694.subrange::<0, 4, 4>();
        (sys_pmp_count(core_ctx, ()) > bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b10)), BitDynamic::from(idx)).unsigned())}
        v__3696 if {(v__3696.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111110))} => {let idx: BitDynamic = v__3696.subrange::<0, 4, 4>();
        (sys_pmp_count(core_ctx, ()) > bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b11)), BitDynamic::from(idx)).unsigned())}
        b__29 if {(b__29 == BitDynamic::new(12, 0b000000001000))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__30 if {(b__30 == BitDynamic::new(12, 0b000000001001))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__31 if {(b__31 == BitDynamic::new(12, 0b000000001010))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__32 if {(b__32 == BitDynamic::new(12, 0b000000001111))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__33 if {(b__33 == BitDynamic::new(12, 0b110000100000))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__34 if {(b__34 == BitDynamic::new(12, 0b110000100001))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__35 if {(b__35 == BitDynamic::new(12, 0b110000100010))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__36 if {(b__36 == BitDynamic::new(12, 0b000100000101))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__37 if {(b__37 == BitDynamic::new(12, 0b000101000001))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__38 if {(b__38 == BitDynamic::new(12, 0b001100000101))} => {true}
        b__39 if {(b__39 == BitDynamic::new(12, 0b001101000001))} => {true}
        v__3698 if {let index_var_1: BitDynamic = v__3698.subrange::<0, 5, 5>();
        ((v__3698.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b0011001)) && ((index_var_1.unsigned() >= 3) as bool))} => {currentlyEnabled(core_ctx, extension::Ext_Zihpm)}
        v__3700 if {let index_var_2: BitDynamic = v__3700.subrange::<0, 5, 5>();
        ((v__3700.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b1011000)) && ((index_var_2.unsigned() >= 3) as bool))} => {currentlyEnabled(core_ctx, extension::Ext_Zihpm)}
        v__3702 if {let index_var_3: BitDynamic = v__3702.subrange::<0, 5, 5>();
        ((v__3702.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b1011100)) && ((index_var_3.unsigned() >= 3) as bool))} => {false}
        v__3704 if {let index_var_4: BitDynamic = v__3704.subrange::<0, 5, 5>();
        ((v__3704.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b1100000)) && ((index_var_4.unsigned() >= 3) as bool))} => {(currentlyEnabled(core_ctx, extension::Ext_Zihpm) && currentlyEnabled(core_ctx, extension::Ext_U))}
        v__3706 if {let index_var_5: BitDynamic = v__3706.subrange::<0, 5, 5>();
        ((v__3706.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b1100100)) && ((index_var_5.unsigned() >= 3) as bool))} => {false}
        v__3708 if {let index_var_6: BitDynamic = v__3708.subrange::<0, 5, 5>();
        ((v__3708.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b0111001)) && ((index_var_6.unsigned() >= 3) as bool))} => {false}
        b__40 if {(b__40 == BitDynamic::new(12, 0b110110100000))} => {(currentlyEnabled(core_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(core_ctx, extension::Ext_S))}
        b__41 if {(b__41 == BitDynamic::new(12, 0b000000010101))} => {currentlyEnabled(core_ctx, extension::Ext_Zkr)}
        b__42 if {(b__42 == BitDynamic::new(12, 0b110000000000))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__43 if {(b__43 == BitDynamic::new(12, 0b110000000001))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__44 if {(b__44 == BitDynamic::new(12, 0b110000000010))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__45 if {(b__45 == BitDynamic::new(12, 0b110010000000))} => {false}
        b__46 if {(b__46 == BitDynamic::new(12, 0b110010000001))} => {false}
        b__47 if {(b__47 == BitDynamic::new(12, 0b110010000010))} => {false}
        b__48 if {(b__48 == BitDynamic::new(12, 0b101100000000))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__49 if {(b__49 == BitDynamic::new(12, 0b101100000010))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__50 if {(b__50 == BitDynamic::new(12, 0b101110000000))} => {false}
        b__51 if {(b__51 == BitDynamic::new(12, 0b101110000010))} => {false}
        b__52 if {(b__52 == BitDynamic::new(12, 0b000000000001))} => {(currentlyEnabled(core_ctx, extension::Ext_F) || currentlyEnabled(core_ctx, extension::Ext_Zfinx))}
        b__53 if {(b__53 == BitDynamic::new(12, 0b000000000010))} => {(currentlyEnabled(core_ctx, extension::Ext_F) || currentlyEnabled(core_ctx, extension::Ext_Zfinx))}
        b__54 if {(b__54 == BitDynamic::new(12, 0b000000000011))} => {(currentlyEnabled(core_ctx, extension::Ext_F) || currentlyEnabled(core_ctx, extension::Ext_Zfinx))}
        b__55 if {(b__55 == BitDynamic::new(12, 0b001100100001))} => {currentlyEnabled(core_ctx, extension::Ext_Smcntrpmf)}
        b__56 if {(b__56 == BitDynamic::new(12, 0b011100100001))} => {false}
        b__57 if {(b__57 == BitDynamic::new(12, 0b001100100010))} => {currentlyEnabled(core_ctx, extension::Ext_Smcntrpmf)}
        b__58 if {(b__58 == BitDynamic::new(12, 0b011100100010))} => {false}
        b__59 if {(b__59 == BitDynamic::new(12, 0b000101001101))} => {(currentlyEnabled(core_ctx, extension::Ext_S) && currentlyEnabled(core_ctx, extension::Ext_Sstc))}
        b__60 if {(b__60 == BitDynamic::new(12, 0b000101011101))} => {false}
        b__61 if {(b__61 == BitDynamic::new(12, 0b000110000000))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// check_CSR
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L71-81.
pub fn check_CSR(core_ctx: &mut Core, csr: BitDynamic, p: Privilege, isWrite: bool) -> bool {
    (is_CSR_defined(core_ctx, csr) && (check_CSR_priv(csr, p) && (check_CSR_access(csr, isWrite) && (check_TVM_SATP(core_ctx, csr, p) && (check_Counteren(core_ctx, csr, p) && (check_Stimecmp(core_ctx, csr, p) && check_seed_CSR(csr, p, isWrite)))))))
}

/// exception_delegatee
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L104-111.
pub fn exception_delegatee(core_ctx: &mut Core, e: ExceptionType, p: Privilege) -> Privilege {
    let idx: i128 = num_of_ExceptionType(e);
    let _super_: bool = {
        let var_1: bool = bitvector_access(core_ctx.medeleg.bits, idx);
        bit_to_bool(var_1)
    };
    let deleg: Privilege = if {(currentlyEnabled(core_ctx, extension::Ext_S) && _super_)} {
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

/// findPendingInterrupt
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L116-125.
pub fn findPendingInterrupt(ip: BitDynamic) -> Option<InterruptType> {
    let ip: Minterrupts = Mk_Minterrupts(ip);
    if {(_get_Minterrupts_MEI(ip) == BitDynamic::new(1, 0b1))} {
        Some(InterruptType::I_M_External)
    } else if {(_get_Minterrupts_MSI(ip) == BitDynamic::new(1, 0b1))} {
        Some(InterruptType::I_M_Software)
    } else if {(_get_Minterrupts_MTI(ip) == BitDynamic::new(1, 0b1))} {
        Some(InterruptType::I_M_Timer)
    } else if {(_get_Minterrupts_SEI(ip) == BitDynamic::new(1, 0b1))} {
        Some(InterruptType::I_S_External)
    } else if {(_get_Minterrupts_SSI(ip) == BitDynamic::new(1, 0b1))} {
        Some(InterruptType::I_S_Software)
    } else if {(_get_Minterrupts_STI(ip) == BitDynamic::new(1, 0b1))} {
        Some(InterruptType::I_S_Timer)
    } else {
        None
    }
}

/// getPendingSet
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L135-148.
pub fn getPendingSet(core_ctx: &mut Core, _priv_: Privilege) -> Option<(BitDynamic, Privilege)> {
    assert!((currentlyEnabled(core_ctx, extension::Ext_S) || (core_ctx.mideleg.bits == zeros(64))), "riscv_sys_control.sail:137.58-137.59");
    let pending_m: BitDynamic = (core_ctx.mip.bits & (core_ctx.mie.bits & !(core_ctx.mideleg.bits)));
    let pending_s: BitDynamic = (core_ctx.mip.bits & (core_ctx.mie.bits & core_ctx.mideleg.bits));
    let mIE: bool = (((_priv_ == Privilege::Machine) && ({
        let var_2: Mstatus = core_ctx.mstatus;
        _get_Mstatus_MIE(var_2)
    } == BitDynamic::new(1, 0b1))) || ((_priv_ == Privilege::Supervisor) || (_priv_ == Privilege::User)));
    let sIE: bool = (((_priv_ == Privilege::Supervisor) && ({
        let var_1: Mstatus = core_ctx.mstatus;
        _get_Mstatus_SIE(var_1)
    } == BitDynamic::new(1, 0b1))) || (_priv_ == Privilege::User));
    if {(mIE && (pending_m != zeros(64)))} {
        Some((pending_m, Privilege::Machine))
    } else if {(sIE && (pending_s != zeros(64)))} {
        Some((pending_s, Privilege::Supervisor))
    } else {
        None
    }
}

/// dispatchInterrupt
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L177-185.
pub fn dispatchInterrupt(core_ctx: &mut Core, _priv_: Privilege) -> Option<(InterruptType, Privilege)> {
    match getPendingSet(core_ctx, _priv_) {
        None => {None}
        Some((ip, p)) => {match findPendingInterrupt(ip) {
            None => {None}
            Some(i) => {Some((i, p))}
            _ => {panic!("Unreachable code")}
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// ctl_result
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L189-193.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ctl_result {
    CTL_TRAP(sync_exception),
    CTL_SRET(()),
    CTL_MRET(())
}

/// tval
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L197-202.
pub fn tval(excinfo: Option<BitDynamic>) -> BitDynamic {
    match excinfo {
        Some(e) => {e}
        None => {zeros(64)}
        _ => {panic!("Unreachable code")}
    }
}

/// track_trap
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L204-219.
pub fn track_trap(core_ctx: &mut Core, p: Privilege) {
    match p {
        Privilege::Machine => {{

        }}
        Privilege::Supervisor => {{

        }}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sys_control.sail", 217, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

/// trap_handler
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L222-275.
pub fn trap_handler(core_ctx: &mut Core, del_priv: Privilege, intr: bool, c: BitDynamic, pc: BitDynamic, info: Option<BitDynamic>, ext: Option<()>) -> BitDynamic {
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
            track_trap(core_ctx, del_priv);
            {
                let var_3: Mcause = core_ctx.mcause;
                prepare_trap_vector(core_ctx, del_priv, var_3)
            }
        }}
        Privilege::Supervisor => {{
            assert!(currentlyEnabled(core_ctx, extension::Ext_S), "no supervisor mode present for delegation");
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
                Privilege::Machine => {panic!("{}, l {}: {}", "riscv_sys_control.sail", 260, "invalid privilege for s-mode trap")}
                _ => {panic!("Unreachable code")}
            }, 8, 8);
            core_ctx.stval = tval(info);
            core_ctx.sepc = pc;
            core_ctx.cur_privilege = del_priv;
            track_trap(core_ctx, del_priv);
            {
                let var_5: Mcause = core_ctx.scause;
                prepare_trap_vector(core_ctx, del_priv, var_5)
            }
        }}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sys_control.sail", 273, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

/// exception_handler
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L277-321.
pub fn exception_handler(core_ctx: &mut Core, cur_priv: Privilege, ctl: ctl_result, pc: BitDynamic) -> BitDynamic {
    match (cur_priv, ctl) {
        (_, ctl_result::CTL_TRAP(e)) => {{
            let del_priv: Privilege = exception_delegatee(core_ctx, e.trap, cur_priv);
            trap_handler(core_ctx, del_priv, false, exceptionType_to_bits(e.trap), pc, e.excinfo, e.ext)
        }}
        (_, ctl_result::CTL_MRET(())) => {{
            let prev_priv: Privilege = core_ctx.cur_privilege;
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange({
                let var_1: Mstatus = core_ctx.mstatus;
                _get_Mstatus_MPIE(var_1)
            }, 3, 3);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b1), 7, 7);
            core_ctx.cur_privilege = {
                let var_2: BitDynamic = {
                    let var_3: Mstatus = core_ctx.mstatus;
                    _get_Mstatus_MPP(var_3)
                };
                privLevel_of_bits(var_2)
            };
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange({
                let var_4: Privilege = if {currentlyEnabled(core_ctx, extension::Ext_U)} {
                    Privilege::User
                } else {
                    Privilege::Machine
                };
                privLevel_to_bits(var_4)
            }, 12, 11);
            if {(core_ctx.cur_privilege != Privilege::Machine)} {
                core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 17, 17)
            } else {
                ()
            };
            prepare_xret_target(core_ctx, Privilege::Machine)
        }}
        (_, ctl_result::CTL_SRET(())) => {{
            let prev_priv: Privilege = core_ctx.cur_privilege;
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange({
                let var_5: Mstatus = core_ctx.mstatus;
                _get_Mstatus_SPIE(var_5)
            }, 1, 1);
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b1), 5, 5);
            core_ctx.cur_privilege = if {({
                let var_6: Mstatus = core_ctx.mstatus;
                _get_Mstatus_SPP(var_6)
            } == BitDynamic::new(1, 0b1))} {
                Privilege::Supervisor
            } else {
                Privilege::User
            };
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 8, 8);
            if {(core_ctx.cur_privilege != Privilege::Machine)} {
                core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 17, 17)
            } else {
                ()
            };
            prepare_xret_target(core_ctx, Privilege::Supervisor)
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// handle_interrupt
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L337-338.
pub fn handle_interrupt(core_ctx: &mut Core, i: InterruptType, del_priv: Privilege) {
    let var_1: BitDynamic = {
        let var_2: BitDynamic = core_ctx.PC;
        trap_handler(core_ctx, del_priv, true, interruptType_to_bits(i), var_2, None, None)
    };
    set_next_pc(core_ctx, var_1)
}

/// reset_misa
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L341-360.
pub fn reset_misa(core_ctx: &mut Core, unit_arg: ()) {
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_1: bool = hartSupports(core_ctx, extension::Ext_A);
        bool_to_bits(var_1)
    }, 0, 0);
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_2: bool = hartSupports(core_ctx, extension::Ext_C);
        bool_to_bits(var_2)
    }, 2, 2);
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_3: bool = hartSupports(core_ctx, extension::Ext_B);
        bool_to_bits(var_3)
    }, 1, 1);
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange(BitDynamic::new(1, 0b1), 8, 8);
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_4: bool = hartSupports(core_ctx, extension::Ext_M);
        bool_to_bits(var_4)
    }, 12, 12);
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_5: bool = hartSupports(core_ctx, extension::Ext_U);
        bool_to_bits(var_5)
    }, 20, 20);
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_6: bool = hartSupports(core_ctx, extension::Ext_S);
        bool_to_bits(var_6)
    }, 18, 18);
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_7: bool = hartSupports(core_ctx, extension::Ext_V);
        bool_to_bits(var_7)
    }, 21, 21);
    if {(hartSupports(core_ctx, extension::Ext_F) && hartSupports(core_ctx, extension::Ext_Zfinx))} {
        panic!("{}, l {}: {}", "riscv_sys_control.sail", 352, "F and Zfinx cannot both be enabled!")
    } else {
        ()
    };
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_8: bool = hartSupports(core_ctx, extension::Ext_F);
        bool_to_bits(var_8)
    }, 5, 5);
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange({
        let var_9: bool = hartSupports(core_ctx, extension::Ext_D);
        bool_to_bits(var_9)
    }, 3, 3)
}

/// reset_sys
///
/// Generated from the Sail sources at `riscv_sys_control.sail` L364-417.
pub fn reset_sys(core_ctx: &mut Core, unit_arg: ()) {
    core_ctx.cur_privilege = Privilege::Machine;
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 3, 3);
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange(BitDynamic::new(1, 0b0), 17, 17);
    reset_misa(core_ctx, ());
    cancel_reservation(());
    core_ctx.mcause.bits = zeros(64);
    reset_pmp(core_ctx, ());
    core_ctx.vstart = zeros(16);
    core_ctx.vl = zeros(64);
    core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange(BitDynamic::new(2, 0b00), 2, 1);
    core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange(BitDynamic::new(1, 0b0), 0, 0);
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange(BitDynamic::new(1, 0b1), 63, 63);
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange(zeros(55), 62, 8);
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange(BitDynamic::new(1, 0b0), 7, 7);
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange(BitDynamic::new(1, 0b0), 6, 6);
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange(BitDynamic::new(3, 0b000), 5, 3);
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange(BitDynamic::new(3, 0b000), 2, 0)
}

pub type MemoryOpResult<A> = result::<A, ExceptionType>;

/// plat_cache_block_size_exp
///
/// Generated from the Sail sources at `riscv_platform.sail` L31.
pub fn plat_cache_block_size_exp(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    core_ctx.config.platform.cache_block_size_exp
}

/// plat_mtval_has_illegal_inst_bits
///
/// Generated from the Sail sources at `riscv_platform.sail` L46.
pub fn plat_mtval_has_illegal_inst_bits(core_ctx: &mut Core, unit_arg: ()) -> bool {
    core_ctx.config.base.mtval_has_illegal_instruction_bits
}

/// MSIP_BASE
///
/// Generated from the Sail sources at `riscv_platform.sail` L139.
pub const MSIP_BASE: physaddrbits = BitDynamic::new(20, 0b00000000000000000000).zero_extend_dyn(64);

/// MTIMECMP_BASE
///
/// Generated from the Sail sources at `riscv_platform.sail` L140.
pub const MTIMECMP_BASE: physaddrbits = BitDynamic::new(20, 0b00000100000000000000).zero_extend_dyn(64);

/// MTIMECMP_BASE_HI
///
/// Generated from the Sail sources at `riscv_platform.sail` L141.
pub const MTIMECMP_BASE_HI: physaddrbits = BitDynamic::new(20, 0b00000100000000000100).zero_extend_dyn(64);

/// MTIME_BASE
///
/// Generated from the Sail sources at `riscv_platform.sail` L142.
pub const MTIME_BASE: physaddrbits = BitDynamic::new(20, 0b00001011111111111000).zero_extend_dyn(64);

/// MTIME_BASE_HI
///
/// Generated from the Sail sources at `riscv_platform.sail` L143.
pub const MTIME_BASE_HI: physaddrbits = BitDynamic::new(20, 0b00001011111111111100).zero_extend_dyn(64);

/// handle_illegal
///
/// Generated from the Sail sources at `riscv_platform.sail` L448-456.
pub fn handle_illegal(core_ctx: &mut Core, instbits: BitDynamic) {
    let info: Option<BitDynamic> = if {plat_mtval_has_illegal_inst_bits(core_ctx, ())} {
        Some(instbits.zero_extend_dyn(64))
    } else {
        None
    };
    let t: sync_exception = sync_exception {
        trap: ExceptionType::E_Illegal_Instr(()),
        excinfo: info,
        ext: None
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

/// phys_access_check
///
/// Generated from the Sail sources at `riscv_mem.sail` L99-103.
pub fn phys_access_check(core_ctx: &mut Core, t: AccessType::<()>, p: Privilege, paddr: physaddr, width: i128) -> Option<ExceptionType> {
    let pmpError: Option<ExceptionType> = if {(sys_pmp_count(core_ctx, ()) == 0)} {
        None
    } else {
        pmpCheck(core_ctx, paddr, width, t, p)
    };
    pmpError
}

/// ExecutionResult
///
/// Generated from the Sail sources at `riscv_inst_retire.sail` L10-28.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExecutionResult {
    Retire_Success(()),
    Wait_For_Interrupt(()),
    Illegal_Instruction(()),
    Trap((Privilege, ctl_result, xlenbits)),
    Memory_Exception((virtaddr, ExceptionType)),
    Ext_CSR_Check_Failure(()),
    Ext_ControlAddr_Check_Failure(ext_control_addr_error),
    Ext_DataAddr_Check_Failure(ext_data_addr_error),
    Ext_XRET_Priv_Failure(())
}

/// RETIRE_SUCCESS
///
/// Generated from the Sail sources at `riscv_inst_retire.sail` L35.
pub const RETIRE_SUCCESS: ExecutionResult = ExecutionResult::Retire_Success(());

pub type pte_flags_bits = BitDynamic;

pub type pte_ext_bits = BitDynamic;

/// default_sv32_ext_pte
///
/// Generated from the Sail sources at `riscv_vmem_pte.sail` L42.
pub const default_sv32_ext_pte: pte_ext_bits = zeros(10);

/// PTE_Check
///
/// Generated from the Sail sources at `riscv_vmem_pte.sail` L94-97.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PTE_Check {
    PTE_Check_Success(ext_ptw),
    PTE_Check_Failure((ext_ptw, ext_ptw_fail))
}

/// tlb_vpn_bits
///
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L18.
pub const tlb_vpn_bits: i128 = 45;

/// tlb_ppn_bits
///
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L20.
pub const tlb_ppn_bits: i128 = 44;

/// TLB_Entry
///
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L25-33.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TLB_Entry {
    pub asid: asidbits,
    pub global: bool,
    pub vpn: BitDynamic,
    pub levelMask: BitDynamic,
    pub ppn: BitDynamic,
    pub pte: BitDynamic,
    pub pteAddr: physaddr,
}

/// num_tlb_entries
///
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L69.
pub const num_tlb_entries: i128 = 64;

pub type tlb_index_range = i128;

/// flush_TLB_Entry
///
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L98-113.
pub fn flush_TLB_Entry(ent: TLB_Entry, asid: Option<BitDynamic>, vaddr: Option<BitDynamic>) -> bool {
    let asid_matches: bool = match asid {
        Some(asid) => {((ent.asid == asid) && !(ent.global))}
        None => {true}
        _ => {panic!("Unreachable code")}
    };
    let addr_matches: bool = match vaddr {
        Some(vaddr) => {{
            let vaddr: BitDynamic = sign_extend(64, vaddr);
            (ent.vpn == (vaddr.subrange::<12, 57, 45>() & !(ent.levelMask)))
        }}
        None => {true}
        _ => {panic!("Unreachable code")}
    };
    (asid_matches && addr_matches)
}

/// flush_TLB
///
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L165-173.
pub fn flush_TLB(core_ctx: &mut Core, asid: Option<BitDynamic>, addr: Option<BitDynamic>) {
    for i in 0..=(vector_length(&core_ctx.tlb) - 1) {
        match core_ctx.tlb[(i as usize)] {
            None => {()}
            Some(entry) => {if {flush_TLB_Entry(entry, asid, addr)} {
                core_ctx.tlb[(i as usize)] = None
            } else {
                ()
            }}
            _ => {panic!("Unreachable code")}
        }
    }
}

/// TR_Result
///
/// Generated from the Sail sources at `riscv_vmem.sail` L212-215.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TR_Result<PADDR, FAILURE> {
    TR_Address((PADDR, ext_ptw)),
    TR_Failure((FAILURE, ext_ptw))
}

/// sm4_sbox_table
///
/// Generated from the Sail sources at `riscv_types_kext.sail` L104-125.
pub const sm4_sbox_table: [BitDynamic; (256 as usize)] = [BitDynamic::new(8, 0b11010110), BitDynamic::new(8, 0b10010000), BitDynamic::new(8, 0b11101001), BitDynamic::new(8, 0b11111110), BitDynamic::new(8, 0b11001100), BitDynamic::new(8, 0b11100001), BitDynamic::new(8, 0b00111101), BitDynamic::new(8, 0b10110111), BitDynamic::new(8, 0b00010110), BitDynamic::new(8, 0b10110110), BitDynamic::new(8, 0b00010100), BitDynamic::new(8, 0b11000010), BitDynamic::new(8, 0b00101000), BitDynamic::new(8, 0b11111011), BitDynamic::new(8, 0b00101100), BitDynamic::new(8, 0b00000101), BitDynamic::new(8, 0b00101011), BitDynamic::new(8, 0b01100111), BitDynamic::new(8, 0b10011010), BitDynamic::new(8, 0b01110110), BitDynamic::new(8, 0b00101010), BitDynamic::new(8, 0b10111110), BitDynamic::new(8, 0b00000100), BitDynamic::new(8, 0b11000011), BitDynamic::new(8, 0b10101010), BitDynamic::new(8, 0b01000100), BitDynamic::new(8, 0b00010011), BitDynamic::new(8, 0b00100110), BitDynamic::new(8, 0b01001001), BitDynamic::new(8, 0b10000110), BitDynamic::new(8, 0b00000110), BitDynamic::new(8, 0b10011001), BitDynamic::new(8, 0b10011100), BitDynamic::new(8, 0b01000010), BitDynamic::new(8, 0b01010000), BitDynamic::new(8, 0b11110100), BitDynamic::new(8, 0b10010001), BitDynamic::new(8, 0b11101111), BitDynamic::new(8, 0b10011000), BitDynamic::new(8, 0b01111010), BitDynamic::new(8, 0b00110011), BitDynamic::new(8, 0b01010100), BitDynamic::new(8, 0b00001011), BitDynamic::new(8, 0b01000011), BitDynamic::new(8, 0b11101101), BitDynamic::new(8, 0b11001111), BitDynamic::new(8, 0b10101100), BitDynamic::new(8, 0b01100010), BitDynamic::new(8, 0b11100100), BitDynamic::new(8, 0b10110011), BitDynamic::new(8, 0b00011100), BitDynamic::new(8, 0b10101001), BitDynamic::new(8, 0b11001001), BitDynamic::new(8, 0b00001000), BitDynamic::new(8, 0b11101000), BitDynamic::new(8, 0b10010101), BitDynamic::new(8, 0b10000000), BitDynamic::new(8, 0b11011111), BitDynamic::new(8, 0b10010100), BitDynamic::new(8, 0b11111010), BitDynamic::new(8, 0b01110101), BitDynamic::new(8, 0b10001111), BitDynamic::new(8, 0b00111111), BitDynamic::new(8, 0b10100110), BitDynamic::new(8, 0b01000111), BitDynamic::new(8, 0b00000111), BitDynamic::new(8, 0b10100111), BitDynamic::new(8, 0b11111100), BitDynamic::new(8, 0b11110011), BitDynamic::new(8, 0b01110011), BitDynamic::new(8, 0b00010111), BitDynamic::new(8, 0b10111010), BitDynamic::new(8, 0b10000011), BitDynamic::new(8, 0b01011001), BitDynamic::new(8, 0b00111100), BitDynamic::new(8, 0b00011001), BitDynamic::new(8, 0b11100110), BitDynamic::new(8, 0b10000101), BitDynamic::new(8, 0b01001111), BitDynamic::new(8, 0b10101000), BitDynamic::new(8, 0b01101000), BitDynamic::new(8, 0b01101011), BitDynamic::new(8, 0b10000001), BitDynamic::new(8, 0b10110010), BitDynamic::new(8, 0b01110001), BitDynamic::new(8, 0b01100100), BitDynamic::new(8, 0b11011010), BitDynamic::new(8, 0b10001011), BitDynamic::new(8, 0b11111000), BitDynamic::new(8, 0b11101011), BitDynamic::new(8, 0b00001111), BitDynamic::new(8, 0b01001011), BitDynamic::new(8, 0b01110000), BitDynamic::new(8, 0b01010110), BitDynamic::new(8, 0b10011101), BitDynamic::new(8, 0b00110101), BitDynamic::new(8, 0b00011110), BitDynamic::new(8, 0b00100100), BitDynamic::new(8, 0b00001110), BitDynamic::new(8, 0b01011110), BitDynamic::new(8, 0b01100011), BitDynamic::new(8, 0b01011000), BitDynamic::new(8, 0b11010001), BitDynamic::new(8, 0b10100010), BitDynamic::new(8, 0b00100101), BitDynamic::new(8, 0b00100010), BitDynamic::new(8, 0b01111100), BitDynamic::new(8, 0b00111011), BitDynamic::new(8, 0b00000001), BitDynamic::new(8, 0b00100001), BitDynamic::new(8, 0b01111000), BitDynamic::new(8, 0b10000111), BitDynamic::new(8, 0b11010100), BitDynamic::new(8, 0b00000000), BitDynamic::new(8, 0b01000110), BitDynamic::new(8, 0b01010111), BitDynamic::new(8, 0b10011111), BitDynamic::new(8, 0b11010011), BitDynamic::new(8, 0b00100111), BitDynamic::new(8, 0b01010010), BitDynamic::new(8, 0b01001100), BitDynamic::new(8, 0b00110110), BitDynamic::new(8, 0b00000010), BitDynamic::new(8, 0b11100111), BitDynamic::new(8, 0b10100000), BitDynamic::new(8, 0b11000100), BitDynamic::new(8, 0b11001000), BitDynamic::new(8, 0b10011110), BitDynamic::new(8, 0b11101010), BitDynamic::new(8, 0b10111111), BitDynamic::new(8, 0b10001010), BitDynamic::new(8, 0b11010010), BitDynamic::new(8, 0b01000000), BitDynamic::new(8, 0b11000111), BitDynamic::new(8, 0b00111000), BitDynamic::new(8, 0b10110101), BitDynamic::new(8, 0b10100011), BitDynamic::new(8, 0b11110111), BitDynamic::new(8, 0b11110010), BitDynamic::new(8, 0b11001110), BitDynamic::new(8, 0b11111001), BitDynamic::new(8, 0b01100001), BitDynamic::new(8, 0b00010101), BitDynamic::new(8, 0b10100001), BitDynamic::new(8, 0b11100000), BitDynamic::new(8, 0b10101110), BitDynamic::new(8, 0b01011101), BitDynamic::new(8, 0b10100100), BitDynamic::new(8, 0b10011011), BitDynamic::new(8, 0b00110100), BitDynamic::new(8, 0b00011010), BitDynamic::new(8, 0b01010101), BitDynamic::new(8, 0b10101101), BitDynamic::new(8, 0b10010011), BitDynamic::new(8, 0b00110010), BitDynamic::new(8, 0b00110000), BitDynamic::new(8, 0b11110101), BitDynamic::new(8, 0b10001100), BitDynamic::new(8, 0b10110001), BitDynamic::new(8, 0b11100011), BitDynamic::new(8, 0b00011101), BitDynamic::new(8, 0b11110110), BitDynamic::new(8, 0b11100010), BitDynamic::new(8, 0b00101110), BitDynamic::new(8, 0b10000010), BitDynamic::new(8, 0b01100110), BitDynamic::new(8, 0b11001010), BitDynamic::new(8, 0b01100000), BitDynamic::new(8, 0b11000000), BitDynamic::new(8, 0b00101001), BitDynamic::new(8, 0b00100011), BitDynamic::new(8, 0b10101011), BitDynamic::new(8, 0b00001101), BitDynamic::new(8, 0b01010011), BitDynamic::new(8, 0b01001110), BitDynamic::new(8, 0b01101111), BitDynamic::new(8, 0b11010101), BitDynamic::new(8, 0b11011011), BitDynamic::new(8, 0b00110111), BitDynamic::new(8, 0b01000101), BitDynamic::new(8, 0b11011110), BitDynamic::new(8, 0b11111101), BitDynamic::new(8, 0b10001110), BitDynamic::new(8, 0b00101111), BitDynamic::new(8, 0b00000011), BitDynamic::new(8, 0b11111111), BitDynamic::new(8, 0b01101010), BitDynamic::new(8, 0b01110010), BitDynamic::new(8, 0b01101101), BitDynamic::new(8, 0b01101100), BitDynamic::new(8, 0b01011011), BitDynamic::new(8, 0b01010001), BitDynamic::new(8, 0b10001101), BitDynamic::new(8, 0b00011011), BitDynamic::new(8, 0b10101111), BitDynamic::new(8, 0b10010010), BitDynamic::new(8, 0b10111011), BitDynamic::new(8, 0b11011101), BitDynamic::new(8, 0b10111100), BitDynamic::new(8, 0b01111111), BitDynamic::new(8, 0b00010001), BitDynamic::new(8, 0b11011001), BitDynamic::new(8, 0b01011100), BitDynamic::new(8, 0b01000001), BitDynamic::new(8, 0b00011111), BitDynamic::new(8, 0b00010000), BitDynamic::new(8, 0b01011010), BitDynamic::new(8, 0b11011000), BitDynamic::new(8, 0b00001010), BitDynamic::new(8, 0b11000001), BitDynamic::new(8, 0b00110001), BitDynamic::new(8, 0b10001000), BitDynamic::new(8, 0b10100101), BitDynamic::new(8, 0b11001101), BitDynamic::new(8, 0b01111011), BitDynamic::new(8, 0b10111101), BitDynamic::new(8, 0b00101101), BitDynamic::new(8, 0b01110100), BitDynamic::new(8, 0b11010000), BitDynamic::new(8, 0b00010010), BitDynamic::new(8, 0b10111000), BitDynamic::new(8, 0b11100101), BitDynamic::new(8, 0b10110100), BitDynamic::new(8, 0b10110000), BitDynamic::new(8, 0b10001001), BitDynamic::new(8, 0b01101001), BitDynamic::new(8, 0b10010111), BitDynamic::new(8, 0b01001010), BitDynamic::new(8, 0b00001100), BitDynamic::new(8, 0b10010110), BitDynamic::new(8, 0b01110111), BitDynamic::new(8, 0b01111110), BitDynamic::new(8, 0b01100101), BitDynamic::new(8, 0b10111001), BitDynamic::new(8, 0b11110001), BitDynamic::new(8, 0b00001001), BitDynamic::new(8, 0b11000101), BitDynamic::new(8, 0b01101110), BitDynamic::new(8, 0b11000110), BitDynamic::new(8, 0b10000100), BitDynamic::new(8, 0b00011000), BitDynamic::new(8, 0b11110000), BitDynamic::new(8, 0b01111101), BitDynamic::new(8, 0b11101100), BitDynamic::new(8, 0b00111010), BitDynamic::new(8, 0b11011100), BitDynamic::new(8, 0b01001101), BitDynamic::new(8, 0b00100000), BitDynamic::new(8, 0b01111001), BitDynamic::new(8, 0b11101110), BitDynamic::new(8, 0b01011111), BitDynamic::new(8, 0b00111110), BitDynamic::new(8, 0b11010111), BitDynamic::new(8, 0b11001011), BitDynamic::new(8, 0b00111001), BitDynamic::new(8, 0b01001000)];

/// aes_sbox_fwd_table
///
/// Generated from the Sail sources at `riscv_types_kext.sail` L127-148.
pub const aes_sbox_fwd_table: [BitDynamic; (256 as usize)] = [BitDynamic::new(8, 0b01100011), BitDynamic::new(8, 0b01111100), BitDynamic::new(8, 0b01110111), BitDynamic::new(8, 0b01111011), BitDynamic::new(8, 0b11110010), BitDynamic::new(8, 0b01101011), BitDynamic::new(8, 0b01101111), BitDynamic::new(8, 0b11000101), BitDynamic::new(8, 0b00110000), BitDynamic::new(8, 0b00000001), BitDynamic::new(8, 0b01100111), BitDynamic::new(8, 0b00101011), BitDynamic::new(8, 0b11111110), BitDynamic::new(8, 0b11010111), BitDynamic::new(8, 0b10101011), BitDynamic::new(8, 0b01110110), BitDynamic::new(8, 0b11001010), BitDynamic::new(8, 0b10000010), BitDynamic::new(8, 0b11001001), BitDynamic::new(8, 0b01111101), BitDynamic::new(8, 0b11111010), BitDynamic::new(8, 0b01011001), BitDynamic::new(8, 0b01000111), BitDynamic::new(8, 0b11110000), BitDynamic::new(8, 0b10101101), BitDynamic::new(8, 0b11010100), BitDynamic::new(8, 0b10100010), BitDynamic::new(8, 0b10101111), BitDynamic::new(8, 0b10011100), BitDynamic::new(8, 0b10100100), BitDynamic::new(8, 0b01110010), BitDynamic::new(8, 0b11000000), BitDynamic::new(8, 0b10110111), BitDynamic::new(8, 0b11111101), BitDynamic::new(8, 0b10010011), BitDynamic::new(8, 0b00100110), BitDynamic::new(8, 0b00110110), BitDynamic::new(8, 0b00111111), BitDynamic::new(8, 0b11110111), BitDynamic::new(8, 0b11001100), BitDynamic::new(8, 0b00110100), BitDynamic::new(8, 0b10100101), BitDynamic::new(8, 0b11100101), BitDynamic::new(8, 0b11110001), BitDynamic::new(8, 0b01110001), BitDynamic::new(8, 0b11011000), BitDynamic::new(8, 0b00110001), BitDynamic::new(8, 0b00010101), BitDynamic::new(8, 0b00000100), BitDynamic::new(8, 0b11000111), BitDynamic::new(8, 0b00100011), BitDynamic::new(8, 0b11000011), BitDynamic::new(8, 0b00011000), BitDynamic::new(8, 0b10010110), BitDynamic::new(8, 0b00000101), BitDynamic::new(8, 0b10011010), BitDynamic::new(8, 0b00000111), BitDynamic::new(8, 0b00010010), BitDynamic::new(8, 0b10000000), BitDynamic::new(8, 0b11100010), BitDynamic::new(8, 0b11101011), BitDynamic::new(8, 0b00100111), BitDynamic::new(8, 0b10110010), BitDynamic::new(8, 0b01110101), BitDynamic::new(8, 0b00001001), BitDynamic::new(8, 0b10000011), BitDynamic::new(8, 0b00101100), BitDynamic::new(8, 0b00011010), BitDynamic::new(8, 0b00011011), BitDynamic::new(8, 0b01101110), BitDynamic::new(8, 0b01011010), BitDynamic::new(8, 0b10100000), BitDynamic::new(8, 0b01010010), BitDynamic::new(8, 0b00111011), BitDynamic::new(8, 0b11010110), BitDynamic::new(8, 0b10110011), BitDynamic::new(8, 0b00101001), BitDynamic::new(8, 0b11100011), BitDynamic::new(8, 0b00101111), BitDynamic::new(8, 0b10000100), BitDynamic::new(8, 0b01010011), BitDynamic::new(8, 0b11010001), BitDynamic::new(8, 0b00000000), BitDynamic::new(8, 0b11101101), BitDynamic::new(8, 0b00100000), BitDynamic::new(8, 0b11111100), BitDynamic::new(8, 0b10110001), BitDynamic::new(8, 0b01011011), BitDynamic::new(8, 0b01101010), BitDynamic::new(8, 0b11001011), BitDynamic::new(8, 0b10111110), BitDynamic::new(8, 0b00111001), BitDynamic::new(8, 0b01001010), BitDynamic::new(8, 0b01001100), BitDynamic::new(8, 0b01011000), BitDynamic::new(8, 0b11001111), BitDynamic::new(8, 0b11010000), BitDynamic::new(8, 0b11101111), BitDynamic::new(8, 0b10101010), BitDynamic::new(8, 0b11111011), BitDynamic::new(8, 0b01000011), BitDynamic::new(8, 0b01001101), BitDynamic::new(8, 0b00110011), BitDynamic::new(8, 0b10000101), BitDynamic::new(8, 0b01000101), BitDynamic::new(8, 0b11111001), BitDynamic::new(8, 0b00000010), BitDynamic::new(8, 0b01111111), BitDynamic::new(8, 0b01010000), BitDynamic::new(8, 0b00111100), BitDynamic::new(8, 0b10011111), BitDynamic::new(8, 0b10101000), BitDynamic::new(8, 0b01010001), BitDynamic::new(8, 0b10100011), BitDynamic::new(8, 0b01000000), BitDynamic::new(8, 0b10001111), BitDynamic::new(8, 0b10010010), BitDynamic::new(8, 0b10011101), BitDynamic::new(8, 0b00111000), BitDynamic::new(8, 0b11110101), BitDynamic::new(8, 0b10111100), BitDynamic::new(8, 0b10110110), BitDynamic::new(8, 0b11011010), BitDynamic::new(8, 0b00100001), BitDynamic::new(8, 0b00010000), BitDynamic::new(8, 0b11111111), BitDynamic::new(8, 0b11110011), BitDynamic::new(8, 0b11010010), BitDynamic::new(8, 0b11001101), BitDynamic::new(8, 0b00001100), BitDynamic::new(8, 0b00010011), BitDynamic::new(8, 0b11101100), BitDynamic::new(8, 0b01011111), BitDynamic::new(8, 0b10010111), BitDynamic::new(8, 0b01000100), BitDynamic::new(8, 0b00010111), BitDynamic::new(8, 0b11000100), BitDynamic::new(8, 0b10100111), BitDynamic::new(8, 0b01111110), BitDynamic::new(8, 0b00111101), BitDynamic::new(8, 0b01100100), BitDynamic::new(8, 0b01011101), BitDynamic::new(8, 0b00011001), BitDynamic::new(8, 0b01110011), BitDynamic::new(8, 0b01100000), BitDynamic::new(8, 0b10000001), BitDynamic::new(8, 0b01001111), BitDynamic::new(8, 0b11011100), BitDynamic::new(8, 0b00100010), BitDynamic::new(8, 0b00101010), BitDynamic::new(8, 0b10010000), BitDynamic::new(8, 0b10001000), BitDynamic::new(8, 0b01000110), BitDynamic::new(8, 0b11101110), BitDynamic::new(8, 0b10111000), BitDynamic::new(8, 0b00010100), BitDynamic::new(8, 0b11011110), BitDynamic::new(8, 0b01011110), BitDynamic::new(8, 0b00001011), BitDynamic::new(8, 0b11011011), BitDynamic::new(8, 0b11100000), BitDynamic::new(8, 0b00110010), BitDynamic::new(8, 0b00111010), BitDynamic::new(8, 0b00001010), BitDynamic::new(8, 0b01001001), BitDynamic::new(8, 0b00000110), BitDynamic::new(8, 0b00100100), BitDynamic::new(8, 0b01011100), BitDynamic::new(8, 0b11000010), BitDynamic::new(8, 0b11010011), BitDynamic::new(8, 0b10101100), BitDynamic::new(8, 0b01100010), BitDynamic::new(8, 0b10010001), BitDynamic::new(8, 0b10010101), BitDynamic::new(8, 0b11100100), BitDynamic::new(8, 0b01111001), BitDynamic::new(8, 0b11100111), BitDynamic::new(8, 0b11001000), BitDynamic::new(8, 0b00110111), BitDynamic::new(8, 0b01101101), BitDynamic::new(8, 0b10001101), BitDynamic::new(8, 0b11010101), BitDynamic::new(8, 0b01001110), BitDynamic::new(8, 0b10101001), BitDynamic::new(8, 0b01101100), BitDynamic::new(8, 0b01010110), BitDynamic::new(8, 0b11110100), BitDynamic::new(8, 0b11101010), BitDynamic::new(8, 0b01100101), BitDynamic::new(8, 0b01111010), BitDynamic::new(8, 0b10101110), BitDynamic::new(8, 0b00001000), BitDynamic::new(8, 0b10111010), BitDynamic::new(8, 0b01111000), BitDynamic::new(8, 0b00100101), BitDynamic::new(8, 0b00101110), BitDynamic::new(8, 0b00011100), BitDynamic::new(8, 0b10100110), BitDynamic::new(8, 0b10110100), BitDynamic::new(8, 0b11000110), BitDynamic::new(8, 0b11101000), BitDynamic::new(8, 0b11011101), BitDynamic::new(8, 0b01110100), BitDynamic::new(8, 0b00011111), BitDynamic::new(8, 0b01001011), BitDynamic::new(8, 0b10111101), BitDynamic::new(8, 0b10001011), BitDynamic::new(8, 0b10001010), BitDynamic::new(8, 0b01110000), BitDynamic::new(8, 0b00111110), BitDynamic::new(8, 0b10110101), BitDynamic::new(8, 0b01100110), BitDynamic::new(8, 0b01001000), BitDynamic::new(8, 0b00000011), BitDynamic::new(8, 0b11110110), BitDynamic::new(8, 0b00001110), BitDynamic::new(8, 0b01100001), BitDynamic::new(8, 0b00110101), BitDynamic::new(8, 0b01010111), BitDynamic::new(8, 0b10111001), BitDynamic::new(8, 0b10000110), BitDynamic::new(8, 0b11000001), BitDynamic::new(8, 0b00011101), BitDynamic::new(8, 0b10011110), BitDynamic::new(8, 0b11100001), BitDynamic::new(8, 0b11111000), BitDynamic::new(8, 0b10011000), BitDynamic::new(8, 0b00010001), BitDynamic::new(8, 0b01101001), BitDynamic::new(8, 0b11011001), BitDynamic::new(8, 0b10001110), BitDynamic::new(8, 0b10010100), BitDynamic::new(8, 0b10011011), BitDynamic::new(8, 0b00011110), BitDynamic::new(8, 0b10000111), BitDynamic::new(8, 0b11101001), BitDynamic::new(8, 0b11001110), BitDynamic::new(8, 0b01010101), BitDynamic::new(8, 0b00101000), BitDynamic::new(8, 0b11011111), BitDynamic::new(8, 0b10001100), BitDynamic::new(8, 0b10100001), BitDynamic::new(8, 0b10001001), BitDynamic::new(8, 0b00001101), BitDynamic::new(8, 0b10111111), BitDynamic::new(8, 0b11100110), BitDynamic::new(8, 0b01000010), BitDynamic::new(8, 0b01101000), BitDynamic::new(8, 0b01000001), BitDynamic::new(8, 0b10011001), BitDynamic::new(8, 0b00101101), BitDynamic::new(8, 0b00001111), BitDynamic::new(8, 0b10110000), BitDynamic::new(8, 0b01010100), BitDynamic::new(8, 0b10111011), BitDynamic::new(8, 0b00010110)];

/// aes_sbox_inv_table
///
/// Generated from the Sail sources at `riscv_types_kext.sail` L150-171.
pub const aes_sbox_inv_table: [BitDynamic; (256 as usize)] = [BitDynamic::new(8, 0b01010010), BitDynamic::new(8, 0b00001001), BitDynamic::new(8, 0b01101010), BitDynamic::new(8, 0b11010101), BitDynamic::new(8, 0b00110000), BitDynamic::new(8, 0b00110110), BitDynamic::new(8, 0b10100101), BitDynamic::new(8, 0b00111000), BitDynamic::new(8, 0b10111111), BitDynamic::new(8, 0b01000000), BitDynamic::new(8, 0b10100011), BitDynamic::new(8, 0b10011110), BitDynamic::new(8, 0b10000001), BitDynamic::new(8, 0b11110011), BitDynamic::new(8, 0b11010111), BitDynamic::new(8, 0b11111011), BitDynamic::new(8, 0b01111100), BitDynamic::new(8, 0b11100011), BitDynamic::new(8, 0b00111001), BitDynamic::new(8, 0b10000010), BitDynamic::new(8, 0b10011011), BitDynamic::new(8, 0b00101111), BitDynamic::new(8, 0b11111111), BitDynamic::new(8, 0b10000111), BitDynamic::new(8, 0b00110100), BitDynamic::new(8, 0b10001110), BitDynamic::new(8, 0b01000011), BitDynamic::new(8, 0b01000100), BitDynamic::new(8, 0b11000100), BitDynamic::new(8, 0b11011110), BitDynamic::new(8, 0b11101001), BitDynamic::new(8, 0b11001011), BitDynamic::new(8, 0b01010100), BitDynamic::new(8, 0b01111011), BitDynamic::new(8, 0b10010100), BitDynamic::new(8, 0b00110010), BitDynamic::new(8, 0b10100110), BitDynamic::new(8, 0b11000010), BitDynamic::new(8, 0b00100011), BitDynamic::new(8, 0b00111101), BitDynamic::new(8, 0b11101110), BitDynamic::new(8, 0b01001100), BitDynamic::new(8, 0b10010101), BitDynamic::new(8, 0b00001011), BitDynamic::new(8, 0b01000010), BitDynamic::new(8, 0b11111010), BitDynamic::new(8, 0b11000011), BitDynamic::new(8, 0b01001110), BitDynamic::new(8, 0b00001000), BitDynamic::new(8, 0b00101110), BitDynamic::new(8, 0b10100001), BitDynamic::new(8, 0b01100110), BitDynamic::new(8, 0b00101000), BitDynamic::new(8, 0b11011001), BitDynamic::new(8, 0b00100100), BitDynamic::new(8, 0b10110010), BitDynamic::new(8, 0b01110110), BitDynamic::new(8, 0b01011011), BitDynamic::new(8, 0b10100010), BitDynamic::new(8, 0b01001001), BitDynamic::new(8, 0b01101101), BitDynamic::new(8, 0b10001011), BitDynamic::new(8, 0b11010001), BitDynamic::new(8, 0b00100101), BitDynamic::new(8, 0b01110010), BitDynamic::new(8, 0b11111000), BitDynamic::new(8, 0b11110110), BitDynamic::new(8, 0b01100100), BitDynamic::new(8, 0b10000110), BitDynamic::new(8, 0b01101000), BitDynamic::new(8, 0b10011000), BitDynamic::new(8, 0b00010110), BitDynamic::new(8, 0b11010100), BitDynamic::new(8, 0b10100100), BitDynamic::new(8, 0b01011100), BitDynamic::new(8, 0b11001100), BitDynamic::new(8, 0b01011101), BitDynamic::new(8, 0b01100101), BitDynamic::new(8, 0b10110110), BitDynamic::new(8, 0b10010010), BitDynamic::new(8, 0b01101100), BitDynamic::new(8, 0b01110000), BitDynamic::new(8, 0b01001000), BitDynamic::new(8, 0b01010000), BitDynamic::new(8, 0b11111101), BitDynamic::new(8, 0b11101101), BitDynamic::new(8, 0b10111001), BitDynamic::new(8, 0b11011010), BitDynamic::new(8, 0b01011110), BitDynamic::new(8, 0b00010101), BitDynamic::new(8, 0b01000110), BitDynamic::new(8, 0b01010111), BitDynamic::new(8, 0b10100111), BitDynamic::new(8, 0b10001101), BitDynamic::new(8, 0b10011101), BitDynamic::new(8, 0b10000100), BitDynamic::new(8, 0b10010000), BitDynamic::new(8, 0b11011000), BitDynamic::new(8, 0b10101011), BitDynamic::new(8, 0b00000000), BitDynamic::new(8, 0b10001100), BitDynamic::new(8, 0b10111100), BitDynamic::new(8, 0b11010011), BitDynamic::new(8, 0b00001010), BitDynamic::new(8, 0b11110111), BitDynamic::new(8, 0b11100100), BitDynamic::new(8, 0b01011000), BitDynamic::new(8, 0b00000101), BitDynamic::new(8, 0b10111000), BitDynamic::new(8, 0b10110011), BitDynamic::new(8, 0b01000101), BitDynamic::new(8, 0b00000110), BitDynamic::new(8, 0b11010000), BitDynamic::new(8, 0b00101100), BitDynamic::new(8, 0b00011110), BitDynamic::new(8, 0b10001111), BitDynamic::new(8, 0b11001010), BitDynamic::new(8, 0b00111111), BitDynamic::new(8, 0b00001111), BitDynamic::new(8, 0b00000010), BitDynamic::new(8, 0b11000001), BitDynamic::new(8, 0b10101111), BitDynamic::new(8, 0b10111101), BitDynamic::new(8, 0b00000011), BitDynamic::new(8, 0b00000001), BitDynamic::new(8, 0b00010011), BitDynamic::new(8, 0b10001010), BitDynamic::new(8, 0b01101011), BitDynamic::new(8, 0b00111010), BitDynamic::new(8, 0b10010001), BitDynamic::new(8, 0b00010001), BitDynamic::new(8, 0b01000001), BitDynamic::new(8, 0b01001111), BitDynamic::new(8, 0b01100111), BitDynamic::new(8, 0b11011100), BitDynamic::new(8, 0b11101010), BitDynamic::new(8, 0b10010111), BitDynamic::new(8, 0b11110010), BitDynamic::new(8, 0b11001111), BitDynamic::new(8, 0b11001110), BitDynamic::new(8, 0b11110000), BitDynamic::new(8, 0b10110100), BitDynamic::new(8, 0b11100110), BitDynamic::new(8, 0b01110011), BitDynamic::new(8, 0b10010110), BitDynamic::new(8, 0b10101100), BitDynamic::new(8, 0b01110100), BitDynamic::new(8, 0b00100010), BitDynamic::new(8, 0b11100111), BitDynamic::new(8, 0b10101101), BitDynamic::new(8, 0b00110101), BitDynamic::new(8, 0b10000101), BitDynamic::new(8, 0b11100010), BitDynamic::new(8, 0b11111001), BitDynamic::new(8, 0b00110111), BitDynamic::new(8, 0b11101000), BitDynamic::new(8, 0b00011100), BitDynamic::new(8, 0b01110101), BitDynamic::new(8, 0b11011111), BitDynamic::new(8, 0b01101110), BitDynamic::new(8, 0b01000111), BitDynamic::new(8, 0b11110001), BitDynamic::new(8, 0b00011010), BitDynamic::new(8, 0b01110001), BitDynamic::new(8, 0b00011101), BitDynamic::new(8, 0b00101001), BitDynamic::new(8, 0b11000101), BitDynamic::new(8, 0b10001001), BitDynamic::new(8, 0b01101111), BitDynamic::new(8, 0b10110111), BitDynamic::new(8, 0b01100010), BitDynamic::new(8, 0b00001110), BitDynamic::new(8, 0b10101010), BitDynamic::new(8, 0b00011000), BitDynamic::new(8, 0b10111110), BitDynamic::new(8, 0b00011011), BitDynamic::new(8, 0b11111100), BitDynamic::new(8, 0b01010110), BitDynamic::new(8, 0b00111110), BitDynamic::new(8, 0b01001011), BitDynamic::new(8, 0b11000110), BitDynamic::new(8, 0b11010010), BitDynamic::new(8, 0b01111001), BitDynamic::new(8, 0b00100000), BitDynamic::new(8, 0b10011010), BitDynamic::new(8, 0b11011011), BitDynamic::new(8, 0b11000000), BitDynamic::new(8, 0b11111110), BitDynamic::new(8, 0b01111000), BitDynamic::new(8, 0b11001101), BitDynamic::new(8, 0b01011010), BitDynamic::new(8, 0b11110100), BitDynamic::new(8, 0b00011111), BitDynamic::new(8, 0b11011101), BitDynamic::new(8, 0b10101000), BitDynamic::new(8, 0b00110011), BitDynamic::new(8, 0b10001000), BitDynamic::new(8, 0b00000111), BitDynamic::new(8, 0b11000111), BitDynamic::new(8, 0b00110001), BitDynamic::new(8, 0b10110001), BitDynamic::new(8, 0b00010010), BitDynamic::new(8, 0b00010000), BitDynamic::new(8, 0b01011001), BitDynamic::new(8, 0b00100111), BitDynamic::new(8, 0b10000000), BitDynamic::new(8, 0b11101100), BitDynamic::new(8, 0b01011111), BitDynamic::new(8, 0b01100000), BitDynamic::new(8, 0b01010001), BitDynamic::new(8, 0b01111111), BitDynamic::new(8, 0b10101001), BitDynamic::new(8, 0b00011001), BitDynamic::new(8, 0b10110101), BitDynamic::new(8, 0b01001010), BitDynamic::new(8, 0b00001101), BitDynamic::new(8, 0b00101101), BitDynamic::new(8, 0b11100101), BitDynamic::new(8, 0b01111010), BitDynamic::new(8, 0b10011111), BitDynamic::new(8, 0b10010011), BitDynamic::new(8, 0b11001001), BitDynamic::new(8, 0b10011100), BitDynamic::new(8, 0b11101111), BitDynamic::new(8, 0b10100000), BitDynamic::new(8, 0b11100000), BitDynamic::new(8, 0b00111011), BitDynamic::new(8, 0b01001101), BitDynamic::new(8, 0b10101110), BitDynamic::new(8, 0b00101010), BitDynamic::new(8, 0b11110101), BitDynamic::new(8, 0b10110000), BitDynamic::new(8, 0b11001000), BitDynamic::new(8, 0b11101011), BitDynamic::new(8, 0b10111011), BitDynamic::new(8, 0b00111100), BitDynamic::new(8, 0b10000011), BitDynamic::new(8, 0b01010011), BitDynamic::new(8, 0b10011001), BitDynamic::new(8, 0b01100001), BitDynamic::new(8, 0b00010111), BitDynamic::new(8, 0b00101011), BitDynamic::new(8, 0b00000100), BitDynamic::new(8, 0b01111110), BitDynamic::new(8, 0b10111010), BitDynamic::new(8, 0b01110111), BitDynamic::new(8, 0b11010110), BitDynamic::new(8, 0b00100110), BitDynamic::new(8, 0b11100001), BitDynamic::new(8, 0b01101001), BitDynamic::new(8, 0b00010100), BitDynamic::new(8, 0b01100011), BitDynamic::new(8, 0b01010101), BitDynamic::new(8, 0b00100001), BitDynamic::new(8, 0b00001100), BitDynamic::new(8, 0b01111101)];

/// sbox_lookup
///
/// Generated from the Sail sources at `riscv_types_kext.sail` L178-180.
pub fn sbox_lookup(x: BitDynamic, table: [BitDynamic; (256 as usize)]) -> BitDynamic {
    table[((255 - x.unsigned()) as usize)]
}

/// sm4_sbox
///
/// Generated from the Sail sources at `riscv_types_kext.sail` L214.
pub fn sm4_sbox(x: BitDynamic) -> BitDynamic {
    sbox_lookup(x, sm4_sbox_table)
}

/// zvk_valid_reg_overlap
///
/// Generated from the Sail sources at `riscv_zvk_utils.sail` L10-15.
pub fn zvk_valid_reg_overlap(rs: vregidx, rd: vregidx, emul_pow: i128) -> bool {
    let reg_group_size: i128 = if {(emul_pow > 0)} {
        i128::pow(2, (emul_pow as u32))
    } else {
        1
    };
    let rs_int: i128 = vregidx_bits(rs).unsigned();
    let rd_int: i128 = vregidx_bits(rd).unsigned();
    (((rs_int + reg_group_size) <= rd_int) || ((rd_int + reg_group_size) <= rs_int))
}

/// zvk_check_encdec
///
/// Generated from the Sail sources at `riscv_zvk_utils.sail` L17-25.
pub fn zvk_check_encdec(core_ctx: &mut Core, EGW: i128, EGS: i128) -> bool {
    let LMUL_pow: i128 = get_lmul_pow(core_ctx, ());
    let LMUL_times_VLEN: i128 = if {(LMUL_pow < 0)} {
        (get_vlen(core_ctx, ()) / i128::pow(2, (i128::abs(LMUL_pow) as u32)))
    } else {
        (i128::pow(2, (LMUL_pow as u32)) * get_vlen(core_ctx, ()))
    };
    ((((core_ctx.vl.unsigned() as usize) % (EGS as usize)) == 0) && (((((core_ctx.vstart.unsigned() as usize) % (EGS as usize)) == 0) && (LMUL_times_VLEN >= EGW)) as bool))
}

/// zvknhab_check_encdec
///
/// Generated from the Sail sources at `riscv_zvk_utils.sail` L34-38.
pub fn zvknhab_check_encdec(core_ctx: &mut Core, vs2: vregidx, vs1: vregidx, vd: vregidx) -> bool {
    let SEW: i128 = get_sew(core_ctx, ());
    let LMUL_pow: i128 = get_lmul_pow(core_ctx, ());
    (zvk_check_encdec(core_ctx, SEW, 4) && (zvk_valid_reg_overlap(vs1, vd, LMUL_pow) && zvk_valid_reg_overlap(vs2, vd, LMUL_pow)))
}

/// encdec_uop_forwards
///
/// Generated from the Sail sources.
pub fn encdec_uop_forwards(arg_hashtag_: uop) -> BitDynamic {
    match arg_hashtag_ {
        uop::LUI => {BitDynamic::new(7, 0b0110111)}
        uop::AUIPC => {BitDynamic::new(7, 0b0010111)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_uop_backwards
///
/// Generated from the Sail sources.
pub fn encdec_uop_backwards(arg_hashtag_: BitDynamic) -> uop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(7, 0b0110111))} => {uop::LUI}
        b__1 if {(b__1 == BitDynamic::new(7, 0b0010111))} => {uop::AUIPC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_uop_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_uop_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(7, 0b0110111))} => {true}
        b__1 if {(b__1 == BitDynamic::new(7, 0b0010111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_bop_forwards
///
/// Generated from the Sail sources.
pub fn encdec_bop_forwards(arg_hashtag_: bop) -> BitDynamic {
    match arg_hashtag_ {
        bop::BEQ => {BitDynamic::new(3, 0b000)}
        bop::BNE => {BitDynamic::new(3, 0b001)}
        bop::BLT => {BitDynamic::new(3, 0b100)}
        bop::BGE => {BitDynamic::new(3, 0b101)}
        bop::BLTU => {BitDynamic::new(3, 0b110)}
        bop::BGEU => {BitDynamic::new(3, 0b111)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_bop_backwards
///
/// Generated from the Sail sources.
pub fn encdec_bop_backwards(arg_hashtag_: BitDynamic) -> bop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {bop::BEQ}
        b__1 if {(b__1 == BitDynamic::new(3, 0b001))} => {bop::BNE}
        b__2 if {(b__2 == BitDynamic::new(3, 0b100))} => {bop::BLT}
        b__3 if {(b__3 == BitDynamic::new(3, 0b101))} => {bop::BGE}
        b__4 if {(b__4 == BitDynamic::new(3, 0b110))} => {bop::BLTU}
        b__5 if {(b__5 == BitDynamic::new(3, 0b111))} => {bop::BGEU}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_bop_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_bop_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(3, 0b001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(3, 0b100))} => {true}
        b__3 if {(b__3 == BitDynamic::new(3, 0b101))} => {true}
        b__4 if {(b__4 == BitDynamic::new(3, 0b110))} => {true}
        b__5 if {(b__5 == BitDynamic::new(3, 0b111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_iop_forwards
///
/// Generated from the Sail sources.
pub fn encdec_iop_forwards(arg_hashtag_: iop) -> BitDynamic {
    match arg_hashtag_ {
        iop::ADDI => {BitDynamic::new(3, 0b000)}
        iop::SLTI => {BitDynamic::new(3, 0b010)}
        iop::SLTIU => {BitDynamic::new(3, 0b011)}
        iop::ANDI => {BitDynamic::new(3, 0b111)}
        iop::ORI => {BitDynamic::new(3, 0b110)}
        iop::XORI => {BitDynamic::new(3, 0b100)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_iop_backwards
///
/// Generated from the Sail sources.
pub fn encdec_iop_backwards(arg_hashtag_: BitDynamic) -> iop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {iop::ADDI}
        b__1 if {(b__1 == BitDynamic::new(3, 0b010))} => {iop::SLTI}
        b__2 if {(b__2 == BitDynamic::new(3, 0b011))} => {iop::SLTIU}
        b__3 if {(b__3 == BitDynamic::new(3, 0b111))} => {iop::ANDI}
        b__4 if {(b__4 == BitDynamic::new(3, 0b110))} => {iop::ORI}
        b__5 if {(b__5 == BitDynamic::new(3, 0b100))} => {iop::XORI}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_iop_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_iop_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(3, 0b010))} => {true}
        b__2 if {(b__2 == BitDynamic::new(3, 0b011))} => {true}
        b__3 if {(b__3 == BitDynamic::new(3, 0b111))} => {true}
        b__4 if {(b__4 == BitDynamic::new(3, 0b110))} => {true}
        b__5 if {(b__5 == BitDynamic::new(3, 0b100))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// valid_load_encdec
///
/// Generated from the Sail sources at `riscv_insts_base.sail` L279-280.
pub fn valid_load_encdec(width: word_width, is_unsigned: bool) -> bool {
    ((size_bytes_forwards(width) < 8) || (!(is_unsigned) && ((size_bytes_forwards(width) <= 8) as bool)))
}

/// effective_fence_set
///
/// Generated from the Sail sources at `riscv_insts_base.sail` L452-457.
pub fn effective_fence_set(set: BitDynamic, fiom: bool) -> BitDynamic {
    if {fiom} {
        bitvector_concat(BitDynamic::from(set.subrange::<2, 4, 2>()), BitDynamic::from((set.subrange::<0, 2, 2>() | set.subrange::<2, 4, 2>())))
    } else {
        set
    }
}

/// lrsc_width_valid
///
/// Generated from the Sail sources at `riscv_insts_aext.sail` L31-37.
pub fn lrsc_width_valid(size: word_width) -> bool {
    match size {
        word_width::WORD => {true}
        word_width::DOUBLE => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// amo_width_valid
///
/// Generated from the Sail sources at `riscv_insts_aext.sail` L39-46.
pub fn amo_width_valid(core_ctx: &mut Core, size: word_width) -> bool {
    match size {
        word_width::BYTE => {currentlyEnabled(core_ctx, extension::Ext_Zabha)}
        word_width::HALF => {currentlyEnabled(core_ctx, extension::Ext_Zabha)}
        word_width::WORD => {true}
        word_width::DOUBLE => {true}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_amoop_backwards
///
/// Generated from the Sail sources.
pub fn encdec_amoop_backwards(arg_hashtag_: BitDynamic) -> amoop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00001))} => {amoop::AMOSWAP}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00000))} => {amoop::AMOADD}
        b__2 if {(b__2 == BitDynamic::new(5, 0b00100))} => {amoop::AMOXOR}
        b__3 if {(b__3 == BitDynamic::new(5, 0b01100))} => {amoop::AMOAND}
        b__4 if {(b__4 == BitDynamic::new(5, 0b01000))} => {amoop::AMOOR}
        b__5 if {(b__5 == BitDynamic::new(5, 0b10000))} => {amoop::AMOMIN}
        b__6 if {(b__6 == BitDynamic::new(5, 0b10100))} => {amoop::AMOMAX}
        b__7 if {(b__7 == BitDynamic::new(5, 0b11000))} => {amoop::AMOMINU}
        b__8 if {(b__8 == BitDynamic::new(5, 0b11100))} => {amoop::AMOMAXU}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_amoop_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_amoop_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00001))} => {true}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00000))} => {true}
        b__2 if {(b__2 == BitDynamic::new(5, 0b00100))} => {true}
        b__3 if {(b__3 == BitDynamic::new(5, 0b01100))} => {true}
        b__4 if {(b__4 == BitDynamic::new(5, 0b01000))} => {true}
        b__5 if {(b__5 == BitDynamic::new(5, 0b10000))} => {true}
        b__6 if {(b__6 == BitDynamic::new(5, 0b10100))} => {true}
        b__7 if {(b__7 == BitDynamic::new(5, 0b11000))} => {true}
        b__8 if {(b__8 == BitDynamic::new(5, 0b11100))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mul_op_forwards
///
/// Generated from the Sail sources.
pub fn encdec_mul_op_forwards(arg_hashtag_: mul_op) -> BitDynamic {
    match arg_hashtag_ {
        TODO_PAT_struct => {BitDynamic::new(3, 0b000)}
        TODO_PAT_struct => {BitDynamic::new(3, 0b001)}
        TODO_PAT_struct => {BitDynamic::new(3, 0b010)}
        TODO_PAT_struct => {BitDynamic::new(3, 0b011)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mul_op_backwards
///
/// Generated from the Sail sources.
pub fn encdec_mul_op_backwards(arg_hashtag_: BitDynamic) -> mul_op {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {mul_op {
            high: false,
            signed_rs1: true,
            signed_rs2: true
        }}
        b__1 if {(b__1 == BitDynamic::new(3, 0b001))} => {mul_op {
            high: true,
            signed_rs1: true,
            signed_rs2: true
        }}
        b__2 if {(b__2 == BitDynamic::new(3, 0b010))} => {mul_op {
            high: true,
            signed_rs1: true,
            signed_rs2: false
        }}
        b__3 if {(b__3 == BitDynamic::new(3, 0b011))} => {mul_op {
            high: true,
            signed_rs1: false,
            signed_rs2: false
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mul_op_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_mul_op_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(3, 0b001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(3, 0b010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(3, 0b011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
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

/// read_CSR
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L134.
pub fn read_CSR(core_ctx: &mut Core, merge_hashtag_var: BitDynamic) -> BitDynamic {
    match merge_hashtag_var {
        b__0 if {(b__0 == BitDynamic::new(12, 0b001100000001))} => {core_ctx.misa.bits}
        b__1 if {(b__1 == BitDynamic::new(12, 0b001100000000))} => {core_ctx.mstatus.bits.subrange::<0, 64, 64>()}
        b__3 if {(b__3 == BitDynamic::new(12, 0b001100001010))} => {core_ctx.menvcfg.bits.subrange::<0, 64, 64>()}
        b__5 if {(b__5 == BitDynamic::new(12, 0b000100001010))} => {core_ctx.senvcfg.bits.subrange::<0, 64, 64>()}
        b__6 if {(b__6 == BitDynamic::new(12, 0b001100000100))} => {core_ctx.mie.bits}
        b__7 if {(b__7 == BitDynamic::new(12, 0b001101000100))} => {core_ctx.mip.bits}
        b__8 if {(b__8 == BitDynamic::new(12, 0b001100000010))} => {core_ctx.medeleg.bits.subrange::<0, 64, 64>()}
        b__10 if {(b__10 == BitDynamic::new(12, 0b001100000011))} => {core_ctx.mideleg.bits}
        b__11 if {(b__11 == BitDynamic::new(12, 0b001101000010))} => {core_ctx.mcause.bits}
        b__12 if {(b__12 == BitDynamic::new(12, 0b001101000011))} => {core_ctx.mtval}
        b__13 if {(b__13 == BitDynamic::new(12, 0b001101000000))} => {core_ctx.mscratch}
        b__14 if {(b__14 == BitDynamic::new(12, 0b000100000110))} => {{
            let var_1: BitDynamic = core_ctx.scounteren.bits;
            var_1.zero_extend_dyn(64)
        }}
        b__15 if {(b__15 == BitDynamic::new(12, 0b001100000110))} => {{
            let var_2: BitDynamic = core_ctx.mcounteren.bits;
            var_2.zero_extend_dyn(64)
        }}
        b__16 if {(b__16 == BitDynamic::new(12, 0b001100100000))} => {{
            let var_3: BitDynamic = core_ctx.mcountinhibit.bits;
            var_3.zero_extend_dyn(64)
        }}
        b__17 if {(b__17 == BitDynamic::new(12, 0b111100010001))} => {{
            let var_4: BitDynamic = core_ctx.mvendorid;
            var_4.zero_extend_dyn(64)
        }}
        b__18 if {(b__18 == BitDynamic::new(12, 0b111100010010))} => {core_ctx.marchid}
        b__19 if {(b__19 == BitDynamic::new(12, 0b111100010011))} => {core_ctx.mimpid}
        b__20 if {(b__20 == BitDynamic::new(12, 0b111100010100))} => {core_ctx.mhartid}
        b__21 if {(b__21 == BitDynamic::new(12, 0b111100010101))} => {core_ctx.mconfigptr}
        b__22 if {(b__22 == BitDynamic::new(12, 0b000100000000))} => {{
            let var_5: Mstatus = core_ctx.mstatus;
            lower_mstatus(var_5)
        }.bits.subrange::<0, 64, 64>()}
        b__23 if {(b__23 == BitDynamic::new(12, 0b000101000100))} => {{
            let var_6: Minterrupts = core_ctx.mip;
            let var_7: Minterrupts = core_ctx.mideleg;
            lower_mip(var_6, var_7)
        }.bits}
        b__24 if {(b__24 == BitDynamic::new(12, 0b000100000100))} => {{
            let var_8: Minterrupts = core_ctx.mie;
            let var_9: Minterrupts = core_ctx.mideleg;
            lower_mie(var_8, var_9)
        }.bits}
        b__25 if {(b__25 == BitDynamic::new(12, 0b000101000000))} => {core_ctx.sscratch}
        b__26 if {(b__26 == BitDynamic::new(12, 0b000101000010))} => {core_ctx.scause.bits}
        b__27 if {(b__27 == BitDynamic::new(12, 0b000101000011))} => {core_ctx.stval}
        b__28 if {(b__28 == BitDynamic::new(12, 0b011110100000))} => {!(core_ctx.tselect)}
        v__3710 if {let idx_var_10: BitDynamic = v__3710.subrange::<0, 4, 4>();
        ((v__3710.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111010)) && (bitvector_access(idx_var_10, 0) == false))} => {let idx: BitDynamic = v__3710.subrange::<0, 4, 4>();
        pmpReadCfgReg(core_ctx, idx.unsigned())}
        v__3712 if {(v__3712.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111011))} => {let idx: BitDynamic = v__3712.subrange::<0, 4, 4>();
        pmpReadAddrReg(core_ctx, bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b00)), BitDynamic::from(idx)).unsigned())}
        v__3714 if {(v__3714.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111100))} => {let idx: BitDynamic = v__3714.subrange::<0, 4, 4>();
        pmpReadAddrReg(core_ctx, bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b01)), BitDynamic::from(idx)).unsigned())}
        v__3716 if {(v__3716.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111101))} => {let idx: BitDynamic = v__3716.subrange::<0, 4, 4>();
        pmpReadAddrReg(core_ctx, bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b10)), BitDynamic::from(idx)).unsigned())}
        v__3718 if {(v__3718.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111110))} => {let idx: BitDynamic = v__3718.subrange::<0, 4, 4>();
        pmpReadAddrReg(core_ctx, bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b11)), BitDynamic::from(idx)).unsigned())}
        b__29 if {(b__29 == BitDynamic::new(12, 0b000000001000))} => {{
            let var_11: BitDynamic = core_ctx.vstart;
            var_11.zero_extend_dyn(64)
        }}
        b__30 if {(b__30 == BitDynamic::new(12, 0b000000001001))} => {{
            let var_12: BitDynamic = {
                let var_13: Vcsr = core_ctx.vcsr;
                _get_Vcsr_vxsat(var_13)
            };
            var_12.zero_extend_dyn(64)
        }}
        b__31 if {(b__31 == BitDynamic::new(12, 0b000000001010))} => {{
            let var_14: BitDynamic = {
                let var_15: Vcsr = core_ctx.vcsr;
                _get_Vcsr_vxrm(var_15)
            };
            var_14.zero_extend_dyn(64)
        }}
        b__32 if {(b__32 == BitDynamic::new(12, 0b000000001111))} => {{
            let var_16: BitDynamic = core_ctx.vcsr.bits;
            var_16.zero_extend_dyn(64)
        }}
        b__33 if {(b__33 == BitDynamic::new(12, 0b110000100000))} => {core_ctx.vl}
        b__34 if {(b__34 == BitDynamic::new(12, 0b110000100001))} => {core_ctx.vtype.bits}
        b__35 if {(b__35 == BitDynamic::new(12, 0b110000100010))} => {get_vlenb(core_ctx, ())}
        b__36 if {(b__36 == BitDynamic::new(12, 0b000100000101))} => {get_stvec(core_ctx, ())}
        b__37 if {(b__37 == BitDynamic::new(12, 0b000101000001))} => {get_xepc(core_ctx, Privilege::Supervisor)}
        b__38 if {(b__38 == BitDynamic::new(12, 0b001100000101))} => {get_mtvec(core_ctx, ())}
        b__39 if {(b__39 == BitDynamic::new(12, 0b001101000001))} => {get_xepc(core_ctx, Privilege::Machine)}
        v__3720 if {let index_var_17: BitDynamic = v__3720.subrange::<0, 5, 5>();
        ((v__3720.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b0011001)) && ((index_var_17.unsigned() >= 3) as bool))} => {let index: BitDynamic = v__3720.subrange::<0, 5, 5>();
        read_mhpmevent(core_ctx, hpmidx_from_bits(index))}
        v__3722 if {let index_var_18: BitDynamic = v__3722.subrange::<0, 5, 5>();
        ((v__3722.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b1011000)) && ((index_var_18.unsigned() >= 3) as bool))} => {let index: BitDynamic = v__3722.subrange::<0, 5, 5>();
        read_mhpmcounter(core_ctx, hpmidx_from_bits(index))}
        v__3726 if {let index_var_19: BitDynamic = v__3726.subrange::<0, 5, 5>();
        ((v__3726.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b1100000)) && ((index_var_19.unsigned() >= 3) as bool))} => {let index: BitDynamic = v__3726.subrange::<0, 5, 5>();
        read_mhpmcounter(core_ctx, hpmidx_from_bits(index))}
        b__40 if {(b__40 == BitDynamic::new(12, 0b110110100000))} => {{
            let var_20: BitDynamic = {
                let var_21: Privilege = core_ctx.cur_privilege;
                get_scountovf(core_ctx, var_21)
            };
            var_20.zero_extend_dyn(64)
        }}
        b__41 if {(b__41 == BitDynamic::new(12, 0b000000010101))} => {read_seed_csr(())}
        b__42 if {(b__42 == BitDynamic::new(12, 0b110000000000))} => {subrange_bits(core_ctx.mcycle, 63, 0)}
        b__43 if {(b__43 == BitDynamic::new(12, 0b110000000001))} => {subrange_bits(core_ctx.mtime, 63, 0)}
        b__44 if {(b__44 == BitDynamic::new(12, 0b110000000010))} => {subrange_bits(core_ctx.minstret, 63, 0)}
        b__48 if {(b__48 == BitDynamic::new(12, 0b101100000000))} => {subrange_bits(core_ctx.mcycle, 63, 0)}
        b__49 if {(b__49 == BitDynamic::new(12, 0b101100000010))} => {subrange_bits(core_ctx.minstret, 63, 0)}
        b__55 if {(b__55 == BitDynamic::new(12, 0b001100100001))} => {core_ctx.mcyclecfg.bits.subrange::<0, 64, 64>()}
        b__57 if {(b__57 == BitDynamic::new(12, 0b001100100010))} => {core_ctx.minstretcfg.bits.subrange::<0, 64, 64>()}
        b__59 if {(b__59 == BitDynamic::new(12, 0b000101001101))} => {subrange_bits(core_ctx.stimecmp, 63, 0)}
        b__61 if {(b__61 == BitDynamic::new(12, 0b000110000000))} => {core_ctx.satp}
        csr => {{
            panic!("{}, l {}: {}", "riscv_csr_end.sail", 17, format!("{}{}", "Read from CSR that does not exist: ", bits_str(csr)))
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// write_CSR
///
/// Generated from the Sail sources at `riscv_sys_regs.sail` L135.
pub fn write_CSR(core_ctx: &mut Core, merge_hashtag_var: BitDynamic, missing_arg_0: BitDynamic) -> BitDynamic {
    match (merge_hashtag_var, missing_arg_0) {
        (b__0, value) if {(b__0 == BitDynamic::new(12, 0b001100000001))} => {{
            core_ctx.misa = {
                let var_1: Misa = core_ctx.misa;
                legalize_misa(core_ctx, var_1, value)
            };
            core_ctx.misa.bits
        }}
        (b__1, value) if {(b__1 == BitDynamic::new(12, 0b001100000000))} => {{
            core_ctx.mstatus = {
                let var_2: Mstatus = core_ctx.mstatus;
                legalize_mstatus(core_ctx, var_2, value)
            };
            core_ctx.mstatus.bits
        }}
        (b__5, value) if {(b__5 == BitDynamic::new(12, 0b001100001010))} => {{
            core_ctx.menvcfg = {
                let var_3: MEnvcfg = core_ctx.menvcfg;
                legalize_menvcfg(core_ctx, var_3, value)
            };
            core_ctx.menvcfg.bits
        }}
        (b__7, value) if {(b__7 == BitDynamic::new(12, 0b000100001010))} => {{
            core_ctx.senvcfg = {
                let var_4: SEnvcfg = core_ctx.senvcfg;
                legalize_senvcfg(core_ctx, var_4, value.zero_extend_dyn(64))
            };
            core_ctx.senvcfg.bits.subrange::<0, 64, 64>()
        }}
        (b__8, value) if {(b__8 == BitDynamic::new(12, 0b001100000100))} => {{
            core_ctx.mie = {
                let var_5: Minterrupts = core_ctx.mie;
                legalize_mie(core_ctx, var_5, value)
            };
            core_ctx.mie.bits
        }}
        (b__9, value) if {(b__9 == BitDynamic::new(12, 0b001101000100))} => {{
            core_ctx.mip = {
                let var_6: Minterrupts = core_ctx.mip;
                legalize_mip(core_ctx, var_6, value)
            };
            core_ctx.mip.bits
        }}
        (b__10, value) if {(b__10 == BitDynamic::new(12, 0b001100000010))} => {{
            core_ctx.medeleg = {
                let var_7: Medeleg = core_ctx.medeleg;
                legalize_medeleg(var_7, value)
            };
            core_ctx.medeleg.bits
        }}
        (b__13, value) if {(b__13 == BitDynamic::new(12, 0b001100000011))} => {{
            core_ctx.mideleg = {
                let var_8: Minterrupts = core_ctx.mideleg;
                legalize_mideleg(var_8, value)
            };
            core_ctx.mideleg.bits
        }}
        (b__14, value) if {(b__14 == BitDynamic::new(12, 0b001101000010))} => {{
            core_ctx.mcause.bits = value;
            core_ctx.mcause.bits
        }}
        (b__15, value) if {(b__15 == BitDynamic::new(12, 0b001101000011))} => {{
            core_ctx.mtval = value;
            core_ctx.mtval
        }}
        (b__16, value) if {(b__16 == BitDynamic::new(12, 0b001101000000))} => {{
            core_ctx.mscratch = value;
            core_ctx.mscratch
        }}
        (b__17, value) if {(b__17 == BitDynamic::new(12, 0b000100000110))} => {{
            core_ctx.scounteren = {
                let var_9: Counteren = core_ctx.scounteren;
                legalize_scounteren(core_ctx, var_9, value)
            };
            {
                let var_10: BitDynamic = core_ctx.scounteren.bits;
                var_10.zero_extend_dyn(64)
            }
        }}
        (b__18, value) if {(b__18 == BitDynamic::new(12, 0b001100000110))} => {{
            core_ctx.mcounteren = {
                let var_11: Counteren = core_ctx.mcounteren;
                legalize_mcounteren(core_ctx, var_11, value)
            };
            {
                let var_12: BitDynamic = core_ctx.mcounteren.bits;
                var_12.zero_extend_dyn(64)
            }
        }}
        (b__19, value) if {(b__19 == BitDynamic::new(12, 0b001100100000))} => {{
            core_ctx.mcountinhibit = {
                let var_13: Counterin = core_ctx.mcountinhibit;
                legalize_mcountinhibit(core_ctx, var_13, value)
            };
            {
                let var_14: BitDynamic = core_ctx.mcountinhibit.bits;
                var_14.zero_extend_dyn(64)
            }
        }}
        (b__20, value) if {(b__20 == BitDynamic::new(12, 0b000100000000))} => {{
            core_ctx.mstatus = {
                let var_15: Mstatus = core_ctx.mstatus;
                legalize_sstatus(core_ctx, var_15, value)
            };
            core_ctx.mstatus.bits.subrange::<0, 64, 64>()
        }}
        (b__21, value) if {(b__21 == BitDynamic::new(12, 0b000101000100))} => {{
            core_ctx.mip = {
                let var_16: Minterrupts = core_ctx.mip;
                let var_17: Minterrupts = core_ctx.mideleg;
                legalize_sip(var_16, var_17, value)
            };
            core_ctx.mip.bits
        }}
        (b__22, value) if {(b__22 == BitDynamic::new(12, 0b000100000100))} => {{
            core_ctx.mie = {
                let var_18: Minterrupts = core_ctx.mie;
                let var_19: Minterrupts = core_ctx.mideleg;
                legalize_sie(var_18, var_19, value)
            };
            core_ctx.mie.bits
        }}
        (b__23, value) if {(b__23 == BitDynamic::new(12, 0b000101000000))} => {{
            core_ctx.sscratch = value;
            core_ctx.sscratch
        }}
        (b__24, value) if {(b__24 == BitDynamic::new(12, 0b000101000010))} => {{
            core_ctx.scause.bits = value;
            core_ctx.scause.bits
        }}
        (b__25, value) if {(b__25 == BitDynamic::new(12, 0b000101000011))} => {{
            core_ctx.stval = value;
            core_ctx.stval
        }}
        (b__26, value) if {(b__26 == BitDynamic::new(12, 0b011110100000))} => {{
            core_ctx.tselect = value;
            core_ctx.tselect
        }}
        (v__3732, value) if {let idx_var_20: BitDynamic = v__3732.subrange::<0, 4, 4>();
        ((v__3732.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111010)) && (bitvector_access(idx_var_20, 0) == false))} => {let idx: BitDynamic = v__3732.subrange::<0, 4, 4>();
        let idx: i128 = idx.unsigned();
        pmpWriteCfgReg(core_ctx, idx, value);
        pmpReadCfgReg(core_ctx, idx)}
        (v__3734, value) if {(v__3734.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111011))} => {let idx: BitDynamic = v__3734.subrange::<0, 4, 4>();
        let idx: i128 = bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b00)), BitDynamic::from(idx)).unsigned();
        pmpWriteAddrReg(core_ctx, idx, value);
        pmpReadAddrReg(core_ctx, idx)}
        (v__3736, value) if {(v__3736.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111100))} => {let idx: BitDynamic = v__3736.subrange::<0, 4, 4>();
        let idx: i128 = bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b01)), BitDynamic::from(idx)).unsigned();
        pmpWriteAddrReg(core_ctx, idx, value);
        pmpReadAddrReg(core_ctx, idx)}
        (v__3738, value) if {(v__3738.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111101))} => {let idx: BitDynamic = v__3738.subrange::<0, 4, 4>();
        let idx: i128 = bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b10)), BitDynamic::from(idx)).unsigned();
        pmpWriteAddrReg(core_ctx, idx, value);
        pmpReadAddrReg(core_ctx, idx)}
        (v__3740, value) if {(v__3740.subrange::<4, 12, 8>() == BitDynamic::new(8, 0b00111110))} => {let idx: BitDynamic = v__3740.subrange::<0, 4, 4>();
        let idx: i128 = bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b11)), BitDynamic::from(idx)).unsigned();
        pmpWriteAddrReg(core_ctx, idx, value);
        pmpReadAddrReg(core_ctx, idx)}
        (b__27, value) if {(b__27 == BitDynamic::new(12, 0b000000001000))} => {{
            set_vstart(core_ctx, value.subrange::<0, 16, 16>());
            {
                let var_21: BitDynamic = core_ctx.vstart;
                var_21.zero_extend_dyn(64)
            }
        }}
        (b__28, value) if {(b__28 == BitDynamic::new(12, 0b000000001001))} => {{
            {
                let var_22: BitDynamic = {
                    let var_23: Vcsr = core_ctx.vcsr;
                    _get_Vcsr_vxrm(var_23)
                };
                ext_write_vcsr(core_ctx, var_22, value.subrange::<0, 1, 1>())
            };
            {
                let var_24: BitDynamic = {
                    let var_25: Vcsr = core_ctx.vcsr;
                    _get_Vcsr_vxsat(var_25)
                };
                var_24.zero_extend_dyn(64)
            }
        }}
        (b__29, value) if {(b__29 == BitDynamic::new(12, 0b000000001010))} => {{
            {
                let var_26: BitDynamic = {
                    let var_27: Vcsr = core_ctx.vcsr;
                    _get_Vcsr_vxsat(var_27)
                };
                ext_write_vcsr(core_ctx, value.subrange::<0, 2, 2>(), var_26)
            };
            {
                let var_28: BitDynamic = {
                    let var_29: Vcsr = core_ctx.vcsr;
                    _get_Vcsr_vxrm(var_29)
                };
                var_28.zero_extend_dyn(64)
            }
        }}
        (b__30, value) if {(b__30 == BitDynamic::new(12, 0b000000001111))} => {{
            ext_write_vcsr(core_ctx, value.subrange::<1, 3, 2>(), value.subrange::<0, 1, 1>());
            {
                let var_30: BitDynamic = core_ctx.vcsr.bits;
                var_30.zero_extend_dyn(64)
            }
        }}
        (b__31, value) if {(b__31 == BitDynamic::new(12, 0b000100000101))} => {{
            set_stvec(core_ctx, value)
        }}
        (b__32, value) if {(b__32 == BitDynamic::new(12, 0b000101000001))} => {{
            set_xepc(core_ctx, Privilege::Supervisor, value)
        }}
        (b__33, value) if {(b__33 == BitDynamic::new(12, 0b001100000101))} => {{
            set_mtvec(core_ctx, value)
        }}
        (b__34, value) if {(b__34 == BitDynamic::new(12, 0b001101000001))} => {{
            set_xepc(core_ctx, Privilege::Machine, value)
        }}
        (v__3742, value) if {let index_var_31: BitDynamic = v__3742.subrange::<0, 5, 5>();
        ((v__3742.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b0011001)) && ((index_var_31.unsigned() >= 3) as bool))} => {let index: BitDynamic = v__3742.subrange::<0, 5, 5>();
        let index: i128 = hpmidx_from_bits(index);
        write_mhpmevent(core_ctx, index, value);
        read_mhpmevent(core_ctx, index)}
        (v__3744, value) if {let index_var_32: BitDynamic = v__3744.subrange::<0, 5, 5>();
        ((v__3744.subrange::<5, 12, 7>() == BitDynamic::new(7, 0b1011000)) && ((index_var_32.unsigned() >= 3) as bool))} => {let index: BitDynamic = v__3744.subrange::<0, 5, 5>();
        let index: i128 = hpmidx_from_bits(index);
        write_mhpmcounter(core_ctx, index, value);
        read_mhpmcounter(core_ctx, index)}
        (b__35, value) if {(b__35 == BitDynamic::new(12, 0b000000010101))} => {write_seed_csr(())}
        (b__36, value) if {(b__36 == BitDynamic::new(12, 0b101100000000))} => {{
            core_ctx.mcycle = core_ctx.mcycle.set_subrange(value, 63, 0);
            value
        }}
        (b__37, value) if {(b__37 == BitDynamic::new(12, 0b101100000010))} => {{
            core_ctx.minstret = core_ctx.minstret.set_subrange(value, 63, 0);
            core_ctx.minstret_increment = false;
            value
        }}
        (b__43, value) if {(b__43 == BitDynamic::new(12, 0b001100100001))} => {{
            core_ctx.mcyclecfg = {
                let var_42: CountSmcntrpmf = core_ctx.mcyclecfg;
                legalize_smcntrpmf(core_ctx, var_42, value)
            };
            core_ctx.mcyclecfg.bits
        }}
        (b__46, value) if {(b__46 == BitDynamic::new(12, 0b001100100010))} => {{
            core_ctx.minstretcfg = {
                let var_43: CountSmcntrpmf = core_ctx.minstretcfg;
                legalize_smcntrpmf(core_ctx, var_43, value)
            };
            core_ctx.minstretcfg.bits.subrange::<0, 64, 64>()
        }}
        (b__49, value) if {(b__49 == BitDynamic::new(12, 0b000101001101))} => {{
            core_ctx.stimecmp = core_ctx.stimecmp.set_subrange(value, 63, 0);
            subrange_bits(core_ctx.stimecmp, 63, 0)
        }}
        (b__51, value) if {(b__51 == BitDynamic::new(12, 0b000110000000))} => {{
            core_ctx.satp = {
                let var_44: Architecture = cur_architecture(core_ctx, ());
                let var_45: BitDynamic = core_ctx.satp;
                legalize_satp(core_ctx, var_44, var_45, value)
            };
            core_ctx.satp
        }}
        (csr, _) => {{
            panic!("{}, l {}: {}", "riscv_csr_end.sail", 23, format!("{}{}", "Write to CSR that does not exist: ", bits_str(csr)))
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// doCSR
///
/// Generated from the Sail sources at `riscv_insts_zicsr.sail` L27-50.
pub fn doCSR(core_ctx: &mut Core, csr: BitDynamic, rs1_val: BitDynamic, rd: regidx, op: csrop, is_CSR_Write: bool) -> ExecutionResult {
    if {!({
        let var_1: Privilege = core_ctx.cur_privilege;
        check_CSR(core_ctx, csr, var_1, is_CSR_Write)
    })} {
        ExecutionResult::Illegal_Instruction(())
    } else if {!(true)} {
        ExecutionResult::Ext_CSR_Check_Failure(())
    } else {
        let is_CSR_Read: bool = !(((op == csrop::CSRRW) && (rd == zreg)));
        let csr_val: xlenbits = if {is_CSR_Read} {
            read_CSR(core_ctx, csr)
        } else {
            zeros(64)
        };
        if {is_CSR_Write} {
            let new_val: xlenbits = match op {
                csrop::CSRRW => {rs1_val}
                csrop::CSRRS => {(csr_val | rs1_val)}
                csrop::CSRRC => {(csr_val & !(rs1_val))}
                _ => {panic!("Unreachable code")}
            };
            let final_val: BitDynamic = write_CSR(core_ctx, csr, new_val);

        } else {
            csr_id_read_callback(csr, csr_val)
        };
        wX_bits(core_ctx, rd, csr_val);
        RETIRE_SUCCESS
    }
}

/// encdec_rounding_mode_backwards
///
/// Generated from the Sail sources.
pub fn encdec_rounding_mode_backwards(arg_hashtag_: BitDynamic) -> rounding_mode {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {rounding_mode::RM_RNE}
        b__1 if {(b__1 == BitDynamic::new(3, 0b001))} => {rounding_mode::RM_RTZ}
        b__2 if {(b__2 == BitDynamic::new(3, 0b010))} => {rounding_mode::RM_RDN}
        b__3 if {(b__3 == BitDynamic::new(3, 0b011))} => {rounding_mode::RM_RUP}
        b__4 if {(b__4 == BitDynamic::new(3, 0b100))} => {rounding_mode::RM_RMM}
        b__5 if {(b__5 == BitDynamic::new(3, 0b111))} => {rounding_mode::RM_DYN}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_rounding_mode_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_rounding_mode_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(3, 0b001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(3, 0b010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(3, 0b011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(3, 0b100))} => {true}
        b__5 if {(b__5 == BitDynamic::new(3, 0b111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// haveSingleFPU
///
/// Generated from the Sail sources at `riscv_insts_fext.sail` L265.
pub fn haveSingleFPU(core_ctx: &mut Core, unit_arg: ()) -> bool {
    (currentlyEnabled(core_ctx, extension::Ext_F) || currentlyEnabled(core_ctx, extension::Ext_Zfinx))
}

/// haveDoubleFPU
///
/// Generated from the Sail sources at `riscv_insts_dext.sail` L229.
pub fn haveDoubleFPU(core_ctx: &mut Core, unit_arg: ()) -> bool {
    (currentlyEnabled(core_ctx, extension::Ext_D) || currentlyEnabled(core_ctx, extension::Ext_Zdinx))
}

/// haveHalfFPU
///
/// Generated from the Sail sources at `riscv_insts_zfh.sail` L167.
pub fn haveHalfFPU(core_ctx: &mut Core, unit_arg: ()) -> bool {
    (currentlyEnabled(core_ctx, extension::Ext_Zfh) || currentlyEnabled(core_ctx, extension::Ext_Zhinx))
}

/// haveHalfMin
///
/// Generated from the Sail sources at `riscv_insts_zfh.sail` L170.
pub fn haveHalfMin(core_ctx: &mut Core, unit_arg: ()) -> bool {
    (haveHalfFPU(core_ctx, ()) || currentlyEnabled(core_ctx, extension::Ext_Zfhmin))
}

pub type nfields = i128;

/// valid_vtype
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L42-44.
pub fn valid_vtype(core_ctx: &mut Core, unit_arg: ()) -> bool {
    ({
        let var_1: Vtype = core_ctx.vtype;
        _get_Vtype_vill(var_1)
    } == BitDynamic::new(1, 0b0))
}

/// valid_rd_mask
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L59-61.
pub fn valid_rd_mask(rd: vregidx, vm: BitDynamic) -> bool {
    ((vm != BitDynamic::new(1, 0b0)) || (rd != zvreg))
}

/// illegal_normal
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L99-101.
pub fn illegal_normal(core_ctx: &mut Core, vd: vregidx, vm: BitDynamic) -> bool {
    (!(valid_vtype(core_ctx, ())) || !(valid_rd_mask(vd, vm)))
}

/// illegal_vd_unmasked
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L116-118.
pub fn illegal_vd_unmasked(core_ctx: &mut Core, unit_arg: ()) -> bool {
    !(valid_vtype(core_ctx, ()))
}

/// get_scalar
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L173-181.
pub fn get_scalar(core_ctx: &mut Core, rs1: regidx, SEW: i128) -> BitDynamic {
    if {(SEW <= 64)} {
        subrange_bits(rX_bits(core_ctx, rs1), (SEW - 1), 0)
    } else {
        sign_extend(SEW, rX_bits(core_ctx, rs1))
    }
}

/// get_start_element
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L209-222.
pub fn get_start_element(core_ctx: &mut Core, unit_arg: ()) -> result::<nat, ()> {
    let start_element: i128 = core_ctx.vstart.unsigned();
    let VLEN_pow: i128 = get_vlen_pow(core_ctx, ());
    let SEW_pow: i128 = get_sew_pow(core_ctx, ());
    if {(start_element > (i128::pow(2, (((3 + VLEN_pow) - SEW_pow) as u32)) - 1))} {
        result::Err(())
    } else {
        result::Ok(start_element)
    }
}

/// get_end_element
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L226.
pub fn get_end_element(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    (core_ctx.vl.unsigned() - 1)
}

/// init_masked_result
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L237-285.
pub fn init_masked_result(core_ctx: &mut Core, num_elem: i128, SEW: i128, LMUL_pow: i128, vd_val: Vec::<BitDynamic>, vm_val: BitDynamic) -> result::<(Vec::<BitDynamic>, BitDynamic), ()> {
    let start_element: nat = match get_start_element(core_ctx, ()) {
        result::Ok(v) => {v}
        result::Err(()) => {return result::Err(());}
        _ => {panic!("Unreachable code")}
    };
    let end_element: i128 = get_end_element(core_ctx, ());
    let tail_ag: agtype = get_vtype_vta(core_ctx, ());
    let mask_ag: agtype = get_vtype_vma(core_ctx, ());
    let mut mask: BitDynamic = undefined_bitvector(bitvector_length(vm_val));
    {
        let mut result: Vec::<BitDynamic> = undefined_vector(bitvector_length(vm_val), undefined_bitvector(__id(SEW)));
        {
            let real_num_elem: i128 = if {(LMUL_pow >= 0)} {
                num_elem
            } else {
                (num_elem / i128::pow(2, ((0 - LMUL_pow) as u32)))
            };
            assert!((num_elem >= real_num_elem), "riscv_insts_vext_utils.sail:250.34-250.35");
            for i in 0..=(num_elem - 1) {
                if {(i < start_element)} {
                    result[(i as usize)] = vd_val[(i as usize)];
                    mask = mask.set_bit(i, false)
                } else if {(i > end_element)} {
                    result[(i as usize)] = match tail_ag {
                        agtype::UNDISTURBED => {vd_val[(i as usize)]}
                        agtype::AGNOSTIC => {vd_val[(i as usize)]}
                        _ => {panic!("Unreachable code")}
                    };
                    mask = mask.set_bit(i, false)
                } else if {(i >= real_num_elem)} {
                    result[(i as usize)] = match tail_ag {
                        agtype::UNDISTURBED => {vd_val[(i as usize)]}
                        agtype::AGNOSTIC => {vd_val[(i as usize)]}
                        _ => {panic!("Unreachable code")}
                    };
                    mask = mask.set_bit(i, false)
                } else if {(bitvector_access(vm_val, i) == false)} {
                    result[(i as usize)] = match mask_ag {
                        agtype::UNDISTURBED => {vd_val[(i as usize)]}
                        agtype::AGNOSTIC => {vd_val[(i as usize)]}
                        _ => {panic!("Unreachable code")}
                    };
                    mask = mask.set_bit(i, false)
                } else {
                    mask = mask.set_bit(i, true)
                }
            };
            result::Ok((result, mask))
        }
    }
}

/// get_shift_amount
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L437-441.
pub fn get_shift_amount(bit_val: BitDynamic, SEW: i128) -> nat {
    let lowlog2bits: i128 = log2(SEW);
    assert!(((0 < lowlog2bits) && (lowlog2bits < bitvector_length(bit_val))), "riscv_insts_vext_utils.sail:439.43-439.44");
    subrange_bits(bit_val, (lowlog2bits - 1), 0).unsigned()
}

/// get_fixed_rounding_incr
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L445-458.
pub fn get_fixed_rounding_incr(core_ctx: &mut Core, vec_elem: BitDynamic, shift_amount: i128) -> BitDynamic {
    if {(shift_amount == 0)} {
        BitDynamic::new(1, 0b0)
    } else {
        let rounding_mode: BitDynamic = {
            let var_1: Vcsr = core_ctx.vcsr;
            _get_Vcsr_vxrm(var_1)
        };
        match rounding_mode {
            b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {slice(vec_elem, (shift_amount - 1), 1)}
            b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {bool_to_bits(((slice(vec_elem, (shift_amount - 1), 1) == BitDynamic::new(1, 0b1)) && ((slice(vec_elem, 0, (shift_amount - 1)) != zeros((__id(shift_amount) - 1))) || (slice(vec_elem, shift_amount, 1) == BitDynamic::new(1, 0b1)))))}
            b__2 if {(b__2 == BitDynamic::new(2, 0b10))} => {BitDynamic::new(1, 0b0)}
            _ => {bool_to_bits((!((slice(vec_elem, shift_amount, 1) == BitDynamic::new(1, 0b1))) && (slice(vec_elem, 0, shift_amount) != zeros(__id(shift_amount)))))}
            _ => {panic!("Unreachable code")}
        }
    }
}

/// unsigned_saturation
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L462-469.
pub fn unsigned_saturation(core_ctx: &mut Core, len: i128, elem: BitDynamic) -> BitDynamic {
    if {(elem.unsigned() > ones(__id(len)).unsigned())} {
        core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange(BitDynamic::new(1, 0b1), 0, 0);
        ones(__id(len))
    } else {
        subrange_bits(elem, (__id(len) - 1), 0)
    }
}

/// signed_saturation
///
/// Generated from the Sail sources at `riscv_insts_vext_utils.sail` L473-483.
pub fn signed_saturation(core_ctx: &mut Core, len: i128, elem: BitDynamic) -> BitDynamic {
    if {(elem.signed() > bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b0)), BitDynamic::from(ones((__id(len) - 1)))).signed())} {
        core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange(BitDynamic::new(1, 0b1), 0, 0);
        bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b0)), BitDynamic::from(ones((__id(len) - 1))))
    } else if {(elem.signed() < bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b1)), BitDynamic::from(zeros((__id(len) - 1)))).signed())} {
        core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange(BitDynamic::new(1, 0b1), 0, 0);
        bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b1)), BitDynamic::from(zeros((__id(len) - 1))))
    } else {
        subrange_bits(elem, (__id(len) - 1), 0)
    }
}

/// handle_illegal_vtype
///
/// Generated from the Sail sources at `riscv_insts_vext_vset.sail` L45-53.
pub fn handle_illegal_vtype(core_ctx: &mut Core, unit_arg: ()) {
    core_ctx.vtype.bits = bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b1)), BitDynamic::from(zeros(63)));
    core_ctx.vl = zeros(64)
}

/// vl_use_ceil
///
/// Generated from the Sail sources at `riscv_insts_vext_vset.sail` L55.
pub fn vl_use_ceil(core_ctx: &mut Core, unit_arg: ()) -> bool {
    core_ctx.config.extensions.V.vl_use_ceil
}

/// calculate_new_vl
///
/// Generated from the Sail sources at `riscv_insts_vext_vset.sail` L57-70.
pub fn calculate_new_vl(core_ctx: &mut Core, AVL: i128, VLMAX: i128) -> BitDynamic {
    let new_vl: i128 = if {(AVL <= VLMAX)} {
        AVL
    } else if {(AVL < (2 * VLMAX))} {
        if {vl_use_ceil(core_ctx, ())} {
            ((AVL + 1) / 2)
        } else {
            VLMAX
        }
    } else {
        VLMAX
    };
    to_bits(64, new_vl)
}

/// encdec_vvfunct6_forwards
///
/// Generated from the Sail sources.
pub fn encdec_vvfunct6_forwards(arg_hashtag_: vvfunct6) -> BitDynamic {
    match arg_hashtag_ {
        vvfunct6::VV_VADD => {BitDynamic::new(6, 0b000000)}
        vvfunct6::VV_VSUB => {BitDynamic::new(6, 0b000010)}
        vvfunct6::VV_VMINU => {BitDynamic::new(6, 0b000100)}
        vvfunct6::VV_VMIN => {BitDynamic::new(6, 0b000101)}
        vvfunct6::VV_VMAXU => {BitDynamic::new(6, 0b000110)}
        vvfunct6::VV_VMAX => {BitDynamic::new(6, 0b000111)}
        vvfunct6::VV_VAND => {BitDynamic::new(6, 0b001001)}
        vvfunct6::VV_VOR => {BitDynamic::new(6, 0b001010)}
        vvfunct6::VV_VXOR => {BitDynamic::new(6, 0b001011)}
        vvfunct6::VV_VRGATHER => {BitDynamic::new(6, 0b001100)}
        vvfunct6::VV_VRGATHEREI16 => {BitDynamic::new(6, 0b001110)}
        vvfunct6::VV_VSADDU => {BitDynamic::new(6, 0b100000)}
        vvfunct6::VV_VSADD => {BitDynamic::new(6, 0b100001)}
        vvfunct6::VV_VSSUBU => {BitDynamic::new(6, 0b100010)}
        vvfunct6::VV_VSSUB => {BitDynamic::new(6, 0b100011)}
        vvfunct6::VV_VSLL => {BitDynamic::new(6, 0b100101)}
        vvfunct6::VV_VSMUL => {BitDynamic::new(6, 0b100111)}
        vvfunct6::VV_VSRL => {BitDynamic::new(6, 0b101000)}
        vvfunct6::VV_VSRA => {BitDynamic::new(6, 0b101001)}
        vvfunct6::VV_VSSRL => {BitDynamic::new(6, 0b101010)}
        vvfunct6::VV_VSSRA => {BitDynamic::new(6, 0b101011)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vvfunct6_backwards(arg_hashtag_: BitDynamic) -> vvfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {vvfunct6::VV_VADD}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000010))} => {vvfunct6::VV_VSUB}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000100))} => {vvfunct6::VV_VMINU}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000101))} => {vvfunct6::VV_VMIN}
        b__4 if {(b__4 == BitDynamic::new(6, 0b000110))} => {vvfunct6::VV_VMAXU}
        b__5 if {(b__5 == BitDynamic::new(6, 0b000111))} => {vvfunct6::VV_VMAX}
        b__6 if {(b__6 == BitDynamic::new(6, 0b001001))} => {vvfunct6::VV_VAND}
        b__7 if {(b__7 == BitDynamic::new(6, 0b001010))} => {vvfunct6::VV_VOR}
        b__8 if {(b__8 == BitDynamic::new(6, 0b001011))} => {vvfunct6::VV_VXOR}
        b__9 if {(b__9 == BitDynamic::new(6, 0b001100))} => {vvfunct6::VV_VRGATHER}
        b__10 if {(b__10 == BitDynamic::new(6, 0b001110))} => {vvfunct6::VV_VRGATHEREI16}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100000))} => {vvfunct6::VV_VSADDU}
        b__12 if {(b__12 == BitDynamic::new(6, 0b100001))} => {vvfunct6::VV_VSADD}
        b__13 if {(b__13 == BitDynamic::new(6, 0b100010))} => {vvfunct6::VV_VSSUBU}
        b__14 if {(b__14 == BitDynamic::new(6, 0b100011))} => {vvfunct6::VV_VSSUB}
        b__15 if {(b__15 == BitDynamic::new(6, 0b100101))} => {vvfunct6::VV_VSLL}
        b__16 if {(b__16 == BitDynamic::new(6, 0b100111))} => {vvfunct6::VV_VSMUL}
        b__17 if {(b__17 == BitDynamic::new(6, 0b101000))} => {vvfunct6::VV_VSRL}
        b__18 if {(b__18 == BitDynamic::new(6, 0b101001))} => {vvfunct6::VV_VSRA}
        b__19 if {(b__19 == BitDynamic::new(6, 0b101010))} => {vvfunct6::VV_VSSRL}
        b__20 if {(b__20 == BitDynamic::new(6, 0b101011))} => {vvfunct6::VV_VSSRA}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vvfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000010))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000100))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000101))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b000110))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b000111))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b001001))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b001010))} => {true}
        b__8 if {(b__8 == BitDynamic::new(6, 0b001011))} => {true}
        b__9 if {(b__9 == BitDynamic::new(6, 0b001100))} => {true}
        b__10 if {(b__10 == BitDynamic::new(6, 0b001110))} => {true}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100000))} => {true}
        b__12 if {(b__12 == BitDynamic::new(6, 0b100001))} => {true}
        b__13 if {(b__13 == BitDynamic::new(6, 0b100010))} => {true}
        b__14 if {(b__14 == BitDynamic::new(6, 0b100011))} => {true}
        b__15 if {(b__15 == BitDynamic::new(6, 0b100101))} => {true}
        b__16 if {(b__16 == BitDynamic::new(6, 0b100111))} => {true}
        b__17 if {(b__17 == BitDynamic::new(6, 0b101000))} => {true}
        b__18 if {(b__18 == BitDynamic::new(6, 0b101001))} => {true}
        b__19 if {(b__19 == BitDynamic::new(6, 0b101010))} => {true}
        b__20 if {(b__20 == BitDynamic::new(6, 0b101011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nvsfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_nvsfunct6_backwards(arg_hashtag_: BitDynamic) -> nvsfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101100))} => {nvsfunct6::NVS_VNSRL}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101101))} => {nvsfunct6::NVS_VNSRA}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nvsfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_nvsfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101100))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101101))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nvfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_nvfunct6_backwards(arg_hashtag_: BitDynamic) -> nvfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101110))} => {nvfunct6::NV_VNCLIPU}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {nvfunct6::NV_VNCLIP}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nvfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_nvfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101110))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vxfunct6_backwards(arg_hashtag_: BitDynamic) -> vxfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {vxfunct6::VX_VADD}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000010))} => {vxfunct6::VX_VSUB}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000011))} => {vxfunct6::VX_VRSUB}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000100))} => {vxfunct6::VX_VMINU}
        b__4 if {(b__4 == BitDynamic::new(6, 0b000101))} => {vxfunct6::VX_VMIN}
        b__5 if {(b__5 == BitDynamic::new(6, 0b000110))} => {vxfunct6::VX_VMAXU}
        b__6 if {(b__6 == BitDynamic::new(6, 0b000111))} => {vxfunct6::VX_VMAX}
        b__7 if {(b__7 == BitDynamic::new(6, 0b001001))} => {vxfunct6::VX_VAND}
        b__8 if {(b__8 == BitDynamic::new(6, 0b001010))} => {vxfunct6::VX_VOR}
        b__9 if {(b__9 == BitDynamic::new(6, 0b001011))} => {vxfunct6::VX_VXOR}
        b__10 if {(b__10 == BitDynamic::new(6, 0b100000))} => {vxfunct6::VX_VSADDU}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100001))} => {vxfunct6::VX_VSADD}
        b__12 if {(b__12 == BitDynamic::new(6, 0b100010))} => {vxfunct6::VX_VSSUBU}
        b__13 if {(b__13 == BitDynamic::new(6, 0b100011))} => {vxfunct6::VX_VSSUB}
        b__14 if {(b__14 == BitDynamic::new(6, 0b100101))} => {vxfunct6::VX_VSLL}
        b__15 if {(b__15 == BitDynamic::new(6, 0b100111))} => {vxfunct6::VX_VSMUL}
        b__16 if {(b__16 == BitDynamic::new(6, 0b101000))} => {vxfunct6::VX_VSRL}
        b__17 if {(b__17 == BitDynamic::new(6, 0b101001))} => {vxfunct6::VX_VSRA}
        b__18 if {(b__18 == BitDynamic::new(6, 0b101010))} => {vxfunct6::VX_VSSRL}
        b__19 if {(b__19 == BitDynamic::new(6, 0b101011))} => {vxfunct6::VX_VSSRA}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vxfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000010))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000011))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000100))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b000101))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b000110))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b000111))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b001001))} => {true}
        b__8 if {(b__8 == BitDynamic::new(6, 0b001010))} => {true}
        b__9 if {(b__9 == BitDynamic::new(6, 0b001011))} => {true}
        b__10 if {(b__10 == BitDynamic::new(6, 0b100000))} => {true}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100001))} => {true}
        b__12 if {(b__12 == BitDynamic::new(6, 0b100010))} => {true}
        b__13 if {(b__13 == BitDynamic::new(6, 0b100011))} => {true}
        b__14 if {(b__14 == BitDynamic::new(6, 0b100101))} => {true}
        b__15 if {(b__15 == BitDynamic::new(6, 0b100111))} => {true}
        b__16 if {(b__16 == BitDynamic::new(6, 0b101000))} => {true}
        b__17 if {(b__17 == BitDynamic::new(6, 0b101001))} => {true}
        b__18 if {(b__18 == BitDynamic::new(6, 0b101010))} => {true}
        b__19 if {(b__19 == BitDynamic::new(6, 0b101011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nxsfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_nxsfunct6_backwards(arg_hashtag_: BitDynamic) -> nxsfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101100))} => {nxsfunct6::NXS_VNSRL}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101101))} => {nxsfunct6::NXS_VNSRA}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nxsfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_nxsfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101100))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101101))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nxfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_nxfunct6_backwards(arg_hashtag_: BitDynamic) -> nxfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101110))} => {nxfunct6::NX_VNCLIPU}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {nxfunct6::NX_VNCLIP}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nxfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_nxfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101110))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxsgfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vxsgfunct6_backwards(arg_hashtag_: BitDynamic) -> vxsgfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b001110))} => {vxsgfunct6::VX_VSLIDEUP}
        b__1 if {(b__1 == BitDynamic::new(6, 0b001111))} => {vxsgfunct6::VX_VSLIDEDOWN}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001100))} => {vxsgfunct6::VX_VRGATHER}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxsgfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vxsgfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b001110))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b001111))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001100))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vifunct6_forwards
///
/// Generated from the Sail sources.
pub fn encdec_vifunct6_forwards(arg_hashtag_: vifunct6) -> BitDynamic {
    match arg_hashtag_ {
        vifunct6::VI_VADD => {BitDynamic::new(6, 0b000000)}
        vifunct6::VI_VRSUB => {BitDynamic::new(6, 0b000011)}
        vifunct6::VI_VAND => {BitDynamic::new(6, 0b001001)}
        vifunct6::VI_VOR => {BitDynamic::new(6, 0b001010)}
        vifunct6::VI_VXOR => {BitDynamic::new(6, 0b001011)}
        vifunct6::VI_VSADDU => {BitDynamic::new(6, 0b100000)}
        vifunct6::VI_VSADD => {BitDynamic::new(6, 0b100001)}
        vifunct6::VI_VSLL => {BitDynamic::new(6, 0b100101)}
        vifunct6::VI_VSRL => {BitDynamic::new(6, 0b101000)}
        vifunct6::VI_VSRA => {BitDynamic::new(6, 0b101001)}
        vifunct6::VI_VSSRL => {BitDynamic::new(6, 0b101010)}
        vifunct6::VI_VSSRA => {BitDynamic::new(6, 0b101011)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vifunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vifunct6_backwards(arg_hashtag_: BitDynamic) -> vifunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {vifunct6::VI_VADD}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000011))} => {vifunct6::VI_VRSUB}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001001))} => {vifunct6::VI_VAND}
        b__3 if {(b__3 == BitDynamic::new(6, 0b001010))} => {vifunct6::VI_VOR}
        b__4 if {(b__4 == BitDynamic::new(6, 0b001011))} => {vifunct6::VI_VXOR}
        b__5 if {(b__5 == BitDynamic::new(6, 0b100000))} => {vifunct6::VI_VSADDU}
        b__6 if {(b__6 == BitDynamic::new(6, 0b100001))} => {vifunct6::VI_VSADD}
        b__7 if {(b__7 == BitDynamic::new(6, 0b100101))} => {vifunct6::VI_VSLL}
        b__8 if {(b__8 == BitDynamic::new(6, 0b101000))} => {vifunct6::VI_VSRL}
        b__9 if {(b__9 == BitDynamic::new(6, 0b101001))} => {vifunct6::VI_VSRA}
        b__10 if {(b__10 == BitDynamic::new(6, 0b101010))} => {vifunct6::VI_VSSRL}
        b__11 if {(b__11 == BitDynamic::new(6, 0b101011))} => {vifunct6::VI_VSSRA}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vifunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vifunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000011))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001001))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b001010))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b001011))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b100000))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b100001))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b100101))} => {true}
        b__8 if {(b__8 == BitDynamic::new(6, 0b101000))} => {true}
        b__9 if {(b__9 == BitDynamic::new(6, 0b101001))} => {true}
        b__10 if {(b__10 == BitDynamic::new(6, 0b101010))} => {true}
        b__11 if {(b__11 == BitDynamic::new(6, 0b101011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nisfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_nisfunct6_backwards(arg_hashtag_: BitDynamic) -> nisfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101100))} => {nisfunct6::NIS_VNSRL}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101101))} => {nisfunct6::NIS_VNSRA}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nisfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_nisfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101100))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101101))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nifunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_nifunct6_backwards(arg_hashtag_: BitDynamic) -> nifunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101110))} => {nifunct6::NI_VNCLIPU}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {nifunct6::NI_VNCLIP}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_nifunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_nifunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101110))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_visgfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_visgfunct6_backwards(arg_hashtag_: BitDynamic) -> visgfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b001110))} => {visgfunct6::VI_VSLIDEUP}
        b__1 if {(b__1 == BitDynamic::new(6, 0b001111))} => {visgfunct6::VI_VSLIDEDOWN}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001100))} => {visgfunct6::VI_VRGATHER}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_visgfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_visgfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b001110))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b001111))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001100))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mvvfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_mvvfunct6_backwards(arg_hashtag_: BitDynamic) -> mvvfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b001000))} => {mvvfunct6::MVV_VAADDU}
        b__1 if {(b__1 == BitDynamic::new(6, 0b001001))} => {mvvfunct6::MVV_VAADD}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001010))} => {mvvfunct6::MVV_VASUBU}
        b__3 if {(b__3 == BitDynamic::new(6, 0b001011))} => {mvvfunct6::MVV_VASUB}
        b__4 if {(b__4 == BitDynamic::new(6, 0b100101))} => {mvvfunct6::MVV_VMUL}
        b__5 if {(b__5 == BitDynamic::new(6, 0b100111))} => {mvvfunct6::MVV_VMULH}
        b__6 if {(b__6 == BitDynamic::new(6, 0b100100))} => {mvvfunct6::MVV_VMULHU}
        b__7 if {(b__7 == BitDynamic::new(6, 0b100110))} => {mvvfunct6::MVV_VMULHSU}
        b__8 if {(b__8 == BitDynamic::new(6, 0b100000))} => {mvvfunct6::MVV_VDIVU}
        b__9 if {(b__9 == BitDynamic::new(6, 0b100001))} => {mvvfunct6::MVV_VDIV}
        b__10 if {(b__10 == BitDynamic::new(6, 0b100010))} => {mvvfunct6::MVV_VREMU}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100011))} => {mvvfunct6::MVV_VREM}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mvvfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_mvvfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b001000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b001001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b001011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b100101))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b100111))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b100100))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b100110))} => {true}
        b__8 if {(b__8 == BitDynamic::new(6, 0b100000))} => {true}
        b__9 if {(b__9 == BitDynamic::new(6, 0b100001))} => {true}
        b__10 if {(b__10 == BitDynamic::new(6, 0b100010))} => {true}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mvvmafunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_mvvmafunct6_backwards(arg_hashtag_: BitDynamic) -> mvvmafunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101101))} => {mvvmafunct6::MVV_VMACC}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {mvvmafunct6::MVV_VNMSAC}
        b__2 if {(b__2 == BitDynamic::new(6, 0b101001))} => {mvvmafunct6::MVV_VMADD}
        b__3 if {(b__3 == BitDynamic::new(6, 0b101011))} => {mvvmafunct6::MVV_VNMSUB}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mvvmafunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_mvvmafunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101101))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b101001))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b101011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// vext2_vs1_backwards
///
/// Generated from the Sail sources.
pub fn vext2_vs1_backwards(arg_hashtag_: BitDynamic) -> vext2funct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00110))} => {vext2funct6::VEXT2_ZVF2}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00111))} => {vext2funct6::VEXT2_SVF2}
        _ => {panic!("Unreachable code")}
    }
}

/// vext2_vs1_backwards_matches
///
/// Generated from the Sail sources.
pub fn vext2_vs1_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00110))} => {true}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// vext4_vs1_backwards
///
/// Generated from the Sail sources.
pub fn vext4_vs1_backwards(arg_hashtag_: BitDynamic) -> vext4funct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00100))} => {vext4funct6::VEXT4_ZVF4}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00101))} => {vext4funct6::VEXT4_SVF4}
        _ => {panic!("Unreachable code")}
    }
}

/// vext4_vs1_backwards_matches
///
/// Generated from the Sail sources.
pub fn vext4_vs1_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00100))} => {true}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00101))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// vext8_vs1_backwards
///
/// Generated from the Sail sources.
pub fn vext8_vs1_backwards(arg_hashtag_: BitDynamic) -> vext8funct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00010))} => {vext8funct6::VEXT8_ZVF8}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00011))} => {vext8funct6::VEXT8_SVF8}
        _ => {panic!("Unreachable code")}
    }
}

/// vext8_vs1_backwards_matches
///
/// Generated from the Sail sources.
pub fn vext8_vs1_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00010))} => {true}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mvxfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_mvxfunct6_backwards(arg_hashtag_: BitDynamic) -> mvxfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b001000))} => {mvxfunct6::MVX_VAADDU}
        b__1 if {(b__1 == BitDynamic::new(6, 0b001001))} => {mvxfunct6::MVX_VAADD}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001010))} => {mvxfunct6::MVX_VASUBU}
        b__3 if {(b__3 == BitDynamic::new(6, 0b001011))} => {mvxfunct6::MVX_VASUB}
        b__4 if {(b__4 == BitDynamic::new(6, 0b001110))} => {mvxfunct6::MVX_VSLIDE1UP}
        b__5 if {(b__5 == BitDynamic::new(6, 0b001111))} => {mvxfunct6::MVX_VSLIDE1DOWN}
        b__6 if {(b__6 == BitDynamic::new(6, 0b100101))} => {mvxfunct6::MVX_VMUL}
        b__7 if {(b__7 == BitDynamic::new(6, 0b100111))} => {mvxfunct6::MVX_VMULH}
        b__8 if {(b__8 == BitDynamic::new(6, 0b100100))} => {mvxfunct6::MVX_VMULHU}
        b__9 if {(b__9 == BitDynamic::new(6, 0b100110))} => {mvxfunct6::MVX_VMULHSU}
        b__10 if {(b__10 == BitDynamic::new(6, 0b100000))} => {mvxfunct6::MVX_VDIVU}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100001))} => {mvxfunct6::MVX_VDIV}
        b__12 if {(b__12 == BitDynamic::new(6, 0b100010))} => {mvxfunct6::MVX_VREMU}
        b__13 if {(b__13 == BitDynamic::new(6, 0b100011))} => {mvxfunct6::MVX_VREM}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mvxfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_mvxfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b001000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b001001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b001010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b001011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b001110))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b001111))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b100101))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b100111))} => {true}
        b__8 if {(b__8 == BitDynamic::new(6, 0b100100))} => {true}
        b__9 if {(b__9 == BitDynamic::new(6, 0b100110))} => {true}
        b__10 if {(b__10 == BitDynamic::new(6, 0b100000))} => {true}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100001))} => {true}
        b__12 if {(b__12 == BitDynamic::new(6, 0b100010))} => {true}
        b__13 if {(b__13 == BitDynamic::new(6, 0b100011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mvxmafunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_mvxmafunct6_backwards(arg_hashtag_: BitDynamic) -> mvxmafunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101101))} => {mvxmafunct6::MVX_VMACC}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {mvxmafunct6::MVX_VNMSAC}
        b__2 if {(b__2 == BitDynamic::new(6, 0b101001))} => {mvxmafunct6::MVX_VMADD}
        b__3 if {(b__3 == BitDynamic::new(6, 0b101011))} => {mvxmafunct6::MVX_VNMSUB}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mvxmafunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_mvxmafunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101101))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b101001))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b101011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvvfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_fvvfunct6_backwards(arg_hashtag_: BitDynamic) -> fvvfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {fvvfunct6::FVV_VADD}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000010))} => {fvvfunct6::FVV_VSUB}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000100))} => {fvvfunct6::FVV_VMIN}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000110))} => {fvvfunct6::FVV_VMAX}
        b__4 if {(b__4 == BitDynamic::new(6, 0b001000))} => {fvvfunct6::FVV_VSGNJ}
        b__5 if {(b__5 == BitDynamic::new(6, 0b001001))} => {fvvfunct6::FVV_VSGNJN}
        b__6 if {(b__6 == BitDynamic::new(6, 0b001010))} => {fvvfunct6::FVV_VSGNJX}
        b__7 if {(b__7 == BitDynamic::new(6, 0b100000))} => {fvvfunct6::FVV_VDIV}
        b__8 if {(b__8 == BitDynamic::new(6, 0b100100))} => {fvvfunct6::FVV_VMUL}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvvfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_fvvfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000010))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000100))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000110))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b001000))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b001001))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b001010))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b100000))} => {true}
        b__8 if {(b__8 == BitDynamic::new(6, 0b100100))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvvmafunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_fvvmafunct6_backwards(arg_hashtag_: BitDynamic) -> fvvmafunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101000))} => {fvvmafunct6::FVV_VMADD}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101001))} => {fvvmafunct6::FVV_VNMADD}
        b__2 if {(b__2 == BitDynamic::new(6, 0b101010))} => {fvvmafunct6::FVV_VMSUB}
        b__3 if {(b__3 == BitDynamic::new(6, 0b101011))} => {fvvmafunct6::FVV_VNMSUB}
        b__4 if {(b__4 == BitDynamic::new(6, 0b101100))} => {fvvmafunct6::FVV_VMACC}
        b__5 if {(b__5 == BitDynamic::new(6, 0b101101))} => {fvvmafunct6::FVV_VNMACC}
        b__6 if {(b__6 == BitDynamic::new(6, 0b101110))} => {fvvmafunct6::FVV_VMSAC}
        b__7 if {(b__7 == BitDynamic::new(6, 0b101111))} => {fvvmafunct6::FVV_VNMSAC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvvmafunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_fvvmafunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b101010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b101011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b101100))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b101101))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b101110))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b101111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vfunary0_vs1_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vfunary0_vs1_backwards(arg_hashtag_: BitDynamic) -> vfunary0 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00000))} => {vfunary0::FV_CVT_XU_F}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00001))} => {vfunary0::FV_CVT_X_F}
        b__2 if {(b__2 == BitDynamic::new(5, 0b00010))} => {vfunary0::FV_CVT_F_XU}
        b__3 if {(b__3 == BitDynamic::new(5, 0b00011))} => {vfunary0::FV_CVT_F_X}
        b__4 if {(b__4 == BitDynamic::new(5, 0b00110))} => {vfunary0::FV_CVT_RTZ_XU_F}
        b__5 if {(b__5 == BitDynamic::new(5, 0b00111))} => {vfunary0::FV_CVT_RTZ_X_F}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vfunary0_vs1_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vfunary0_vs1_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(5, 0b00010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(5, 0b00011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(5, 0b00110))} => {true}
        b__5 if {(b__5 == BitDynamic::new(5, 0b00111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vfnunary0_vs1_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vfnunary0_vs1_backwards(arg_hashtag_: BitDynamic) -> vfnunary0 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b10000))} => {vfnunary0::FNV_CVT_XU_F}
        b__1 if {(b__1 == BitDynamic::new(5, 0b10001))} => {vfnunary0::FNV_CVT_X_F}
        b__2 if {(b__2 == BitDynamic::new(5, 0b10010))} => {vfnunary0::FNV_CVT_F_XU}
        b__3 if {(b__3 == BitDynamic::new(5, 0b10011))} => {vfnunary0::FNV_CVT_F_X}
        b__4 if {(b__4 == BitDynamic::new(5, 0b10100))} => {vfnunary0::FNV_CVT_F_F}
        b__5 if {(b__5 == BitDynamic::new(5, 0b10101))} => {vfnunary0::FNV_CVT_ROD_F_F}
        b__6 if {(b__6 == BitDynamic::new(5, 0b10110))} => {vfnunary0::FNV_CVT_RTZ_XU_F}
        b__7 if {(b__7 == BitDynamic::new(5, 0b10111))} => {vfnunary0::FNV_CVT_RTZ_X_F}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vfnunary0_vs1_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vfnunary0_vs1_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b10000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(5, 0b10001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(5, 0b10010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(5, 0b10011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(5, 0b10100))} => {true}
        b__5 if {(b__5 == BitDynamic::new(5, 0b10101))} => {true}
        b__6 if {(b__6 == BitDynamic::new(5, 0b10110))} => {true}
        b__7 if {(b__7 == BitDynamic::new(5, 0b10111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vfunary1_vs1_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vfunary1_vs1_backwards(arg_hashtag_: BitDynamic) -> vfunary1 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00000))} => {vfunary1::FVV_VSQRT}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00100))} => {vfunary1::FVV_VRSQRT7}
        b__2 if {(b__2 == BitDynamic::new(5, 0b00101))} => {vfunary1::FVV_VREC7}
        b__3 if {(b__3 == BitDynamic::new(5, 0b10000))} => {vfunary1::FVV_VCLASS}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vfunary1_vs1_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vfunary1_vs1_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(5, 0b00000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(5, 0b00100))} => {true}
        b__2 if {(b__2 == BitDynamic::new(5, 0b00101))} => {true}
        b__3 if {(b__3 == BitDynamic::new(5, 0b10000))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvffunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_fvffunct6_backwards(arg_hashtag_: BitDynamic) -> fvffunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {fvffunct6::VF_VADD}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000010))} => {fvffunct6::VF_VSUB}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000100))} => {fvffunct6::VF_VMIN}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000110))} => {fvffunct6::VF_VMAX}
        b__4 if {(b__4 == BitDynamic::new(6, 0b001000))} => {fvffunct6::VF_VSGNJ}
        b__5 if {(b__5 == BitDynamic::new(6, 0b001001))} => {fvffunct6::VF_VSGNJN}
        b__6 if {(b__6 == BitDynamic::new(6, 0b001010))} => {fvffunct6::VF_VSGNJX}
        b__7 if {(b__7 == BitDynamic::new(6, 0b001110))} => {fvffunct6::VF_VSLIDE1UP}
        b__8 if {(b__8 == BitDynamic::new(6, 0b001111))} => {fvffunct6::VF_VSLIDE1DOWN}
        b__9 if {(b__9 == BitDynamic::new(6, 0b100000))} => {fvffunct6::VF_VDIV}
        b__10 if {(b__10 == BitDynamic::new(6, 0b100001))} => {fvffunct6::VF_VRDIV}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100100))} => {fvffunct6::VF_VMUL}
        b__12 if {(b__12 == BitDynamic::new(6, 0b100111))} => {fvffunct6::VF_VRSUB}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvffunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_fvffunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000010))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000100))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000110))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b001000))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b001001))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b001010))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b001110))} => {true}
        b__8 if {(b__8 == BitDynamic::new(6, 0b001111))} => {true}
        b__9 if {(b__9 == BitDynamic::new(6, 0b100000))} => {true}
        b__10 if {(b__10 == BitDynamic::new(6, 0b100001))} => {true}
        b__11 if {(b__11 == BitDynamic::new(6, 0b100100))} => {true}
        b__12 if {(b__12 == BitDynamic::new(6, 0b100111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvfmafunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_fvfmafunct6_backwards(arg_hashtag_: BitDynamic) -> fvfmafunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101000))} => {fvfmafunct6::VF_VMADD}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101001))} => {fvfmafunct6::VF_VNMADD}
        b__2 if {(b__2 == BitDynamic::new(6, 0b101010))} => {fvfmafunct6::VF_VMSUB}
        b__3 if {(b__3 == BitDynamic::new(6, 0b101011))} => {fvfmafunct6::VF_VNMSUB}
        b__4 if {(b__4 == BitDynamic::new(6, 0b101100))} => {fvfmafunct6::VF_VMACC}
        b__5 if {(b__5 == BitDynamic::new(6, 0b101101))} => {fvfmafunct6::VF_VNMACC}
        b__6 if {(b__6 == BitDynamic::new(6, 0b101110))} => {fvfmafunct6::VF_VMSAC}
        b__7 if {(b__7 == BitDynamic::new(6, 0b101111))} => {fvfmafunct6::VF_VNMSAC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvfmafunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_fvfmafunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b101010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b101011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b101100))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b101101))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b101110))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b101111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vlewidth_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vlewidth_backwards(arg_hashtag_: BitDynamic) -> vlewidth {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {vlewidth::VLE8}
        b__1 if {(b__1 == BitDynamic::new(3, 0b101))} => {vlewidth::VLE16}
        b__2 if {(b__2 == BitDynamic::new(3, 0b110))} => {vlewidth::VLE32}
        b__3 if {(b__3 == BitDynamic::new(3, 0b111))} => {vlewidth::VLE64}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vlewidth_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vlewidth_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(3, 0b000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(3, 0b101))} => {true}
        b__2 if {(b__2 == BitDynamic::new(3, 0b110))} => {true}
        b__3 if {(b__3 == BitDynamic::new(3, 0b111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_lsop_backwards
///
/// Generated from the Sail sources.
pub fn encdec_lsop_backwards(arg_hashtag_: BitDynamic) -> vmlsop {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(7, 0b0000111))} => {vmlsop::VLM}
        b__1 if {(b__1 == BitDynamic::new(7, 0b0100111))} => {vmlsop::VSM}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_lsop_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_lsop_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(7, 0b0000111))} => {true}
        b__1 if {(b__1 == BitDynamic::new(7, 0b0100111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mmfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_mmfunct6_backwards(arg_hashtag_: BitDynamic) -> mmfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011001))} => {mmfunct6::MM_VMAND}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011101))} => {mmfunct6::MM_VMNAND}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011000))} => {mmfunct6::MM_VMANDN}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011011))} => {mmfunct6::MM_VMXOR}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011010))} => {mmfunct6::MM_VMOR}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011110))} => {mmfunct6::MM_VMNOR}
        b__6 if {(b__6 == BitDynamic::new(6, 0b011100))} => {mmfunct6::MM_VMORN}
        b__7 if {(b__7 == BitDynamic::new(6, 0b011111))} => {mmfunct6::MM_VMXNOR}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_mmfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_mmfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011001))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011101))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011000))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011010))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011110))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b011100))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b011111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvmfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vvmfunct6_backwards(arg_hashtag_: BitDynamic) -> vvmfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {vvmfunct6::VVM_VMADC}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010011))} => {vvmfunct6::VVM_VMSBC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvmfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vvmfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvmcfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vvmcfunct6_backwards(arg_hashtag_: BitDynamic) -> vvmcfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {vvmcfunct6::VVMC_VMADC}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010011))} => {vvmcfunct6::VVMC_VMSBC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvmcfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vvmcfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvmsfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vvmsfunct6_backwards(arg_hashtag_: BitDynamic) -> vvmsfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010000))} => {vvmsfunct6::VVMS_VADC}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010010))} => {vvmsfunct6::VVMS_VSBC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvmsfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vvmsfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010010))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvcmpfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vvcmpfunct6_backwards(arg_hashtag_: BitDynamic) -> vvcmpfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {vvcmpfunct6::VVCMP_VMSEQ}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {vvcmpfunct6::VVCMP_VMSNE}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011010))} => {vvcmpfunct6::VVCMP_VMSLTU}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011011))} => {vvcmpfunct6::VVCMP_VMSLT}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011100))} => {vvcmpfunct6::VVCMP_VMSLEU}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011101))} => {vvcmpfunct6::VVCMP_VMSLE}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vvcmpfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vvcmpfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011100))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011101))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxmfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vxmfunct6_backwards(arg_hashtag_: BitDynamic) -> vxmfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {vxmfunct6::VXM_VMADC}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010011))} => {vxmfunct6::VXM_VMSBC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxmfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vxmfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxmcfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vxmcfunct6_backwards(arg_hashtag_: BitDynamic) -> vxmcfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {vxmcfunct6::VXMC_VMADC}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010011))} => {vxmcfunct6::VXMC_VMSBC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxmcfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vxmcfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010011))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxmsfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vxmsfunct6_backwards(arg_hashtag_: BitDynamic) -> vxmsfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010000))} => {vxmsfunct6::VXMS_VADC}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010010))} => {vxmsfunct6::VXMS_VSBC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxmsfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vxmsfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b010010))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxcmpfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vxcmpfunct6_backwards(arg_hashtag_: BitDynamic) -> vxcmpfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {vxcmpfunct6::VXCMP_VMSEQ}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {vxcmpfunct6::VXCMP_VMSNE}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011010))} => {vxcmpfunct6::VXCMP_VMSLTU}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011011))} => {vxcmpfunct6::VXCMP_VMSLT}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011100))} => {vxcmpfunct6::VXCMP_VMSLEU}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011101))} => {vxcmpfunct6::VXCMP_VMSLE}
        b__6 if {(b__6 == BitDynamic::new(6, 0b011110))} => {vxcmpfunct6::VXCMP_VMSGTU}
        b__7 if {(b__7 == BitDynamic::new(6, 0b011111))} => {vxcmpfunct6::VXCMP_VMSGT}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vxcmpfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vxcmpfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011100))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011101))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b011110))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b011111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vimfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vimfunct6_backwards(arg_hashtag_: BitDynamic) -> vimfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {vimfunct6::VIM_VMADC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vimfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vimfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vimcfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vimcfunct6_backwards(arg_hashtag_: BitDynamic) -> vimcfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {vimcfunct6::VIMC_VMADC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vimcfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vimcfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010001))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vimsfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vimsfunct6_backwards(arg_hashtag_: BitDynamic) -> vimsfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010000))} => {vimsfunct6::VIMS_VADC}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vimsfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vimsfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b010000))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vicmpfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_vicmpfunct6_backwards(arg_hashtag_: BitDynamic) -> vicmpfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {vicmpfunct6::VICMP_VMSEQ}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {vicmpfunct6::VICMP_VMSNE}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011100))} => {vicmpfunct6::VICMP_VMSLEU}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011101))} => {vicmpfunct6::VICMP_VMSLE}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011110))} => {vicmpfunct6::VICMP_VMSGTU}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011111))} => {vicmpfunct6::VICMP_VMSGT}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_vicmpfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_vicmpfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011100))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011101))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011110))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvvmfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_fvvmfunct6_backwards(arg_hashtag_: BitDynamic) -> fvvmfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {fvvmfunct6::FVVM_VMFEQ}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {fvvmfunct6::FVVM_VMFLE}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011011))} => {fvvmfunct6::FVVM_VMFLT}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011100))} => {fvvmfunct6::FVVM_VMFNE}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvvmfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_fvvmfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011011))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011100))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvfmfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_fvfmfunct6_backwards(arg_hashtag_: BitDynamic) -> fvfmfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {fvfmfunct6::VFM_VMFEQ}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {fvfmfunct6::VFM_VMFLE}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011011))} => {fvfmfunct6::VFM_VMFLT}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011100))} => {fvfmfunct6::VFM_VMFNE}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011101))} => {fvfmfunct6::VFM_VMFGT}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011111))} => {fvfmfunct6::VFM_VMFGE}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_fvfmfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_fvfmfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b011000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b011001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b011011))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b011100))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b011101))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b011111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_rmvvfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_rmvvfunct6_backwards(arg_hashtag_: BitDynamic) -> rmvvfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {rmvvfunct6::MVV_VREDSUM}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000001))} => {rmvvfunct6::MVV_VREDAND}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000010))} => {rmvvfunct6::MVV_VREDOR}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000011))} => {rmvvfunct6::MVV_VREDXOR}
        b__4 if {(b__4 == BitDynamic::new(6, 0b000100))} => {rmvvfunct6::MVV_VREDMINU}
        b__5 if {(b__5 == BitDynamic::new(6, 0b000101))} => {rmvvfunct6::MVV_VREDMIN}
        b__6 if {(b__6 == BitDynamic::new(6, 0b000110))} => {rmvvfunct6::MVV_VREDMAXU}
        b__7 if {(b__7 == BitDynamic::new(6, 0b000111))} => {rmvvfunct6::MVV_VREDMAX}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_rmvvfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_rmvvfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b000000))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b000001))} => {true}
        b__2 if {(b__2 == BitDynamic::new(6, 0b000010))} => {true}
        b__3 if {(b__3 == BitDynamic::new(6, 0b000011))} => {true}
        b__4 if {(b__4 == BitDynamic::new(6, 0b000100))} => {true}
        b__5 if {(b__5 == BitDynamic::new(6, 0b000101))} => {true}
        b__6 if {(b__6 == BitDynamic::new(6, 0b000110))} => {true}
        b__7 if {(b__7 == BitDynamic::new(6, 0b000111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// cbo_clean_flush_enabled
///
/// Generated from the Sail sources at `riscv_insts_zicbom.sail` L13.
pub fn cbo_clean_flush_enabled(core_ctx: &mut Core, p: Privilege) -> bool {
    let var_1: bool = bitvector_access({
        let var_4: MEnvcfg = core_ctx.menvcfg;
        _get_MEnvcfg_CBCFE(var_4)
    }, 0);
    let var_2: bool = bitvector_access({
        let var_3: SEnvcfg = core_ctx.senvcfg;
        _get_SEnvcfg_CBCFE(var_3)
    }, 0);
    feature_enabled_for_priv(core_ctx, p, var_1, var_2)
}

/// encdec_cbop_forwards
///
/// Generated from the Sail sources.
pub fn encdec_cbop_forwards(arg_hashtag_: cbop_zicbom) -> BitDynamic {
    match arg_hashtag_ {
        cbop_zicbom::CBO_CLEAN => {BitDynamic::new(12, 0b000000000001)}
        cbop_zicbom::CBO_FLUSH => {BitDynamic::new(12, 0b000000000010)}
        cbop_zicbom::CBO_INVAL => {BitDynamic::new(12, 0b000000000000)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_cbop_backwards
///
/// Generated from the Sail sources.
pub fn encdec_cbop_backwards(arg_hashtag_: BitDynamic) -> cbop_zicbom {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(12, 0b000000000001))} => {cbop_zicbom::CBO_CLEAN}
        b__1 if {(b__1 == BitDynamic::new(12, 0b000000000010))} => {cbop_zicbom::CBO_FLUSH}
        b__2 if {(b__2 == BitDynamic::new(12, 0b000000000000))} => {cbop_zicbom::CBO_INVAL}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_cbop_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_cbop_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(12, 0b000000000001))} => {true}
        b__1 if {(b__1 == BitDynamic::new(12, 0b000000000010))} => {true}
        b__2 if {(b__2 == BitDynamic::new(12, 0b000000000000))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// cbie
///
/// Generated from the Sail sources at `riscv_insts_zicbom.sail` L38.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum cbie {
    CBIE_ILLEGAL,
    CBIE_EXEC_FLUSH,
    CBIE_EXEC_INVAL
}

/// encdec_cbie_backwards
///
/// Generated from the Sail sources.
pub fn encdec_cbie_backwards(arg_hashtag_: BitDynamic) -> cbie {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(2, 0b00))} => {cbie::CBIE_ILLEGAL}
        b__1 if {(b__1 == BitDynamic::new(2, 0b01))} => {cbie::CBIE_EXEC_FLUSH}
        b__2 if {(b__2 == BitDynamic::new(2, 0b11))} => {cbie::CBIE_EXEC_INVAL}
        _ => {panic!("{}, l {}: {}", "riscv_insts_zicbom.sail", 44, "reserved CBIE")}
        _ => {panic!("Unreachable code")}
    }
}

/// checked_cbop
///
/// Generated from the Sail sources at `riscv_insts_zicbom.sail` L48.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum checked_cbop {
    CBOP_ILLEGAL,
    CBOP_ILLEGAL_VIRTUAL,
    CBOP_INVAL_FLUSH,
    CBOP_INVAL_INVAL
}

/// cbop_priv_check
///
/// Generated from the Sail sources at `riscv_insts_zicbom.sail` L51-64.
pub fn cbop_priv_check(core_ctx: &mut Core, p: Privilege) -> checked_cbop {
    let mCBIE: cbie = {
        let var_5: BitDynamic = {
            let var_6: MEnvcfg = core_ctx.menvcfg;
            _get_MEnvcfg_CBIE(var_6)
        };
        encdec_cbie_backwards(var_5)
    };
    let sCBIE: cbie = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
        {
            let var_3: BitDynamic = {
                let var_4: SEnvcfg = core_ctx.senvcfg;
                _get_SEnvcfg_CBIE(var_4)
            };
            encdec_cbie_backwards(var_3)
        }
    } else {
        {
            let var_1: BitDynamic = {
                let var_2: MEnvcfg = core_ctx.menvcfg;
                _get_MEnvcfg_CBIE(var_2)
            };
            encdec_cbie_backwards(var_1)
        }
    };
    match (p, mCBIE, sCBIE) {
        (Privilege::Machine, _, _) => {checked_cbop::CBOP_INVAL_INVAL}
        (_, cbie::CBIE_ILLEGAL, _) => {checked_cbop::CBOP_ILLEGAL}
        (Privilege::User, _, cbie::CBIE_ILLEGAL) => {checked_cbop::CBOP_ILLEGAL}
        (_, cbie::CBIE_EXEC_FLUSH, _) => {checked_cbop::CBOP_INVAL_FLUSH}
        (Privilege::User, _, cbie::CBIE_EXEC_FLUSH) => {checked_cbop::CBOP_INVAL_FLUSH}
        _ => {checked_cbop::CBOP_INVAL_INVAL}
        _ => {panic!("Unreachable code")}
    }
}

/// process_clean_inval
///
/// Generated from the Sail sources at `riscv_insts_zicbom.sail` L67-136.
pub fn process_clean_inval(core_ctx: &mut Core, rs1: regidx, cbop: cbop_zicbom) -> ExecutionResult {
    let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
    let cache_block_size_exp: i128 = plat_cache_block_size_exp(core_ctx, ());
    let cache_block_size: i128 = i128::pow(2, (cache_block_size_exp as u32));
    let negative_offset: BitDynamic = sub_vec((rs1_val & !(ones(cache_block_size_exp).zero_extend_dyn(64))), rs1_val);
    match ext_data_get_addr(core_ctx, rs1, negative_offset, AccessType::Read(Data), cache_block_size) {
        Ext_DataAddr_Check::Ext_DataAddr_Error(e) => {ExecutionResult::Ext_DataAddr_Check_Failure(e)}
        Ext_DataAddr_Check::Ext_DataAddr_OK(vaddr) => {{
            let res: Option<ExceptionType> = match panic!("Unsupported function: 'translateAddr'") {
                TR_Result::TR_Address((paddr, _)) => {{
                    let ep: Privilege = {
                        let var_1: Mstatus = core_ctx.mstatus;
                        let var_2: Privilege = core_ctx.cur_privilege;
                        effectivePrivilege(AccessType::Read(Data), var_1, var_2)
                    };
                    let exc_read: Option<ExceptionType> = phys_access_check(core_ctx, AccessType::Read(Data), ep, paddr, cache_block_size);
                    let exc_write: Option<ExceptionType> = phys_access_check(core_ctx, AccessType::Write(Data), ep, paddr, cache_block_size);
                    match (exc_read, exc_write) {
                        (Some(exc_read), Some(exc_write)) => {Some(exc_write)}
                        _ => {None}
                        _ => {panic!("Unreachable code")}
                    }
                }}
                TR_Result::TR_Failure((e, _)) => {Some(e)}
                _ => {panic!("Unreachable code")}
            };
            match res {
                None => {RETIRE_SUCCESS}
                Some(e) => {{
                    let e: ExceptionType = match e {
                        ExceptionType::E_Load_Access_Fault(()) => {ExceptionType::E_SAMO_Access_Fault(())}
                        ExceptionType::E_SAMO_Access_Fault(()) => {ExceptionType::E_SAMO_Access_Fault(())}
                        ExceptionType::E_Load_Page_Fault(()) => {ExceptionType::E_SAMO_Page_Fault(())}
                        ExceptionType::E_SAMO_Page_Fault(()) => {ExceptionType::E_SAMO_Page_Fault(())}
                        _ => {panic!("{}, l {}: {}", "riscv_insts_zicbom.sail", 125, "unexpected exception for cmo.clean/inval")}
                        _ => {panic!("Unreachable code")}
                    };
                    ExecutionResult::Memory_Exception((sub_virtaddr_xlenbits(vaddr, negative_offset), e))
                }}
                _ => {panic!("Unreachable code")}
            }
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_zvkfunct6_backwards
///
/// Generated from the Sail sources.
pub fn encdec_zvkfunct6_backwards(arg_hashtag_: BitDynamic) -> zvkfunct6 {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101110))} => {zvkfunct6::ZVK_VSHA2CH}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {zvkfunct6::ZVK_VSHA2CL}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_zvkfunct6_backwards_matches
///
/// Generated from the Sail sources.
pub fn encdec_zvkfunct6_backwards_matches(arg_hashtag_: BitDynamic) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitDynamic::new(6, 0b101110))} => {true}
        b__1 if {(b__1 == BitDynamic::new(6, 0b101111))} => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_forwards
///
/// Generated from the Sail sources.
pub fn encdec_forwards(core_ctx: &mut Core, arg_hashtag_: ast) -> BitDynamic {
    match arg_hashtag_ {
        ast::UTYPE((imm, rd, op)) => {bitvector_concat(BitDynamic::from((imm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(encdec_uop_forwards(op)))))}
        ast::JAL((v__2, rd)) if {(v__2.subrange::<0, 1, 1>() == BitDynamic::new(1, 0b0))} => {let imm_19: BitDynamic = v__2.subrange::<20, 21, 1>();
        let imm_8: BitDynamic = v__2.subrange::<11, 12, 1>();
        let imm_7_0: BitDynamic = v__2.subrange::<12, 20, 8>();
        let imm_19: BitDynamic = v__2.subrange::<20, 21, 1>();
        let imm_18_13: BitDynamic = v__2.subrange::<5, 11, 6>();
        let imm_12_9: BitDynamic = v__2.subrange::<1, 5, 4>();
        bitvector_concat(BitDynamic::from((imm_19 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((imm_18_13 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((imm_12_9 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((imm_8 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((imm_7_0 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1101111)))))))))))))}
        ast::JALR((imm, rs1, rd)) => {bitvector_concat(BitDynamic::from((imm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1100111)))))))))}
        ast::BTYPE((v__4, rs2, rs1, op)) if {(v__4.subrange::<0, 1, 1>() == BitDynamic::new(1, 0b0))} => {let imm7_6: BitDynamic = v__4.subrange::<12, 13, 1>();
        let imm7_6: BitDynamic = v__4.subrange::<12, 13, 1>();
        let imm7_5_0: BitDynamic = v__4.subrange::<5, 11, 6>();
        let imm5_4_1: BitDynamic = v__4.subrange::<1, 5, 4>();
        let imm5_0: BitDynamic = v__4.subrange::<11, 12, 1>();
        bitvector_concat(BitDynamic::from((imm7_6 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((imm7_5_0 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_bop_forwards(op)), BitDynamic::from(bitvector_concat(BitDynamic::from((imm5_4_1 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((imm5_0 as BitDynamic)), BitDynamic::from(BitDynamic::new(7, 0b1100011)))))))))))))))}
        ast::ITYPE((imm, rs1, rd, op)) => {bitvector_concat(BitDynamic::from((imm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_iop_forwards(op)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))}
        ast::SHIFTIOP((shamt, rs1, rd, sop::SLLI)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b000000)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))))}
        ast::SHIFTIOP((shamt, rs1, rd, sop::SRLI)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b000000)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))))}
        ast::SHIFTIOP((shamt, rs1, rd, sop::SRAI)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b010000)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::ADD)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::SLT)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b010)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::SLTU)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b011)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::AND)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::OR)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b110)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::XOR)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::SLL)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::SRL)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::SUB)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0100000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RTYPE((rs2, rs1, rd, rop::SRA)) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0100000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::LOAD((imm, rs1, rd, is_unsigned, width, false, false)) if {valid_load_encdec(width, is_unsigned)} => {todo!("Unsupported: 'LOAD'")}
        ast::STORE((v__6, rs2, rs1, width, false, false)) if {(size_bytes_forwards(width) <= 8)} => {todo!("Unsupported: 'STORE'")}
        ast::ADDIW((imm, rs1, rd)) if {true} => {bitvector_concat(BitDynamic::from((imm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0011011)))))))))}
        ast::RTYPEW((rs2, rs1, rd, ropw::ADDW)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::RTYPEW((rs2, rs1, rd, ropw::SUBW)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0100000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::RTYPEW((rs2, rs1, rd, ropw::SLLW)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::RTYPEW((rs2, rs1, rd, ropw::SRLW)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::RTYPEW((rs2, rs1, rd, ropw::SRAW)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0100000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::SHIFTIWOP((shamt, rs1, rd, sopw::SLLIW)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0011011)))))))))))}
        ast::SHIFTIWOP((shamt, rs1, rd, sopw::SRLIW)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000000)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0011011)))))))))))}
        ast::SHIFTIWOP((shamt, rs1, rd, sopw::SRAIW)) if {true} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0100000)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0011011)))))))))))}
        ast::ECALL(()) => {bitvector_concat(BitDynamic::from(BitDynamic::new(12, 0b000000000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))}
        ast::MRET(()) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0011000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00010)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::SRET(()) => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0001000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00010)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::EBREAK(()) => {bitvector_concat(BitDynamic::from(BitDynamic::new(12, 0b000000000001)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))}
        ast::LOADRES((aq, rl, rs1, size, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zalrsc) && lrsc_width_valid(size))} => {todo!("Unsupported: 'LOADRES'")}
        ast::STORECON((aq, rl, rs2, rs1, size, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zalrsc) && lrsc_width_valid(size))} => {todo!("Unsupported: 'STORECON'")}
        ast::AMO((op, aq, rl, rs2, rs1, size, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zaamo) && amo_width_valid(core_ctx, size))} => {todo!("Unsupported: 'AMO'")}
        ast::MUL((rs2, rs1, rd, mul_op)) if {(currentlyEnabled(core_ctx, extension::Ext_M) || currentlyEnabled(core_ctx, extension::Ext_Zmmul))} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_mul_op_forwards(mul_op)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::DIV((rs2, rs1, rd, s)) if {currentlyEnabled(core_ctx, extension::Ext_M)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b10)), BitDynamic::from(bitvector_concat(BitDynamic::from(bool_not_bits_forwards(s)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))))}
        ast::REM((rs2, rs1, rd, s)) if {currentlyEnabled(core_ctx, extension::Ext_M)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b11)), BitDynamic::from(bitvector_concat(BitDynamic::from(bool_not_bits_forwards(s)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))))}
        ast::MULW((rs2, rs1, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_M) || currentlyEnabled(core_ctx, extension::Ext_Zmmul))} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::DIVW((rs2, rs1, rd, s)) if {currentlyEnabled(core_ctx, extension::Ext_M)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b10)), BitDynamic::from(bitvector_concat(BitDynamic::from(bool_not_bits_forwards(s)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))))}
        ast::REMW((rs2, rs1, rd, s)) if {currentlyEnabled(core_ctx, extension::Ext_M)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b11)), BitDynamic::from(bitvector_concat(BitDynamic::from(bool_not_bits_forwards(s)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))))}
        ast::CSRReg((csr, rs1, rd, op)) => {bitvector_concat(BitDynamic::from((csr as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b0)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_csrop_forwards(op)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::CSRImm((csr, imm, rd, op)) => {bitvector_concat(BitDynamic::from((csr as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((imm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b1)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_csrop_forwards(op)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::SINVAL_VMA((rs1, rs2)) if {currentlyEnabled(core_ctx, extension::Ext_Svinval)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0001011)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))}
        ast::SLLIUW((shamt, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zba)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b000010)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0011011)))))))))))}
        ast::ZBA_RTYPEUW((rs2, rs1, rd, bropw_zba::ADDUW)) if {currentlyEnabled(core_ctx, extension::Ext_Zba)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::ZBA_RTYPEUW((rs2, rs1, rd, bropw_zba::SH1ADDUW)) if {currentlyEnabled(core_ctx, extension::Ext_Zba)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b010)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::ZBA_RTYPEUW((rs2, rs1, rd, bropw_zba::SH2ADDUW)) if {currentlyEnabled(core_ctx, extension::Ext_Zba)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::ZBA_RTYPEUW((rs2, rs1, rd, bropw_zba::SH3ADDUW)) if {currentlyEnabled(core_ctx, extension::Ext_Zba)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b110)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::ZBA_RTYPE((rs2, rs1, rd, brop_zba::SH1ADD)) if {currentlyEnabled(core_ctx, extension::Ext_Zba)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b010)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZBA_RTYPE((rs2, rs1, rd, brop_zba::SH2ADD)) if {currentlyEnabled(core_ctx, extension::Ext_Zba)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZBA_RTYPE((rs2, rs1, rd, brop_zba::SH3ADD)) if {currentlyEnabled(core_ctx, extension::Ext_Zba)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b110)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::RORIW((shamt, rs1, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'RORIW'")}
        ast::RORI((shamt, rs1, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'RORI'")}
        ast::ZBB_RTYPEW((rs2, rs1, rd, bropw_zbb::ROLW)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'ZBB_RTYPEW'")}
        ast::ZBB_RTYPEW((rs2, rs1, rd, bropw_zbb::RORW)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'ZBB_RTYPEW'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::ANDN)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::ORN)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::XNOR)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::MAX)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::MAXU)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::MIN)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::MINU)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::ROL)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, brop_zbb::ROR)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_EXTOP((rs1, rd, extop_zbb::SEXTB)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'ZBB_EXTOP'")}
        ast::ZBB_EXTOP((rs1, rd, extop_zbb::SEXTH)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'ZBB_EXTOP'")}
        ast::ZBB_EXTOP((rs1, rd, extop_zbb::ZEXTH)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'ZBB_EXTOP'")}
        ast::REV8((rs1, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbb) || currentlyEnabled(core_ctx, extension::Ext_Zbkb))} => {todo!("Unsupported: 'REV8'")}
        ast::ORCB((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'ORCB'")}
        ast::CPOP((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'CPOP'")}
        ast::CPOPW((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'CPOPW'")}
        ast::CLZ((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'CLZ'")}
        ast::CLZW((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'CLZW'")}
        ast::CTZ((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'CTZ'")}
        ast::CTZW((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbb)} => {todo!("Unsupported: 'CTZW'")}
        ast::CLMUL((rs2, rs1, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbc) || currentlyEnabled(core_ctx, extension::Ext_Zbkc))} => {todo!("Unsupported: 'CLMUL'")}
        ast::CLMULH((rs2, rs1, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zbc) || currentlyEnabled(core_ctx, extension::Ext_Zbkc))} => {todo!("Unsupported: 'CLMULH'")}
        ast::CLMULR((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbc)} => {todo!("Unsupported: 'CLMULR'")}
        ast::ZBS_IOP((shamt, rs1, rd, biop_zbs::BCLRI)) if {currentlyEnabled(core_ctx, extension::Ext_Zbs)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b010010)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))))}
        ast::ZBS_IOP((shamt, rs1, rd, biop_zbs::BEXTI)) if {currentlyEnabled(core_ctx, extension::Ext_Zbs)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b010010)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))))}
        ast::ZBS_IOP((shamt, rs1, rd, biop_zbs::BINVI)) if {currentlyEnabled(core_ctx, extension::Ext_Zbs)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b011010)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))))}
        ast::ZBS_IOP((shamt, rs1, rd, biop_zbs::BSETI)) if {currentlyEnabled(core_ctx, extension::Ext_Zbs)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b001010)), BitDynamic::from(bitvector_concat(BitDynamic::from((shamt as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))))}
        ast::ZBS_RTYPE((rs2, rs1, rd, brop_zbs::BCLR)) if {currentlyEnabled(core_ctx, extension::Ext_Zbs)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0100100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZBS_RTYPE((rs2, rs1, rd, brop_zbs::BEXT)) if {currentlyEnabled(core_ctx, extension::Ext_Zbs)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0100100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZBS_RTYPE((rs2, rs1, rd, brop_zbs::BINV)) if {currentlyEnabled(core_ctx, extension::Ext_Zbs)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0110100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZBS_RTYPE((rs2, rs1, rd, brop_zbs::BSET)) if {currentlyEnabled(core_ctx, extension::Ext_Zbs)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b001)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::AES64KS1I((rnum, rs1, rd)) if {((currentlyEnabled(core_ctx, extension::Ext_Zkne) || currentlyEnabled(core_ctx, extension::Ext_Zknd)) && _operator_smaller_u_(rnum, BitDynamic::new(4, 0b1011)))} => {todo!("Unsupported: 'AES64KS1I'")}
        ast::AES64IM((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zknd)} => {todo!("Unsupported: 'AES64IM'")}
        ast::AES64KS2((rs2, rs1, rd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zkne) || currentlyEnabled(core_ctx, extension::Ext_Zknd))} => {todo!("Unsupported: 'AES64KS2'")}
        ast::AES64ESM((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zkne)} => {todo!("Unsupported: 'AES64ESM'")}
        ast::AES64ES((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zkne)} => {todo!("Unsupported: 'AES64ES'")}
        ast::AES64DSM((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zknd)} => {todo!("Unsupported: 'AES64DSM'")}
        ast::AES64DS((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zknd)} => {todo!("Unsupported: 'AES64DS'")}
        ast::ZBKB_RTYPE((rs2, rs1, rd, brop_zbkb::PACK)) if {currentlyEnabled(core_ctx, extension::Ext_Zbkb)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZBKB_RTYPE((rs2, rs1, rd, brop_zbkb::PACKH)) if {currentlyEnabled(core_ctx, extension::Ext_Zbkb)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZBKB_PACKW((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbkb)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0111011)))))))))))}
        ast::BREV8((rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbkb)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(12, 0b011010000111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0010011)))))))))}
        ast::XPERM8((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbkx)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::XPERM4((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zbkx)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0010100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b010)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZICOND_RTYPE((rs2, rs1, rd, zicondop::CZERO_EQZ)) if {currentlyEnabled(core_ctx, extension::Ext_Zicond)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b101)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::ZICOND_RTYPE((rs2, rs1, rd, zicondop::CZERO_NEZ)) if {currentlyEnabled(core_ctx, extension::Ext_Zicond)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b0000111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b0110011)))))))))))}
        ast::VSETVLI((ma, ta, sew, lmul, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(4, 0b0000)), BitDynamic::from(bitvector_concat(BitDynamic::from((ma as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((ta as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((sew as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((lmul as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1010111)))))))))))))))))}
        ast::VSETVL((rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(7, 0b1000000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1010111)))))))))))}
        ast::VSETIVLI((ma, ta, sew, lmul, uimm, rd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(4, 0b1100)), BitDynamic::from(bitvector_concat(BitDynamic::from((ma as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((ta as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((sew as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((lmul as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from((uimm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b111)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1010111)))))))))))))))))}
        ast::VVTYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {bitvector_concat(BitDynamic::from(encdec_vvfunct6_forwards(funct6)), BitDynamic::from(bitvector_concat(BitDynamic::from((vm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_vreg_forwards(vs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_vreg_forwards(vs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_vreg_forwards(vd)), BitDynamic::from(BitDynamic::new(7, 0b1010111)))))))))))))}
        ast::NVSTYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'NVSTYPE'")}
        ast::NVTYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'NVTYPE'")}
        ast::MASKTYPEV((vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MASKTYPEV'")}
        ast::MOVETYPEV((vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MOVETYPEV'")}
        ast::VXTYPE((funct6, vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VXTYPE'")}
        ast::NXSTYPE((funct6, vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'NXSTYPE'")}
        ast::NXTYPE((funct6, vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'NXTYPE'")}
        ast::VXSG((funct6, vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VXSG'")}
        ast::MASKTYPEX((vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MASKTYPEX'")}
        ast::MOVETYPEX((rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {bitvector_concat(BitDynamic::from(BitDynamic::new(6, 0b010111)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_vreg_forwards(vd)), BitDynamic::from(BitDynamic::new(7, 0b1010111)))))))))))))}
        ast::VITYPE((funct6, vm, vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {bitvector_concat(BitDynamic::from(encdec_vifunct6_forwards(funct6)), BitDynamic::from(bitvector_concat(BitDynamic::from((vm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_vreg_forwards(vs2)), BitDynamic::from(bitvector_concat(BitDynamic::from((simm as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b011)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_vreg_forwards(vd)), BitDynamic::from(BitDynamic::new(7, 0b1010111)))))))))))))}
        ast::NISTYPE((funct6, vm, vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'NISTYPE'")}
        ast::NITYPE((funct6, vm, vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'NITYPE'")}
        ast::VISG((funct6, vm, vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VISG'")}
        ast::MASKTYPEI((vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MASKTYPEI'")}
        ast::MOVETYPEI((vd, simm)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MOVETYPEI'")}
        ast::VMVRTYPE((vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VMVRTYPE'")}
        ast::MVVTYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MVVTYPE'")}
        ast::MVVMATYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MVVMATYPE'")}
        ast::VEXT2TYPE((funct6, vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VEXT2TYPE'")}
        ast::VEXT4TYPE((funct6, vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VEXT4TYPE'")}
        ast::VEXT8TYPE((funct6, vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VEXT8TYPE'")}
        ast::VMVXS((vs2, rd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VMVXS'")}
        ast::MVVCOMPRESS((vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MVVCOMPRESS'")}
        ast::MVXTYPE((funct6, vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MVXTYPE'")}
        ast::MVXMATYPE((funct6, vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MVXMATYPE'")}
        ast::VMVSX((rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VMVSX'")}
        ast::FVVTYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'FVVTYPE'")}
        ast::FVVMATYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'FVVMATYPE'")}
        ast::VFUNARY0((vm, vs2, vfunary0, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VFUNARY0'")}
        ast::VFNUNARY0((vm, vs2, vfnunary0, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VFNUNARY0'")}
        ast::VFUNARY1((vm, vs2, vfunary1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VFUNARY1'")}
        ast::VLSEGTYPE((nf, vm, rs1, width, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VLSEGTYPE'")}
        ast::VLSEGFFTYPE((nf, vm, rs1, width, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VLSEGFFTYPE'")}
        ast::VSSEGTYPE((nf, vm, rs1, width, vs3)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VSSEGTYPE'")}
        ast::VLSSEGTYPE((nf, vm, rs2, rs1, width, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VLSSEGTYPE'")}
        ast::VSSSEGTYPE((nf, vm, rs2, rs1, width, vs3)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VSSSEGTYPE'")}
        ast::VLUXSEGTYPE((nf, vm, vs2, rs1, width, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VLUXSEGTYPE'")}
        ast::VLOXSEGTYPE((nf, vm, vs2, rs1, width, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VLOXSEGTYPE'")}
        ast::VSUXSEGTYPE((nf, vm, vs2, rs1, width, vs3)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VSUXSEGTYPE'")}
        ast::VSOXSEGTYPE((nf, vm, vs2, rs1, width, vs3)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VSOXSEGTYPE'")}
        ast::VLRETYPE((nf, rs1, width, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VLRETYPE'")}
        ast::VSRETYPE((nf, rs1, vs3)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VSRETYPE'")}
        ast::VMTYPE((rs1, vd_or_vs3, op)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VMTYPE'")}
        ast::MMTYPE((funct6, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'MMTYPE'")}
        ast::VCPOP_M((vm, vs2, rd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VCPOP_M'")}
        ast::VFIRST_M((vm, vs2, rd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VFIRST_M'")}
        ast::VMSBF_M((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VMSBF_M'")}
        ast::VMSIF_M((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VMSIF_M'")}
        ast::VMSOF_M((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VMSOF_M'")}
        ast::VIOTA_M((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VIOTA_M'")}
        ast::VID_V((vm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VID_V'")}
        ast::VVMTYPE((funct6, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VVMTYPE'")}
        ast::VVMCTYPE((funct6, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VVMCTYPE'")}
        ast::VVMSTYPE((funct6, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VVMSTYPE'")}
        ast::VVCMPTYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VVCMPTYPE'")}
        ast::VXMTYPE((funct6, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VXMTYPE'")}
        ast::VXMCTYPE((funct6, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VXMCTYPE'")}
        ast::VXMSTYPE((funct6, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VXMSTYPE'")}
        ast::VXCMPTYPE((funct6, vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VXCMPTYPE'")}
        ast::VIMTYPE((funct6, vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VIMTYPE'")}
        ast::VIMCTYPE((funct6, vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VIMCTYPE'")}
        ast::VIMSTYPE((funct6, vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VIMSTYPE'")}
        ast::VICMPTYPE((funct6, vm, vs2, simm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'VICMPTYPE'")}
        ast::FVVMTYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'FVVMTYPE'")}
        ast::RMVVTYPE((funct6, vm, vs2, vs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_V)} => {todo!("Unsupported: 'RMVVTYPE'")}
        ast::ZICBOM((cbop, rs1)) if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} => {bitvector_concat(BitDynamic::from(encdec_cbop_forwards(cbop)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b010)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(5, 0b00000)), BitDynamic::from(BitDynamic::new(7, 0b0001111)))))))))}
        ast::ZICBOZ(rs1) if {currentlyEnabled(core_ctx, extension::Ext_Zicboz)} => {todo!("Unsupported: 'ZICBOZ'")}
        ast::VANDN_VV((vm, vs1, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VANDN_VV'")}
        ast::VANDN_VX((vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VANDN_VX'")}
        ast::VBREV_V((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvbb)} => {todo!("Unsupported: 'VBREV_V'")}
        ast::VBREV8_V((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VBREV8_V'")}
        ast::VREV8_V((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VREV8_V'")}
        ast::VCLZ_V((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvbb)} => {todo!("Unsupported: 'VCLZ_V'")}
        ast::VCTZ_V((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvbb)} => {todo!("Unsupported: 'VCTZ_V'")}
        ast::VCPOP_V((vm, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvbb)} => {todo!("Unsupported: 'VCPOP_V'")}
        ast::VROL_VV((vm, vs1, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VROL_VV'")}
        ast::VROL_VX((vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VROL_VX'")}
        ast::VROR_VV((vm, vs1, vs2, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VROR_VV'")}
        ast::VROR_VX((vm, vs2, rs1, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VROR_VX'")}
        ast::VROR_VI((vm, vs2, uimm, vd)) if {currentlyEnabled(core_ctx, extension::Ext_Zvkb)} => {todo!("Unsupported: 'VROR_VI'")}
        ast::VCLMUL_VV((vm, vs2, vs1, vd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zvbc) && ((get_sew(core_ctx, ()) == 64) as bool))} => {todo!("Unsupported: 'VCLMUL_VV'")}
        ast::VCLMUL_VX((vm, vs2, rs1, vd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zvbc) && ((get_sew(core_ctx, ()) == 64) as bool))} => {todo!("Unsupported: 'VCLMUL_VX'")}
        ast::VCLMULH_VV((vm, vs2, vs1, vd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zvbc) && ((get_sew(core_ctx, ()) == 64) as bool))} => {todo!("Unsupported: 'VCLMULH_VV'")}
        ast::VCLMULH_VX((vm, vs2, rs1, vd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zvbc) && ((get_sew(core_ctx, ()) == 64) as bool))} => {todo!("Unsupported: 'VCLMULH_VX'")}
        ast::VSHA2MS_VV((vs2, vs1, vd)) if {((currentlyEnabled(core_ctx, extension::Ext_Zvknha) && ((get_sew(core_ctx, ()) == 32) as bool)) || ((currentlyEnabled(core_ctx, extension::Ext_Zvknhb) && (((get_sew(core_ctx, ()) == 32) || ((get_sew(core_ctx, ()) == 64) as bool)) as bool)) && zvknhab_check_encdec(core_ctx, vs2, vs1, vd)))} => {todo!("Unsupported: 'VSHA2MS_VV'")}
        ast::ZVKSHA2TYPE((funct6, vs2, vs1, vd)) if {((currentlyEnabled(core_ctx, extension::Ext_Zvknha) && ((get_sew(core_ctx, ()) == 32) as bool)) || ((currentlyEnabled(core_ctx, extension::Ext_Zvknhb) && (((get_sew(core_ctx, ()) == 32) || ((get_sew(core_ctx, ()) == 64) as bool)) as bool)) && zvknhab_check_encdec(core_ctx, vs2, vs1, vd)))} => {todo!("Unsupported: 'ZVKSHA2TYPE'")}
        ast::VSM3ME_VV((vs2, vs1, vd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zvksh) && (((get_sew(core_ctx, ()) == 32) && (zvk_check_encdec(core_ctx, 256, 8) && {
                let var_1: i128 = get_lmul_pow(core_ctx, ());
                zvk_valid_reg_overlap(vs2, vd, var_1)
            })) as bool))} => {todo!("Unsupported: 'VSM3ME_VV'")}
        ast::VSM3C_VI((vs2, uimm, vd)) if {(currentlyEnabled(core_ctx, extension::Ext_Zvksh) && (((get_sew(core_ctx, ()) == 32) && (zvk_check_encdec(core_ctx, 256, 8) && {
                let var_2: i128 = get_lmul_pow(core_ctx, ());
                zvk_valid_reg_overlap(vs2, vd, var_2)
            })) as bool))} => {todo!("Unsupported: 'VSM3C_VI'")}
        ast::ZIMOP_MOP_R((v__10, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zimop)} => {let mop_30: BitDynamic = v__10.subrange::<4, 5, 1>();
        let mop_30: BitDynamic = v__10.subrange::<4, 5, 1>();
        let mop_27_26: BitDynamic = v__10.subrange::<2, 4, 2>();
        let mop_21_20: BitDynamic = v__10.subrange::<0, 2, 2>();
        bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b1)), BitDynamic::from(bitvector_concat(BitDynamic::from((mop_30 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b00)), BitDynamic::from(bitvector_concat(BitDynamic::from((mop_27_26 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(4, 0b0111)), BitDynamic::from(bitvector_concat(BitDynamic::from((mop_21_20 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))))))))))}
        ast::ZIMOP_MOP_RR((v__11, rs2, rs1, rd)) if {currentlyEnabled(core_ctx, extension::Ext_Zimop)} => {let mop_30: BitDynamic = v__11.subrange::<2, 3, 1>();
        let mop_30: BitDynamic = v__11.subrange::<2, 3, 1>();
        let mop_27_26: BitDynamic = v__11.subrange::<0, 2, 2>();
        bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b1)), BitDynamic::from(bitvector_concat(BitDynamic::from((mop_30 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b00)), BitDynamic::from(bitvector_concat(BitDynamic::from((mop_27_26 as BitDynamic)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b1)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs2)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rs1)), BitDynamic::from(bitvector_concat(BitDynamic::from(BitDynamic::new(3, 0b100)), BitDynamic::from(bitvector_concat(BitDynamic::from(encdec_reg_forwards(rd)), BitDynamic::from(BitDynamic::new(7, 0b1110011)))))))))))))))))))}
        ast::ILLEGAL(s) => {s}
        _ => {panic!("Unreachable code")}
    }
}

/// execute
///
/// Generated from the Sail sources at `riscv_insts_base.sail` L26-33.
pub fn execute(core_ctx: &mut Core, merge_hashtag_var: ast) -> ExecutionResult {
    match merge_hashtag_var {
        ast::UTYPE((imm, rd, op)) => {{
            let off: xlenbits = sign_extend(64, bitvector_concat(BitDynamic::from(imm), BitDynamic::from(BitDynamic::new(12, 0b000000000000))));
            {
                let var_1: BitDynamic = match op {
                    uop::LUI => {off}
                    uop::AUIPC => {get_arch_pc(core_ctx, ()).wrapped_add(off)}
                    _ => {panic!("Unreachable code")}
                };
                wX_bits(core_ctx, rd, var_1)
            };
            RETIRE_SUCCESS
        }}
        ast::JAL((imm, rd)) => {{
            let target: BitDynamic = core_ctx.PC.wrapped_add(sign_extend(64, imm));
            match ext_control_check_pc(target) {
                Ext_ControlAddr_Check::Ext_ControlAddr_Error(e) => {ExecutionResult::Ext_ControlAddr_Check_Failure(e)}
                Ext_ControlAddr_Check::Ext_ControlAddr_OK(target) => {{
                    let target_bits: BitDynamic = bits_of_virtaddr(target);
                    if {(bit_to_bool(bitvector_access(target_bits, 1)) && !(currentlyEnabled(core_ctx, extension::Ext_Zca)))} {
                        ExecutionResult::Memory_Exception((target, ExceptionType::E_Fetch_Addr_Align(())))
                    } else {
                        {
                            let var_2: BitDynamic = get_next_pc(core_ctx, ());
                            wX_bits(core_ctx, rd, var_2)
                        };
                        set_next_pc(core_ctx, target_bits);
                        RETIRE_SUCCESS
                    }
                }}
                _ => {panic!("Unreachable code")}
            }
        }}
        ast::BTYPE((imm, rs2, rs1, op)) => {{
            let taken: bool = match op {
                bop::BEQ => {(rX_bits(core_ctx, rs1) == rX_bits(core_ctx, rs2))}
                bop::BNE => {(rX_bits(core_ctx, rs1) != rX_bits(core_ctx, rs2))}
                bop::BLT => {{
                    let var_3: BitDynamic = rX_bits(core_ctx, rs1);
                    let var_4: BitDynamic = rX_bits(core_ctx, rs2);
                    _operator_smaller_s_(var_3, var_4)
                }}
                bop::BGE => {{
                    let var_5: BitDynamic = rX_bits(core_ctx, rs1);
                    let var_6: BitDynamic = rX_bits(core_ctx, rs2);
                    _operator_biggerequal_s_(var_5, var_6)
                }}
                bop::BLTU => {{
                    let var_7: BitDynamic = rX_bits(core_ctx, rs1);
                    let var_8: BitDynamic = rX_bits(core_ctx, rs2);
                    _operator_smaller_u_(var_7, var_8)
                }}
                bop::BGEU => {{
                    let var_9: BitDynamic = rX_bits(core_ctx, rs1);
                    let var_10: BitDynamic = rX_bits(core_ctx, rs2);
                    _operator_biggerequal_u_(var_9, var_10)
                }}
                _ => {panic!("Unreachable code")}
            };
            if {taken} {
                let target: BitDynamic = core_ctx.PC.wrapped_add(sign_extend(64, imm));
                match ext_control_check_pc(target) {
                    Ext_ControlAddr_Check::Ext_ControlAddr_Error(e) => {ExecutionResult::Ext_ControlAddr_Check_Failure(e)}
                    Ext_ControlAddr_Check::Ext_ControlAddr_OK(target) => {{
                        let target_bits: BitDynamic = bits_of_virtaddr(target);
                        if {(bit_to_bool(bitvector_access(target_bits, 1)) && !(currentlyEnabled(core_ctx, extension::Ext_Zca)))} {
                            ExecutionResult::Memory_Exception((target, ExceptionType::E_Fetch_Addr_Align(())))
                        } else {
                            set_next_pc(core_ctx, target_bits);
                            RETIRE_SUCCESS
                        }
                    }}
                    _ => {panic!("Unreachable code")}
                }
            } else {
                RETIRE_SUCCESS
            }
        }}
        ast::ITYPE((imm, rs1, rd, op)) => {{
            let immext: xlenbits = sign_extend(64, imm);
            {
                let var_11: BitDynamic = match op {
                    iop::ADDI => {rX_bits(core_ctx, rs1).wrapped_add(immext)}
                    iop::SLTI => {{
                        let var_12: BitDynamic = {
                            let var_13: bool = {
                                let var_14: BitDynamic = rX_bits(core_ctx, rs1);
                                _operator_smaller_s_(var_14, immext)
                            };
                            bool_to_bits(var_13)
                        };
                        var_12.zero_extend_dyn(64)
                    }}
                    iop::SLTIU => {{
                        let var_15: BitDynamic = {
                            let var_16: bool = {
                                let var_17: BitDynamic = rX_bits(core_ctx, rs1);
                                _operator_smaller_u_(var_17, immext)
                            };
                            bool_to_bits(var_16)
                        };
                        var_15.zero_extend_dyn(64)
                    }}
                    iop::ANDI => {(rX_bits(core_ctx, rs1) & immext)}
                    iop::ORI => {(rX_bits(core_ctx, rs1) | immext)}
                    iop::XORI => {(rX_bits(core_ctx, rs1) ^ immext)}
                    _ => {panic!("Unreachable code")}
                };
                wX_bits(core_ctx, rd, var_11)
            };
            RETIRE_SUCCESS
        }}
        ast::SHIFTIOP((shamt, rs1, rd, op)) => {{
            {
                let var_18: BitDynamic = match op {
                    sop::SLLI => {shift_bits_left(rX_bits(core_ctx, rs1), shamt)}
                    sop::SRLI => {shift_bits_right(rX_bits(core_ctx, rs1), shamt)}
                    sop::SRAI => {{
                        let var_19: BitDynamic = rX_bits(core_ctx, rs1);
                        shift_bits_right_arith(var_19, shamt)
                    }}
                    _ => {panic!("Unreachable code")}
                };
                wX_bits(core_ctx, rd, var_18)
            };
            RETIRE_SUCCESS
        }}
        ast::RTYPE((rs2, rs1, rd, op)) => {{
            {
                let var_20: BitDynamic = match op {
                    rop::ADD => {{
                        let var_21: BitDynamic = rX_bits(core_ctx, rs2);
                        rX_bits(core_ctx, rs1).wrapped_add(var_21)
                    }}
                    rop::SLT => {{
                        let var_22: BitDynamic = {
                            let var_23: bool = {
                                let var_24: BitDynamic = rX_bits(core_ctx, rs1);
                                let var_25: BitDynamic = rX_bits(core_ctx, rs2);
                                _operator_smaller_s_(var_24, var_25)
                            };
                            bool_to_bits(var_23)
                        };
                        var_22.zero_extend_dyn(64)
                    }}
                    rop::SLTU => {{
                        let var_26: BitDynamic = {
                            let var_27: bool = {
                                let var_28: BitDynamic = rX_bits(core_ctx, rs1);
                                let var_29: BitDynamic = rX_bits(core_ctx, rs2);
                                _operator_smaller_u_(var_28, var_29)
                            };
                            bool_to_bits(var_27)
                        };
                        var_26.zero_extend_dyn(64)
                    }}
                    rop::AND => {(rX_bits(core_ctx, rs1) & rX_bits(core_ctx, rs2))}
                    rop::OR => {(rX_bits(core_ctx, rs1) | rX_bits(core_ctx, rs2))}
                    rop::XOR => {(rX_bits(core_ctx, rs1) ^ rX_bits(core_ctx, rs2))}
                    rop::SLL => {shift_bits_left(rX_bits(core_ctx, rs1), subrange_bits(rX_bits(core_ctx, rs2), 5, 0))}
                    rop::SRL => {shift_bits_right(rX_bits(core_ctx, rs1), subrange_bits(rX_bits(core_ctx, rs2), 5, 0))}
                    rop::SUB => {sub_vec(rX_bits(core_ctx, rs1), rX_bits(core_ctx, rs2))}
                    rop::SRA => {{
                        let var_30: BitDynamic = rX_bits(core_ctx, rs1);
                        let var_31: BitDynamic = subrange_bits(rX_bits(core_ctx, rs2), 5, 0);
                        shift_bits_right_arith(var_30, var_31)
                    }}
                    _ => {panic!("Unreachable code")}
                };
                wX_bits(core_ctx, rd, var_20)
            };
            RETIRE_SUCCESS
        }}
        ast::LOAD((imm, rs1, rd, is_unsigned, width, aq, rl)) => {todo!("Unsupported: 'LOAD'")}
        ast::STORE((imm, rs2, rs1, width, aq, rl)) => {todo!("Unsupported: 'STORE'")}
        ast::ADDIW((imm, rs1, rd)) => {{
            let result: BitDynamic = rX_bits(core_ctx, rs1).wrapped_add(sign_extend(64, imm));
            wX_bits(core_ctx, rd, sign_extend(64, result.subrange::<0, 32, 32>()));
            RETIRE_SUCCESS
        }}
        ast::RTYPEW((rs2, rs1, rd, op)) => {{
            let rs1_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs1), 31, 0);
            let rs2_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs2), 31, 0);
            let result: BitDynamic = match op {
                ropw::ADDW => {rs1_val.wrapped_add(rs2_val)}
                ropw::SUBW => {sub_vec(rs1_val, rs2_val)}
                ropw::SLLW => {shift_bits_left(rs1_val, rs2_val.subrange::<0, 5, 5>())}
                ropw::SRLW => {shift_bits_right(rs1_val, rs2_val.subrange::<0, 5, 5>())}
                ropw::SRAW => {shift_bits_right_arith(rs1_val, rs2_val.subrange::<0, 5, 5>())}
                _ => {panic!("Unreachable code")}
            };
            wX_bits(core_ctx, rd, sign_extend(64, result));
            RETIRE_SUCCESS
        }}
        ast::SHIFTIWOP((shamt, rs1, rd, op)) => {{
            let rs1_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs1), 31, 0);
            let result: BitDynamic = match op {
                sopw::SLLIW => {shift_bits_left(rs1_val, shamt)}
                sopw::SRLIW => {shift_bits_right(rs1_val, shamt)}
                sopw::SRAIW => {shift_bits_right_arith(rs1_val, shamt)}
                _ => {panic!("Unreachable code")}
            };
            wX_bits(core_ctx, rd, sign_extend(64, result));
            RETIRE_SUCCESS
        }}
        ast::ECALL(()) => {{
            let t: sync_exception = sync_exception {
                trap: match core_ctx.cur_privilege {
                    Privilege::User => {ExceptionType::E_U_EnvCall(())}
                    Privilege::Supervisor => {ExceptionType::E_S_EnvCall(())}
                    Privilege::Machine => {ExceptionType::E_M_EnvCall(())}
                    _ => {panic!("Unreachable code")}
                },
                excinfo: (None as Option<xlenbits>),
                ext: None
            };
            ExecutionResult::Trap((core_ctx.cur_privilege, ctl_result::CTL_TRAP(t), core_ctx.PC))
        }}
        ast::MRET(()) => {{
            if {(core_ctx.cur_privilege != Privilege::Machine)} {
                ExecutionResult::Illegal_Instruction(())
            } else if {!(true)} {
                ExecutionResult::Ext_XRET_Priv_Failure(())
            } else {
                {
                    let var_32: BitDynamic = {
                        let var_33: Privilege = core_ctx.cur_privilege;
                        let var_34: BitDynamic = core_ctx.PC;
                        exception_handler(core_ctx, var_33, ctl_result::CTL_MRET(()), var_34)
                    };
                    set_next_pc(core_ctx, var_32)
                };
                RETIRE_SUCCESS
            }
        }}
        ast::SRET(()) => {{
            let sret_illegal: bool = match core_ctx.cur_privilege {
                Privilege::User => {true}
                Privilege::Supervisor => {(!(currentlyEnabled(core_ctx, extension::Ext_S)) || ({
                    let var_38: Mstatus = core_ctx.mstatus;
                    _get_Mstatus_TSR(var_38)
                } == BitDynamic::new(1, 0b1)))}
                Privilege::Machine => {!(currentlyEnabled(core_ctx, extension::Ext_S))}
                _ => {panic!("Unreachable code")}
            };
            if {sret_illegal} {
                ExecutionResult::Illegal_Instruction(())
            } else if {!(true)} {
                ExecutionResult::Ext_XRET_Priv_Failure(())
            } else {
                {
                    let var_35: BitDynamic = {
                        let var_36: Privilege = core_ctx.cur_privilege;
                        let var_37: BitDynamic = core_ctx.PC;
                        exception_handler(core_ctx, var_36, ctl_result::CTL_SRET(()), var_37)
                    };
                    set_next_pc(core_ctx, var_35)
                };
                RETIRE_SUCCESS
            }
        }}
        ast::EBREAK(()) => {ExecutionResult::Memory_Exception(({
            let var_39: BitDynamic = core_ctx.PC;
            virtaddr::Virtaddr(var_39)
        }, ExceptionType::E_Breakpoint(())))}
        ast::LOADRES((aq, rl, rs1, width, rd)) => {todo!("Unsupported: 'LOADRES'")}
        ast::STORECON((aq, rl, rs2, rs1, width, rd)) => {todo!("Unsupported: 'STORECON'")}
        ast::AMO((op, aq, rl, rs2, rs1, width, rd)) => {todo!("Unsupported: 'AMO'")}
        ast::C_NOP(()) => {RETIRE_SUCCESS}
        ast::C_ADDI4SPN((rdc, nzimm)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(BitDynamic::new(2, 0b00)), BitDynamic::from(bitvector_concat(BitDynamic::from(nzimm), BitDynamic::from(BitDynamic::new(2, 0b00)))));
            let rd: regidx = creg2reg_idx(rdc);
            execute(core_ctx, ast::ITYPE((imm, sp, rd, iop::ADDI)))
        }}
        ast::C_LW((uimm, rsc, rdc)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(uimm), BitDynamic::from(BitDynamic::new(2, 0b00))).zero_extend_dyn(12);
            let rd: regidx = creg2reg_idx(rdc);
            let rs: regidx = creg2reg_idx(rsc);
            execute(core_ctx, ast::LOAD((imm, rs, rd, false, word_width::WORD, false, false)))
        }}
        ast::C_LD((uimm, rsc, rdc)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(uimm), BitDynamic::from(BitDynamic::new(3, 0b000))).zero_extend_dyn(12);
            let rd: regidx = creg2reg_idx(rdc);
            let rs: regidx = creg2reg_idx(rsc);
            execute(core_ctx, ast::LOAD((imm, rs, rd, false, word_width::DOUBLE, false, false)))
        }}
        ast::C_SW((uimm, rsc1, rsc2)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(uimm), BitDynamic::from(BitDynamic::new(2, 0b00))).zero_extend_dyn(12);
            let rs1: regidx = creg2reg_idx(rsc1);
            let rs2: regidx = creg2reg_idx(rsc2);
            execute(core_ctx, ast::STORE((imm, rs2, rs1, word_width::WORD, false, false)))
        }}
        ast::C_SD((uimm, rsc1, rsc2)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(uimm), BitDynamic::from(BitDynamic::new(3, 0b000))).zero_extend_dyn(12);
            let rs1: regidx = creg2reg_idx(rsc1);
            let rs2: regidx = creg2reg_idx(rsc2);
            execute(core_ctx, ast::STORE((imm, rs2, rs1, word_width::DOUBLE, false, false)))
        }}
        ast::C_ADDI((nzi, rsd)) => {{
            let imm: BitDynamic = sign_extend(12, nzi);
            execute(core_ctx, ast::ITYPE((imm, rsd, rsd, iop::ADDI)))
        }}
        ast::C_JAL(imm) => {execute(core_ctx, ast::JAL((sign_extend(21, bitvector_concat(BitDynamic::from(imm), BitDynamic::from(BitDynamic::new(1, 0b0)))), ra)))}
        ast::C_ADDIW((imm, rsd)) => {execute(core_ctx, ast::ADDIW((sign_extend(12, imm), rsd, rsd)))}
        ast::C_LI((imm, rd)) => {{
            let imm: BitDynamic = sign_extend(12, imm);
            execute(core_ctx, ast::ITYPE((imm, zreg, rd, iop::ADDI)))
        }}
        ast::C_ADDI16SP(imm) => {{
            let imm: BitDynamic = sign_extend(12, bitvector_concat(BitDynamic::from(imm), BitDynamic::from(BitDynamic::new(4, 0b0000))));
            execute(core_ctx, ast::ITYPE((imm, sp, sp, iop::ADDI)))
        }}
        ast::C_LUI((imm, rd)) => {{
            let res: BitDynamic = sign_extend(20, imm);
            execute(core_ctx, ast::UTYPE((res, rd, uop::LUI)))
        }}
        ast::C_SRLI((shamt, rsd)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            execute(core_ctx, ast::SHIFTIOP((shamt, rsd, rsd, sop::SRLI)))
        }}
        ast::C_SRAI((shamt, rsd)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            execute(core_ctx, ast::SHIFTIOP((shamt, rsd, rsd, sop::SRAI)))
        }}
        ast::C_ANDI((imm, rsd)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            execute(core_ctx, ast::ITYPE((sign_extend(12, imm), rsd, rsd, iop::ANDI)))
        }}
        ast::C_SUB((rsd, rs2)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            let rs2: regidx = creg2reg_idx(rs2);
            execute(core_ctx, ast::RTYPE((rs2, rsd, rsd, rop::SUB)))
        }}
        ast::C_XOR((rsd, rs2)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            let rs2: regidx = creg2reg_idx(rs2);
            execute(core_ctx, ast::RTYPE((rs2, rsd, rsd, rop::XOR)))
        }}
        ast::C_OR((rsd, rs2)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            let rs2: regidx = creg2reg_idx(rs2);
            execute(core_ctx, ast::RTYPE((rs2, rsd, rsd, rop::OR)))
        }}
        ast::C_AND((rsd, rs2)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            let rs2: regidx = creg2reg_idx(rs2);
            execute(core_ctx, ast::RTYPE((rs2, rsd, rsd, rop::AND)))
        }}
        ast::C_SUBW((rsd, rs2)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            let rs2: regidx = creg2reg_idx(rs2);
            execute(core_ctx, ast::RTYPEW((rs2, rsd, rsd, ropw::SUBW)))
        }}
        ast::C_ADDW((rsd, rs2)) => {{
            let rsd: regidx = creg2reg_idx(rsd);
            let rs2: regidx = creg2reg_idx(rs2);
            execute(core_ctx, ast::RTYPEW((rs2, rsd, rsd, ropw::ADDW)))
        }}
        ast::C_J(imm) => {execute(core_ctx, ast::JAL((sign_extend(21, bitvector_concat(BitDynamic::from(imm), BitDynamic::from(BitDynamic::new(1, 0b0)))), zreg)))}
        ast::C_BEQZ((imm, rs)) => {execute(core_ctx, ast::BTYPE((sign_extend(13, bitvector_concat(BitDynamic::from(imm), BitDynamic::from(BitDynamic::new(1, 0b0)))), zreg, creg2reg_idx(rs), bop::BEQ)))}
        ast::C_BNEZ((imm, rs)) => {execute(core_ctx, ast::BTYPE((sign_extend(13, bitvector_concat(BitDynamic::from(imm), BitDynamic::from(BitDynamic::new(1, 0b0)))), zreg, creg2reg_idx(rs), bop::BNE)))}
        ast::C_SLLI((shamt, rsd)) => {execute(core_ctx, ast::SHIFTIOP((shamt, rsd, rsd, sop::SLLI)))}
        ast::C_LWSP((uimm, rd)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(uimm), BitDynamic::from(BitDynamic::new(2, 0b00))).zero_extend_dyn(12);
            execute(core_ctx, ast::LOAD((imm, sp, rd, false, word_width::WORD, false, false)))
        }}
        ast::C_LDSP((uimm, rd)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(uimm), BitDynamic::from(BitDynamic::new(3, 0b000))).zero_extend_dyn(12);
            execute(core_ctx, ast::LOAD((imm, sp, rd, false, word_width::DOUBLE, false, false)))
        }}
        ast::C_SWSP((uimm, rs2)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(uimm), BitDynamic::from(BitDynamic::new(2, 0b00))).zero_extend_dyn(12);
            execute(core_ctx, ast::STORE((imm, rs2, sp, word_width::WORD, false, false)))
        }}
        ast::C_SDSP((uimm, rs2)) => {{
            let imm: BitDynamic = bitvector_concat(BitDynamic::from(uimm), BitDynamic::from(BitDynamic::new(3, 0b000))).zero_extend_dyn(12);
            execute(core_ctx, ast::STORE((imm, rs2, sp, word_width::DOUBLE, false, false)))
        }}
        ast::C_JR(rs1) => {execute(core_ctx, ast::JALR((zeros(12), rs1, zreg)))}
        ast::C_JALR(rs1) => {execute(core_ctx, ast::JALR((zeros(12), rs1, ra)))}
        ast::C_MV((rd, rs2)) => {execute(core_ctx, ast::RTYPE((rs2, zreg, rd, rop::ADD)))}
        ast::C_EBREAK(()) => {execute(core_ctx, ast::EBREAK(()))}
        ast::C_ADD((rsd, rs2)) => {execute(core_ctx, ast::RTYPE((rs2, rsd, rsd, rop::ADD)))}
        ast::MUL((rs2, rs1, rd, mul_op)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let rs1_int: i128 = if {mul_op.signed_rs1} {
                rs1_val.signed()
            } else {
                rs1_val.unsigned()
            };
            let rs2_int: i128 = if {mul_op.signed_rs2} {
                rs2_val.signed()
            } else {
                rs2_val.unsigned()
            };
            let result_wide: BitDynamic = to_bits(128, (rs1_int * rs2_int));
            let result: BitDynamic = if {mul_op.high} {
                result_wide.subrange::<64, 128, 64>()
            } else {
                result_wide.subrange::<0, 64, 64>()
            };
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::DIV((rs2, rs1, rd, s)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let rs1_int: i128 = if {s} {
                rs1_val.signed()
            } else {
                rs1_val.unsigned()
            };
            let rs2_int: i128 = if {s} {
                rs2_val.signed()
            } else {
                rs2_val.unsigned()
            };
            let q: i128 = if {(rs2_int == 0)} {
                -1
            } else {
                quot_round_zero(rs1_int, rs2_int)
            };
            let q__quote: i128 = if {(s && (q > 9223372036854775807))} {
                -9223372036854775808
            } else {
                q
            };
            wX_bits(core_ctx, rd, to_bits(64, q__quote));
            RETIRE_SUCCESS
        }}
        ast::REM((rs2, rs1, rd, s)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let rs1_int: i128 = if {s} {
                rs1_val.signed()
            } else {
                rs1_val.unsigned()
            };
            let rs2_int: i128 = if {s} {
                rs2_val.signed()
            } else {
                rs2_val.unsigned()
            };
            let r: i128 = if {(rs2_int == 0)} {
                rs1_int
            } else {
                rem_round_zero(rs1_int, rs2_int)
            };
            wX_bits(core_ctx, rd, to_bits(64, r));
            RETIRE_SUCCESS
        }}
        ast::MULW((rs2, rs1, rd)) => {{
            let rs1_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs1), 31, 0);
            let rs2_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs2), 31, 0);
            let rs1_int: i128 = rs1_val.signed();
            let rs2_int: i128 = rs2_val.signed();
            let result32: BitDynamic = subrange_bits(to_bits(64, (rs1_int * rs2_int)), 31, 0);
            let result: xlenbits = sign_extend(64, result32);
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::DIVW((rs2, rs1, rd, s)) => {{
            let rs1_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs1), 31, 0);
            let rs2_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs2), 31, 0);
            let rs1_int: i128 = if {s} {
                rs1_val.signed()
            } else {
                rs1_val.unsigned()
            };
            let rs2_int: i128 = if {s} {
                rs2_val.signed()
            } else {
                rs2_val.unsigned()
            };
            let q: i128 = if {(rs2_int == 0)} {
                -1
            } else {
                quot_round_zero(rs1_int, rs2_int)
            };
            let q__quote: i128 = if {(s && (q > 2147483647))} {
                -2147483648
            } else {
                q
            };
            wX_bits(core_ctx, rd, sign_extend(64, to_bits(32, q__quote)));
            RETIRE_SUCCESS
        }}
        ast::REMW((rs2, rs1, rd, s)) => {{
            let rs1_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs1), 31, 0);
            let rs2_val: BitDynamic = subrange_bits(rX_bits(core_ctx, rs2), 31, 0);
            let rs1_int: i128 = if {s} {
                rs1_val.signed()
            } else {
                rs1_val.unsigned()
            };
            let rs2_int: i128 = if {s} {
                rs2_val.signed()
            } else {
                rs2_val.unsigned()
            };
            let r: i128 = if {(rs2_int == 0)} {
                rs1_int
            } else {
                rem_round_zero(rs1_int, rs2_int)
            };
            wX_bits(core_ctx, rd, sign_extend(64, to_bits(32, r)));
            RETIRE_SUCCESS
        }}
        ast::CSRReg((csr, rs1, rd, op)) => {{
            let var_42: BitDynamic = rX_bits(core_ctx, rs1);
            doCSR(core_ctx, csr, var_42, rd, op, ((op == csrop::CSRRW) || (rs1 != zreg)))
        }}
        ast::CSRImm((csr, imm, rd, op)) => {doCSR(core_ctx, csr, imm.zero_extend_dyn(64), rd, op, ((op == csrop::CSRRW) || (imm != zeros(5))))}
        ast::C_NOP_HINT(imm) => {RETIRE_SUCCESS}
        ast::C_ADDI_HINT(rsd) => {RETIRE_SUCCESS}
        ast::C_LI_HINT(imm) => {RETIRE_SUCCESS}
        ast::C_LUI_HINT(imm) => {RETIRE_SUCCESS}
        ast::C_MV_HINT(rs2) => {RETIRE_SUCCESS}
        ast::C_ADD_HINT(rs2) => {RETIRE_SUCCESS}
        ast::C_SLLI_HINT((shamt, rsd)) => {RETIRE_SUCCESS}
        ast::C_SRLI_HINT(rsd) => {RETIRE_SUCCESS}
        ast::C_SRAI_HINT(rsd) => {RETIRE_SUCCESS}
        ast::SLLIUW((shamt, rs1, rd)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let result: xlenbits = shift_bits_left(rs1_val.subrange::<0, 32, 32>().zero_extend_dyn(64), shamt);
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::ZBA_RTYPEUW((rs2, rs1, rd, op)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let shamt: BitDynamic = match op {
                bropw_zba::ADDUW => {BitDynamic::new(2, 0b00)}
                bropw_zba::SH1ADDUW => {BitDynamic::new(2, 0b01)}
                bropw_zba::SH2ADDUW => {BitDynamic::new(2, 0b10)}
                bropw_zba::SH3ADDUW => {BitDynamic::new(2, 0b11)}
                _ => {panic!("Unreachable code")}
            };
            let result: xlenbits = shift_bits_left(rs1_val.subrange::<0, 32, 32>().zero_extend_dyn(64), shamt).wrapped_add(rs2_val);
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::ZBA_RTYPE((rs2, rs1, rd, op)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let shamt: BitDynamic = match op {
                brop_zba::SH1ADD => {BitDynamic::new(2, 0b01)}
                brop_zba::SH2ADD => {BitDynamic::new(2, 0b10)}
                brop_zba::SH3ADD => {BitDynamic::new(2, 0b11)}
                _ => {panic!("Unreachable code")}
            };
            let result: xlenbits = shift_bits_left(rs1_val, shamt).wrapped_add(rs2_val);
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::RORIW((shamt, rs1, rd)) => {todo!("Unsupported: 'RORIW'")}
        ast::RORI((shamt, rs1, rd)) => {todo!("Unsupported: 'RORI'")}
        ast::ZBB_RTYPEW((rs2, rs1, rd, op)) => {todo!("Unsupported: 'ZBB_RTYPEW'")}
        ast::ZBB_RTYPE((rs2, rs1, rd, op)) => {todo!("Unsupported: 'ZBB_RTYPE'")}
        ast::ZBB_EXTOP((rs1, rd, op)) => {todo!("Unsupported: 'ZBB_EXTOP'")}
        ast::REV8((rs1, rd)) => {todo!("Unsupported: 'REV8'")}
        ast::ORCB((rs1, rd)) => {todo!("Unsupported: 'ORCB'")}
        ast::CPOP((rs1, rd)) => {todo!("Unsupported: 'CPOP'")}
        ast::CPOPW((rs1, rd)) => {todo!("Unsupported: 'CPOPW'")}
        ast::CLZ((rs1, rd)) => {todo!("Unsupported: 'CLZ'")}
        ast::CLZW((rs1, rd)) => {todo!("Unsupported: 'CLZW'")}
        ast::CTZ((rs1, rd)) => {todo!("Unsupported: 'CTZ'")}
        ast::CTZW((rs1, rd)) => {todo!("Unsupported: 'CTZW'")}
        ast::CLMUL((rs2, rs1, rd)) => {todo!("Unsupported: 'CLMUL'")}
        ast::CLMULH((rs2, rs1, rd)) => {todo!("Unsupported: 'CLMULH'")}
        ast::CLMULR((rs2, rs1, rd)) => {todo!("Unsupported: 'CLMULR'")}
        ast::ZBS_IOP((shamt, rs1, rd, op)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let mask: xlenbits = shift_bits_left(BitDynamic::new(1, 0b1).zero_extend_dyn(64), shamt);
            let result: xlenbits = match op {
                biop_zbs::BCLRI => {(rs1_val & !(mask))}
                biop_zbs::BEXTI => {bool_to_bits(((rs1_val & mask) != zeros(64))).zero_extend_dyn(64)}
                biop_zbs::BINVI => {(rs1_val ^ mask)}
                biop_zbs::BSETI => {(rs1_val | mask)}
                _ => {panic!("Unreachable code")}
            };
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::ZBS_RTYPE((rs2, rs1, rd, op)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let mask: xlenbits = shift_bits_left(BitDynamic::new(1, 0b1).zero_extend_dyn(64), rs2_val.subrange::<0, 6, 6>());
            let result: xlenbits = match op {
                brop_zbs::BCLR => {(rs1_val & !(mask))}
                brop_zbs::BEXT => {bool_to_bits(((rs1_val & mask) != zeros(64))).zero_extend_dyn(64)}
                brop_zbs::BINV => {(rs1_val ^ mask)}
                brop_zbs::BSET => {(rs1_val | mask)}
                _ => {panic!("Unreachable code")}
            };
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::C_LBU((uimm, rdc, rs1c)) => {{
            let imm: BitDynamic = uimm.zero_extend_dyn(12);
            let rd: regidx = creg2reg_idx(rdc);
            let rs1: regidx = creg2reg_idx(rs1c);
            execute(core_ctx, ast::LOAD((imm, rs1, rd, true, word_width::BYTE, false, false)))
        }}
        ast::C_LHU((uimm, rdc, rs1c)) => {{
            let imm: BitDynamic = uimm.zero_extend_dyn(12);
            let rd: regidx = creg2reg_idx(rdc);
            let rs1: regidx = creg2reg_idx(rs1c);
            execute(core_ctx, ast::LOAD((imm, rs1, rd, true, word_width::HALF, false, false)))
        }}
        ast::C_LH((uimm, rdc, rs1c)) => {{
            let imm: BitDynamic = uimm.zero_extend_dyn(12);
            let rd: regidx = creg2reg_idx(rdc);
            let rs1: regidx = creg2reg_idx(rs1c);
            execute(core_ctx, ast::LOAD((imm, rs1, rd, false, word_width::HALF, false, false)))
        }}
        ast::C_SB((uimm, rs1c, rs2c)) => {{
            let imm: BitDynamic = uimm.zero_extend_dyn(12);
            let rs1: regidx = creg2reg_idx(rs1c);
            let rs2: regidx = creg2reg_idx(rs2c);
            execute(core_ctx, ast::STORE((imm, rs2, rs1, word_width::BYTE, false, false)))
        }}
        ast::C_SH((uimm, rs1c, rs2c)) => {{
            let imm: BitDynamic = uimm.zero_extend_dyn(12);
            let rs1: regidx = creg2reg_idx(rs1c);
            let rs2: regidx = creg2reg_idx(rs2c);
            execute(core_ctx, ast::STORE((imm, rs2, rs1, word_width::HALF, false, false)))
        }}
        ast::C_ZEXT_B(rsdc) => {{
            let rsd: regidx = creg2reg_idx(rsdc);
            {
                let var_43: BitDynamic = {
                    let var_44: BitDynamic = subrange_bits(rX_bits(core_ctx, rsd), 7, 0);
                    var_44.zero_extend_dyn(64)
                };
                wX_bits(core_ctx, rsd, var_43)
            };
            RETIRE_SUCCESS
        }}
        ast::C_SEXT_B(rsdc) => {{
            let rsd: regidx = creg2reg_idx(rsdc);
            execute(core_ctx, ast::ZBB_EXTOP((rsd, rsd, extop_zbb::SEXTB)))
        }}
        ast::C_ZEXT_H(rsdc) => {{
            let rsd: regidx = creg2reg_idx(rsdc);
            execute(core_ctx, ast::ZBB_EXTOP((rsd, rsd, extop_zbb::ZEXTH)))
        }}
        ast::C_SEXT_H(rsdc) => {{
            let rsd: regidx = creg2reg_idx(rsdc);
            execute(core_ctx, ast::ZBB_EXTOP((rsd, rsd, extop_zbb::SEXTH)))
        }}
        ast::C_ZEXT_W(rsdc) => {{
            let rsd: regidx = creg2reg_idx(rsdc);
            execute(core_ctx, ast::ZBA_RTYPEUW((zreg, rsd, rsd, bropw_zba::ADDUW)))
        }}
        ast::C_NOT(rsdc) => {{
            let r: regidx = creg2reg_idx(rsdc);
            {
                let var_45: BitDynamic = !(rX_bits(core_ctx, r));
                wX_bits(core_ctx, r, var_45)
            };
            RETIRE_SUCCESS
        }}
        ast::C_MUL((rsdc, rs2c)) => {{
            let rd: regidx = creg2reg_idx(rsdc);
            let rs: regidx = creg2reg_idx(rs2c);
            execute(core_ctx, ast::MUL((rs, rd, rd, mul_op {
                high: false,
                signed_rs1: true,
                signed_rs2: true
            })))
        }}
        ast::AES32ESMI((bs, rs2, rs1, rd)) => {todo!("Unsupported: 'AES32ESMI'")}
        ast::AES32ESI((bs, rs2, rs1, rd)) => {todo!("Unsupported: 'AES32ESI'")}
        ast::AES32DSMI((bs, rs2, rs1, rd)) => {todo!("Unsupported: 'AES32DSMI'")}
        ast::AES32DSI((bs, rs2, rs1, rd)) => {todo!("Unsupported: 'AES32DSI'")}
        ast::AES64KS1I((rnum, rs1, rd)) => {todo!("Unsupported: 'AES64KS1I'")}
        ast::AES64KS2((rs2, rs1, rd)) => {todo!("Unsupported: 'AES64KS2'")}
        ast::AES64IM((rs1, rd)) => {todo!("Unsupported: 'AES64IM'")}
        ast::AES64ESM((rs2, rs1, rd)) => {todo!("Unsupported: 'AES64ESM'")}
        ast::AES64ES((rs2, rs1, rd)) => {todo!("Unsupported: 'AES64ES'")}
        ast::AES64DSM((rs2, rs1, rd)) => {todo!("Unsupported: 'AES64DSM'")}
        ast::AES64DS((rs2, rs1, rd)) => {todo!("Unsupported: 'AES64DS'")}
        ast::ZBKB_RTYPE((rs2, rs1, rd, op)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let result: xlenbits = match op {
                brop_zbkb::PACK => {bitvector_concat(BitDynamic::from(rs2_val.subrange::<0, 32, 32>()), BitDynamic::from(rs1_val.subrange::<0, 32, 32>()))}
                brop_zbkb::PACKH => {bitvector_concat(BitDynamic::from(rs2_val.subrange::<0, 8, 8>()), BitDynamic::from(rs1_val.subrange::<0, 8, 8>())).zero_extend_dyn(64)}
                _ => {panic!("Unreachable code")}
            };
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::ZBKB_PACKW((rs2, rs1, rd)) => {{
            assert!(true, "riscv_insts_zbkb.sail:50.19-50.20");
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let result: BitDynamic = bitvector_concat(BitDynamic::from(rs2_val.subrange::<0, 16, 16>()), BitDynamic::from(rs1_val.subrange::<0, 16, 16>()));
            wX_bits(core_ctx, rd, sign_extend(64, result));
            RETIRE_SUCCESS
        }}
        ast::BREV8((rs1, rd)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let mut result: xlenbits = zeros(64);
            {
                todo!("E_for");
                wX_bits(core_ctx, rd, result);
                RETIRE_SUCCESS
            }
        }}
        ast::XPERM8((rs2, rs1, rd)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let mut result: xlenbits = zeros(64);
            {
                todo!("E_for");
                wX_bits(core_ctx, rd, result);
                RETIRE_SUCCESS
            }
        }}
        ast::XPERM4((rs2, rs1, rd)) => {{
            let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
            let rs2_val: BitDynamic = rX_bits(core_ctx, rs2);
            let mut result: xlenbits = zeros(64);
            {
                todo!("E_for");
                wX_bits(core_ctx, rd, result);
                RETIRE_SUCCESS
            }
        }}
        ast::ZICOND_RTYPE((rs2, rs1, rd, zicondop::CZERO_EQZ)) => {{
            let value: BitDynamic = rX_bits(core_ctx, rs1);
            let condition: BitDynamic = rX_bits(core_ctx, rs2);
            let result: xlenbits = if {(condition == zeros(64))} {
                zeros(64)
            } else {
                value
            };
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::ZICOND_RTYPE((rs2, rs1, rd, zicondop::CZERO_NEZ)) => {{
            let value: BitDynamic = rX_bits(core_ctx, rs1);
            let condition: BitDynamic = rX_bits(core_ctx, rs2);
            let result: xlenbits = if {(condition != zeros(64))} {
                zeros(64)
            } else {
                value
            };
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::VSETVLI((ma, ta, sew, lmul, rs1, rd)) => {{
            let LMUL_pow_ori: i128 = get_lmul_pow(core_ctx, ());
            let SEW_pow_ori: i128 = get_sew_pow(core_ctx, ());
            let ratio_pow_ori: i128 = (SEW_pow_ori - LMUL_pow_ori);
            core_ctx.vtype.bits = bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b0)), BitDynamic::from(bitvector_concat(BitDynamic::from(zeros(55)), BitDynamic::from(bitvector_concat(BitDynamic::from(ma), BitDynamic::from(bitvector_concat(BitDynamic::from(ta), BitDynamic::from(bitvector_concat(BitDynamic::from(sew), BitDynamic::from(lmul))))))))));
            let VLEN_pow: i128 = get_vlen_pow(core_ctx, ());
            let ELEN_pow: i128 = get_elen_pow(core_ctx, ());
            let LMUL_pow_new: i128 = get_lmul_pow(core_ctx, ());
            let SEW_pow_new: i128 = get_sew_pow(core_ctx, ());
            if {(SEW_pow_new > (LMUL_pow_new + ELEN_pow))} {
                handle_illegal_vtype(core_ctx, ());
                return RETIRE_SUCCESS;
            } else {
                ()
            };
            let VLMAX: i128 = i128::pow(2, (((VLEN_pow + LMUL_pow_new) - SEW_pow_new) as u32));
            if {(rs1 != zreg)} {
                let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
                let AVL: i128 = rs1_val.unsigned();
                core_ctx.vl = calculate_new_vl(core_ctx, AVL, VLMAX);
                {
                    let var_47: BitDynamic = core_ctx.vl;
                    wX_bits(core_ctx, rd, var_47)
                }
            } else if {(rd != zreg)} {
                let AVL: i128 = ones(64).unsigned();
                core_ctx.vl = to_bits(64, VLMAX);
                {
                    let var_46: BitDynamic = core_ctx.vl;
                    wX_bits(core_ctx, rd, var_46)
                }
            } else {
                let AVL: i128 = core_ctx.vl.unsigned();
                let ratio_pow_new: i128 = (SEW_pow_new - LMUL_pow_new);
                if {(ratio_pow_new != ratio_pow_ori)} {
                    handle_illegal_vtype(core_ctx, ());
                    return RETIRE_SUCCESS;
                } else {
                    ()
                }
            };
            set_vstart(core_ctx, zeros(16));
            RETIRE_SUCCESS
        }}
        ast::VSETVL((rs2, rs1, rd)) => {{
            let LMUL_pow_ori: i128 = get_lmul_pow(core_ctx, ());
            let SEW_pow_ori: i128 = get_sew_pow(core_ctx, ());
            let ratio_pow_ori: i128 = (SEW_pow_ori - LMUL_pow_ori);
            core_ctx.vtype.bits = rX_bits(core_ctx, rs2);
            let VLEN_pow: i128 = get_vlen_pow(core_ctx, ());
            let ELEN_pow: i128 = get_elen_pow(core_ctx, ());
            let LMUL_pow_new: i128 = get_lmul_pow(core_ctx, ());
            let SEW_pow_new: i128 = get_sew_pow(core_ctx, ());
            if {(SEW_pow_new > (LMUL_pow_new + ELEN_pow))} {
                handle_illegal_vtype(core_ctx, ());
                return RETIRE_SUCCESS;
            } else {
                ()
            };
            let VLMAX: i128 = i128::pow(2, (((VLEN_pow + LMUL_pow_new) - SEW_pow_new) as u32));
            if {(rs1 != zreg)} {
                let rs1_val: BitDynamic = rX_bits(core_ctx, rs1);
                let AVL: i128 = rs1_val.unsigned();
                core_ctx.vl = calculate_new_vl(core_ctx, AVL, VLMAX);
                {
                    let var_49: BitDynamic = core_ctx.vl;
                    wX_bits(core_ctx, rd, var_49)
                }
            } else if {(rd != zreg)} {
                let AVL: i128 = ones(64).unsigned();
                core_ctx.vl = to_bits(64, VLMAX);
                {
                    let var_48: BitDynamic = core_ctx.vl;
                    wX_bits(core_ctx, rd, var_48)
                }
            } else {
                let AVL: i128 = core_ctx.vl.unsigned();
                let ratio_pow_new: i128 = (SEW_pow_new - LMUL_pow_new);
                if {(ratio_pow_new != ratio_pow_ori)} {
                    handle_illegal_vtype(core_ctx, ());
                    return RETIRE_SUCCESS;
                } else {
                    ()
                }
            };
            set_vstart(core_ctx, zeros(16));
            RETIRE_SUCCESS
        }}
        ast::VSETIVLI((ma, ta, sew, lmul, uimm, rd)) => {{
            core_ctx.vtype.bits = bitvector_concat(BitDynamic::from(BitDynamic::new(1, 0b0)), BitDynamic::from(bitvector_concat(BitDynamic::from(zeros(55)), BitDynamic::from(bitvector_concat(BitDynamic::from(ma), BitDynamic::from(bitvector_concat(BitDynamic::from(ta), BitDynamic::from(bitvector_concat(BitDynamic::from(sew), BitDynamic::from(lmul))))))))));
            let VLEN_pow: i128 = get_vlen_pow(core_ctx, ());
            let ELEN_pow: i128 = get_elen_pow(core_ctx, ());
            let LMUL_pow_new: i128 = get_lmul_pow(core_ctx, ());
            let SEW_pow_new: i128 = get_sew_pow(core_ctx, ());
            if {(SEW_pow_new > (LMUL_pow_new + ELEN_pow))} {
                handle_illegal_vtype(core_ctx, ());
                return RETIRE_SUCCESS;
            } else {
                ()
            };
            let VLMAX: i128 = i128::pow(2, (((VLEN_pow + LMUL_pow_new) - SEW_pow_new) as u32));
            let AVL: i128 = uimm.unsigned();
            core_ctx.vl = calculate_new_vl(core_ctx, AVL, VLMAX);
            {
                let var_50: BitDynamic = core_ctx.vl;
                wX_bits(core_ctx, rd, var_50)
            };
            set_vstart(core_ctx, zeros(16));
            RETIRE_SUCCESS
        }}
        ast::VVTYPE((funct6, vm, vs2, vs1, vd)) => {{
            let SEW_pow: i128 = get_sew_pow(core_ctx, ());
            let SEW: i128 = get_sew(core_ctx, ());
            let LMUL_pow: i128 = get_lmul_pow(core_ctx, ());
            let VLEN_pow: i128 = get_vlen_pow(core_ctx, ());
            let num_elem: i128 = get_num_elem(core_ctx, LMUL_pow, SEW);
            if {illegal_normal(core_ctx, vd, vm)} {
                return ExecutionResult::Illegal_Instruction(());
            } else {
                ()
            };
            let n: i128 = num_elem;
            let m: i128 = SEW;
            let vm_val: BitDynamic = read_vmask(core_ctx, num_elem, vm, zvreg);
            let vs1_val: Vec::<BitDynamic> = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vs1);
            let vs2_val: Vec::<BitDynamic> = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vs2);
            let vd_val: Vec::<BitDynamic> = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vd);
            let (initial_result, mask): (Vec::<BitDynamic>, BitDynamic) = match init_masked_result(core_ctx, num_elem, SEW, LMUL_pow, vd_val, vm_val) {
                result::Ok(v) => {v}
                result::Err(()) => {return ExecutionResult::Illegal_Instruction(());}
                _ => {panic!("Unreachable code")}
            };
            let mut result: Vec::<BitDynamic> = initial_result;
            {
                for i in 0..=(num_elem - 1) {
                    if {(bitvector_access(mask, i) == true)} {
                        result[(i as usize)] = match funct6 {
                            vvfunct6::VV_VADD => {vs2_val[(i as usize)].wrapped_add(vs1_val[(i as usize)])}
                            vvfunct6::VV_VSUB => {sub_vec(vs2_val[(i as usize)], vs1_val[(i as usize)])}
                            vvfunct6::VV_VAND => {(vs2_val[(i as usize)] & vs1_val[(i as usize)])}
                            vvfunct6::VV_VOR => {(vs2_val[(i as usize)] | vs1_val[(i as usize)])}
                            vvfunct6::VV_VXOR => {(vs2_val[(i as usize)] ^ vs1_val[(i as usize)])}
                            vvfunct6::VV_VSADDU => {unsigned_saturation(core_ctx, __id(m), vs2_val[(i as usize)].zero_extend_dyn((__id(m) + 1)).wrapped_add(vs1_val[(i as usize)].zero_extend_dyn((__id(m) + 1))))}
                            vvfunct6::VV_VSADD => {signed_saturation(core_ctx, __id(m), sign_extend((__id(m) + 1), vs2_val[(i as usize)]).wrapped_add(sign_extend((__id(m) + 1), vs1_val[(i as usize)])))}
                            vvfunct6::VV_VSSUBU => {{
                                if {(vs2_val[(i as usize)].unsigned() < vs1_val[(i as usize)].unsigned())} {
                                    zeros(__id(m))
                                } else {
                                    unsigned_saturation(core_ctx, __id(m), sub_vec(vs2_val[(i as usize)].zero_extend_dyn((__id(m) + 1)), vs1_val[(i as usize)].zero_extend_dyn((__id(m) + 1))))
                                }
                            }}
                            vvfunct6::VV_VSSUB => {signed_saturation(core_ctx, __id(m), sub_vec(sign_extend((__id(m) + 1), vs2_val[(i as usize)]), sign_extend((__id(m) + 1), vs1_val[(i as usize)])))}
                            vvfunct6::VV_VSMUL => {{
                                let result_mul: BitDynamic = to_bits((__id(m) * 2), (vs2_val[(i as usize)].signed() * vs1_val[(i as usize)].signed()));
                                let rounding_incr: BitDynamic = get_fixed_rounding_incr(core_ctx, result_mul, (__id(m) - 1));
                                let result_wide: BitDynamic = (result_mul >> (__id(m) - 1)).wrapped_add(rounding_incr.zero_extend_dyn((__id(m) * 2)));
                                signed_saturation(core_ctx, __id(m), subrange_bits(result_wide, __id(m), 0))
                            }}
                            vvfunct6::VV_VSLL => {{
                                let shift_amount: nat = get_shift_amount(vs1_val[(i as usize)], SEW);
                                (vs2_val[(i as usize)] << shift_amount)
                            }}
                            vvfunct6::VV_VSRL => {{
                                let shift_amount: nat = get_shift_amount(vs1_val[(i as usize)], SEW);
                                (vs2_val[(i as usize)] >> shift_amount)
                            }}
                            vvfunct6::VV_VSRA => {{
                                let shift_amount: nat = get_shift_amount(vs1_val[(i as usize)], SEW);
                                let v_double: BitDynamic = sign_extend((__id(m) * 2), vs2_val[(i as usize)]);
                                slice((v_double >> shift_amount), 0, SEW)
                            }}
                            vvfunct6::VV_VSSRL => {{
                                let shift_amount: nat = get_shift_amount(vs1_val[(i as usize)], SEW);
                                let rounding_incr: BitDynamic = get_fixed_rounding_incr(core_ctx, vs2_val[(i as usize)], shift_amount);
                                (vs2_val[(i as usize)] >> shift_amount).wrapped_add(rounding_incr.zero_extend_dyn(__id(m)))
                            }}
                            vvfunct6::VV_VSSRA => {{
                                let shift_amount: nat = get_shift_amount(vs1_val[(i as usize)], SEW);
                                let rounding_incr: BitDynamic = get_fixed_rounding_incr(core_ctx, vs2_val[(i as usize)], shift_amount);
                                let v_double: BitDynamic = sign_extend((__id(m) * 2), vs2_val[(i as usize)]);
                                slice((v_double >> shift_amount), 0, SEW).wrapped_add(rounding_incr.zero_extend_dyn(__id(m)))
                            }}
                            vvfunct6::VV_VMINU => {to_bits(SEW, min_int(vs2_val[(i as usize)].unsigned(), vs1_val[(i as usize)].unsigned()))}
                            vvfunct6::VV_VMIN => {to_bits(SEW, min_int(vs2_val[(i as usize)].signed(), vs1_val[(i as usize)].signed()))}
                            vvfunct6::VV_VMAXU => {to_bits(SEW, max_int(vs2_val[(i as usize)].unsigned(), vs1_val[(i as usize)].unsigned()))}
                            vvfunct6::VV_VMAX => {to_bits(SEW, max_int(vs2_val[(i as usize)].signed(), vs1_val[(i as usize)].signed()))}
                            vvfunct6::VV_VRGATHER => {{
                                if {((vs1 == vd) || (vs2 == vd))} {
                                    return ExecutionResult::Illegal_Instruction(());
                                } else {
                                    ()
                                };
                                let idx: i128 = vs1_val[(i as usize)].unsigned();
                                let VLMAX: i128 = i128::pow(2, (((LMUL_pow + VLEN_pow) - SEW_pow) as u32));
                                assert!((VLMAX <= __id(n)), "riscv_insts_vext_arith.sail:123.48-123.49");
                                if {(idx < VLMAX)} {
                                    vs2_val[(idx as usize)]
                                } else {
                                    zeros(__id(m))
                                }
                            }}
                            vvfunct6::VV_VRGATHEREI16 => {{
                                if {((vs1 == vd) || (vs2 == vd))} {
                                    return ExecutionResult::Illegal_Instruction(());
                                } else {
                                    ()
                                };
                                let vs1_new: Vec::<BitDynamic> = read_vreg(core_ctx, num_elem, 16, ((4 + LMUL_pow) - SEW_pow), vs1);
                                let idx: i128 = vs1_new[(i as usize)].unsigned();
                                let VLMAX: i128 = i128::pow(2, (((LMUL_pow + VLEN_pow) - SEW_pow) as u32));
                                assert!((VLMAX <= __id(n)), "riscv_insts_vext_arith.sail:132.48-132.49");
                                if {(idx < VLMAX)} {
                                    vs2_val[(idx as usize)]
                                } else {
                                    zeros(__id(m))
                                }
                            }}
                            _ => {panic!("Unreachable code")}
                        }
                    } else {
                        ()
                    }
                };
                write_vreg(core_ctx, num_elem, SEW, LMUL_pow, vd, result);
                set_vstart(core_ctx, zeros(16));
                RETIRE_SUCCESS
            }
        }}
        ast::NVSTYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'NVSTYPE'")}
        ast::NVTYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'NVTYPE'")}
        ast::MASKTYPEV((vs2, vs1, vd)) => {todo!("Unsupported: 'MASKTYPEV'")}
        ast::MOVETYPEV((vs1, vd)) => {todo!("Unsupported: 'MOVETYPEV'")}
        ast::VXTYPE((funct6, vm, vs2, rs1, vd)) => {todo!("Unsupported: 'VXTYPE'")}
        ast::NXSTYPE((funct6, vm, vs2, rs1, vd)) => {todo!("Unsupported: 'NXSTYPE'")}
        ast::NXTYPE((funct6, vm, vs2, rs1, vd)) => {todo!("Unsupported: 'NXTYPE'")}
        ast::VXSG((funct6, vm, vs2, rs1, vd)) => {todo!("Unsupported: 'VXSG'")}
        ast::MASKTYPEX((vs2, rs1, vd)) => {todo!("Unsupported: 'MASKTYPEX'")}
        ast::MOVETYPEX((rs1, vd)) => {{
            let SEW: i128 = get_sew(core_ctx, ());
            let LMUL_pow: i128 = get_lmul_pow(core_ctx, ());
            let num_elem: i128 = get_num_elem(core_ctx, LMUL_pow, SEW);
            if {illegal_vd_unmasked(core_ctx, ())} {
                return ExecutionResult::Illegal_Instruction(());
            } else {
                ()
            };
            let n: i128 = num_elem;
            let m: i128 = SEW;
            let rs1_val: BitDynamic = get_scalar(core_ctx, rs1, __id(m));
            let vm_val: BitDynamic = read_vmask(core_ctx, num_elem, BitDynamic::new(1, 0b1), zvreg);
            let vd_val: Vec::<BitDynamic> = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vd);
            let (initial_result, mask): (Vec::<BitDynamic>, BitDynamic) = match init_masked_result(core_ctx, num_elem, SEW, LMUL_pow, vd_val, vm_val) {
                result::Ok(v) => {v}
                result::Err(()) => {return ExecutionResult::Illegal_Instruction(());}
                _ => {panic!("Unreachable code")}
            };
            let mut result: Vec::<BitDynamic> = initial_result;
            {
                for i in 0..=(num_elem - 1) {
                    if {(bitvector_access(mask, i) == true)} {
                        result[(i as usize)] = rs1_val
                    } else {
                        ()
                    }
                };
                write_vreg(core_ctx, num_elem, SEW, LMUL_pow, vd, result);
                set_vstart(core_ctx, zeros(16));
                RETIRE_SUCCESS
            }
        }}
        ast::VITYPE((funct6, vm, vs2, simm, vd)) => {{
            let SEW: i128 = get_sew(core_ctx, ());
            let LMUL_pow: i128 = get_lmul_pow(core_ctx, ());
            let num_elem: i128 = get_num_elem(core_ctx, LMUL_pow, SEW);
            if {illegal_normal(core_ctx, vd, vm)} {
                return ExecutionResult::Illegal_Instruction(());
            } else {
                ()
            };
            let n: i128 = num_elem;
            let m: i128 = SEW;
            let vm_val: BitDynamic = read_vmask(core_ctx, num_elem, vm, zvreg);
            let imm_val: BitDynamic = sign_extend(__id(m), simm);
            let vs2_val: Vec::<BitDynamic> = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vs2);
            let vd_val: Vec::<BitDynamic> = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vd);
            let (initial_result, mask): (Vec::<BitDynamic>, BitDynamic) = match init_masked_result(core_ctx, num_elem, SEW, LMUL_pow, vd_val, vm_val) {
                result::Ok(v) => {v}
                result::Err(()) => {return ExecutionResult::Illegal_Instruction(());}
                _ => {panic!("Unreachable code")}
            };
            let mut result: Vec::<BitDynamic> = initial_result;
            {
                for i in 0..=(num_elem - 1) {
                    if {(bitvector_access(mask, i) == true)} {
                        result[(i as usize)] = match funct6 {
                            vifunct6::VI_VADD => {vs2_val[(i as usize)].wrapped_add(imm_val)}
                            vifunct6::VI_VRSUB => {sub_vec(imm_val, vs2_val[(i as usize)])}
                            vifunct6::VI_VAND => {(vs2_val[(i as usize)] & imm_val)}
                            vifunct6::VI_VOR => {(vs2_val[(i as usize)] | imm_val)}
                            vifunct6::VI_VXOR => {(vs2_val[(i as usize)] ^ imm_val)}
                            vifunct6::VI_VSADDU => {unsigned_saturation(core_ctx, __id(m), vs2_val[(i as usize)].zero_extend_dyn((__id(m) + 1)).wrapped_add(imm_val.zero_extend_dyn((__id(m) + 1))))}
                            vifunct6::VI_VSADD => {signed_saturation(core_ctx, __id(m), sign_extend((__id(m) + 1), vs2_val[(i as usize)]).wrapped_add(sign_extend((__id(m) + 1), imm_val)))}
                            vifunct6::VI_VSLL => {{
                                let shift_amount: nat = get_shift_amount(simm.zero_extend_dyn(__id(m)), SEW);
                                (vs2_val[(i as usize)] << shift_amount)
                            }}
                            vifunct6::VI_VSRL => {{
                                let shift_amount: nat = get_shift_amount(simm.zero_extend_dyn(__id(m)), SEW);
                                (vs2_val[(i as usize)] >> shift_amount)
                            }}
                            vifunct6::VI_VSRA => {{
                                let shift_amount: nat = get_shift_amount(simm.zero_extend_dyn(__id(m)), SEW);
                                let v_double: BitDynamic = sign_extend((__id(m) * 2), vs2_val[(i as usize)]);
                                slice((v_double >> shift_amount), 0, SEW)
                            }}
                            vifunct6::VI_VSSRL => {{
                                let shift_amount: nat = get_shift_amount(simm.zero_extend_dyn(__id(m)), SEW);
                                let rounding_incr: BitDynamic = get_fixed_rounding_incr(core_ctx, vs2_val[(i as usize)], shift_amount);
                                (vs2_val[(i as usize)] >> shift_amount).wrapped_add(rounding_incr.zero_extend_dyn(__id(m)))
                            }}
                            vifunct6::VI_VSSRA => {{
                                let shift_amount: nat = get_shift_amount(simm.zero_extend_dyn(__id(m)), SEW);
                                let rounding_incr: BitDynamic = get_fixed_rounding_incr(core_ctx, vs2_val[(i as usize)], shift_amount);
                                let v_double: BitDynamic = sign_extend((__id(m) * 2), vs2_val[(i as usize)]);
                                slice((v_double >> shift_amount), 0, SEW).wrapped_add(rounding_incr.zero_extend_dyn(__id(m)))
                            }}
                            _ => {panic!("Unreachable code")}
                        }
                    } else {
                        ()
                    }
                };
                write_vreg(core_ctx, num_elem, SEW, LMUL_pow, vd, result);
                set_vstart(core_ctx, zeros(16));
                RETIRE_SUCCESS
            }
        }}
        ast::NISTYPE((funct6, vm, vs2, simm, vd)) => {todo!("Unsupported: 'NISTYPE'")}
        ast::NITYPE((funct6, vm, vs2, simm, vd)) => {todo!("Unsupported: 'NITYPE'")}
        ast::VISG((funct6, vm, vs2, simm, vd)) => {todo!("Unsupported: 'VISG'")}
        ast::MASKTYPEI((vs2, simm, vd)) => {todo!("Unsupported: 'MASKTYPEI'")}
        ast::MOVETYPEI((vd, simm)) => {todo!("Unsupported: 'MOVETYPEI'")}
        ast::VMVRTYPE((vs2, simm, vd)) => {todo!("Unsupported: 'VMVRTYPE'")}
        ast::MVVTYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'MVVTYPE'")}
        ast::MVVMATYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'MVVMATYPE'")}
        ast::VEXT2TYPE((funct6, vm, vs2, vd)) => {todo!("Unsupported: 'VEXT2TYPE'")}
        ast::VEXT4TYPE((funct6, vm, vs2, vd)) => {todo!("Unsupported: 'VEXT4TYPE'")}
        ast::VEXT8TYPE((funct6, vm, vs2, vd)) => {todo!("Unsupported: 'VEXT8TYPE'")}
        ast::VMVXS((vs2, rd)) => {todo!("Unsupported: 'VMVXS'")}
        ast::MVVCOMPRESS((vs2, vs1, vd)) => {todo!("Unsupported: 'MVVCOMPRESS'")}
        ast::MVXTYPE((funct6, vm, vs2, rs1, vd)) => {todo!("Unsupported: 'MVXTYPE'")}
        ast::MVXMATYPE((funct6, vm, vs2, rs1, vd)) => {todo!("Unsupported: 'MVXMATYPE'")}
        ast::VMVSX((rs1, vd)) => {todo!("Unsupported: 'VMVSX'")}
        ast::FVVTYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'FVVTYPE'")}
        ast::FVVMATYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'FVVMATYPE'")}
        ast::VFUNARY0((vm, vs2, vfunary0, vd)) => {todo!("Unsupported: 'VFUNARY0'")}
        ast::VFNUNARY0((vm, vs2, vfnunary0, vd)) => {todo!("Unsupported: 'VFNUNARY0'")}
        ast::VFUNARY1((vm, vs2, vfunary1, vd)) => {todo!("Unsupported: 'VFUNARY1'")}
        ast::VLSEGTYPE((nf, vm, rs1, width, vd)) => {todo!("Unsupported: 'VLSEGTYPE'")}
        ast::VLSEGFFTYPE((nf, vm, rs1, width, vd)) => {todo!("Unsupported: 'VLSEGFFTYPE'")}
        ast::VSSEGTYPE((nf, vm, rs1, width, vs3)) => {todo!("Unsupported: 'VSSEGTYPE'")}
        ast::VLSSEGTYPE((nf, vm, rs2, rs1, width, vd)) => {todo!("Unsupported: 'VLSSEGTYPE'")}
        ast::VSSSEGTYPE((nf, vm, rs2, rs1, width, vs3)) => {todo!("Unsupported: 'VSSSEGTYPE'")}
        ast::VLUXSEGTYPE((nf, vm, vs2, rs1, width, vd)) => {todo!("Unsupported: 'VLUXSEGTYPE'")}
        ast::VLOXSEGTYPE((nf, vm, vs2, rs1, width, vd)) => {todo!("Unsupported: 'VLOXSEGTYPE'")}
        ast::VSUXSEGTYPE((nf, vm, vs2, rs1, width, vs3)) => {todo!("Unsupported: 'VSUXSEGTYPE'")}
        ast::VSOXSEGTYPE((nf, vm, vs2, rs1, width, vs3)) => {todo!("Unsupported: 'VSOXSEGTYPE'")}
        ast::VLRETYPE((nf, rs1, width, vd)) => {todo!("Unsupported: 'VLRETYPE'")}
        ast::VSRETYPE((nf, rs1, vs3)) => {todo!("Unsupported: 'VSRETYPE'")}
        ast::VMTYPE((rs1, vd_or_vs3, op)) => {todo!("Unsupported: 'VMTYPE'")}
        ast::MMTYPE((funct6, vs2, vs1, vd)) => {todo!("Unsupported: 'MMTYPE'")}
        ast::VCPOP_M((vm, vs2, rd)) => {todo!("Unsupported: 'VCPOP_M'")}
        ast::VFIRST_M((vm, vs2, rd)) => {todo!("Unsupported: 'VFIRST_M'")}
        ast::VMSBF_M((vm, vs2, vd)) => {todo!("Unsupported: 'VMSBF_M'")}
        ast::VMSIF_M((vm, vs2, vd)) => {todo!("Unsupported: 'VMSIF_M'")}
        ast::VMSOF_M((vm, vs2, vd)) => {todo!("Unsupported: 'VMSOF_M'")}
        ast::VIOTA_M((vm, vs2, vd)) => {todo!("Unsupported: 'VIOTA_M'")}
        ast::VID_V((vm, vd)) => {todo!("Unsupported: 'VID_V'")}
        ast::VVMTYPE((funct6, vs2, vs1, vd)) => {todo!("Unsupported: 'VVMTYPE'")}
        ast::VVMCTYPE((funct6, vs2, vs1, vd)) => {todo!("Unsupported: 'VVMCTYPE'")}
        ast::VVMSTYPE((funct6, vs2, vs1, vd)) => {todo!("Unsupported: 'VVMSTYPE'")}
        ast::VVCMPTYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'VVCMPTYPE'")}
        ast::VXMTYPE((funct6, vs2, rs1, vd)) => {todo!("Unsupported: 'VXMTYPE'")}
        ast::VXMCTYPE((funct6, vs2, rs1, vd)) => {todo!("Unsupported: 'VXMCTYPE'")}
        ast::VXMSTYPE((funct6, vs2, rs1, vd)) => {todo!("Unsupported: 'VXMSTYPE'")}
        ast::VXCMPTYPE((funct6, vm, vs2, rs1, vd)) => {todo!("Unsupported: 'VXCMPTYPE'")}
        ast::VIMTYPE((funct6, vs2, simm, vd)) => {todo!("Unsupported: 'VIMTYPE'")}
        ast::VIMCTYPE((funct6, vs2, simm, vd)) => {todo!("Unsupported: 'VIMCTYPE'")}
        ast::VIMSTYPE((funct6, vs2, simm, vd)) => {todo!("Unsupported: 'VIMSTYPE'")}
        ast::VICMPTYPE((funct6, vm, vs2, simm, vd)) => {todo!("Unsupported: 'VICMPTYPE'")}
        ast::FVVMTYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'FVVMTYPE'")}
        ast::RMVVTYPE((funct6, vm, vs2, vs1, vd)) => {todo!("Unsupported: 'RMVVTYPE'")}
        ast::ZICBOM((cbop_zicbom::CBO_CLEAN, rs1)) => {if {{
            let var_51: Privilege = core_ctx.cur_privilege;
            cbo_clean_flush_enabled(core_ctx, var_51)
        }} {
            process_clean_inval(core_ctx, rs1, cbop_zicbom::CBO_CLEAN)
        } else {
            ExecutionResult::Illegal_Instruction(())
        }}
        ast::ZICBOM((cbop_zicbom::CBO_FLUSH, rs1)) => {if {{
            let var_52: Privilege = core_ctx.cur_privilege;
            cbo_clean_flush_enabled(core_ctx, var_52)
        }} {
            process_clean_inval(core_ctx, rs1, cbop_zicbom::CBO_FLUSH)
        } else {
            ExecutionResult::Illegal_Instruction(())
        }}
        ast::ZICBOM((cbop_zicbom::CBO_INVAL, rs1)) => {match {
            let var_53: Privilege = core_ctx.cur_privilege;
            cbop_priv_check(core_ctx, var_53)
        } {
            checked_cbop::CBOP_ILLEGAL => {ExecutionResult::Illegal_Instruction(())}
            checked_cbop::CBOP_ILLEGAL_VIRTUAL => {panic!("{}, l {}: {}", "riscv_insts_zicbom.sail", 151, "unimplemented")}
            checked_cbop::CBOP_INVAL_INVAL => {process_clean_inval(core_ctx, rs1, cbop_zicbom::CBO_INVAL)}
            checked_cbop::CBOP_INVAL_FLUSH => {process_clean_inval(core_ctx, rs1, cbop_zicbom::CBO_FLUSH)}
            _ => {panic!("Unreachable code")}
        }}
        ast::ZICBOZ(rs1) => {todo!("Unsupported: 'ZICBOZ'")}
        ast::VANDN_VV((vm, vs1, vs2, vd)) => {todo!("Unsupported: 'VANDN_VV'")}
        ast::VANDN_VX((vm, vs2, rs1, vd)) => {todo!("Unsupported: 'VANDN_VX'")}
        ast::VBREV_V((vm, vs2, vd)) => {todo!("Unsupported: 'VBREV_V'")}
        ast::VBREV8_V((vm, vs2, vd)) => {todo!("Unsupported: 'VBREV8_V'")}
        ast::VREV8_V((vm, vs2, vd)) => {todo!("Unsupported: 'VREV8_V'")}
        ast::VCLZ_V((vm, vs2, vd)) => {todo!("Unsupported: 'VCLZ_V'")}
        ast::VCTZ_V((vm, vs2, vd)) => {todo!("Unsupported: 'VCTZ_V'")}
        ast::VCPOP_V((vm, vs2, vd)) => {todo!("Unsupported: 'VCPOP_V'")}
        ast::VROL_VV((vm, vs1, vs2, vd)) => {todo!("Unsupported: 'VROL_VV'")}
        ast::VROL_VX((vm, vs2, rs1, vd)) => {todo!("Unsupported: 'VROL_VX'")}
        ast::VROR_VV((vm, vs1, vs2, vd)) => {todo!("Unsupported: 'VROR_VV'")}
        ast::VROR_VX((vm, vs2, rs1, vd)) => {todo!("Unsupported: 'VROR_VX'")}
        ast::VROR_VI((vm, vs2, uimm, vd)) => {todo!("Unsupported: 'VROR_VI'")}
        ast::VCLMUL_VV((vm, vs2, vs1, vd)) => {todo!("Unsupported: 'VCLMUL_VV'")}
        ast::VCLMUL_VX((vm, vs2, rs1, vd)) => {todo!("Unsupported: 'VCLMUL_VX'")}
        ast::VCLMULH_VV((vm, vs2, vs1, vd)) => {todo!("Unsupported: 'VCLMULH_VV'")}
        ast::VCLMULH_VX((vm, vs2, rs1, vd)) => {todo!("Unsupported: 'VCLMULH_VX'")}
        ast::VSHA2MS_VV((vs2, vs1, vd)) => {todo!("Unsupported: 'VSHA2MS_VV'")}
        ast::ZVKSHA2TYPE((funct6, vs2, vs1, vd)) => {todo!("Unsupported: 'ZVKSHA2TYPE'")}
        ast::VSM3ME_VV((vs2, vs1, vd)) => {todo!("Unsupported: 'VSM3ME_VV'")}
        ast::VSM3C_VI((vs2, uimm, vd)) => {todo!("Unsupported: 'VSM3C_VI'")}
        ast::ZIMOP_MOP_R((mop, rs1, rd)) => {{
            wX_bits(core_ctx, rd, zeros(64));
            RETIRE_SUCCESS
        }}
        ast::ZIMOP_MOP_RR((mop, rs2, rs1, rd)) => {{
            wX_bits(core_ctx, rd, zeros(64));
            RETIRE_SUCCESS
        }}
        ast::ZCMOP(mop) => {{
            RETIRE_SUCCESS
        }}
        ast::JALR((imm, rs1, rd)) => {{
            let t: xlenbits = rX_bits(core_ctx, rs1).wrapped_add(sign_extend(64, imm));
            match ext_control_check_addr(t) {
                Ext_ControlAddr_Check::Ext_ControlAddr_Error(e) => {ExecutionResult::Ext_ControlAddr_Check_Failure(e)}
                Ext_ControlAddr_Check::Ext_ControlAddr_OK(addr) => {{
                    let target: BitDynamic = bitvector_update(bits_of_virtaddr(addr), 0, false);
                    if {(bit_to_bool(bitvector_access(target, 1)) && !(currentlyEnabled(core_ctx, extension::Ext_Zca)))} {
                        ExecutionResult::Memory_Exception((addr, ExceptionType::E_Fetch_Addr_Align(())))
                    } else {
                        {
                            let var_54: BitDynamic = get_next_pc(core_ctx, ());
                            wX_bits(core_ctx, rd, var_54)
                        };
                        set_next_pc(core_ctx, target);
                        RETIRE_SUCCESS
                    }
                }}
                _ => {panic!("Unreachable code")}
            }
        }}
        ast::ILLEGAL(s) => {ExecutionResult::Illegal_Instruction(())}
        ast::C_ILLEGAL(s) => {ExecutionResult::Illegal_Instruction(())}
        _ => {panic!("Unreachable code")}
    }
}

/// csr_name_map_backwards_matches
///
/// Generated from the Sail sources.
pub fn csr_name_map_backwards_matches(arg_hashtag_: &'static str) -> bool {
    let head_exp_hashtag_: &'static str = arg_hashtag_;
    match match head_exp_hashtag_ {
        "misa" => {Some(true)}
        "mstatus" => {Some(true)}
        "menvcfg" => {Some(true)}
        "menvcfgh" => {Some(true)}
        "senvcfg" => {Some(true)}
        "mie" => {Some(true)}
        "mip" => {Some(true)}
        "medeleg" => {Some(true)}
        "medelegh" => {Some(true)}
        "mideleg" => {Some(true)}
        "mcause" => {Some(true)}
        "mtval" => {Some(true)}
        "mscratch" => {Some(true)}
        "scounteren" => {Some(true)}
        "mcounteren" => {Some(true)}
        "mcountinhibit" => {Some(true)}
        "mvendorid" => {Some(true)}
        "marchid" => {Some(true)}
        "mimpid" => {Some(true)}
        "mhartid" => {Some(true)}
        "mconfigptr" => {Some(true)}
        "sstatus" => {Some(true)}
        "sip" => {Some(true)}
        "sie" => {Some(true)}
        "sscratch" => {Some(true)}
        "scause" => {Some(true)}
        "stval" => {Some(true)}
        "tselect" => {Some(true)}
        "tdata1" => {Some(true)}
        "tdata2" => {Some(true)}
        "tdata3" => {Some(true)}
        "pmpcfg0" => {Some(true)}
        "pmpcfg1" => {Some(true)}
        "pmpcfg2" => {Some(true)}
        "pmpcfg3" => {Some(true)}
        "pmpcfg4" => {Some(true)}
        "pmpcfg5" => {Some(true)}
        "pmpcfg6" => {Some(true)}
        "pmpcfg7" => {Some(true)}
        "pmpcfg8" => {Some(true)}
        "pmpcfg9" => {Some(true)}
        "pmpcfg10" => {Some(true)}
        "pmpcfg11" => {Some(true)}
        "pmpcfg12" => {Some(true)}
        "pmpcfg13" => {Some(true)}
        "pmpcfg14" => {Some(true)}
        "pmpcfg15" => {Some(true)}
        "pmpaddr0" => {Some(true)}
        "pmpaddr1" => {Some(true)}
        "pmpaddr2" => {Some(true)}
        "pmpaddr3" => {Some(true)}
        "pmpaddr4" => {Some(true)}
        "pmpaddr5" => {Some(true)}
        "pmpaddr6" => {Some(true)}
        "pmpaddr7" => {Some(true)}
        "pmpaddr8" => {Some(true)}
        "pmpaddr9" => {Some(true)}
        "pmpaddr10" => {Some(true)}
        "pmpaddr11" => {Some(true)}
        "pmpaddr12" => {Some(true)}
        "pmpaddr13" => {Some(true)}
        "pmpaddr14" => {Some(true)}
        "pmpaddr15" => {Some(true)}
        "pmpaddr16" => {Some(true)}
        "pmpaddr17" => {Some(true)}
        "pmpaddr18" => {Some(true)}
        "pmpaddr19" => {Some(true)}
        "pmpaddr20" => {Some(true)}
        "pmpaddr21" => {Some(true)}
        "pmpaddr22" => {Some(true)}
        "pmpaddr23" => {Some(true)}
        "pmpaddr24" => {Some(true)}
        "pmpaddr25" => {Some(true)}
        "pmpaddr26" => {Some(true)}
        "pmpaddr27" => {Some(true)}
        "pmpaddr28" => {Some(true)}
        "pmpaddr29" => {Some(true)}
        "pmpaddr30" => {Some(true)}
        "pmpaddr31" => {Some(true)}
        "pmpaddr32" => {Some(true)}
        "pmpaddr33" => {Some(true)}
        "pmpaddr34" => {Some(true)}
        "pmpaddr35" => {Some(true)}
        "pmpaddr36" => {Some(true)}
        "pmpaddr37" => {Some(true)}
        "pmpaddr38" => {Some(true)}
        "pmpaddr39" => {Some(true)}
        "pmpaddr40" => {Some(true)}
        "pmpaddr41" => {Some(true)}
        "pmpaddr42" => {Some(true)}
        "pmpaddr43" => {Some(true)}
        "pmpaddr44" => {Some(true)}
        "pmpaddr45" => {Some(true)}
        "pmpaddr46" => {Some(true)}
        "pmpaddr47" => {Some(true)}
        "pmpaddr48" => {Some(true)}
        "pmpaddr49" => {Some(true)}
        "pmpaddr50" => {Some(true)}
        "pmpaddr51" => {Some(true)}
        "pmpaddr52" => {Some(true)}
        "pmpaddr53" => {Some(true)}
        "pmpaddr54" => {Some(true)}
        "pmpaddr55" => {Some(true)}
        "pmpaddr56" => {Some(true)}
        "pmpaddr57" => {Some(true)}
        "pmpaddr58" => {Some(true)}
        "pmpaddr59" => {Some(true)}
        "pmpaddr60" => {Some(true)}
        "pmpaddr61" => {Some(true)}
        "pmpaddr62" => {Some(true)}
        "pmpaddr63" => {Some(true)}
        "vstart" => {Some(true)}
        "vxsat" => {Some(true)}
        "vxrm" => {Some(true)}
        "vcsr" => {Some(true)}
        "vl" => {Some(true)}
        "vtype" => {Some(true)}
        "vlenb" => {Some(true)}
        "stvec" => {Some(true)}
        "sepc" => {Some(true)}
        "mtvec" => {Some(true)}
        "mepc" => {Some(true)}
        "hpmcounter3" => {Some(true)}
        "hpmcounter4" => {Some(true)}
        "hpmcounter5" => {Some(true)}
        "hpmcounter6" => {Some(true)}
        "hpmcounter7" => {Some(true)}
        "hpmcounter8" => {Some(true)}
        "hpmcounter9" => {Some(true)}
        "hpmcounter10" => {Some(true)}
        "hpmcounter11" => {Some(true)}
        "hpmcounter12" => {Some(true)}
        "hpmcounter13" => {Some(true)}
        "hpmcounter14" => {Some(true)}
        "hpmcounter15" => {Some(true)}
        "hpmcounter16" => {Some(true)}
        "hpmcounter17" => {Some(true)}
        "hpmcounter18" => {Some(true)}
        "hpmcounter19" => {Some(true)}
        "hpmcounter20" => {Some(true)}
        "hpmcounter21" => {Some(true)}
        "hpmcounter22" => {Some(true)}
        "hpmcounter23" => {Some(true)}
        "hpmcounter24" => {Some(true)}
        "hpmcounter25" => {Some(true)}
        "hpmcounter26" => {Some(true)}
        "hpmcounter27" => {Some(true)}
        "hpmcounter28" => {Some(true)}
        "hpmcounter29" => {Some(true)}
        "hpmcounter30" => {Some(true)}
        "hpmcounter31" => {Some(true)}
        "hpmcounter3h" => {Some(true)}
        "hpmcounter4h" => {Some(true)}
        "hpmcounter5h" => {Some(true)}
        "hpmcounter6h" => {Some(true)}
        "hpmcounter7h" => {Some(true)}
        "hpmcounter8h" => {Some(true)}
        "hpmcounter9h" => {Some(true)}
        "hpmcounter10h" => {Some(true)}
        "hpmcounter11h" => {Some(true)}
        "hpmcounter12h" => {Some(true)}
        "hpmcounter13h" => {Some(true)}
        "hpmcounter14h" => {Some(true)}
        "hpmcounter15h" => {Some(true)}
        "hpmcounter16h" => {Some(true)}
        "hpmcounter17h" => {Some(true)}
        "hpmcounter18h" => {Some(true)}
        "hpmcounter19h" => {Some(true)}
        "hpmcounter20h" => {Some(true)}
        "hpmcounter21h" => {Some(true)}
        "hpmcounter22h" => {Some(true)}
        "hpmcounter23h" => {Some(true)}
        "hpmcounter24h" => {Some(true)}
        "hpmcounter25h" => {Some(true)}
        "hpmcounter26h" => {Some(true)}
        "hpmcounter27h" => {Some(true)}
        "hpmcounter28h" => {Some(true)}
        "hpmcounter29h" => {Some(true)}
        "hpmcounter30h" => {Some(true)}
        "hpmcounter31h" => {Some(true)}
        "mhpmevent3" => {Some(true)}
        "mhpmevent4" => {Some(true)}
        "mhpmevent5" => {Some(true)}
        "mhpmevent6" => {Some(true)}
        "mhpmevent7" => {Some(true)}
        "mhpmevent8" => {Some(true)}
        "mhpmevent9" => {Some(true)}
        "mhpmevent10" => {Some(true)}
        "mhpmevent11" => {Some(true)}
        "mhpmevent12" => {Some(true)}
        "mhpmevent13" => {Some(true)}
        "mhpmevent14" => {Some(true)}
        "mhpmevent15" => {Some(true)}
        "mhpmevent16" => {Some(true)}
        "mhpmevent17" => {Some(true)}
        "mhpmevent18" => {Some(true)}
        "mhpmevent19" => {Some(true)}
        "mhpmevent20" => {Some(true)}
        "mhpmevent21" => {Some(true)}
        "mhpmevent22" => {Some(true)}
        "mhpmevent23" => {Some(true)}
        "mhpmevent24" => {Some(true)}
        "mhpmevent25" => {Some(true)}
        "mhpmevent26" => {Some(true)}
        "mhpmevent27" => {Some(true)}
        "mhpmevent28" => {Some(true)}
        "mhpmevent29" => {Some(true)}
        "mhpmevent30" => {Some(true)}
        "mhpmevent31" => {Some(true)}
        "mhpmcounter3" => {Some(true)}
        "mhpmcounter4" => {Some(true)}
        "mhpmcounter5" => {Some(true)}
        "mhpmcounter6" => {Some(true)}
        "mhpmcounter7" => {Some(true)}
        "mhpmcounter8" => {Some(true)}
        "mhpmcounter9" => {Some(true)}
        "mhpmcounter10" => {Some(true)}
        "mhpmcounter11" => {Some(true)}
        "mhpmcounter12" => {Some(true)}
        "mhpmcounter13" => {Some(true)}
        "mhpmcounter14" => {Some(true)}
        "mhpmcounter15" => {Some(true)}
        "mhpmcounter16" => {Some(true)}
        "mhpmcounter17" => {Some(true)}
        "mhpmcounter18" => {Some(true)}
        "mhpmcounter19" => {Some(true)}
        "mhpmcounter20" => {Some(true)}
        "mhpmcounter21" => {Some(true)}
        "mhpmcounter22" => {Some(true)}
        "mhpmcounter23" => {Some(true)}
        "mhpmcounter24" => {Some(true)}
        "mhpmcounter25" => {Some(true)}
        "mhpmcounter26" => {Some(true)}
        "mhpmcounter27" => {Some(true)}
        "mhpmcounter28" => {Some(true)}
        "mhpmcounter29" => {Some(true)}
        "mhpmcounter30" => {Some(true)}
        "mhpmcounter31" => {Some(true)}
        "mhpmcounter3h" => {Some(true)}
        "mhpmcounter4h" => {Some(true)}
        "mhpmcounter5h" => {Some(true)}
        "mhpmcounter6h" => {Some(true)}
        "mhpmcounter7h" => {Some(true)}
        "mhpmcounter8h" => {Some(true)}
        "mhpmcounter9h" => {Some(true)}
        "mhpmcounter10h" => {Some(true)}
        "mhpmcounter11h" => {Some(true)}
        "mhpmcounter12h" => {Some(true)}
        "mhpmcounter13h" => {Some(true)}
        "mhpmcounter14h" => {Some(true)}
        "mhpmcounter15h" => {Some(true)}
        "mhpmcounter16h" => {Some(true)}
        "mhpmcounter17h" => {Some(true)}
        "mhpmcounter18h" => {Some(true)}
        "mhpmcounter19h" => {Some(true)}
        "mhpmcounter20h" => {Some(true)}
        "mhpmcounter21h" => {Some(true)}
        "mhpmcounter22h" => {Some(true)}
        "mhpmcounter23h" => {Some(true)}
        "mhpmcounter24h" => {Some(true)}
        "mhpmcounter25h" => {Some(true)}
        "mhpmcounter26h" => {Some(true)}
        "mhpmcounter27h" => {Some(true)}
        "mhpmcounter28h" => {Some(true)}
        "mhpmcounter29h" => {Some(true)}
        "mhpmcounter30h" => {Some(true)}
        "mhpmcounter31h" => {Some(true)}
        "mhpmcounter3h" => {Some(true)}
        "mhpmcounter4h" => {Some(true)}
        "mhpmcounter5h" => {Some(true)}
        "mhpmcounter6h" => {Some(true)}
        "mhpmcounter7h" => {Some(true)}
        "mhpmcounter8h" => {Some(true)}
        "mhpmcounter9h" => {Some(true)}
        "mhpmcounter10h" => {Some(true)}
        "mhpmcounter11h" => {Some(true)}
        "mhpmcounter12h" => {Some(true)}
        "mhpmcounter13h" => {Some(true)}
        "mhpmcounter14h" => {Some(true)}
        "mhpmcounter15h" => {Some(true)}
        "mhpmcounter16h" => {Some(true)}
        "mhpmcounter17h" => {Some(true)}
        "mhpmcounter18h" => {Some(true)}
        "mhpmcounter19h" => {Some(true)}
        "mhpmcounter20h" => {Some(true)}
        "mhpmcounter21h" => {Some(true)}
        "mhpmcounter22h" => {Some(true)}
        "mhpmcounter23h" => {Some(true)}
        "mhpmcounter24h" => {Some(true)}
        "mhpmcounter25h" => {Some(true)}
        "mhpmcounter26h" => {Some(true)}
        "mhpmcounter27h" => {Some(true)}
        "mhpmcounter28h" => {Some(true)}
        "mhpmcounter29h" => {Some(true)}
        "mhpmcounter30h" => {Some(true)}
        "mhpmcounter31h" => {Some(true)}
        "scountovf" => {Some(true)}
        "seed" => {Some(true)}
        "cycle" => {Some(true)}
        "time" => {Some(true)}
        "instret" => {Some(true)}
        "cycleh" => {Some(true)}
        "timeh" => {Some(true)}
        "instreth" => {Some(true)}
        "mcycle" => {Some(true)}
        "minstret" => {Some(true)}
        "mcycleh" => {Some(true)}
        "minstreth" => {Some(true)}
        "fflags" => {Some(true)}
        "frm" => {Some(true)}
        "fcsr" => {Some(true)}
        "mcyclecfg" => {Some(true)}
        "mcyclecfgh" => {Some(true)}
        "minstretcfg" => {Some(true)}
        "minstretcfgh" => {Some(true)}
        "stimecmp" => {Some(true)}
        "stimecmph" => {Some(true)}
        "satp" => {Some(true)}
        mapping0_hashtag_ if {hex_bits_12_backwards_matches(mapping0_hashtag_)} => {match hex_bits_12_backwards(mapping0_hashtag_) {
            reg => {Some(true)}
            _ => {None}
            _ => {panic!("Unreachable code")}
        }}
        _ => {None}
        _ => {panic!("Unreachable code")}
    } {
        Some(result) => {result}
        None => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// HartState
///
/// Generated from the Sail sources at `riscv_step_common.sail` L21-28.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum HartState {
    HART_ACTIVE(()),
    HART_WAITING(instbits)
}

/// FetchResult
///
/// Generated from the Sail sources at `riscv_step_common.sail` L48-53.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FetchResult {
    F_Ext_Error(ext_fetch_addr_error),
    F_Base(word),
    F_RVC(half),
    F_Error((ExceptionType, xlenbits))
}

/// Step
///
/// Generated from the Sail sources at `riscv_step.sail` L13-19.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Step {
    Step_Pending_Interrupt((InterruptType, Privilege)),
    Step_Ext_Fetch_Failure(ext_fetch_addr_error),
    Step_Fetch_Failure((virtaddr, ExceptionType)),
    Step_Execute((ExecutionResult, instbits)),
    Step_Waiting(())
}
