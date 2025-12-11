// #![allow(warnings)]
// #![warn(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_braces)]
#![allow(clippy::unused_unit)]
#![allow(clippy::blocks_in_conditions)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::double_parens)]

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
    pub mcycle: BitVector<64>,
    pub mtime: BitVector<64>,
    pub minstret: BitVector<64>,
    pub minstret_increment: bool,
    pub mvendorid: BitVector<32>,
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
    pub vstart: BitVector<16>,
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
    pub mhpmcounter: [BitVector<64>; (32 as usize)],
    pub float_result: BitVector<64>,
    pub float_fflags: BitVector<64>,
    pub f0: fregtype,
    pub f1: fregtype,
    pub f2: fregtype,
    pub f3: fregtype,
    pub f4: fregtype,
    pub f5: fregtype,
    pub f6: fregtype,
    pub f7: fregtype,
    pub f8: fregtype,
    pub f9: fregtype,
    pub f10: fregtype,
    pub f11: fregtype,
    pub f12: fregtype,
    pub f13: fregtype,
    pub f14: fregtype,
    pub f15: fregtype,
    pub f16: fregtype,
    pub f17: fregtype,
    pub f18: fregtype,
    pub f19: fregtype,
    pub f20: fregtype,
    pub f21: fregtype,
    pub f22: fregtype,
    pub f23: fregtype,
    pub f24: fregtype,
    pub f25: fregtype,
    pub f26: fregtype,
    pub f27: fregtype,
    pub f28: fregtype,
    pub f29: fregtype,
    pub f30: fregtype,
    pub f31: fregtype,
    pub fcsr: Fcsr,
    pub mcyclecfg: CountSmcntrpmf,
    pub minstretcfg: CountSmcntrpmf,
    pub mtimecmp: BitVector<64>,
    pub stimecmp: BitVector<64>,
    pub htif_tohost: BitVector<64>,
    pub htif_done: bool,
    pub htif_exit_code: BitVector<64>,
    pub htif_cmd_write: bool,
    pub htif_payload_writes: BitVector<4>,
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
    pub writable_fiom: bool,
    pub writable_hpm_counters: BitVector<32>,
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
    pub supported: bool,
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
    _update_Misa_MXL(Mk_Misa(zeros::<64>(64)), architecture_forwards(Architecture::RV64))
}

/// Initialize the mstatus register.
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L275-284.
pub fn _reset_mstatus(core_ctx: &mut Core) -> Mstatus {
    let mxl = architecture_forwards(Architecture::RV64);
    {
        let var_1 = {
            let var_3 = if {((64 != 32) && hartSupports(core_ctx, extension::Ext_S))} {
                mxl
            } else {
                zeros::<2>(2)
            };
            _update_Mstatus_SXL(Mk_Mstatus(zeros::<64>(64)), var_3)
        };
        let var_2 = if {((64 != 32) && hartSupports(core_ctx, extension::Ext_U))} {
            mxl
        } else {
            zeros::<2>(2)
        };
        _update_Mstatus_UXL(var_1, var_2)
    }
}

/// Initialize the menvcfg register.
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L371.
pub fn _reset_menvcfg(core_ctx: &mut Core) -> MEnvcfg {
    legalize_menvcfg(core_ctx, Mk_MEnvcfg(zeros::<64>(64)), zeros::<64>(64))
}

/// Initialize the senvcfg register.
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L372.
pub fn _reset_senvcfg(core_ctx: &mut Core) -> SEnvcfg {
    legalize_senvcfg(core_ctx, Mk_SEnvcfg(zeros::<64>(64)), zeros::<64>(64))
}

/// Initialize the mvendorid register.
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L652.
pub fn _reset_mvendorid(core_ctx: &mut Core) -> BitVector<32> {
    to_bits::<32>(32, (core_ctx.config.platform.vendorid as i128))
}

/// Initialize the mimpid register.
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L653.
pub fn _reset_mimpid(core_ctx: &mut Core) -> xlenbits {
    to_bits::<64>(64, (core_ctx.config.platform.impid as i128))
}

/// Initialize the marchid register.
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L654.
pub fn _reset_marchid(core_ctx: &mut Core) -> xlenbits {
    to_bits::<64>(64, (core_ctx.config.platform.archid as i128))
}

/// Initialize the mhartid register.
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L655.
pub fn _reset_mhartid(core_ctx: &mut Core) -> xlenbits {
    to_bits::<64>(64, (core_ctx.config.platform.hartid as i128))
}

/// Initialize the mconfigptr register.
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L657.
pub fn _reset_mconfigptr() -> xlenbits {
    zeros::<64>(64)
}

/// Initialize the tlb register.
/// 
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L73.
pub fn _reset_tlb() -> [Option<TLB_Entry>; (num_tlb_entries as usize)] {
    [None; 64]
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

pub type xlenbits = BitVector<xlen>;

/// virtaddr
/// 
/// Generated from the Sail sources at `prelude_mem_addrtype.sail` L17.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum virtaddr {
    Virtaddr(xlenbits)
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
pub const fn zeros<const N: i128>(n: i128) -> BitVector<N> {
    sail_zeros(n)
}

/// ones
/// 
/// Generated from the Sail sources at `prelude.sail` L87.
pub const fn ones<const N: i128>(n: i128) -> BitVector<N> {
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
pub fn bool_bits_forwards(arg_hashtag_: bool) -> BitVector<1> {
    match arg_hashtag_ {
        true => {BitVector::<1>::new(0b1)}
        false => {BitVector::<1>::new(0b0)}
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
pub fn bool_to_bits(x: bool) -> BitVector<1> {
    bool_bits_forwards(x)
}

/// to_bits
/// 
/// Generated from the Sail sources at `prelude.sail` L117.
pub fn to_bits<const L: i128>(l: i128, n: i128) -> BitVector<L> {
    get_slice_int(l, n, 0)
}

/// (operator <_s)
/// 
/// Generated from the Sail sources at `prelude.sail` L137.
pub fn _operator_smaller_s_<const N: i128>(x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.signed() < y.signed())
}

/// (operator >=_s)
/// 
/// Generated from the Sail sources at `prelude.sail` L140.
pub fn _operator_biggerequal_s_<const N: i128>(x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.signed() >= y.signed())
}

/// (operator <_u)
/// 
/// Generated from the Sail sources at `prelude.sail` L141.
pub fn _operator_smaller_u_<const N: i128>(x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.unsigned() < y.unsigned())
}

/// (operator >=_u)
/// 
/// Generated from the Sail sources at `prelude.sail` L144.
pub fn _operator_biggerequal_u_<const N: i128>(x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.unsigned() >= y.unsigned())
}

/// shift_bits_right_arith
/// 
/// Generated from the Sail sources at `prelude.sail` L166-167.
pub fn shift_bits_right_arith<const M: i128, const N: i128>(value: BitVector<N>, shift: BitVector<M>) -> BitVector<N> {
    shift_right_arith(value, shift.unsigned())
}

/// rotate_bits_left
/// 
/// Generated from the Sail sources at `prelude.sail` L177-178.
pub fn rotate_bits_left<const N: i128, const M: i128>(v: BitVector<N>, n: BitVector<M>) -> BitVector<N> {
    (shift_bits_left(v, n) | shift_bits_right(v, sub_vec(to_bits(bitvector_length(n), bitvector_length(v)), n)))
}

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

pub const log2_xlen: i128 = 6;

pub const log2_xlen_bytes: i128 = 3;

pub const xlen_bytes: i128 = 8;

pub const xlen: i128 = 64;

pub const asidlen: i128 = 16;

pub type asidbits = BitVector<asidlen>;

pub type flenbits = BitVector<flen>;

pub const flen_bytes: i128 = 8;

pub const flen: i128 = 64;

/// get_vlen_pow
/// 
/// Generated from the Sail sources at `riscv_vlen.sail` L16.
pub const fn get_vlen_pow(unit_arg: ()) -> i128 {
    3
}

pub const vlenmax: i128 = 65536;

pub const VLEN: i128 = 8;

pub type physaddrbits = BitVector<physaddrbits_len>;

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
pub fn bits_of_virtaddr(virtaddr::Virtaddr(vaddr): virtaddr) -> BitVector<{
    64
}> {
    vaddr
}

pub type mem_meta = ();

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
pub const fn sail_branch_announce<const ADDRSIZE: i128>(_: i128, _: BitVector<ADDRSIZE>) {
    ()
}

/// Access_variety
/// 
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L57-61.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_variety {
    AV_plain,
    AV_exclusive,
    AV_atomic_rmw
}

/// Access_strength
/// 
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L66-70.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_strength {
    AS_normal,
    AS_rel_or_acq,
    AS_acq_rcpc
}

/// Explicit_access_kind
/// 
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L75-78.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Explicit_access_kind {
    pub variety: Access_variety,
    pub strength: Access_strength,
}

/// Access_kind
/// 
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L83-88.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_kind<ARCH_AK> {
    AK_explicit(Explicit_access_kind),
    AK_ifetch(()),
    AK_ttw(()),
    AK_arch(ARCH_AK)
}

/// Mem_read_request
/// 
/// Generated from the Sail sources at `sail/lib/concurrency_interface/read_write.sail` L93-104.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mem_read_request<const N: i128, const VASIZE: i128, PA, TS, ARCH_AK> {
    pub access_kind: Access_kind<ARCH_AK>,
    pub va: Option<BitVector<VASIZE>>,
    pub pa: PA,
    pub translation: TS,
    pub size: i128,
    pub tag: bool,
}

pub const __monomorphize_reads: bool = false;

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

pub type exc_code = BitVector<8>;

pub type ext_ptw = ();

pub const init_ext_ptw: ext_ptw = ();

pub type ext_ptw_fail = ();

pub type ext_ptw_error = ();

pub type ext_exc_type = ();

/// ext_exc_type_to_bits
/// 
/// Generated from the Sail sources at `riscv_types_ext.sail` L61.
pub fn ext_exc_type_to_bits(unit_arg: ()) -> BitVector<8> {
    BitVector::<8>::new(0b00011000)
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

pub const xlen_max_unsigned: i128 = 18446744073709551615;

pub const xlen_max_signed: i128 = 9223372036854775807;

pub const xlen_min_signed: i128 = -9223372036854775808;

pub type half = BitVector<16>;

pub type word = BitVector<32>;

pub type instbits = BitVector<32>;

pub const pagesize_bits: i128 = 12;

/// regidx
/// 
/// Generated from the Sail sources at `riscv_types.sail` L25.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum regidx {
    Regidx(BitVector<5>)
}

/// cregidx
/// 
/// Generated from the Sail sources at `riscv_types.sail` L26.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum cregidx {
    Cregidx(BitVector<3>)
}

pub type csreg = BitVector<12>;

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
    regidx::Regidx(to_bits::<5>(5, b))
}

/// creg2reg_idx
/// 
/// Generated from the Sail sources at `riscv_types.sail` L41.
pub fn creg2reg_idx(cregidx::Cregidx(i): cregidx) -> regidx {
    regidx::Regidx(bitvector_concat::<2, 3, 5>(BitVector::<2>::new(0b01), i))
}

pub const zreg: regidx = regidx::Regidx(BitVector::<5>::new(0b00000));

pub const ra: regidx = regidx::Regidx(BitVector::<5>::new(0b00001));

pub const sp: regidx = regidx::Regidx(BitVector::<5>::new(0b00010));

/// Architecture
/// 
/// Generated from the Sail sources at `riscv_types.sail` L50.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Architecture {
    RV32,
    RV64,
    RV128
}

pub type arch_xlen = BitVector<2>;

/// architecture_forwards
/// 
/// Generated from the Sail sources.
pub fn architecture_forwards(arg_hashtag_: Architecture) -> BitVector<2> {
    match arg_hashtag_ {
        Architecture::RV32 => {BitVector::<2>::new(0b01)}
        Architecture::RV64 => {BitVector::<2>::new(0b10)}
        Architecture::RV128 => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// architecture_backwards
/// 
/// Generated from the Sail sources.
pub fn architecture_backwards(arg_hashtag_: BitVector<2>) -> Architecture {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b01))} => {Architecture::RV32}
        b__1 if {(b__1 == BitVector::<2>::new(0b10))} => {Architecture::RV64}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {Architecture::RV128}
        _ => {panic!("{}, l {}: {}", "riscv_types.sail", 57, "architecture(0b00) is invalid")}
        _ => {panic!("Unreachable code")}
    }
}

pub type priv_level = BitVector<2>;

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
pub fn privLevel_bits_forwards(arg_hashtag_: Privilege) -> BitVector<2> {
    match arg_hashtag_ {
        Privilege::User => {BitVector::<2>::new(0b00)}
        Privilege::Supervisor => {BitVector::<2>::new(0b01)}
        Privilege::Machine => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// privLevel_bits_backwards
/// 
/// Generated from the Sail sources.
pub fn privLevel_bits_backwards(arg_hashtag_: BitVector<2>) -> Privilege {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {Privilege::User}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {Privilege::Supervisor}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {Privilege::Machine}
        _ => {panic!("{}, l {}: {}", "riscv_types.sail", 69, format!("{}{}", "Invalid privilege level: ", bits_str(BitVector::<2>::new(0b10))))}
        _ => {panic!("Unreachable code")}
    }
}

/// privLevel_to_bits
/// 
/// Generated from the Sail sources at `riscv_types.sail` L72.
pub fn privLevel_to_bits(p: Privilege) -> BitVector<2> {
    privLevel_bits_forwards(p)
}

/// privLevel_of_bits
/// 
/// Generated from the Sail sources at `riscv_types.sail` L73.
pub fn privLevel_of_bits(b: BitVector<2>) -> Privilege {
    privLevel_bits_backwards(b)
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
pub fn csr_name_map_forwards(arg_hashtag_: BitVector<12>) -> &'static str {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<12>::new(0b001100000001))} => {"misa"}
        b__1 if {(b__1 == BitVector::<12>::new(0b001100000000))} => {"mstatus"}
        b__2 if {(b__2 == BitVector::<12>::new(0b001100001010))} => {"menvcfg"}
        b__3 if {(b__3 == BitVector::<12>::new(0b001100011010))} => {"menvcfgh"}
        b__4 if {(b__4 == BitVector::<12>::new(0b000100001010))} => {"senvcfg"}
        b__5 if {(b__5 == BitVector::<12>::new(0b001100000100))} => {"mie"}
        b__6 if {(b__6 == BitVector::<12>::new(0b001101000100))} => {"mip"}
        b__7 if {(b__7 == BitVector::<12>::new(0b001100000010))} => {"medeleg"}
        b__8 if {(b__8 == BitVector::<12>::new(0b001100010010))} => {"medelegh"}
        b__9 if {(b__9 == BitVector::<12>::new(0b001100000011))} => {"mideleg"}
        b__10 if {(b__10 == BitVector::<12>::new(0b001101000010))} => {"mcause"}
        b__11 if {(b__11 == BitVector::<12>::new(0b001101000011))} => {"mtval"}
        b__12 if {(b__12 == BitVector::<12>::new(0b001101000000))} => {"mscratch"}
        b__13 if {(b__13 == BitVector::<12>::new(0b000100000110))} => {"scounteren"}
        b__14 if {(b__14 == BitVector::<12>::new(0b001100000110))} => {"mcounteren"}
        b__15 if {(b__15 == BitVector::<12>::new(0b001100100000))} => {"mcountinhibit"}
        b__16 if {(b__16 == BitVector::<12>::new(0b111100010001))} => {"mvendorid"}
        b__17 if {(b__17 == BitVector::<12>::new(0b111100010010))} => {"marchid"}
        b__18 if {(b__18 == BitVector::<12>::new(0b111100010011))} => {"mimpid"}
        b__19 if {(b__19 == BitVector::<12>::new(0b111100010100))} => {"mhartid"}
        b__20 if {(b__20 == BitVector::<12>::new(0b111100010101))} => {"mconfigptr"}
        b__21 if {(b__21 == BitVector::<12>::new(0b000100000000))} => {"sstatus"}
        b__22 if {(b__22 == BitVector::<12>::new(0b000101000100))} => {"sip"}
        b__23 if {(b__23 == BitVector::<12>::new(0b000100000100))} => {"sie"}
        b__24 if {(b__24 == BitVector::<12>::new(0b000101000000))} => {"sscratch"}
        b__25 if {(b__25 == BitVector::<12>::new(0b000101000010))} => {"scause"}
        b__26 if {(b__26 == BitVector::<12>::new(0b000101000011))} => {"stval"}
        b__27 if {(b__27 == BitVector::<12>::new(0b011110100000))} => {"tselect"}
        b__28 if {(b__28 == BitVector::<12>::new(0b011110100001))} => {"tdata1"}
        b__29 if {(b__29 == BitVector::<12>::new(0b011110100010))} => {"tdata2"}
        b__30 if {(b__30 == BitVector::<12>::new(0b011110100011))} => {"tdata3"}
        b__31 if {(b__31 == BitVector::<12>::new(0b001110100000))} => {"pmpcfg0"}
        b__32 if {(b__32 == BitVector::<12>::new(0b001110100001))} => {"pmpcfg1"}
        b__33 if {(b__33 == BitVector::<12>::new(0b001110100010))} => {"pmpcfg2"}
        b__34 if {(b__34 == BitVector::<12>::new(0b001110100011))} => {"pmpcfg3"}
        b__35 if {(b__35 == BitVector::<12>::new(0b001110100100))} => {"pmpcfg4"}
        b__36 if {(b__36 == BitVector::<12>::new(0b001110100101))} => {"pmpcfg5"}
        b__37 if {(b__37 == BitVector::<12>::new(0b001110100110))} => {"pmpcfg6"}
        b__38 if {(b__38 == BitVector::<12>::new(0b001110100111))} => {"pmpcfg7"}
        b__39 if {(b__39 == BitVector::<12>::new(0b001110101000))} => {"pmpcfg8"}
        b__40 if {(b__40 == BitVector::<12>::new(0b001110101001))} => {"pmpcfg9"}
        b__41 if {(b__41 == BitVector::<12>::new(0b001110101010))} => {"pmpcfg10"}
        b__42 if {(b__42 == BitVector::<12>::new(0b001110101011))} => {"pmpcfg11"}
        b__43 if {(b__43 == BitVector::<12>::new(0b001110101100))} => {"pmpcfg12"}
        b__44 if {(b__44 == BitVector::<12>::new(0b001110101101))} => {"pmpcfg13"}
        b__45 if {(b__45 == BitVector::<12>::new(0b001110101110))} => {"pmpcfg14"}
        b__46 if {(b__46 == BitVector::<12>::new(0b001110101111))} => {"pmpcfg15"}
        b__47 if {(b__47 == BitVector::<12>::new(0b001110110000))} => {"pmpaddr0"}
        b__48 if {(b__48 == BitVector::<12>::new(0b001110110001))} => {"pmpaddr1"}
        b__49 if {(b__49 == BitVector::<12>::new(0b001110110010))} => {"pmpaddr2"}
        b__50 if {(b__50 == BitVector::<12>::new(0b001110110011))} => {"pmpaddr3"}
        b__51 if {(b__51 == BitVector::<12>::new(0b001110110100))} => {"pmpaddr4"}
        b__52 if {(b__52 == BitVector::<12>::new(0b001110110101))} => {"pmpaddr5"}
        b__53 if {(b__53 == BitVector::<12>::new(0b001110110110))} => {"pmpaddr6"}
        b__54 if {(b__54 == BitVector::<12>::new(0b001110110111))} => {"pmpaddr7"}
        b__55 if {(b__55 == BitVector::<12>::new(0b001110111000))} => {"pmpaddr8"}
        b__56 if {(b__56 == BitVector::<12>::new(0b001110111001))} => {"pmpaddr9"}
        b__57 if {(b__57 == BitVector::<12>::new(0b001110111010))} => {"pmpaddr10"}
        b__58 if {(b__58 == BitVector::<12>::new(0b001110111011))} => {"pmpaddr11"}
        b__59 if {(b__59 == BitVector::<12>::new(0b001110111100))} => {"pmpaddr12"}
        b__60 if {(b__60 == BitVector::<12>::new(0b001110111101))} => {"pmpaddr13"}
        b__61 if {(b__61 == BitVector::<12>::new(0b001110111110))} => {"pmpaddr14"}
        b__62 if {(b__62 == BitVector::<12>::new(0b001110111111))} => {"pmpaddr15"}
        b__63 if {(b__63 == BitVector::<12>::new(0b001111000000))} => {"pmpaddr16"}
        b__64 if {(b__64 == BitVector::<12>::new(0b001111000001))} => {"pmpaddr17"}
        b__65 if {(b__65 == BitVector::<12>::new(0b001111000010))} => {"pmpaddr18"}
        b__66 if {(b__66 == BitVector::<12>::new(0b001111000011))} => {"pmpaddr19"}
        b__67 if {(b__67 == BitVector::<12>::new(0b001111000100))} => {"pmpaddr20"}
        b__68 if {(b__68 == BitVector::<12>::new(0b001111000101))} => {"pmpaddr21"}
        b__69 if {(b__69 == BitVector::<12>::new(0b001111000110))} => {"pmpaddr22"}
        b__70 if {(b__70 == BitVector::<12>::new(0b001111000111))} => {"pmpaddr23"}
        b__71 if {(b__71 == BitVector::<12>::new(0b001111001000))} => {"pmpaddr24"}
        b__72 if {(b__72 == BitVector::<12>::new(0b001111001001))} => {"pmpaddr25"}
        b__73 if {(b__73 == BitVector::<12>::new(0b001111001010))} => {"pmpaddr26"}
        b__74 if {(b__74 == BitVector::<12>::new(0b001111001011))} => {"pmpaddr27"}
        b__75 if {(b__75 == BitVector::<12>::new(0b001111001100))} => {"pmpaddr28"}
        b__76 if {(b__76 == BitVector::<12>::new(0b001111001101))} => {"pmpaddr29"}
        b__77 if {(b__77 == BitVector::<12>::new(0b001111001110))} => {"pmpaddr30"}
        b__78 if {(b__78 == BitVector::<12>::new(0b001111001111))} => {"pmpaddr31"}
        b__79 if {(b__79 == BitVector::<12>::new(0b001111010000))} => {"pmpaddr32"}
        b__80 if {(b__80 == BitVector::<12>::new(0b001111010001))} => {"pmpaddr33"}
        b__81 if {(b__81 == BitVector::<12>::new(0b001111010010))} => {"pmpaddr34"}
        b__82 if {(b__82 == BitVector::<12>::new(0b001111010011))} => {"pmpaddr35"}
        b__83 if {(b__83 == BitVector::<12>::new(0b001111010100))} => {"pmpaddr36"}
        b__84 if {(b__84 == BitVector::<12>::new(0b001111010101))} => {"pmpaddr37"}
        b__85 if {(b__85 == BitVector::<12>::new(0b001111010110))} => {"pmpaddr38"}
        b__86 if {(b__86 == BitVector::<12>::new(0b001111010111))} => {"pmpaddr39"}
        b__87 if {(b__87 == BitVector::<12>::new(0b001111011000))} => {"pmpaddr40"}
        b__88 if {(b__88 == BitVector::<12>::new(0b001111011001))} => {"pmpaddr41"}
        b__89 if {(b__89 == BitVector::<12>::new(0b001111011010))} => {"pmpaddr42"}
        b__90 if {(b__90 == BitVector::<12>::new(0b001111011011))} => {"pmpaddr43"}
        b__91 if {(b__91 == BitVector::<12>::new(0b001111011100))} => {"pmpaddr44"}
        b__92 if {(b__92 == BitVector::<12>::new(0b001111011101))} => {"pmpaddr45"}
        b__93 if {(b__93 == BitVector::<12>::new(0b001111011110))} => {"pmpaddr46"}
        b__94 if {(b__94 == BitVector::<12>::new(0b001111011111))} => {"pmpaddr47"}
        b__95 if {(b__95 == BitVector::<12>::new(0b001111100000))} => {"pmpaddr48"}
        b__96 if {(b__96 == BitVector::<12>::new(0b001111100001))} => {"pmpaddr49"}
        b__97 if {(b__97 == BitVector::<12>::new(0b001111100010))} => {"pmpaddr50"}
        b__98 if {(b__98 == BitVector::<12>::new(0b001111100011))} => {"pmpaddr51"}
        b__99 if {(b__99 == BitVector::<12>::new(0b001111100100))} => {"pmpaddr52"}
        b__100 if {(b__100 == BitVector::<12>::new(0b001111100101))} => {"pmpaddr53"}
        b__101 if {(b__101 == BitVector::<12>::new(0b001111100110))} => {"pmpaddr54"}
        b__102 if {(b__102 == BitVector::<12>::new(0b001111100111))} => {"pmpaddr55"}
        b__103 if {(b__103 == BitVector::<12>::new(0b001111101000))} => {"pmpaddr56"}
        b__104 if {(b__104 == BitVector::<12>::new(0b001111101001))} => {"pmpaddr57"}
        b__105 if {(b__105 == BitVector::<12>::new(0b001111101010))} => {"pmpaddr58"}
        b__106 if {(b__106 == BitVector::<12>::new(0b001111101011))} => {"pmpaddr59"}
        b__107 if {(b__107 == BitVector::<12>::new(0b001111101100))} => {"pmpaddr60"}
        b__108 if {(b__108 == BitVector::<12>::new(0b001111101101))} => {"pmpaddr61"}
        b__109 if {(b__109 == BitVector::<12>::new(0b001111101110))} => {"pmpaddr62"}
        b__110 if {(b__110 == BitVector::<12>::new(0b001111101111))} => {"pmpaddr63"}
        b__111 if {(b__111 == BitVector::<12>::new(0b000000001000))} => {"vstart"}
        b__112 if {(b__112 == BitVector::<12>::new(0b000000001001))} => {"vxsat"}
        b__113 if {(b__113 == BitVector::<12>::new(0b000000001010))} => {"vxrm"}
        b__114 if {(b__114 == BitVector::<12>::new(0b000000001111))} => {"vcsr"}
        b__115 if {(b__115 == BitVector::<12>::new(0b110000100000))} => {"vl"}
        b__116 if {(b__116 == BitVector::<12>::new(0b110000100001))} => {"vtype"}
        b__117 if {(b__117 == BitVector::<12>::new(0b110000100010))} => {"vlenb"}
        b__118 if {(b__118 == BitVector::<12>::new(0b000100000101))} => {"stvec"}
        b__119 if {(b__119 == BitVector::<12>::new(0b000101000001))} => {"sepc"}
        b__120 if {(b__120 == BitVector::<12>::new(0b001100000101))} => {"mtvec"}
        b__121 if {(b__121 == BitVector::<12>::new(0b001101000001))} => {"mepc"}
        b__122 if {(b__122 == BitVector::<12>::new(0b110000000011))} => {"hpmcounter3"}
        b__123 if {(b__123 == BitVector::<12>::new(0b110000000100))} => {"hpmcounter4"}
        b__124 if {(b__124 == BitVector::<12>::new(0b110000000101))} => {"hpmcounter5"}
        b__125 if {(b__125 == BitVector::<12>::new(0b110000000110))} => {"hpmcounter6"}
        b__126 if {(b__126 == BitVector::<12>::new(0b110000000111))} => {"hpmcounter7"}
        b__127 if {(b__127 == BitVector::<12>::new(0b110000001000))} => {"hpmcounter8"}
        b__128 if {(b__128 == BitVector::<12>::new(0b110000001001))} => {"hpmcounter9"}
        b__129 if {(b__129 == BitVector::<12>::new(0b110000001010))} => {"hpmcounter10"}
        b__130 if {(b__130 == BitVector::<12>::new(0b110000001011))} => {"hpmcounter11"}
        b__131 if {(b__131 == BitVector::<12>::new(0b110000001100))} => {"hpmcounter12"}
        b__132 if {(b__132 == BitVector::<12>::new(0b110000001101))} => {"hpmcounter13"}
        b__133 if {(b__133 == BitVector::<12>::new(0b110000001110))} => {"hpmcounter14"}
        b__134 if {(b__134 == BitVector::<12>::new(0b110000001111))} => {"hpmcounter15"}
        b__135 if {(b__135 == BitVector::<12>::new(0b110000010000))} => {"hpmcounter16"}
        b__136 if {(b__136 == BitVector::<12>::new(0b110000010001))} => {"hpmcounter17"}
        b__137 if {(b__137 == BitVector::<12>::new(0b110000010010))} => {"hpmcounter18"}
        b__138 if {(b__138 == BitVector::<12>::new(0b110000010011))} => {"hpmcounter19"}
        b__139 if {(b__139 == BitVector::<12>::new(0b110000010100))} => {"hpmcounter20"}
        b__140 if {(b__140 == BitVector::<12>::new(0b110000010101))} => {"hpmcounter21"}
        b__141 if {(b__141 == BitVector::<12>::new(0b110000010110))} => {"hpmcounter22"}
        b__142 if {(b__142 == BitVector::<12>::new(0b110000010111))} => {"hpmcounter23"}
        b__143 if {(b__143 == BitVector::<12>::new(0b110000011000))} => {"hpmcounter24"}
        b__144 if {(b__144 == BitVector::<12>::new(0b110000011001))} => {"hpmcounter25"}
        b__145 if {(b__145 == BitVector::<12>::new(0b110000011010))} => {"hpmcounter26"}
        b__146 if {(b__146 == BitVector::<12>::new(0b110000011011))} => {"hpmcounter27"}
        b__147 if {(b__147 == BitVector::<12>::new(0b110000011100))} => {"hpmcounter28"}
        b__148 if {(b__148 == BitVector::<12>::new(0b110000011101))} => {"hpmcounter29"}
        b__149 if {(b__149 == BitVector::<12>::new(0b110000011110))} => {"hpmcounter30"}
        b__150 if {(b__150 == BitVector::<12>::new(0b110000011111))} => {"hpmcounter31"}
        b__151 if {(b__151 == BitVector::<12>::new(0b110010000011))} => {"hpmcounter3h"}
        b__152 if {(b__152 == BitVector::<12>::new(0b110010000100))} => {"hpmcounter4h"}
        b__153 if {(b__153 == BitVector::<12>::new(0b110010000101))} => {"hpmcounter5h"}
        b__154 if {(b__154 == BitVector::<12>::new(0b110010000110))} => {"hpmcounter6h"}
        b__155 if {(b__155 == BitVector::<12>::new(0b110010000111))} => {"hpmcounter7h"}
        b__156 if {(b__156 == BitVector::<12>::new(0b110010001000))} => {"hpmcounter8h"}
        b__157 if {(b__157 == BitVector::<12>::new(0b110010001001))} => {"hpmcounter9h"}
        b__158 if {(b__158 == BitVector::<12>::new(0b110010001010))} => {"hpmcounter10h"}
        b__159 if {(b__159 == BitVector::<12>::new(0b110010001011))} => {"hpmcounter11h"}
        b__160 if {(b__160 == BitVector::<12>::new(0b110010001100))} => {"hpmcounter12h"}
        b__161 if {(b__161 == BitVector::<12>::new(0b110010001101))} => {"hpmcounter13h"}
        b__162 if {(b__162 == BitVector::<12>::new(0b110010001110))} => {"hpmcounter14h"}
        b__163 if {(b__163 == BitVector::<12>::new(0b110010001111))} => {"hpmcounter15h"}
        b__164 if {(b__164 == BitVector::<12>::new(0b110010010000))} => {"hpmcounter16h"}
        b__165 if {(b__165 == BitVector::<12>::new(0b110010010001))} => {"hpmcounter17h"}
        b__166 if {(b__166 == BitVector::<12>::new(0b110010010010))} => {"hpmcounter18h"}
        b__167 if {(b__167 == BitVector::<12>::new(0b110010010011))} => {"hpmcounter19h"}
        b__168 if {(b__168 == BitVector::<12>::new(0b110010010100))} => {"hpmcounter20h"}
        b__169 if {(b__169 == BitVector::<12>::new(0b110010010101))} => {"hpmcounter21h"}
        b__170 if {(b__170 == BitVector::<12>::new(0b110010010110))} => {"hpmcounter22h"}
        b__171 if {(b__171 == BitVector::<12>::new(0b110010010111))} => {"hpmcounter23h"}
        b__172 if {(b__172 == BitVector::<12>::new(0b110010011000))} => {"hpmcounter24h"}
        b__173 if {(b__173 == BitVector::<12>::new(0b110010011001))} => {"hpmcounter25h"}
        b__174 if {(b__174 == BitVector::<12>::new(0b110010011010))} => {"hpmcounter26h"}
        b__175 if {(b__175 == BitVector::<12>::new(0b110010011011))} => {"hpmcounter27h"}
        b__176 if {(b__176 == BitVector::<12>::new(0b110010011100))} => {"hpmcounter28h"}
        b__177 if {(b__177 == BitVector::<12>::new(0b110010011101))} => {"hpmcounter29h"}
        b__178 if {(b__178 == BitVector::<12>::new(0b110010011110))} => {"hpmcounter30h"}
        b__179 if {(b__179 == BitVector::<12>::new(0b110010011111))} => {"hpmcounter31h"}
        b__180 if {(b__180 == BitVector::<12>::new(0b001100100011))} => {"mhpmevent3"}
        b__181 if {(b__181 == BitVector::<12>::new(0b001100100100))} => {"mhpmevent4"}
        b__182 if {(b__182 == BitVector::<12>::new(0b001100100101))} => {"mhpmevent5"}
        b__183 if {(b__183 == BitVector::<12>::new(0b001100100110))} => {"mhpmevent6"}
        b__184 if {(b__184 == BitVector::<12>::new(0b001100100111))} => {"mhpmevent7"}
        b__185 if {(b__185 == BitVector::<12>::new(0b001100101000))} => {"mhpmevent8"}
        b__186 if {(b__186 == BitVector::<12>::new(0b001100101001))} => {"mhpmevent9"}
        b__187 if {(b__187 == BitVector::<12>::new(0b001100101010))} => {"mhpmevent10"}
        b__188 if {(b__188 == BitVector::<12>::new(0b001100101011))} => {"mhpmevent11"}
        b__189 if {(b__189 == BitVector::<12>::new(0b001100101100))} => {"mhpmevent12"}
        b__190 if {(b__190 == BitVector::<12>::new(0b001100101101))} => {"mhpmevent13"}
        b__191 if {(b__191 == BitVector::<12>::new(0b001100101110))} => {"mhpmevent14"}
        b__192 if {(b__192 == BitVector::<12>::new(0b001100101111))} => {"mhpmevent15"}
        b__193 if {(b__193 == BitVector::<12>::new(0b001100110000))} => {"mhpmevent16"}
        b__194 if {(b__194 == BitVector::<12>::new(0b001100110001))} => {"mhpmevent17"}
        b__195 if {(b__195 == BitVector::<12>::new(0b001100110010))} => {"mhpmevent18"}
        b__196 if {(b__196 == BitVector::<12>::new(0b001100110011))} => {"mhpmevent19"}
        b__197 if {(b__197 == BitVector::<12>::new(0b001100110100))} => {"mhpmevent20"}
        b__198 if {(b__198 == BitVector::<12>::new(0b001100110101))} => {"mhpmevent21"}
        b__199 if {(b__199 == BitVector::<12>::new(0b001100110110))} => {"mhpmevent22"}
        b__200 if {(b__200 == BitVector::<12>::new(0b001100110111))} => {"mhpmevent23"}
        b__201 if {(b__201 == BitVector::<12>::new(0b001100111000))} => {"mhpmevent24"}
        b__202 if {(b__202 == BitVector::<12>::new(0b001100111001))} => {"mhpmevent25"}
        b__203 if {(b__203 == BitVector::<12>::new(0b001100111010))} => {"mhpmevent26"}
        b__204 if {(b__204 == BitVector::<12>::new(0b001100111011))} => {"mhpmevent27"}
        b__205 if {(b__205 == BitVector::<12>::new(0b001100111100))} => {"mhpmevent28"}
        b__206 if {(b__206 == BitVector::<12>::new(0b001100111101))} => {"mhpmevent29"}
        b__207 if {(b__207 == BitVector::<12>::new(0b001100111110))} => {"mhpmevent30"}
        b__208 if {(b__208 == BitVector::<12>::new(0b001100111111))} => {"mhpmevent31"}
        b__209 if {(b__209 == BitVector::<12>::new(0b101100000011))} => {"mhpmcounter3"}
        b__210 if {(b__210 == BitVector::<12>::new(0b101100000100))} => {"mhpmcounter4"}
        b__211 if {(b__211 == BitVector::<12>::new(0b101100000101))} => {"mhpmcounter5"}
        b__212 if {(b__212 == BitVector::<12>::new(0b101100000110))} => {"mhpmcounter6"}
        b__213 if {(b__213 == BitVector::<12>::new(0b101100000111))} => {"mhpmcounter7"}
        b__214 if {(b__214 == BitVector::<12>::new(0b101100001000))} => {"mhpmcounter8"}
        b__215 if {(b__215 == BitVector::<12>::new(0b101100001001))} => {"mhpmcounter9"}
        b__216 if {(b__216 == BitVector::<12>::new(0b101100001010))} => {"mhpmcounter10"}
        b__217 if {(b__217 == BitVector::<12>::new(0b101100001011))} => {"mhpmcounter11"}
        b__218 if {(b__218 == BitVector::<12>::new(0b101100001100))} => {"mhpmcounter12"}
        b__219 if {(b__219 == BitVector::<12>::new(0b101100001101))} => {"mhpmcounter13"}
        b__220 if {(b__220 == BitVector::<12>::new(0b101100001110))} => {"mhpmcounter14"}
        b__221 if {(b__221 == BitVector::<12>::new(0b101100001111))} => {"mhpmcounter15"}
        b__222 if {(b__222 == BitVector::<12>::new(0b101100010000))} => {"mhpmcounter16"}
        b__223 if {(b__223 == BitVector::<12>::new(0b101100010001))} => {"mhpmcounter17"}
        b__224 if {(b__224 == BitVector::<12>::new(0b101100010010))} => {"mhpmcounter18"}
        b__225 if {(b__225 == BitVector::<12>::new(0b101100010011))} => {"mhpmcounter19"}
        b__226 if {(b__226 == BitVector::<12>::new(0b101100010100))} => {"mhpmcounter20"}
        b__227 if {(b__227 == BitVector::<12>::new(0b101100010101))} => {"mhpmcounter21"}
        b__228 if {(b__228 == BitVector::<12>::new(0b101100010110))} => {"mhpmcounter22"}
        b__229 if {(b__229 == BitVector::<12>::new(0b101100010111))} => {"mhpmcounter23"}
        b__230 if {(b__230 == BitVector::<12>::new(0b101100011000))} => {"mhpmcounter24"}
        b__231 if {(b__231 == BitVector::<12>::new(0b101100011001))} => {"mhpmcounter25"}
        b__232 if {(b__232 == BitVector::<12>::new(0b101100011010))} => {"mhpmcounter26"}
        b__233 if {(b__233 == BitVector::<12>::new(0b101100011011))} => {"mhpmcounter27"}
        b__234 if {(b__234 == BitVector::<12>::new(0b101100011100))} => {"mhpmcounter28"}
        b__235 if {(b__235 == BitVector::<12>::new(0b101100011101))} => {"mhpmcounter29"}
        b__236 if {(b__236 == BitVector::<12>::new(0b101100011110))} => {"mhpmcounter30"}
        b__237 if {(b__237 == BitVector::<12>::new(0b101100011111))} => {"mhpmcounter31"}
        b__238 if {(b__238 == BitVector::<12>::new(0b101110000011))} => {"mhpmcounter3h"}
        b__239 if {(b__239 == BitVector::<12>::new(0b101110000100))} => {"mhpmcounter4h"}
        b__240 if {(b__240 == BitVector::<12>::new(0b101110000101))} => {"mhpmcounter5h"}
        b__241 if {(b__241 == BitVector::<12>::new(0b101110000110))} => {"mhpmcounter6h"}
        b__242 if {(b__242 == BitVector::<12>::new(0b101110000111))} => {"mhpmcounter7h"}
        b__243 if {(b__243 == BitVector::<12>::new(0b101110001000))} => {"mhpmcounter8h"}
        b__244 if {(b__244 == BitVector::<12>::new(0b101110001001))} => {"mhpmcounter9h"}
        b__245 if {(b__245 == BitVector::<12>::new(0b101110001010))} => {"mhpmcounter10h"}
        b__246 if {(b__246 == BitVector::<12>::new(0b101110001011))} => {"mhpmcounter11h"}
        b__247 if {(b__247 == BitVector::<12>::new(0b101110001100))} => {"mhpmcounter12h"}
        b__248 if {(b__248 == BitVector::<12>::new(0b101110001101))} => {"mhpmcounter13h"}
        b__249 if {(b__249 == BitVector::<12>::new(0b101110001110))} => {"mhpmcounter14h"}
        b__250 if {(b__250 == BitVector::<12>::new(0b101110001111))} => {"mhpmcounter15h"}
        b__251 if {(b__251 == BitVector::<12>::new(0b101110010000))} => {"mhpmcounter16h"}
        b__252 if {(b__252 == BitVector::<12>::new(0b101110010001))} => {"mhpmcounter17h"}
        b__253 if {(b__253 == BitVector::<12>::new(0b101110010010))} => {"mhpmcounter18h"}
        b__254 if {(b__254 == BitVector::<12>::new(0b101110010011))} => {"mhpmcounter19h"}
        b__255 if {(b__255 == BitVector::<12>::new(0b101110010100))} => {"mhpmcounter20h"}
        b__256 if {(b__256 == BitVector::<12>::new(0b101110010101))} => {"mhpmcounter21h"}
        b__257 if {(b__257 == BitVector::<12>::new(0b101110010110))} => {"mhpmcounter22h"}
        b__258 if {(b__258 == BitVector::<12>::new(0b101110010111))} => {"mhpmcounter23h"}
        b__259 if {(b__259 == BitVector::<12>::new(0b101110011000))} => {"mhpmcounter24h"}
        b__260 if {(b__260 == BitVector::<12>::new(0b101110011001))} => {"mhpmcounter25h"}
        b__261 if {(b__261 == BitVector::<12>::new(0b101110011010))} => {"mhpmcounter26h"}
        b__262 if {(b__262 == BitVector::<12>::new(0b101110011011))} => {"mhpmcounter27h"}
        b__263 if {(b__263 == BitVector::<12>::new(0b101110011100))} => {"mhpmcounter28h"}
        b__264 if {(b__264 == BitVector::<12>::new(0b101110011101))} => {"mhpmcounter29h"}
        b__265 if {(b__265 == BitVector::<12>::new(0b101110011110))} => {"mhpmcounter30h"}
        b__266 if {(b__266 == BitVector::<12>::new(0b101110011111))} => {"mhpmcounter31h"}
        b__267 if {(b__267 == BitVector::<12>::new(0b101110000011))} => {"mhpmcounter3h"}
        b__268 if {(b__268 == BitVector::<12>::new(0b101110000100))} => {"mhpmcounter4h"}
        b__269 if {(b__269 == BitVector::<12>::new(0b101110000101))} => {"mhpmcounter5h"}
        b__270 if {(b__270 == BitVector::<12>::new(0b101110000110))} => {"mhpmcounter6h"}
        b__271 if {(b__271 == BitVector::<12>::new(0b101110000111))} => {"mhpmcounter7h"}
        b__272 if {(b__272 == BitVector::<12>::new(0b101110001000))} => {"mhpmcounter8h"}
        b__273 if {(b__273 == BitVector::<12>::new(0b101110001001))} => {"mhpmcounter9h"}
        b__274 if {(b__274 == BitVector::<12>::new(0b101110001010))} => {"mhpmcounter10h"}
        b__275 if {(b__275 == BitVector::<12>::new(0b101110001011))} => {"mhpmcounter11h"}
        b__276 if {(b__276 == BitVector::<12>::new(0b101110001100))} => {"mhpmcounter12h"}
        b__277 if {(b__277 == BitVector::<12>::new(0b101110001101))} => {"mhpmcounter13h"}
        b__278 if {(b__278 == BitVector::<12>::new(0b101110001110))} => {"mhpmcounter14h"}
        b__279 if {(b__279 == BitVector::<12>::new(0b101110001111))} => {"mhpmcounter15h"}
        b__280 if {(b__280 == BitVector::<12>::new(0b101110010000))} => {"mhpmcounter16h"}
        b__281 if {(b__281 == BitVector::<12>::new(0b101110010001))} => {"mhpmcounter17h"}
        b__282 if {(b__282 == BitVector::<12>::new(0b101110010010))} => {"mhpmcounter18h"}
        b__283 if {(b__283 == BitVector::<12>::new(0b101110010011))} => {"mhpmcounter19h"}
        b__284 if {(b__284 == BitVector::<12>::new(0b101110010100))} => {"mhpmcounter20h"}
        b__285 if {(b__285 == BitVector::<12>::new(0b101110010101))} => {"mhpmcounter21h"}
        b__286 if {(b__286 == BitVector::<12>::new(0b101110010110))} => {"mhpmcounter22h"}
        b__287 if {(b__287 == BitVector::<12>::new(0b101110010111))} => {"mhpmcounter23h"}
        b__288 if {(b__288 == BitVector::<12>::new(0b101110011000))} => {"mhpmcounter24h"}
        b__289 if {(b__289 == BitVector::<12>::new(0b101110011001))} => {"mhpmcounter25h"}
        b__290 if {(b__290 == BitVector::<12>::new(0b101110011010))} => {"mhpmcounter26h"}
        b__291 if {(b__291 == BitVector::<12>::new(0b101110011011))} => {"mhpmcounter27h"}
        b__292 if {(b__292 == BitVector::<12>::new(0b101110011100))} => {"mhpmcounter28h"}
        b__293 if {(b__293 == BitVector::<12>::new(0b101110011101))} => {"mhpmcounter29h"}
        b__294 if {(b__294 == BitVector::<12>::new(0b101110011110))} => {"mhpmcounter30h"}
        b__295 if {(b__295 == BitVector::<12>::new(0b101110011111))} => {"mhpmcounter31h"}
        b__296 if {(b__296 == BitVector::<12>::new(0b110110100000))} => {"scountovf"}
        b__297 if {(b__297 == BitVector::<12>::new(0b000000010101))} => {"seed"}
        b__298 if {(b__298 == BitVector::<12>::new(0b110000000000))} => {"cycle"}
        b__299 if {(b__299 == BitVector::<12>::new(0b110000000001))} => {"time"}
        b__300 if {(b__300 == BitVector::<12>::new(0b110000000010))} => {"instret"}
        b__301 if {(b__301 == BitVector::<12>::new(0b110010000000))} => {"cycleh"}
        b__302 if {(b__302 == BitVector::<12>::new(0b110010000001))} => {"timeh"}
        b__303 if {(b__303 == BitVector::<12>::new(0b110010000010))} => {"instreth"}
        b__304 if {(b__304 == BitVector::<12>::new(0b101100000000))} => {"mcycle"}
        b__305 if {(b__305 == BitVector::<12>::new(0b101100000010))} => {"minstret"}
        b__306 if {(b__306 == BitVector::<12>::new(0b101110000000))} => {"mcycleh"}
        b__307 if {(b__307 == BitVector::<12>::new(0b101110000010))} => {"minstreth"}
        b__308 if {(b__308 == BitVector::<12>::new(0b000000000001))} => {"fflags"}
        b__309 if {(b__309 == BitVector::<12>::new(0b000000000010))} => {"frm"}
        b__310 if {(b__310 == BitVector::<12>::new(0b000000000011))} => {"fcsr"}
        b__311 if {(b__311 == BitVector::<12>::new(0b001100100001))} => {"mcyclecfg"}
        b__312 if {(b__312 == BitVector::<12>::new(0b011100100001))} => {"mcyclecfgh"}
        b__313 if {(b__313 == BitVector::<12>::new(0b001100100010))} => {"minstretcfg"}
        b__314 if {(b__314 == BitVector::<12>::new(0b011100100010))} => {"minstretcfgh"}
        b__315 if {(b__315 == BitVector::<12>::new(0b000101001101))} => {"stimecmp"}
        b__316 if {(b__316 == BitVector::<12>::new(0b000101011101))} => {"stimecmph"}
        b__317 if {(b__317 == BitVector::<12>::new(0b000110000000))} => {"satp"}
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

/// fregidx
/// 
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L55.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fregidx {
    Fregidx(BitVector<5>)
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

/// fwffunct6
/// 
/// Generated from the Sail sources at `riscv_vreg_type.sail` L144.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwffunct6 {
    FWF_VADD,
    FWF_VSUB
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

/// fwvffunct6
/// 
/// Generated from the Sail sources at `riscv_vreg_type.sail` L140.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvffunct6 {
    FWVF_VADD,
    FWVF_VSUB,
    FWVF_VMUL
}

/// fwvfunct6
/// 
/// Generated from the Sail sources at `riscv_vreg_type.sail` L121.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvfunct6 {
    FWV_VADD,
    FWV_VSUB
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

/// fwvvfunct6
/// 
/// Generated from the Sail sources at `riscv_vreg_type.sail` L117.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvvfunct6 {
    FWVV_VADD,
    FWVV_VSUB,
    FWVV_VMUL
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

/// vregidx
/// 
/// Generated from the Sail sources at `riscv_vext_regs.sail` L9.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum vregidx {
    Vregidx(BitVector<5>)
}

/// brop_zba
/// 
/// Generated from the Sail sources at `riscv_types.sail` L285.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum brop_zba {
    SH1ADD,
    SH2ADD,
    SH3ADD
}

/// bropw_zba
/// 
/// Generated from the Sail sources at `riscv_types.sail` L293.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum bropw_zba {
    ADDUW,
    SH1ADDUW,
    SH2ADDUW,
    SH3ADDUW
}
/// ast
/// 
/// Generated from the Sail sources at `riscv_insts_begin.sail` L14.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    ILLEGAL(word),
    C_ILLEGAL(half),
    UTYPE((BitVector<20>, regidx, uop)),
    JAL((BitVector<21>, regidx)),
    JALR((BitVector<12>, regidx, regidx)),
    BTYPE((BitVector<13>, regidx, regidx, bop)),
    ITYPE((BitVector<12>, regidx, regidx, iop)),
    SHIFTIOP((BitVector<6>, regidx, regidx, sop)),
    RTYPE((regidx, regidx, regidx, rop)),
    LOAD((BitVector<12>, regidx, regidx, bool, word_width, bool, bool)),
    STORE((BitVector<12>, regidx, regidx, word_width, bool, bool)),
    ADDIW((BitVector<12>, regidx, regidx)),
    RTYPEW((regidx, regidx, regidx, ropw)),
    SHIFTIWOP((BitVector<5>, regidx, regidx, sopw)),
    FENCE((BitVector<4>, BitVector<4>)),
    FENCE_TSO((BitVector<4>, BitVector<4>)),
    ENTER_ANCHOR((regidx, regidx)),
    EXIT_ANCHOR((regidx, regidx)),
    ECALL(()),
    MRET(()),
    SRET(()),
    EBREAK(()),
    WFI(()),
    SFENCE_VMA((regidx, regidx)),
    FENCEI(()),
    LOADRES((bool, bool, regidx, word_width, regidx)),
    STORECON((bool, bool, regidx, regidx, word_width, regidx)),
    AMO((amoop, bool, bool, regidx, regidx, word_width, regidx)),
    MUL((regidx, regidx, regidx, mul_op)),
    DIV((regidx, regidx, regidx, bool)),
    REM((regidx, regidx, regidx, bool)),
    MULW((regidx, regidx, regidx)),
    DIVW((regidx, regidx, regidx, bool)),
    REMW((regidx, regidx, regidx, bool)),
    CSRReg((csreg, regidx, regidx, csrop)),
    CSRImm((csreg, BitVector<5>, regidx, csrop)),
    FENCE_RESERVED((BitVector<4>, BitVector<4>, BitVector<4>, regidx, regidx)),
    FENCEI_RESERVED((BitVector<12>, regidx, regidx)),
    LOAD_FP((BitVector<12>, regidx, fregidx, word_width)),
    STORE_FP((BitVector<12>, fregidx, regidx, word_width)),
    SINVAL_VMA((regidx, regidx)),
    SFENCE_W_INVAL(()),
    SFENCE_INVAL_IR(()),
    SLLIUW((BitVector<6>, regidx, regidx)),
    ZBA_RTYPEUW((regidx, regidx, regidx, bropw_zba)),
    ZBA_RTYPE((regidx, regidx, regidx, brop_zba)),
    RORIW((BitVector<5>, regidx, regidx)),
    RORI((BitVector<6>, regidx, regidx)),
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
pub fn interruptType_to_bits(i: InterruptType) -> BitVector<8> {
    match i {
        InterruptType::I_U_Software => {BitVector::<8>::new(0b00000000)}
        InterruptType::I_S_Software => {BitVector::<8>::new(0b00000001)}
        InterruptType::I_M_Software => {BitVector::<8>::new(0b00000011)}
        InterruptType::I_U_Timer => {BitVector::<8>::new(0b00000100)}
        InterruptType::I_S_Timer => {BitVector::<8>::new(0b00000101)}
        InterruptType::I_M_Timer => {BitVector::<8>::new(0b00000111)}
        InterruptType::I_U_External => {BitVector::<8>::new(0b00001000)}
        InterruptType::I_S_External => {BitVector::<8>::new(0b00001001)}
        InterruptType::I_M_External => {BitVector::<8>::new(0b00001011)}
        _ => {panic!("Unreachable code")}
    }
}

/// exceptionType_to_bits
/// 
/// Generated from the Sail sources at `riscv_types.sail` L148-169.
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

pub type tv_mode = BitVector<2>;

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
pub fn trapVectorMode_of_bits(m: BitVector<2>) -> TrapVectorMode {
    match m {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {TrapVectorMode::TV_Direct}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {TrapVectorMode::TV_Vector}
        _ => {TrapVectorMode::TV_Reserved}
        _ => {panic!("Unreachable code")}
    }
}

pub type ext_status = BitVector<2>;

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
pub fn extStatus_bits_forwards(arg_hashtag_: ExtStatus) -> BitVector<2> {
    match arg_hashtag_ {
        ExtStatus::Off => {BitVector::<2>::new(0b00)}
        ExtStatus::Initial => {BitVector::<2>::new(0b01)}
        ExtStatus::Clean => {BitVector::<2>::new(0b10)}
        ExtStatus::Dirty => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// extStatus_bits_backwards
/// 
/// Generated from the Sail sources.
pub fn extStatus_bits_backwards(arg_hashtag_: BitVector<2>) -> ExtStatus {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {ExtStatus::Off}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {ExtStatus::Initial}
        b__2 if {(b__2 == BitVector::<2>::new(0b10))} => {ExtStatus::Clean}
        _ => {ExtStatus::Dirty}
        _ => {panic!("Unreachable code")}
    }
}

/// extStatus_to_bits
/// 
/// Generated from the Sail sources at `riscv_types.sail` L247.
pub fn extStatus_to_bits(e: ExtStatus) -> BitVector<2> {
    extStatus_bits_forwards(e)
}

/// extStatus_of_bits
/// 
/// Generated from the Sail sources at `riscv_types.sail` L248.
pub fn extStatus_of_bits(b: BitVector<2>) -> ExtStatus {
    extStatus_bits_backwards(b)
}

pub type satp_mode = BitVector<4>;

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
pub fn satpMode_of_bits(a: Architecture, m: BitVector<4>) -> Option<SATPMode> {
    match (a, m) {
        (_, b__0) if {(b__0 == BitVector::<4>::new(0b0000))} => {Some(SATPMode::Bare)}
        (Architecture::RV32, b__1) if {(b__1 == BitVector::<4>::new(0b0001))} => {Some(SATPMode::Sv32)}
        (Architecture::RV64, b__2) if {(b__2 == BitVector::<4>::new(0b1000))} => {Some(SATPMode::Sv39)}
        (Architecture::RV64, b__3) if {(b__3 == BitVector::<4>::new(0b1001))} => {Some(SATPMode::Sv48)}
        (Architecture::RV64, b__4) if {(b__4 == BitVector::<4>::new(0b1010))} => {Some(SATPMode::Sv57)}
        (_, _) => {None}
        _ => {panic!("Unreachable code")}
    }
}

pub type csrRW = BitVector<2>;

/// size_enc_backwards
/// 
/// Generated from the Sail sources.
pub fn size_enc_backwards(arg_hashtag_: BitVector<2>) -> word_width {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {word_width::BYTE}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {word_width::HALF}
        b__2 if {(b__2 == BitVector::<2>::new(0b10))} => {word_width::WORD}
        _ => {word_width::DOUBLE}
        _ => {panic!("Unreachable code")}
    }
}

/// size_enc_backwards_matches
/// 
/// Generated from the Sail sources.
pub fn size_enc_backwards_matches(arg_hashtag_: BitVector<2>) -> bool {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {true}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {true}
        b__2 if {(b__2 == BitVector::<2>::new(0b10))} => {true}
        b__3 if {(b__3 == BitVector::<2>::new(0b11))} => {true}
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

pub const Data: ext_access_type = ();

pub const default_write_acc: ext_access_type = Data;

/// xreg_write_callback
/// 
/// Generated from the Sail sources at `riscv_callbacks.sail` L23.
pub const fn xreg_write_callback(_: regidx, missing_arg_0: BitVector<{
    64
}>) {
    ()
}

/// csr_full_read_callback
/// 
/// Generated from the Sail sources at `riscv_callbacks.sail` L29.
pub const fn csr_full_read_callback(_: &'static str, missing_arg_0: BitVector<12>, missing_arg_1: BitVector<{
    64
}>) {
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
pub fn csr_name_map_backwards(arg_hashtag_: &'static str) -> BitVector<12> {
    let head_exp_hashtag_ = arg_hashtag_;
    match match head_exp_hashtag_ {
        "misa" => {Some(BitVector::<12>::new(0b001100000001))}
        "mstatus" => {Some(BitVector::<12>::new(0b001100000000))}
        "menvcfg" => {Some(BitVector::<12>::new(0b001100001010))}
        "menvcfgh" => {Some(BitVector::<12>::new(0b001100011010))}
        "senvcfg" => {Some(BitVector::<12>::new(0b000100001010))}
        "mie" => {Some(BitVector::<12>::new(0b001100000100))}
        "mip" => {Some(BitVector::<12>::new(0b001101000100))}
        "medeleg" => {Some(BitVector::<12>::new(0b001100000010))}
        "medelegh" => {Some(BitVector::<12>::new(0b001100010010))}
        "mideleg" => {Some(BitVector::<12>::new(0b001100000011))}
        "mcause" => {Some(BitVector::<12>::new(0b001101000010))}
        "mtval" => {Some(BitVector::<12>::new(0b001101000011))}
        "mscratch" => {Some(BitVector::<12>::new(0b001101000000))}
        "scounteren" => {Some(BitVector::<12>::new(0b000100000110))}
        "mcounteren" => {Some(BitVector::<12>::new(0b001100000110))}
        "mcountinhibit" => {Some(BitVector::<12>::new(0b001100100000))}
        "mvendorid" => {Some(BitVector::<12>::new(0b111100010001))}
        "marchid" => {Some(BitVector::<12>::new(0b111100010010))}
        "mimpid" => {Some(BitVector::<12>::new(0b111100010011))}
        "mhartid" => {Some(BitVector::<12>::new(0b111100010100))}
        "mconfigptr" => {Some(BitVector::<12>::new(0b111100010101))}
        "sstatus" => {Some(BitVector::<12>::new(0b000100000000))}
        "sip" => {Some(BitVector::<12>::new(0b000101000100))}
        "sie" => {Some(BitVector::<12>::new(0b000100000100))}
        "sscratch" => {Some(BitVector::<12>::new(0b000101000000))}
        "scause" => {Some(BitVector::<12>::new(0b000101000010))}
        "stval" => {Some(BitVector::<12>::new(0b000101000011))}
        "tselect" => {Some(BitVector::<12>::new(0b011110100000))}
        "tdata1" => {Some(BitVector::<12>::new(0b011110100001))}
        "tdata2" => {Some(BitVector::<12>::new(0b011110100010))}
        "tdata3" => {Some(BitVector::<12>::new(0b011110100011))}
        "pmpcfg0" => {Some(BitVector::<12>::new(0b001110100000))}
        "pmpcfg1" => {Some(BitVector::<12>::new(0b001110100001))}
        "pmpcfg2" => {Some(BitVector::<12>::new(0b001110100010))}
        "pmpcfg3" => {Some(BitVector::<12>::new(0b001110100011))}
        "pmpcfg4" => {Some(BitVector::<12>::new(0b001110100100))}
        "pmpcfg5" => {Some(BitVector::<12>::new(0b001110100101))}
        "pmpcfg6" => {Some(BitVector::<12>::new(0b001110100110))}
        "pmpcfg7" => {Some(BitVector::<12>::new(0b001110100111))}
        "pmpcfg8" => {Some(BitVector::<12>::new(0b001110101000))}
        "pmpcfg9" => {Some(BitVector::<12>::new(0b001110101001))}
        "pmpcfg10" => {Some(BitVector::<12>::new(0b001110101010))}
        "pmpcfg11" => {Some(BitVector::<12>::new(0b001110101011))}
        "pmpcfg12" => {Some(BitVector::<12>::new(0b001110101100))}
        "pmpcfg13" => {Some(BitVector::<12>::new(0b001110101101))}
        "pmpcfg14" => {Some(BitVector::<12>::new(0b001110101110))}
        "pmpcfg15" => {Some(BitVector::<12>::new(0b001110101111))}
        "pmpaddr0" => {Some(BitVector::<12>::new(0b001110110000))}
        "pmpaddr1" => {Some(BitVector::<12>::new(0b001110110001))}
        "pmpaddr2" => {Some(BitVector::<12>::new(0b001110110010))}
        "pmpaddr3" => {Some(BitVector::<12>::new(0b001110110011))}
        "pmpaddr4" => {Some(BitVector::<12>::new(0b001110110100))}
        "pmpaddr5" => {Some(BitVector::<12>::new(0b001110110101))}
        "pmpaddr6" => {Some(BitVector::<12>::new(0b001110110110))}
        "pmpaddr7" => {Some(BitVector::<12>::new(0b001110110111))}
        "pmpaddr8" => {Some(BitVector::<12>::new(0b001110111000))}
        "pmpaddr9" => {Some(BitVector::<12>::new(0b001110111001))}
        "pmpaddr10" => {Some(BitVector::<12>::new(0b001110111010))}
        "pmpaddr11" => {Some(BitVector::<12>::new(0b001110111011))}
        "pmpaddr12" => {Some(BitVector::<12>::new(0b001110111100))}
        "pmpaddr13" => {Some(BitVector::<12>::new(0b001110111101))}
        "pmpaddr14" => {Some(BitVector::<12>::new(0b001110111110))}
        "pmpaddr15" => {Some(BitVector::<12>::new(0b001110111111))}
        "pmpaddr16" => {Some(BitVector::<12>::new(0b001111000000))}
        "pmpaddr17" => {Some(BitVector::<12>::new(0b001111000001))}
        "pmpaddr18" => {Some(BitVector::<12>::new(0b001111000010))}
        "pmpaddr19" => {Some(BitVector::<12>::new(0b001111000011))}
        "pmpaddr20" => {Some(BitVector::<12>::new(0b001111000100))}
        "pmpaddr21" => {Some(BitVector::<12>::new(0b001111000101))}
        "pmpaddr22" => {Some(BitVector::<12>::new(0b001111000110))}
        "pmpaddr23" => {Some(BitVector::<12>::new(0b001111000111))}
        "pmpaddr24" => {Some(BitVector::<12>::new(0b001111001000))}
        "pmpaddr25" => {Some(BitVector::<12>::new(0b001111001001))}
        "pmpaddr26" => {Some(BitVector::<12>::new(0b001111001010))}
        "pmpaddr27" => {Some(BitVector::<12>::new(0b001111001011))}
        "pmpaddr28" => {Some(BitVector::<12>::new(0b001111001100))}
        "pmpaddr29" => {Some(BitVector::<12>::new(0b001111001101))}
        "pmpaddr30" => {Some(BitVector::<12>::new(0b001111001110))}
        "pmpaddr31" => {Some(BitVector::<12>::new(0b001111001111))}
        "pmpaddr32" => {Some(BitVector::<12>::new(0b001111010000))}
        "pmpaddr33" => {Some(BitVector::<12>::new(0b001111010001))}
        "pmpaddr34" => {Some(BitVector::<12>::new(0b001111010010))}
        "pmpaddr35" => {Some(BitVector::<12>::new(0b001111010011))}
        "pmpaddr36" => {Some(BitVector::<12>::new(0b001111010100))}
        "pmpaddr37" => {Some(BitVector::<12>::new(0b001111010101))}
        "pmpaddr38" => {Some(BitVector::<12>::new(0b001111010110))}
        "pmpaddr39" => {Some(BitVector::<12>::new(0b001111010111))}
        "pmpaddr40" => {Some(BitVector::<12>::new(0b001111011000))}
        "pmpaddr41" => {Some(BitVector::<12>::new(0b001111011001))}
        "pmpaddr42" => {Some(BitVector::<12>::new(0b001111011010))}
        "pmpaddr43" => {Some(BitVector::<12>::new(0b001111011011))}
        "pmpaddr44" => {Some(BitVector::<12>::new(0b001111011100))}
        "pmpaddr45" => {Some(BitVector::<12>::new(0b001111011101))}
        "pmpaddr46" => {Some(BitVector::<12>::new(0b001111011110))}
        "pmpaddr47" => {Some(BitVector::<12>::new(0b001111011111))}
        "pmpaddr48" => {Some(BitVector::<12>::new(0b001111100000))}
        "pmpaddr49" => {Some(BitVector::<12>::new(0b001111100001))}
        "pmpaddr50" => {Some(BitVector::<12>::new(0b001111100010))}
        "pmpaddr51" => {Some(BitVector::<12>::new(0b001111100011))}
        "pmpaddr52" => {Some(BitVector::<12>::new(0b001111100100))}
        "pmpaddr53" => {Some(BitVector::<12>::new(0b001111100101))}
        "pmpaddr54" => {Some(BitVector::<12>::new(0b001111100110))}
        "pmpaddr55" => {Some(BitVector::<12>::new(0b001111100111))}
        "pmpaddr56" => {Some(BitVector::<12>::new(0b001111101000))}
        "pmpaddr57" => {Some(BitVector::<12>::new(0b001111101001))}
        "pmpaddr58" => {Some(BitVector::<12>::new(0b001111101010))}
        "pmpaddr59" => {Some(BitVector::<12>::new(0b001111101011))}
        "pmpaddr60" => {Some(BitVector::<12>::new(0b001111101100))}
        "pmpaddr61" => {Some(BitVector::<12>::new(0b001111101101))}
        "pmpaddr62" => {Some(BitVector::<12>::new(0b001111101110))}
        "pmpaddr63" => {Some(BitVector::<12>::new(0b001111101111))}
        "vstart" => {Some(BitVector::<12>::new(0b000000001000))}
        "vxsat" => {Some(BitVector::<12>::new(0b000000001001))}
        "vxrm" => {Some(BitVector::<12>::new(0b000000001010))}
        "vcsr" => {Some(BitVector::<12>::new(0b000000001111))}
        "vl" => {Some(BitVector::<12>::new(0b110000100000))}
        "vtype" => {Some(BitVector::<12>::new(0b110000100001))}
        "vlenb" => {Some(BitVector::<12>::new(0b110000100010))}
        "stvec" => {Some(BitVector::<12>::new(0b000100000101))}
        "sepc" => {Some(BitVector::<12>::new(0b000101000001))}
        "mtvec" => {Some(BitVector::<12>::new(0b001100000101))}
        "mepc" => {Some(BitVector::<12>::new(0b001101000001))}
        "hpmcounter3" => {Some(BitVector::<12>::new(0b110000000011))}
        "hpmcounter4" => {Some(BitVector::<12>::new(0b110000000100))}
        "hpmcounter5" => {Some(BitVector::<12>::new(0b110000000101))}
        "hpmcounter6" => {Some(BitVector::<12>::new(0b110000000110))}
        "hpmcounter7" => {Some(BitVector::<12>::new(0b110000000111))}
        "hpmcounter8" => {Some(BitVector::<12>::new(0b110000001000))}
        "hpmcounter9" => {Some(BitVector::<12>::new(0b110000001001))}
        "hpmcounter10" => {Some(BitVector::<12>::new(0b110000001010))}
        "hpmcounter11" => {Some(BitVector::<12>::new(0b110000001011))}
        "hpmcounter12" => {Some(BitVector::<12>::new(0b110000001100))}
        "hpmcounter13" => {Some(BitVector::<12>::new(0b110000001101))}
        "hpmcounter14" => {Some(BitVector::<12>::new(0b110000001110))}
        "hpmcounter15" => {Some(BitVector::<12>::new(0b110000001111))}
        "hpmcounter16" => {Some(BitVector::<12>::new(0b110000010000))}
        "hpmcounter17" => {Some(BitVector::<12>::new(0b110000010001))}
        "hpmcounter18" => {Some(BitVector::<12>::new(0b110000010010))}
        "hpmcounter19" => {Some(BitVector::<12>::new(0b110000010011))}
        "hpmcounter20" => {Some(BitVector::<12>::new(0b110000010100))}
        "hpmcounter21" => {Some(BitVector::<12>::new(0b110000010101))}
        "hpmcounter22" => {Some(BitVector::<12>::new(0b110000010110))}
        "hpmcounter23" => {Some(BitVector::<12>::new(0b110000010111))}
        "hpmcounter24" => {Some(BitVector::<12>::new(0b110000011000))}
        "hpmcounter25" => {Some(BitVector::<12>::new(0b110000011001))}
        "hpmcounter26" => {Some(BitVector::<12>::new(0b110000011010))}
        "hpmcounter27" => {Some(BitVector::<12>::new(0b110000011011))}
        "hpmcounter28" => {Some(BitVector::<12>::new(0b110000011100))}
        "hpmcounter29" => {Some(BitVector::<12>::new(0b110000011101))}
        "hpmcounter30" => {Some(BitVector::<12>::new(0b110000011110))}
        "hpmcounter31" => {Some(BitVector::<12>::new(0b110000011111))}
        "hpmcounter3h" => {Some(BitVector::<12>::new(0b110010000011))}
        "hpmcounter4h" => {Some(BitVector::<12>::new(0b110010000100))}
        "hpmcounter5h" => {Some(BitVector::<12>::new(0b110010000101))}
        "hpmcounter6h" => {Some(BitVector::<12>::new(0b110010000110))}
        "hpmcounter7h" => {Some(BitVector::<12>::new(0b110010000111))}
        "hpmcounter8h" => {Some(BitVector::<12>::new(0b110010001000))}
        "hpmcounter9h" => {Some(BitVector::<12>::new(0b110010001001))}
        "hpmcounter10h" => {Some(BitVector::<12>::new(0b110010001010))}
        "hpmcounter11h" => {Some(BitVector::<12>::new(0b110010001011))}
        "hpmcounter12h" => {Some(BitVector::<12>::new(0b110010001100))}
        "hpmcounter13h" => {Some(BitVector::<12>::new(0b110010001101))}
        "hpmcounter14h" => {Some(BitVector::<12>::new(0b110010001110))}
        "hpmcounter15h" => {Some(BitVector::<12>::new(0b110010001111))}
        "hpmcounter16h" => {Some(BitVector::<12>::new(0b110010010000))}
        "hpmcounter17h" => {Some(BitVector::<12>::new(0b110010010001))}
        "hpmcounter18h" => {Some(BitVector::<12>::new(0b110010010010))}
        "hpmcounter19h" => {Some(BitVector::<12>::new(0b110010010011))}
        "hpmcounter20h" => {Some(BitVector::<12>::new(0b110010010100))}
        "hpmcounter21h" => {Some(BitVector::<12>::new(0b110010010101))}
        "hpmcounter22h" => {Some(BitVector::<12>::new(0b110010010110))}
        "hpmcounter23h" => {Some(BitVector::<12>::new(0b110010010111))}
        "hpmcounter24h" => {Some(BitVector::<12>::new(0b110010011000))}
        "hpmcounter25h" => {Some(BitVector::<12>::new(0b110010011001))}
        "hpmcounter26h" => {Some(BitVector::<12>::new(0b110010011010))}
        "hpmcounter27h" => {Some(BitVector::<12>::new(0b110010011011))}
        "hpmcounter28h" => {Some(BitVector::<12>::new(0b110010011100))}
        "hpmcounter29h" => {Some(BitVector::<12>::new(0b110010011101))}
        "hpmcounter30h" => {Some(BitVector::<12>::new(0b110010011110))}
        "hpmcounter31h" => {Some(BitVector::<12>::new(0b110010011111))}
        "mhpmevent3" => {Some(BitVector::<12>::new(0b001100100011))}
        "mhpmevent4" => {Some(BitVector::<12>::new(0b001100100100))}
        "mhpmevent5" => {Some(BitVector::<12>::new(0b001100100101))}
        "mhpmevent6" => {Some(BitVector::<12>::new(0b001100100110))}
        "mhpmevent7" => {Some(BitVector::<12>::new(0b001100100111))}
        "mhpmevent8" => {Some(BitVector::<12>::new(0b001100101000))}
        "mhpmevent9" => {Some(BitVector::<12>::new(0b001100101001))}
        "mhpmevent10" => {Some(BitVector::<12>::new(0b001100101010))}
        "mhpmevent11" => {Some(BitVector::<12>::new(0b001100101011))}
        "mhpmevent12" => {Some(BitVector::<12>::new(0b001100101100))}
        "mhpmevent13" => {Some(BitVector::<12>::new(0b001100101101))}
        "mhpmevent14" => {Some(BitVector::<12>::new(0b001100101110))}
        "mhpmevent15" => {Some(BitVector::<12>::new(0b001100101111))}
        "mhpmevent16" => {Some(BitVector::<12>::new(0b001100110000))}
        "mhpmevent17" => {Some(BitVector::<12>::new(0b001100110001))}
        "mhpmevent18" => {Some(BitVector::<12>::new(0b001100110010))}
        "mhpmevent19" => {Some(BitVector::<12>::new(0b001100110011))}
        "mhpmevent20" => {Some(BitVector::<12>::new(0b001100110100))}
        "mhpmevent21" => {Some(BitVector::<12>::new(0b001100110101))}
        "mhpmevent22" => {Some(BitVector::<12>::new(0b001100110110))}
        "mhpmevent23" => {Some(BitVector::<12>::new(0b001100110111))}
        "mhpmevent24" => {Some(BitVector::<12>::new(0b001100111000))}
        "mhpmevent25" => {Some(BitVector::<12>::new(0b001100111001))}
        "mhpmevent26" => {Some(BitVector::<12>::new(0b001100111010))}
        "mhpmevent27" => {Some(BitVector::<12>::new(0b001100111011))}
        "mhpmevent28" => {Some(BitVector::<12>::new(0b001100111100))}
        "mhpmevent29" => {Some(BitVector::<12>::new(0b001100111101))}
        "mhpmevent30" => {Some(BitVector::<12>::new(0b001100111110))}
        "mhpmevent31" => {Some(BitVector::<12>::new(0b001100111111))}
        "mhpmcounter3" => {Some(BitVector::<12>::new(0b101100000011))}
        "mhpmcounter4" => {Some(BitVector::<12>::new(0b101100000100))}
        "mhpmcounter5" => {Some(BitVector::<12>::new(0b101100000101))}
        "mhpmcounter6" => {Some(BitVector::<12>::new(0b101100000110))}
        "mhpmcounter7" => {Some(BitVector::<12>::new(0b101100000111))}
        "mhpmcounter8" => {Some(BitVector::<12>::new(0b101100001000))}
        "mhpmcounter9" => {Some(BitVector::<12>::new(0b101100001001))}
        "mhpmcounter10" => {Some(BitVector::<12>::new(0b101100001010))}
        "mhpmcounter11" => {Some(BitVector::<12>::new(0b101100001011))}
        "mhpmcounter12" => {Some(BitVector::<12>::new(0b101100001100))}
        "mhpmcounter13" => {Some(BitVector::<12>::new(0b101100001101))}
        "mhpmcounter14" => {Some(BitVector::<12>::new(0b101100001110))}
        "mhpmcounter15" => {Some(BitVector::<12>::new(0b101100001111))}
        "mhpmcounter16" => {Some(BitVector::<12>::new(0b101100010000))}
        "mhpmcounter17" => {Some(BitVector::<12>::new(0b101100010001))}
        "mhpmcounter18" => {Some(BitVector::<12>::new(0b101100010010))}
        "mhpmcounter19" => {Some(BitVector::<12>::new(0b101100010011))}
        "mhpmcounter20" => {Some(BitVector::<12>::new(0b101100010100))}
        "mhpmcounter21" => {Some(BitVector::<12>::new(0b101100010101))}
        "mhpmcounter22" => {Some(BitVector::<12>::new(0b101100010110))}
        "mhpmcounter23" => {Some(BitVector::<12>::new(0b101100010111))}
        "mhpmcounter24" => {Some(BitVector::<12>::new(0b101100011000))}
        "mhpmcounter25" => {Some(BitVector::<12>::new(0b101100011001))}
        "mhpmcounter26" => {Some(BitVector::<12>::new(0b101100011010))}
        "mhpmcounter27" => {Some(BitVector::<12>::new(0b101100011011))}
        "mhpmcounter28" => {Some(BitVector::<12>::new(0b101100011100))}
        "mhpmcounter29" => {Some(BitVector::<12>::new(0b101100011101))}
        "mhpmcounter30" => {Some(BitVector::<12>::new(0b101100011110))}
        "mhpmcounter31" => {Some(BitVector::<12>::new(0b101100011111))}
        "mhpmcounter3h" => {Some(BitVector::<12>::new(0b101110000011))}
        "mhpmcounter4h" => {Some(BitVector::<12>::new(0b101110000100))}
        "mhpmcounter5h" => {Some(BitVector::<12>::new(0b101110000101))}
        "mhpmcounter6h" => {Some(BitVector::<12>::new(0b101110000110))}
        "mhpmcounter7h" => {Some(BitVector::<12>::new(0b101110000111))}
        "mhpmcounter8h" => {Some(BitVector::<12>::new(0b101110001000))}
        "mhpmcounter9h" => {Some(BitVector::<12>::new(0b101110001001))}
        "mhpmcounter10h" => {Some(BitVector::<12>::new(0b101110001010))}
        "mhpmcounter11h" => {Some(BitVector::<12>::new(0b101110001011))}
        "mhpmcounter12h" => {Some(BitVector::<12>::new(0b101110001100))}
        "mhpmcounter13h" => {Some(BitVector::<12>::new(0b101110001101))}
        "mhpmcounter14h" => {Some(BitVector::<12>::new(0b101110001110))}
        "mhpmcounter15h" => {Some(BitVector::<12>::new(0b101110001111))}
        "mhpmcounter16h" => {Some(BitVector::<12>::new(0b101110010000))}
        "mhpmcounter17h" => {Some(BitVector::<12>::new(0b101110010001))}
        "mhpmcounter18h" => {Some(BitVector::<12>::new(0b101110010010))}
        "mhpmcounter19h" => {Some(BitVector::<12>::new(0b101110010011))}
        "mhpmcounter20h" => {Some(BitVector::<12>::new(0b101110010100))}
        "mhpmcounter21h" => {Some(BitVector::<12>::new(0b101110010101))}
        "mhpmcounter22h" => {Some(BitVector::<12>::new(0b101110010110))}
        "mhpmcounter23h" => {Some(BitVector::<12>::new(0b101110010111))}
        "mhpmcounter24h" => {Some(BitVector::<12>::new(0b101110011000))}
        "mhpmcounter25h" => {Some(BitVector::<12>::new(0b101110011001))}
        "mhpmcounter26h" => {Some(BitVector::<12>::new(0b101110011010))}
        "mhpmcounter27h" => {Some(BitVector::<12>::new(0b101110011011))}
        "mhpmcounter28h" => {Some(BitVector::<12>::new(0b101110011100))}
        "mhpmcounter29h" => {Some(BitVector::<12>::new(0b101110011101))}
        "mhpmcounter30h" => {Some(BitVector::<12>::new(0b101110011110))}
        "mhpmcounter31h" => {Some(BitVector::<12>::new(0b101110011111))}
        "mhpmcounter3h" => {Some(BitVector::<12>::new(0b101110000011))}
        "mhpmcounter4h" => {Some(BitVector::<12>::new(0b101110000100))}
        "mhpmcounter5h" => {Some(BitVector::<12>::new(0b101110000101))}
        "mhpmcounter6h" => {Some(BitVector::<12>::new(0b101110000110))}
        "mhpmcounter7h" => {Some(BitVector::<12>::new(0b101110000111))}
        "mhpmcounter8h" => {Some(BitVector::<12>::new(0b101110001000))}
        "mhpmcounter9h" => {Some(BitVector::<12>::new(0b101110001001))}
        "mhpmcounter10h" => {Some(BitVector::<12>::new(0b101110001010))}
        "mhpmcounter11h" => {Some(BitVector::<12>::new(0b101110001011))}
        "mhpmcounter12h" => {Some(BitVector::<12>::new(0b101110001100))}
        "mhpmcounter13h" => {Some(BitVector::<12>::new(0b101110001101))}
        "mhpmcounter14h" => {Some(BitVector::<12>::new(0b101110001110))}
        "mhpmcounter15h" => {Some(BitVector::<12>::new(0b101110001111))}
        "mhpmcounter16h" => {Some(BitVector::<12>::new(0b101110010000))}
        "mhpmcounter17h" => {Some(BitVector::<12>::new(0b101110010001))}
        "mhpmcounter18h" => {Some(BitVector::<12>::new(0b101110010010))}
        "mhpmcounter19h" => {Some(BitVector::<12>::new(0b101110010011))}
        "mhpmcounter20h" => {Some(BitVector::<12>::new(0b101110010100))}
        "mhpmcounter21h" => {Some(BitVector::<12>::new(0b101110010101))}
        "mhpmcounter22h" => {Some(BitVector::<12>::new(0b101110010110))}
        "mhpmcounter23h" => {Some(BitVector::<12>::new(0b101110010111))}
        "mhpmcounter24h" => {Some(BitVector::<12>::new(0b101110011000))}
        "mhpmcounter25h" => {Some(BitVector::<12>::new(0b101110011001))}
        "mhpmcounter26h" => {Some(BitVector::<12>::new(0b101110011010))}
        "mhpmcounter27h" => {Some(BitVector::<12>::new(0b101110011011))}
        "mhpmcounter28h" => {Some(BitVector::<12>::new(0b101110011100))}
        "mhpmcounter29h" => {Some(BitVector::<12>::new(0b101110011101))}
        "mhpmcounter30h" => {Some(BitVector::<12>::new(0b101110011110))}
        "mhpmcounter31h" => {Some(BitVector::<12>::new(0b101110011111))}
        "scountovf" => {Some(BitVector::<12>::new(0b110110100000))}
        "seed" => {Some(BitVector::<12>::new(0b000000010101))}
        "cycle" => {Some(BitVector::<12>::new(0b110000000000))}
        "time" => {Some(BitVector::<12>::new(0b110000000001))}
        "instret" => {Some(BitVector::<12>::new(0b110000000010))}
        "cycleh" => {Some(BitVector::<12>::new(0b110010000000))}
        "timeh" => {Some(BitVector::<12>::new(0b110010000001))}
        "instreth" => {Some(BitVector::<12>::new(0b110010000010))}
        "mcycle" => {Some(BitVector::<12>::new(0b101100000000))}
        "minstret" => {Some(BitVector::<12>::new(0b101100000010))}
        "mcycleh" => {Some(BitVector::<12>::new(0b101110000000))}
        "minstreth" => {Some(BitVector::<12>::new(0b101110000010))}
        "fflags" => {Some(BitVector::<12>::new(0b000000000001))}
        "frm" => {Some(BitVector::<12>::new(0b000000000010))}
        "fcsr" => {Some(BitVector::<12>::new(0b000000000011))}
        "mcyclecfg" => {Some(BitVector::<12>::new(0b001100100001))}
        "mcyclecfgh" => {Some(BitVector::<12>::new(0b011100100001))}
        "minstretcfg" => {Some(BitVector::<12>::new(0b001100100010))}
        "minstretcfgh" => {Some(BitVector::<12>::new(0b011100100010))}
        "stimecmp" => {Some(BitVector::<12>::new(0b000101001101))}
        "stimecmph" => {Some(BitVector::<12>::new(0b000101011101))}
        "satp" => {Some(BitVector::<12>::new(0b000110000000))}
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
pub fn csr_id_read_callback(csr: BitVector<12>, value: BitVector<{
    64
}>) {
    let name = csr_name_map_forwards(csr);
    
}

pub type regtype = xlenbits;

pub const zero_reg: regtype = zeros::<64>(64);

/// regval_from_reg
/// 
/// Generated from the Sail sources at `riscv_reg_type.sail` L22.
pub fn regval_from_reg(r: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    r
}

/// regval_into_reg
/// 
/// Generated from the Sail sources at `riscv_reg_type.sail` L25.
pub fn regval_into_reg(v: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    v
}

pub type fregtype = flenbits;

pub const zero_freg: fregtype = zeros::<64>(64);

/// rX
/// 
/// Generated from the Sail sources at `riscv_regs.sail` L48-86.
pub fn rX(core_ctx: &mut Core, regno::Regno(r): regno) -> BitVector<{
    64
}> {
    let v: regtype = match r {
        l__583 if {(l__583 == 0)} => {zero_reg}
        l__584 if {(l__584 == 1)} => {core_ctx.x1}
        l__585 if {(l__585 == 2)} => {core_ctx.x2}
        l__586 if {(l__586 == 3)} => {core_ctx.x3}
        l__587 if {(l__587 == 4)} => {core_ctx.x4}
        l__588 if {(l__588 == 5)} => {core_ctx.x5}
        l__589 if {(l__589 == 6)} => {core_ctx.x6}
        l__590 if {(l__590 == 7)} => {core_ctx.x7}
        l__591 if {(l__591 == 8)} => {core_ctx.x8}
        l__592 if {(l__592 == 9)} => {core_ctx.x9}
        l__593 if {(l__593 == 10)} => {core_ctx.x10}
        l__594 if {(l__594 == 11)} => {core_ctx.x11}
        l__595 if {(l__595 == 12)} => {core_ctx.x12}
        l__596 if {(l__596 == 13)} => {core_ctx.x13}
        l__597 if {(l__597 == 14)} => {core_ctx.x14}
        l__598 if {(l__598 == 15)} => {core_ctx.x15}
        l__599 if {(l__599 == 16)} => {core_ctx.x16}
        l__600 if {(l__600 == 17)} => {core_ctx.x17}
        l__601 if {(l__601 == 18)} => {core_ctx.x18}
        l__602 if {(l__602 == 19)} => {core_ctx.x19}
        l__603 if {(l__603 == 20)} => {core_ctx.x20}
        l__604 if {(l__604 == 21)} => {core_ctx.x21}
        l__605 if {(l__605 == 22)} => {core_ctx.x22}
        l__606 if {(l__606 == 23)} => {core_ctx.x23}
        l__607 if {(l__607 == 24)} => {core_ctx.x24}
        l__608 if {(l__608 == 25)} => {core_ctx.x25}
        l__609 if {(l__609 == 26)} => {core_ctx.x26}
        l__610 if {(l__610 == 27)} => {core_ctx.x27}
        l__611 if {(l__611 == 28)} => {core_ctx.x28}
        l__612 if {(l__612 == 29)} => {core_ctx.x29}
        l__613 if {(l__613 == 30)} => {core_ctx.x30}
        l__614 if {(l__614 == 31)} => {core_ctx.x31}
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
pub fn wX(core_ctx: &mut Core, regno::Regno(r): regno, in_v: BitVector<{
    64
}>) {
    let v = regval_into_reg(in_v);
    match r {
        l__551 if {(l__551 == 0)} => {()}
        l__552 if {(l__552 == 1)} => {core_ctx.x1 = v}
        l__553 if {(l__553 == 2)} => {core_ctx.x2 = v}
        l__554 if {(l__554 == 3)} => {core_ctx.x3 = v}
        l__555 if {(l__555 == 4)} => {core_ctx.x4 = v}
        l__556 if {(l__556 == 5)} => {core_ctx.x5 = v}
        l__557 if {(l__557 == 6)} => {core_ctx.x6 = v}
        l__558 if {(l__558 == 7)} => {core_ctx.x7 = v}
        l__559 if {(l__559 == 8)} => {core_ctx.x8 = v}
        l__560 if {(l__560 == 9)} => {core_ctx.x9 = v}
        l__561 if {(l__561 == 10)} => {core_ctx.x10 = v}
        l__562 if {(l__562 == 11)} => {core_ctx.x11 = v}
        l__563 if {(l__563 == 12)} => {core_ctx.x12 = v}
        l__564 if {(l__564 == 13)} => {core_ctx.x13 = v}
        l__565 if {(l__565 == 14)} => {core_ctx.x14 = v}
        l__566 if {(l__566 == 15)} => {core_ctx.x15 = v}
        l__567 if {(l__567 == 16)} => {core_ctx.x16 = v}
        l__568 if {(l__568 == 17)} => {core_ctx.x17 = v}
        l__569 if {(l__569 == 18)} => {core_ctx.x18 = v}
        l__570 if {(l__570 == 19)} => {core_ctx.x19 = v}
        l__571 if {(l__571 == 20)} => {core_ctx.x20 = v}
        l__572 if {(l__572 == 21)} => {core_ctx.x21 = v}
        l__573 if {(l__573 == 22)} => {core_ctx.x22 = v}
        l__574 if {(l__574 == 23)} => {core_ctx.x23 = v}
        l__575 if {(l__575 == 24)} => {core_ctx.x24 = v}
        l__576 if {(l__576 == 25)} => {core_ctx.x25 = v}
        l__577 if {(l__577 == 26)} => {core_ctx.x26 = v}
        l__578 if {(l__578 == 27)} => {core_ctx.x27 = v}
        l__579 if {(l__579 == 28)} => {core_ctx.x28 = v}
        l__580 if {(l__580 == 29)} => {core_ctx.x29 = v}
        l__581 if {(l__581 == 30)} => {core_ctx.x30 = v}
        l__582 if {(l__582 == 31)} => {core_ctx.x31 = v}
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
pub fn rX_bits(core_ctx: &mut Core, i: regidx) -> BitVector<{
    64
}> {
    rX(core_ctx, regidx_to_regno(i))
}

/// wX_bits
/// 
/// Generated from the Sail sources at `riscv_regs.sail` L130-132.
pub fn wX_bits(core_ctx: &mut Core, i: regidx, data: BitVector<{
    64
}>) {
    wX(core_ctx, regidx_to_regno(i), data)
}

/// encdec_reg_backwards
/// 
/// Generated from the Sail sources.
pub fn encdec_reg_backwards(arg_hashtag_: BitVector<5>) -> regidx {
    match arg_hashtag_ {
        r => {regidx::Regidx(r)}
        _ => {panic!("Unreachable code")}
    }
}

/// encdec_reg_backwards_matches
/// 
/// Generated from the Sail sources.
pub fn encdec_reg_backwards_matches(arg_hashtag_: BitVector<5>) -> bool {
    match arg_hashtag_ {
        r => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// get_arch_pc
/// 
/// Generated from the Sail sources at `riscv_pc_access.sail` L18.
pub fn get_arch_pc(core_ctx: &mut Core, unit_arg: ()) -> BitVector<{
    64
}> {
    core_ctx.PC
}

/// get_next_pc
/// 
/// Generated from the Sail sources at `riscv_pc_access.sail` L21.
pub fn get_next_pc(core_ctx: &mut Core, unit_arg: ()) -> BitVector<{
    64
}> {
    core_ctx.nextPC
}

/// set_next_pc
/// 
/// Generated from the Sail sources at `riscv_pc_access.sail` L24-27.
pub fn set_next_pc(core_ctx: &mut Core, pc: BitVector<{
    64
}>) {
    core_ctx.nextPC = pc
}

/// Misa
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L49-78.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Misa {
    pub bits: BitVector<{
    64
}>,
}

/// Mk_Misa
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Misa(v: BitVector<64>) -> Misa {
    Misa {
        bits: v
    }
}

/// CountSmcntrpmf
/// 
/// Generated from the Sail sources at `riscv_smcntrpmf.sail` L3-9.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct CountSmcntrpmf {
    pub bits: BitVector<64>,
}

/// Counteren
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L581-586.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Counteren {
    pub bits: BitVector<32>,
}

/// Counterin
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L614-618.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Counterin {
    pub bits: BitVector<32>,
}

/// Fcsr
/// 
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L384-387.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Fcsr {
    pub bits: BitVector<32>,
}

/// HpmEvent
/// 
/// Generated from the Sail sources at `riscv_zihpm.sail` L165-175.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct HpmEvent {
    pub bits: BitVector<64>,
}

/// MEnvcfg
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L315-332.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct MEnvcfg {
    pub bits: BitVector<64>,
}

/// Mcause
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L518-521.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mcause {
    pub bits: BitVector<{
    64
}>,
}

/// Medeleg
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L449-464.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Medeleg {
    pub bits: BitVector<64>,
}

/// Minterrupts
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L404-413.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Minterrupts {
    pub bits: BitVector<{
    64
}>,
}

/// Mstatus
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L168-204.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mstatus {
    pub bits: BitVector<64>,
}

/// Mtvec
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L503-506.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Mtvec {
    pub bits: BitVector<{
    64
}>,
}

/// PTE_Ext
/// 
/// Generated from the Sail sources at `riscv_vmem_pte.sail` L25-31.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PTE_Ext {
    pub bits: BitVector<10>,
}

/// PTE_Flags
/// 
/// Generated from the Sail sources at `riscv_vmem_pte.sail` L55-64.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PTE_Flags {
    pub bits: BitVector<8>,
}

/// Pmpcfg_ent
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L33-41.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Pmpcfg_ent {
    pub bits: BitVector<8>,
}

/// SEnvcfg
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L334-345.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SEnvcfg {
    pub bits: BitVector<{
    64
}>,
}

/// Satp32
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L850-854.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Satp32 {
    pub bits: BitVector<32>,
}

/// Satp64
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L844-848.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Satp64 {
    pub bits: BitVector<64>,
}

/// Sinterrupts
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L746-752.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Sinterrupts {
    pub bits: BitVector<{
    64
}>,
}

/// Sstatus
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L680-694.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Sstatus {
    pub bits: BitVector<64>,
}

/// Vcsr
/// 
/// Generated from the Sail sources at `riscv_vext_regs.sail` L226-229.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Vcsr {
    pub bits: BitVector<3>,
}

/// Vtype
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L923-930.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Vtype {
    pub bits: BitVector<{
    64
}>,
}

/// htif_cmd
/// 
/// Generated from the Sail sources at `riscv_platform.sail` L283-287.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct htif_cmd {
    pub bits: BitVector<64>,
}

/// _get_Misa_A
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_A(v: Misa) -> BitVector<1> {
    v.bits.subrange::<0, 1, 1>()
}

/// _update_Misa_A
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_A(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 0, 0, x)
    }
}

/// _update_Pmpcfg_ent_A
/// 
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_A(v: Pmpcfg_ent, x: BitVector<2>) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 4, 3, x)
    }
}

/// _get_Pmpcfg_ent_A
/// 
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_A(v: Pmpcfg_ent) -> BitVector<2> {
    v.bits.subrange::<3, 5, 2>()
}

/// _get_Misa_B
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_B(v: Misa) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// _update_Misa_B
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_B(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _get_Misa_C
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_C(v: Misa) -> BitVector<1> {
    v.bits.subrange::<2, 3, 1>()
}

/// _update_Misa_C
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_C(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 2, 2, x)
    }
}

/// _get_Misa_D
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_D(v: Misa) -> BitVector<1> {
    v.bits.subrange::<3, 4, 1>()
}

/// _update_Misa_D
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_D(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 3, 3, x)
    }
}

/// _get_Misa_F
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_F(v: Misa) -> BitVector<1> {
    v.bits.subrange::<5, 6, 1>()
}

/// _update_Misa_F
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_F(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _update_Misa_I
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_I(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 8, 8, x)
    }
}

/// _update_Pmpcfg_ent_L
/// 
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_L(v: Pmpcfg_ent, x: BitVector<1>) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _get_Pmpcfg_ent_L
/// 
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_L(v: Pmpcfg_ent) -> BitVector<1> {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_Misa_M
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_M(v: Misa) -> BitVector<1> {
    v.bits.subrange::<12, 13, 1>()
}

/// _update_Misa_M
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_M(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 12, 12, x)
    }
}

/// _get_Misa_MXL
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_MXL(v: Misa) -> BitVector<2> {
    v.bits.subrange::<62, 64, 2>()
}

/// _update_Misa_MXL
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_MXL(v: Misa, x: BitVector<2>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 63, 62, x)
    }
}

/// _update_Pmpcfg_ent_R
/// 
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_R(v: Pmpcfg_ent, x: BitVector<1>) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 0, 0, x)
    }
}

/// _get_Pmpcfg_ent_R
/// 
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_R(v: Pmpcfg_ent) -> BitVector<1> {
    v.bits.subrange::<0, 1, 1>()
}

/// _get_Misa_S
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_S(v: Misa) -> BitVector<1> {
    v.bits.subrange::<18, 19, 1>()
}

/// _update_Misa_S
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_S(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 18, 18, x)
    }
}

/// _get_Misa_U
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_U(v: Misa) -> BitVector<1> {
    v.bits.subrange::<20, 21, 1>()
}

/// _update_Misa_U
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_U(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 20, 20, x)
    }
}

/// _get_Misa_V
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_V(v: Misa) -> BitVector<1> {
    v.bits.subrange::<21, 22, 1>()
}

/// _update_Misa_V
/// 
/// Generated from the Sail sources.
pub fn _update_Misa_V(v: Misa, x: BitVector<1>) -> Misa {
    Misa {
        bits: update_subrange_bits(v.bits, 21, 21, x)
    }
}

/// _update_Pmpcfg_ent_W
/// 
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_W(v: Pmpcfg_ent, x: BitVector<1>) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _get_Pmpcfg_ent_W
/// 
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_W(v: Pmpcfg_ent) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// _update_Pmpcfg_ent_X
/// 
/// Generated from the Sail sources.
pub fn _update_Pmpcfg_ent_X(v: Pmpcfg_ent, x: BitVector<1>) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: update_subrange_bits(v.bits, 2, 2, x)
    }
}

/// _get_Pmpcfg_ent_X
/// 
/// Generated from the Sail sources.
pub fn _get_Pmpcfg_ent_X(v: Pmpcfg_ent) -> BitVector<1> {
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
pub fn sys_writable_hpm_counters(core_ctx: &mut Core, unit_arg: ()) -> BitVector<32> {
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
pub fn legalize_misa(core_ctx: &mut Core, m: Misa, v: BitVector<{
    64
}>) -> Misa {
    let v = Mk_Misa(v);
    if {(!(sys_enable_writable_misa(core_ctx, ())) || ((_get_Misa_C(v) == BitVector::<1>::new(0b0)) && (bitvector_access(core_ctx.nextPC, 1) == true)))} {
        m
    } else {
        {
            let var_1 = {
                let var_3 = {
                    let var_5 = {
                        let var_7 = {
                            let var_9 = {
                                let var_10 = {
                                    let var_12 = {
                                        let var_14 = {
                                            let var_16 = {
                                                let var_18 = if {hartSupports(core_ctx, extension::Ext_A)} {
                                                    _get_Misa_A(v)
                                                } else {
                                                    BitVector::<1>::new(0b0)
                                                };
                                                _update_Misa_A(m, var_18)
                                            };
                                            let var_17 = if {hartSupports(core_ctx, extension::Ext_B)} {
                                                _get_Misa_B(v)
                                            } else {
                                                BitVector::<1>::new(0b0)
                                            };
                                            _update_Misa_B(var_16, var_17)
                                        };
                                        let var_15 = if {hartSupports(core_ctx, extension::Ext_C)} {
                                            _get_Misa_C(v)
                                        } else {
                                            BitVector::<1>::new(0b0)
                                        };
                                        _update_Misa_C(var_14, var_15)
                                    };
                                    let var_13 = if {(hartSupports(core_ctx, extension::Ext_D) && (_get_Misa_F(v) == BitVector::<1>::new(0b1)))} {
                                        _get_Misa_D(v)
                                    } else {
                                        BitVector::<1>::new(0b0)
                                    };
                                    _update_Misa_D(var_12, var_13)
                                };
                                let var_11 = if {hartSupports(core_ctx, extension::Ext_F)} {
                                    _get_Misa_F(v)
                                } else {
                                    BitVector::<1>::new(0b0)
                                };
                                _update_Misa_F(var_10, var_11)
                            };
                            _update_Misa_I(var_9, BitVector::<1>::new(0b1))
                        };
                        let var_8 = if {hartSupports(core_ctx, extension::Ext_M)} {
                            _get_Misa_M(v)
                        } else {
                            BitVector::<1>::new(0b0)
                        };
                        _update_Misa_M(var_7, var_8)
                    };
                    let var_6 = if {(hartSupports(core_ctx, extension::Ext_S) && (_get_Misa_U(v) == BitVector::<1>::new(0b1)))} {
                        _get_Misa_S(v)
                    } else {
                        BitVector::<1>::new(0b0)
                    };
                    _update_Misa_S(var_5, var_6)
                };
                let var_4 = if {hartSupports(core_ctx, extension::Ext_U)} {
                    _get_Misa_U(v)
                } else {
                    BitVector::<1>::new(0b0)
                };
                _update_Misa_U(var_3, var_4)
            };
            let var_2 = if {(hartSupports(core_ctx, extension::Ext_V) && ((_get_Misa_F(v) == BitVector::<1>::new(0b1)) && (_get_Misa_D(v) == BitVector::<1>::new(0b1))))} {
                _get_Misa_V(v)
            } else {
                BitVector::<1>::new(0b0)
            };
            _update_Misa_V(var_1, var_2)
        }
    }
}

/// Mk_Mstatus
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Mstatus(v: BitVector<64>) -> Mstatus {
    Mstatus {
        bits: v
    }
}

/// _update_Mstatus_SXL
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_SXL(v: Mstatus, x: BitVector<2>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 35, 34, x)
    }
}

/// _update_Mstatus_UXL
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_UXL(v: Mstatus, x: BitVector<2>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 33, 32, x)
    }
}

/// _get_Mstatus_FS
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_FS(v: Mstatus) -> BitVector<2> {
    v.bits.subrange::<13, 15, 2>()
}

/// _get_Mstatus_VS
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_VS(v: Mstatus) -> BitVector<2> {
    v.bits.subrange::<9, 11, 2>()
}

/// currentlyEnabled
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L103.
pub fn currentlyEnabled(core_ctx: &mut Core, merge_hashtag_var: extension) -> bool {
    match merge_hashtag_var {
        extension::Ext_Sstc => {hartSupports(core_ctx, extension::Ext_Sstc)}
        extension::Ext_U => {(hartSupports(core_ctx, extension::Ext_U) && ({
            let var_1 = core_ctx.misa;
            _get_Misa_U(var_1)
        } == BitVector::<1>::new(0b1)))}
        extension::Ext_S => {(hartSupports(core_ctx, extension::Ext_S) && ({
            let var_2 = core_ctx.misa;
            _get_Misa_S(var_2)
        } == BitVector::<1>::new(0b1)))}
        extension::Ext_Svbare => {currentlyEnabled(core_ctx, extension::Ext_S)}
        extension::Ext_Sv32 => {(hartSupports(core_ctx, extension::Ext_Sv32) && currentlyEnabled(core_ctx, extension::Ext_S))}
        extension::Ext_Sv39 => {(hartSupports(core_ctx, extension::Ext_Sv39) && currentlyEnabled(core_ctx, extension::Ext_S))}
        extension::Ext_Sv48 => {(hartSupports(core_ctx, extension::Ext_Sv48) && currentlyEnabled(core_ctx, extension::Ext_S))}
        extension::Ext_Sv57 => {(hartSupports(core_ctx, extension::Ext_Sv57) && currentlyEnabled(core_ctx, extension::Ext_S))}
        extension::Ext_V => {(hartSupports(core_ctx, extension::Ext_V) && (({
            let var_4 = core_ctx.misa;
            _get_Misa_V(var_4)
        } == BitVector::<1>::new(0b1)) && ({
            let var_3 = core_ctx.mstatus;
            _get_Mstatus_VS(var_3)
        } != BitVector::<2>::new(0b00))))}
        extension::Ext_Zihpm => {(hartSupports(core_ctx, extension::Ext_Zihpm) && currentlyEnabled(core_ctx, extension::Ext_Zicntr))}
        extension::Ext_Sscofpmf => {(hartSupports(core_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(core_ctx, extension::Ext_Zihpm))}
        extension::Ext_Zkr => {hartSupports(core_ctx, extension::Ext_Zkr)}
        extension::Ext_Zicntr => {hartSupports(core_ctx, extension::Ext_Zicntr)}
        extension::Ext_F => {(hartSupports(core_ctx, extension::Ext_F) && (({
            let var_6 = core_ctx.misa;
            _get_Misa_F(var_6)
        } == BitVector::<1>::new(0b1)) && ({
            let var_5 = core_ctx.mstatus;
            _get_Mstatus_FS(var_5)
        } != BitVector::<2>::new(0b00))))}
        extension::Ext_D => {(hartSupports(core_ctx, extension::Ext_D) && (({
            let var_8 = core_ctx.misa;
            _get_Misa_D(var_8)
        } == BitVector::<1>::new(0b1)) && ({
            let var_7 = core_ctx.mstatus;
            _get_Mstatus_FS(var_7)
        } != BitVector::<2>::new(0b00))))}
        extension::Ext_Zfinx => {hartSupports(core_ctx, extension::Ext_Zfinx)}
        extension::Ext_Smcntrpmf => {(hartSupports(core_ctx, extension::Ext_Smcntrpmf) && currentlyEnabled(core_ctx, extension::Ext_Zicntr))}
        extension::Ext_Svnapot => {false}
        extension::Ext_Svpbmt => {false}
        extension::Ext_C => {(hartSupports(core_ctx, extension::Ext_C) && ({
            let var_9 = core_ctx.misa;
            _get_Misa_C(var_9)
        } == BitVector::<1>::new(0b1)))}
        extension::Ext_Zca => {(hartSupports(core_ctx, extension::Ext_Zca) && (currentlyEnabled(core_ctx, extension::Ext_C) || !(hartSupports(core_ctx, extension::Ext_C))))}
        extension::Ext_Zifencei => {hartSupports(core_ctx, extension::Ext_Zifencei)}
        extension::Ext_A => {(hartSupports(core_ctx, extension::Ext_A) && ({
            let var_10 = core_ctx.misa;
            _get_Misa_A(var_10)
        } == BitVector::<1>::new(0b1)))}
        extension::Ext_Zabha => {(hartSupports(core_ctx, extension::Ext_Zabha) && currentlyEnabled(core_ctx, extension::Ext_Zaamo))}
        extension::Ext_Zalrsc => {(hartSupports(core_ctx, extension::Ext_Zalrsc) || currentlyEnabled(core_ctx, extension::Ext_A))}
        extension::Ext_Zaamo => {(hartSupports(core_ctx, extension::Ext_Zaamo) || currentlyEnabled(core_ctx, extension::Ext_A))}
        extension::Ext_M => {(hartSupports(core_ctx, extension::Ext_M) && ({
            let var_11 = core_ctx.misa;
            _get_Misa_M(var_11)
        } == BitVector::<1>::new(0b1)))}
        extension::Ext_Zmmul => {(hartSupports(core_ctx, extension::Ext_Zmmul) || currentlyEnabled(core_ctx, extension::Ext_M))}
        extension::Ext_Zfh => {(hartSupports(core_ctx, extension::Ext_Zfh) && currentlyEnabled(core_ctx, extension::Ext_F))}
        extension::Ext_Zfhmin => {((hartSupports(core_ctx, extension::Ext_Zfhmin) && currentlyEnabled(core_ctx, extension::Ext_F)) || currentlyEnabled(core_ctx, extension::Ext_Zfh))}
        extension::Ext_Zcf => {(hartSupports(core_ctx, extension::Ext_Zcf) && (currentlyEnabled(core_ctx, extension::Ext_F) && (currentlyEnabled(core_ctx, extension::Ext_Zca) && (currentlyEnabled(core_ctx, extension::Ext_C) || !(hartSupports(core_ctx, extension::Ext_C))))))}
        extension::Ext_Zdinx => {hartSupports(core_ctx, extension::Ext_Zdinx)}
        extension::Ext_Zcd => {(hartSupports(core_ctx, extension::Ext_Zcd) && (currentlyEnabled(core_ctx, extension::Ext_D) && (currentlyEnabled(core_ctx, extension::Ext_Zca) && (currentlyEnabled(core_ctx, extension::Ext_C) || !(hartSupports(core_ctx, extension::Ext_C))))))}
        extension::Ext_Svinval => {hartSupports(core_ctx, extension::Ext_Svinval)}
        extension::Ext_B => {(hartSupports(core_ctx, extension::Ext_B) && ({
            let var_12 = core_ctx.misa;
            _get_Misa_B(var_12)
        } == BitVector::<1>::new(0b1)))}
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
pub fn have_privLevel(core_ctx: &mut Core, _priv_: BitVector<2>) -> bool {
    match _priv_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {currentlyEnabled(core_ctx, extension::Ext_U)}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__2 if {(b__2 == BitVector::<2>::new(0b10))} => {false}
        _ => {true}
        _ => {panic!("Unreachable code")}
    }
}

/// _update_Mstatus_FS
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_FS(v: Mstatus, x: BitVector<2>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 14, 13, x)
    }
}

/// _update_Sstatus_FS
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_FS(v: Sstatus, x: BitVector<2>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 14, 13, x)
    }
}

/// _get_Sstatus_FS
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_FS(v: Sstatus) -> BitVector<2> {
    v.bits.subrange::<13, 15, 2>()
}

/// _get_Mstatus_MIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<3, 4, 1>()
}

/// _update_Mstatus_MIE
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_MIE(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 3, 3, x)
    }
}

/// _get_Mstatus_MPIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<7, 8, 1>()
}

/// _update_Mstatus_MPIE
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_MPIE(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _get_Mstatus_MPP
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPP(v: Mstatus) -> BitVector<2> {
    v.bits.subrange::<11, 13, 2>()
}

/// _update_Mstatus_MPP
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_MPP(v: Mstatus, x: BitVector<2>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 12, 11, x)
    }
}

/// _get_Mstatus_MPRV
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MPRV(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<17, 18, 1>()
}

/// _update_Mstatus_MPRV
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_MPRV(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 17, 17, x)
    }
}

/// _get_Mstatus_MXR
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MXR(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<19, 20, 1>()
}

/// _update_Mstatus_MXR
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_MXR(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 19, 19, x)
    }
}

/// _update_Sstatus_MXR
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_MXR(v: Sstatus, x: BitVector<1>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 19, 19, x)
    }
}

/// _get_Sstatus_MXR
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_MXR(v: Sstatus) -> BitVector<1> {
    v.bits.subrange::<19, 20, 1>()
}

/// _get_Mstatus_SD
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_SD(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<63, 64, 1>()
}

/// _update_Mstatus_SD
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_SD(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 63, 63, x)
    }
}

/// _update_Sstatus_SD
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_SD(v: Sstatus, x: BitVector<1>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 63, 63, x)
    }
}

/// _get_Mstatus_SIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_SIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// _update_Mstatus_SIE
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_SIE(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _update_Sstatus_SIE
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_SIE(v: Sstatus, x: BitVector<1>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _get_Sstatus_SIE
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_SIE(v: Sstatus) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// _get_Mstatus_SPIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_SPIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<5, 6, 1>()
}

/// _update_Mstatus_SPIE
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_SPIE(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _update_Sstatus_SPIE
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_SPIE(v: Sstatus, x: BitVector<1>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _get_Sstatus_SPIE
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_SPIE(v: Sstatus) -> BitVector<1> {
    v.bits.subrange::<5, 6, 1>()
}

/// _get_Mstatus_SPP
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_SPP(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<8, 9, 1>()
}

/// _update_Mstatus_SPP
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_SPP(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 8, 8, x)
    }
}

/// _update_Sstatus_SPP
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_SPP(v: Sstatus, x: BitVector<1>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 8, 8, x)
    }
}

/// _get_Sstatus_SPP
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_SPP(v: Sstatus) -> BitVector<1> {
    v.bits.subrange::<8, 9, 1>()
}

/// _get_Mstatus_SUM
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_SUM(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<18, 19, 1>()
}

/// _update_Mstatus_SUM
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_SUM(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 18, 18, x)
    }
}

/// _update_Sstatus_SUM
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_SUM(v: Sstatus, x: BitVector<1>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 18, 18, x)
    }
}

/// _get_Sstatus_SUM
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_SUM(v: Sstatus) -> BitVector<1> {
    v.bits.subrange::<18, 19, 1>()
}

/// _get_Mstatus_SXL
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_SXL(v: Mstatus) -> BitVector<2> {
    v.bits.subrange::<34, 36, 2>()
}

/// _get_Mstatus_TSR
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_TSR(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<22, 23, 1>()
}

/// _update_Mstatus_TSR
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_TSR(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 22, 22, x)
    }
}

/// _get_Mstatus_TVM
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_TVM(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<20, 21, 1>()
}

/// _update_Mstatus_TVM
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_TVM(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 20, 20, x)
    }
}

/// _get_Mstatus_TW
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_TW(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<21, 22, 1>()
}

/// _update_Mstatus_TW
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_TW(v: Mstatus, x: BitVector<1>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 21, 21, x)
    }
}

/// _get_Mstatus_UXL
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_UXL(v: Mstatus) -> BitVector<2> {
    v.bits.subrange::<32, 34, 2>()
}

/// _update_Sstatus_UXL
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_UXL(v: Sstatus, x: BitVector<2>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 33, 32, x)
    }
}

/// _get_Sstatus_UXL
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_UXL(v: Sstatus) -> BitVector<2> {
    v.bits.subrange::<32, 34, 2>()
}

/// _update_Mstatus_VS
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_VS(v: Mstatus, x: BitVector<2>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 10, 9, x)
    }
}

/// _update_Sstatus_VS
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_VS(v: Sstatus, x: BitVector<2>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 10, 9, x)
    }
}

/// _get_Sstatus_VS
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_VS(v: Sstatus) -> BitVector<2> {
    v.bits.subrange::<9, 11, 2>()
}

/// _get_Mstatus_XS
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_XS(v: Mstatus) -> BitVector<2> {
    v.bits.subrange::<15, 17, 2>()
}

/// _update_Mstatus_XS
/// 
/// Generated from the Sail sources.
pub fn _update_Mstatus_XS(v: Mstatus, x: BitVector<2>) -> Mstatus {
    Mstatus {
        bits: update_subrange_bits(v.bits, 16, 15, x)
    }
}

/// _update_Sstatus_XS
/// 
/// Generated from the Sail sources.
pub fn _update_Sstatus_XS(v: Sstatus, x: BitVector<2>) -> Sstatus {
    Sstatus {
        bits: update_subrange_bits(v.bits, 16, 15, x)
    }
}

/// _get_Sstatus_XS
/// 
/// Generated from the Sail sources.
pub fn _get_Sstatus_XS(v: Sstatus) -> BitVector<2> {
    v.bits.subrange::<15, 17, 2>()
}

/// effectivePrivilege
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L206-209.
pub fn effectivePrivilege(t: AccessType<()>, m: Mstatus, _priv_: Privilege) -> Privilege {
    if {((t != AccessType::InstructionFetch(())) && (_get_Mstatus_MPRV(m) == BitVector::<1>::new(0b1)))} {
        privLevel_of_bits(_get_Mstatus_MPP(m))
    } else {
        _priv_
    }
}

/// get_mstatus_SXL
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L211-215.
pub fn get_mstatus_SXL(m: Mstatus) -> BitVector<2> {
    _get_Mstatus_SXL(m)
}

/// get_mstatus_UXL
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L217-221.
pub fn get_mstatus_UXL(m: Mstatus) -> BitVector<2> {
    _get_Mstatus_UXL(m)
}

/// legalize_mstatus
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L223-273.
pub fn legalize_mstatus(core_ctx: &mut Core, o: Mstatus, v: BitVector<64>) -> Mstatus {
    let v = Mk_Mstatus(v);
    let o = {
        let var_1 = {
            let var_3 = {
                let var_4 = {
                    let var_6 = {
                        let var_7 = {
                            let var_8 = {
                                let var_10 = {
                                    let var_13 = {
                                        let var_15 = {
                                            let var_16 = {
                                                let var_18 = {
                                                    let var_20 = {
                                                        let var_22 = {
                                                            let var_24 = {
                                                                let var_26 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                                                                    _get_Mstatus_TSR(v)
                                                                } else {
                                                                    BitVector::<1>::new(0b0)
                                                                };
                                                                _update_Mstatus_TSR(o, var_26)
                                                            };
                                                            let var_25 = if {currentlyEnabled(core_ctx, extension::Ext_U)} {
                                                                _get_Mstatus_TW(v)
                                                            } else {
                                                                BitVector::<1>::new(0b0)
                                                            };
                                                            _update_Mstatus_TW(var_24, var_25)
                                                        };
                                                        let var_23 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                                                            _get_Mstatus_TVM(v)
                                                        } else {
                                                            BitVector::<1>::new(0b0)
                                                        };
                                                        _update_Mstatus_TVM(var_22, var_23)
                                                    };
                                                    let var_21 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                                                        _get_Mstatus_MXR(v)
                                                    } else {
                                                        BitVector::<1>::new(0b0)
                                                    };
                                                    _update_Mstatus_MXR(var_20, var_21)
                                                };
                                                let var_19 = if {virtual_memory_supported(core_ctx, ())} {
                                                    _get_Mstatus_SUM(v)
                                                } else {
                                                    BitVector::<1>::new(0b0)
                                                };
                                                _update_Mstatus_SUM(var_18, var_19)
                                            };
                                            let var_17 = if {currentlyEnabled(core_ctx, extension::Ext_U)} {
                                                _get_Mstatus_MPRV(v)
                                            } else {
                                                BitVector::<1>::new(0b0)
                                            };
                                            _update_Mstatus_MPRV(var_16, var_17)
                                        };
                                        _update_Mstatus_XS(var_15, extStatus_to_bits(ExtStatus::Off))
                                    };
                                    let var_14 = if {hartSupports(core_ctx, extension::Ext_Zfinx)} {
                                        extStatus_to_bits(ExtStatus::Off)
                                    } else {
                                        _get_Mstatus_FS(v)
                                    };
                                    _update_Mstatus_FS(var_13, var_14)
                                };
                                let var_11 = if {have_privLevel(core_ctx, _get_Mstatus_MPP(v))} {
                                    _get_Mstatus_MPP(v)
                                } else {
                                    {
                                        let var_12 = lowest_supported_privLevel(core_ctx, ());
                                        privLevel_to_bits(var_12)
                                    }
                                };
                                _update_Mstatus_MPP(var_10, var_11)
                            };
                            let var_9 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                                _get_Mstatus_SPP(v)
                            } else {
                                BitVector::<1>::new(0b0)
                            };
                            _update_Mstatus_SPP(var_8, var_9)
                        };
                        _update_Mstatus_VS(var_7, _get_Mstatus_VS(v))
                    };
                    _update_Mstatus_MPIE(var_6, _get_Mstatus_MPIE(v))
                };
                let var_5 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                    _get_Mstatus_SPIE(v)
                } else {
                    BitVector::<1>::new(0b0)
                };
                _update_Mstatus_SPIE(var_4, var_5)
            };
            _update_Mstatus_MIE(var_3, _get_Mstatus_MIE(v))
        };
        let var_2 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
            _get_Mstatus_SIE(v)
        } else {
            BitVector::<1>::new(0b0)
        };
        _update_Mstatus_SIE(var_1, var_2)
    };
    let dirty = ((extStatus_of_bits(_get_Mstatus_FS(o)) == ExtStatus::Dirty) || ((extStatus_of_bits(_get_Mstatus_XS(o)) == ExtStatus::Dirty) || (extStatus_of_bits(_get_Mstatus_VS(o)) == ExtStatus::Dirty)));
    _update_Mstatus_SD(o, bool_to_bits(dirty))
}

/// cur_architecture
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L300-308.
pub fn cur_architecture(core_ctx: &mut Core, unit_arg: ()) -> Architecture {
    let a: arch_xlen = match core_ctx.cur_privilege {
        Privilege::Machine => {{
            let var_1 = core_ctx.misa;
            _get_Misa_MXL(var_1)
        }}
        Privilege::Supervisor => {{
            let var_2 = core_ctx.mstatus;
            get_mstatus_SXL(var_2)
        }}
        Privilege::User => {{
            let var_3 = core_ctx.mstatus;
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
pub fn Mk_MEnvcfg(v: BitVector<64>) -> MEnvcfg {
    MEnvcfg {
        bits: v
    }
}

/// _get_MEnvcfg_CBCFE
/// 
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_CBCFE(v: MEnvcfg) -> BitVector<1> {
    v.bits.subrange::<6, 7, 1>()
}

/// _update_MEnvcfg_CBCFE
/// 
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_CBCFE(v: MEnvcfg, x: BitVector<1>) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 6, 6, x)
    }
}

/// _update_SEnvcfg_CBCFE
/// 
/// Generated from the Sail sources.
pub fn _update_SEnvcfg_CBCFE(v: SEnvcfg, x: BitVector<1>) -> SEnvcfg {
    SEnvcfg {
        bits: update_subrange_bits(v.bits, 6, 6, x)
    }
}

/// _get_SEnvcfg_CBCFE
/// 
/// Generated from the Sail sources.
pub fn _get_SEnvcfg_CBCFE(v: SEnvcfg) -> BitVector<1> {
    v.bits.subrange::<6, 7, 1>()
}

/// _get_MEnvcfg_CBIE
/// 
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_CBIE(v: MEnvcfg) -> BitVector<2> {
    v.bits.subrange::<4, 6, 2>()
}

/// _update_MEnvcfg_CBIE
/// 
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_CBIE(v: MEnvcfg, x: BitVector<2>) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 5, 4, x)
    }
}

/// _update_SEnvcfg_CBIE
/// 
/// Generated from the Sail sources.
pub fn _update_SEnvcfg_CBIE(v: SEnvcfg, x: BitVector<2>) -> SEnvcfg {
    SEnvcfg {
        bits: update_subrange_bits(v.bits, 5, 4, x)
    }
}

/// _get_SEnvcfg_CBIE
/// 
/// Generated from the Sail sources.
pub fn _get_SEnvcfg_CBIE(v: SEnvcfg) -> BitVector<2> {
    v.bits.subrange::<4, 6, 2>()
}

/// _get_MEnvcfg_CBZE
/// 
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_CBZE(v: MEnvcfg) -> BitVector<1> {
    v.bits.subrange::<7, 8, 1>()
}

/// _update_MEnvcfg_CBZE
/// 
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_CBZE(v: MEnvcfg, x: BitVector<1>) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _update_SEnvcfg_CBZE
/// 
/// Generated from the Sail sources.
pub fn _update_SEnvcfg_CBZE(v: SEnvcfg, x: BitVector<1>) -> SEnvcfg {
    SEnvcfg {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _get_SEnvcfg_CBZE
/// 
/// Generated from the Sail sources.
pub fn _get_SEnvcfg_CBZE(v: SEnvcfg) -> BitVector<1> {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_MEnvcfg_FIOM
/// 
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_FIOM(v: MEnvcfg) -> BitVector<1> {
    v.bits.subrange::<0, 1, 1>()
}

/// _update_MEnvcfg_FIOM
/// 
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_FIOM(v: MEnvcfg, x: BitVector<1>) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 0, 0, x)
    }
}

/// _update_SEnvcfg_FIOM
/// 
/// Generated from the Sail sources.
pub fn _update_SEnvcfg_FIOM(v: SEnvcfg, x: BitVector<1>) -> SEnvcfg {
    SEnvcfg {
        bits: update_subrange_bits(v.bits, 0, 0, x)
    }
}

/// _get_SEnvcfg_FIOM
/// 
/// Generated from the Sail sources.
pub fn _get_SEnvcfg_FIOM(v: SEnvcfg) -> BitVector<1> {
    v.bits.subrange::<0, 1, 1>()
}

/// _get_MEnvcfg_STCE
/// 
/// Generated from the Sail sources.
pub fn _get_MEnvcfg_STCE(v: MEnvcfg) -> BitVector<1> {
    v.bits.subrange::<63, 64, 1>()
}

/// _update_MEnvcfg_STCE
/// 
/// Generated from the Sail sources.
pub fn _update_MEnvcfg_STCE(v: MEnvcfg, x: BitVector<1>) -> MEnvcfg {
    MEnvcfg {
        bits: update_subrange_bits(v.bits, 63, 63, x)
    }
}

/// Mk_SEnvcfg
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_SEnvcfg(v: BitVector<64>) -> SEnvcfg {
    SEnvcfg {
        bits: v
    }
}

/// legalize_menvcfg
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L347-357.
pub fn legalize_menvcfg(core_ctx: &mut Core, o: MEnvcfg, v: BitVector<64>) -> MEnvcfg {
    let v = Mk_MEnvcfg(v);
    {
        let var_1 = {
            let var_3 = {
                let var_5 = {
                    let var_7 = {
                        let var_9 = if {sys_enable_writable_fiom(core_ctx, ())} {
                            _get_MEnvcfg_FIOM(v)
                        } else {
                            BitVector::<1>::new(0b0)
                        };
                        _update_MEnvcfg_FIOM(o, var_9)
                    };
                    let var_8 = if {currentlyEnabled(core_ctx, extension::Ext_Zicboz)} {
                        _get_MEnvcfg_CBZE(v)
                    } else {
                        BitVector::<1>::new(0b0)
                    };
                    _update_MEnvcfg_CBZE(var_7, var_8)
                };
                let var_6 = if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} {
                    _get_MEnvcfg_CBCFE(v)
                } else {
                    BitVector::<1>::new(0b0)
                };
                _update_MEnvcfg_CBCFE(var_5, var_6)
            };
            let var_4 = if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} {
                if {(_get_MEnvcfg_CBIE(v) != BitVector::<2>::new(0b10))} {
                    _get_MEnvcfg_CBIE(v)
                } else {
                    BitVector::<2>::new(0b00)
                }
            } else {
                BitVector::<2>::new(0b00)
            };
            _update_MEnvcfg_CBIE(var_3, var_4)
        };
        let var_2 = if {currentlyEnabled(core_ctx, extension::Ext_Sstc)} {
            _get_MEnvcfg_STCE(v)
        } else {
            BitVector::<1>::new(0b0)
        };
        _update_MEnvcfg_STCE(var_1, var_2)
    }
}

/// legalize_senvcfg
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L359-368.
pub fn legalize_senvcfg(core_ctx: &mut Core, o: SEnvcfg, v: BitVector<{
    64
}>) -> SEnvcfg {
    let v = Mk_SEnvcfg(v);
    {
        let var_1 = {
            let var_3 = {
                let var_5 = {
                    let var_7 = if {sys_enable_writable_fiom(core_ctx, ())} {
                        _get_SEnvcfg_FIOM(v)
                    } else {
                        BitVector::<1>::new(0b0)
                    };
                    _update_SEnvcfg_FIOM(o, var_7)
                };
                let var_6 = if {currentlyEnabled(core_ctx, extension::Ext_Zicboz)} {
                    _get_SEnvcfg_CBZE(v)
                } else {
                    BitVector::<1>::new(0b0)
                };
                _update_SEnvcfg_CBZE(var_5, var_6)
            };
            let var_4 = if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} {
                _get_SEnvcfg_CBCFE(v)
            } else {
                BitVector::<1>::new(0b0)
            };
            _update_SEnvcfg_CBCFE(var_3, var_4)
        };
        let var_2 = if {currentlyEnabled(core_ctx, extension::Ext_Zicbom)} {
            if {(_get_SEnvcfg_CBIE(v) != BitVector::<2>::new(0b10))} {
                _get_SEnvcfg_CBIE(v)
            } else {
                BitVector::<2>::new(0b00)
            }
        } else {
            BitVector::<2>::new(0b00)
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
            let var_1 = core_ctx.menvcfg;
            _get_MEnvcfg_FIOM(var_1)
        } == BitVector::<1>::new(0b1))}
        Privilege::User => {(({
            let var_3 = core_ctx.menvcfg;
            _get_MEnvcfg_FIOM(var_3)
        } | {
            let var_2 = core_ctx.senvcfg;
            _get_SEnvcfg_FIOM(var_2)
        }) == BitVector::<1>::new(0b1))}
        _ => {panic!("Unreachable code")}
    }
}

/// Mk_Minterrupts
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Minterrupts(v: BitVector<64>) -> Minterrupts {
    Minterrupts {
        bits: v
    }
}

/// _get_Minterrupts_MEI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_MEI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<11, 12, 1>()
}

/// _update_Minterrupts_MEI
/// 
/// Generated from the Sail sources.
pub fn _update_Minterrupts_MEI(v: Minterrupts, x: BitVector<1>) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 11, 11, x)
    }
}

/// _get_Minterrupts_MSI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_MSI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<3, 4, 1>()
}

/// _update_Minterrupts_MSI
/// 
/// Generated from the Sail sources.
pub fn _update_Minterrupts_MSI(v: Minterrupts, x: BitVector<1>) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 3, 3, x)
    }
}

/// _get_Minterrupts_MTI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_MTI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<7, 8, 1>()
}

/// _update_Minterrupts_MTI
/// 
/// Generated from the Sail sources.
pub fn _update_Minterrupts_MTI(v: Minterrupts, x: BitVector<1>) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 7, 7, x)
    }
}

/// _get_Minterrupts_SEI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_SEI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<9, 10, 1>()
}

/// _update_Minterrupts_SEI
/// 
/// Generated from the Sail sources.
pub fn _update_Minterrupts_SEI(v: Minterrupts, x: BitVector<1>) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 9, 9, x)
    }
}

/// _update_Sinterrupts_SEI
/// 
/// Generated from the Sail sources.
pub fn _update_Sinterrupts_SEI(v: Sinterrupts, x: BitVector<1>) -> Sinterrupts {
    Sinterrupts {
        bits: update_subrange_bits(v.bits, 9, 9, x)
    }
}

/// _get_Sinterrupts_SEI
/// 
/// Generated from the Sail sources.
pub fn _get_Sinterrupts_SEI(v: Sinterrupts) -> BitVector<1> {
    v.bits.subrange::<9, 10, 1>()
}

/// _get_Minterrupts_SSI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_SSI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// _update_Minterrupts_SSI
/// 
/// Generated from the Sail sources.
pub fn _update_Minterrupts_SSI(v: Minterrupts, x: BitVector<1>) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _update_Sinterrupts_SSI
/// 
/// Generated from the Sail sources.
pub fn _update_Sinterrupts_SSI(v: Sinterrupts, x: BitVector<1>) -> Sinterrupts {
    Sinterrupts {
        bits: update_subrange_bits(v.bits, 1, 1, x)
    }
}

/// _get_Sinterrupts_SSI
/// 
/// Generated from the Sail sources.
pub fn _get_Sinterrupts_SSI(v: Sinterrupts) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// _get_Minterrupts_STI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_STI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<5, 6, 1>()
}

/// _update_Minterrupts_STI
/// 
/// Generated from the Sail sources.
pub fn _update_Minterrupts_STI(v: Minterrupts, x: BitVector<1>) -> Minterrupts {
    Minterrupts {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _update_Sinterrupts_STI
/// 
/// Generated from the Sail sources.
pub fn _update_Sinterrupts_STI(v: Sinterrupts, x: BitVector<1>) -> Sinterrupts {
    Sinterrupts {
        bits: update_subrange_bits(v.bits, 5, 5, x)
    }
}

/// _get_Sinterrupts_STI
/// 
/// Generated from the Sail sources.
pub fn _get_Sinterrupts_STI(v: Sinterrupts) -> BitVector<1> {
    v.bits.subrange::<5, 6, 1>()
}

/// legalize_mip
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L415-427.
pub fn legalize_mip(core_ctx: &mut Core, o: Minterrupts, v: BitVector<{
    64
}>) -> Minterrupts {
    let v = Mk_Minterrupts(v);
    {
        let var_1 = {
            let var_4 = {
                let var_6 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                    _get_Minterrupts_SEI(v)
                } else {
                    BitVector::<1>::new(0b0)
                };
                _update_Minterrupts_SEI(o, var_6)
            };
            let var_5 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                _get_Minterrupts_SSI(v)
            } else {
                BitVector::<1>::new(0b0)
            };
            _update_Minterrupts_SSI(var_4, var_5)
        };
        let var_2 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
            if {(currentlyEnabled(core_ctx, extension::Ext_Sstc) && ({
                let var_3 = core_ctx.menvcfg;
                _get_MEnvcfg_STCE(var_3)
            } == BitVector::<1>::new(0b1)))} {
                _get_Minterrupts_STI(o)
            } else {
                _get_Minterrupts_STI(v)
            }
        } else {
            BitVector::<1>::new(0b0)
        };
        _update_Minterrupts_STI(var_1, var_2)
    }
}

/// legalize_mie
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L429-439.
pub fn legalize_mie(core_ctx: &mut Core, o: Minterrupts, v: BitVector<{
    64
}>) -> Minterrupts {
    let v = Mk_Minterrupts(v);
    {
        let var_1 = {
            let var_3 = {
                let var_5 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                    _get_Minterrupts_SEI(v)
                } else {
                    BitVector::<1>::new(0b0)
                };
                _update_Minterrupts_SEI(_update_Minterrupts_MSI(_update_Minterrupts_MTI(_update_Minterrupts_MEI(o, _get_Minterrupts_MEI(v)), _get_Minterrupts_MTI(v)), _get_Minterrupts_MSI(v)), var_5)
            };
            let var_4 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                _get_Minterrupts_STI(v)
            } else {
                BitVector::<1>::new(0b0)
            };
            _update_Minterrupts_STI(var_3, var_4)
        };
        let var_2 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
            _get_Minterrupts_SSI(v)
        } else {
            BitVector::<1>::new(0b0)
        };
        _update_Minterrupts_SSI(var_1, var_2)
    }
}

/// legalize_mideleg
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L441-445.
pub fn legalize_mideleg(o: Minterrupts, v: BitVector<{
    64
}>) -> Minterrupts {
    _update_Minterrupts_MSI(_update_Minterrupts_MTI(_update_Minterrupts_MEI(Mk_Minterrupts(v), BitVector::<1>::new(0b0)), BitVector::<1>::new(0b0)), BitVector::<1>::new(0b0))
}

/// Mk_Medeleg
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Medeleg(v: BitVector<64>) -> Medeleg {
    Medeleg {
        bits: v
    }
}

/// _update_Medeleg_MEnvCall
/// 
/// Generated from the Sail sources.
pub fn _update_Medeleg_MEnvCall(v: Medeleg, x: BitVector<1>) -> Medeleg {
    Medeleg {
        bits: update_subrange_bits(v.bits, 11, 11, x)
    }
}

/// legalize_medeleg
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L466-469.
pub fn legalize_medeleg(o: Medeleg, v: BitVector<64>) -> Medeleg {
    _update_Medeleg_MEnvCall(Mk_Medeleg(v), BitVector::<1>::new(0b0))
}

/// Mk_Mtvec
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Mtvec(v: BitVector<64>) -> Mtvec {
    Mtvec {
        bits: v
    }
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

/// _update_Mtvec_Mode
/// 
/// Generated from the Sail sources.
pub fn _update_Mtvec_Mode(v: Mtvec, x: BitVector<2>) -> Mtvec {
    Mtvec {
        bits: update_subrange_bits(v.bits, 1, 0, x)
    }
}

/// _get_Satp32_Mode
/// 
/// Generated from the Sail sources.
pub fn _get_Satp32_Mode(v: Satp32) -> BitVector<1> {
    v.bits.subrange::<31, 32, 1>()
}

/// _get_Satp64_Mode
/// 
/// Generated from the Sail sources.
pub fn _get_Satp64_Mode(v: Satp64) -> BitVector<4> {
    v.bits.subrange::<60, 64, 4>()
}

/// legalize_tvec
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L509-516.
pub fn legalize_tvec(o: Mtvec, v: BitVector<{
    64
}>) -> Mtvec {
    let v = Mk_Mtvec(v);
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
pub fn _get_Mcause_Cause(v: Mcause) -> BitVector<63> {
    v.bits.subrange::<0, 63, 63>()
}

/// _get_Mcause_IsInterrupt
/// 
/// Generated from the Sail sources.
pub fn _get_Mcause_IsInterrupt(v: Mcause) -> BitVector<1> {
    v.bits.subrange::<63, 64, 1>()
}

/// tvec_addr
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L529-538.
pub fn tvec_addr(m: Mtvec, c: Mcause) -> Option<BitVector<{
    64
}>> {
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

/// legalize_xepc
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L547-552.
pub fn legalize_xepc(core_ctx: &mut Core, v: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    if {hartSupports(core_ctx, extension::Ext_C)} {
        bitvector_update(v, 0, false)
    } else {
        update_subrange_bits(v, 1, 0, zeros::<2>(2))
    }
}

/// align_pc
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L556-560.
pub fn align_pc(core_ctx: &mut Core, addr: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    if {({
        let var_1 = core_ctx.misa;
        _get_Misa_C(var_1)
    } == BitVector::<1>::new(0b1))} {
        bitvector_update(addr, 0, false)
    } else {
        update_subrange_bits(addr, 1, 0, zeros::<2>(2))
    }
}

/// Mk_Counteren
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Counteren(v: BitVector<32>) -> Counteren {
    Counteren {
        bits: v
    }
}

/// _get_Counteren_TM
/// 
/// Generated from the Sail sources.
pub fn _get_Counteren_TM(v: Counteren) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// legalize_scounteren
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L589-592.
pub fn legalize_scounteren(core_ctx: &mut Core, c: Counteren, v: BitVector<{
    64
}>) -> Counteren {
    let supported_counters = bitvector_concat::<29, 3, 32>(subrange_bits::<_, 29>(sys_writable_hpm_counters(core_ctx, ()), 31, 3), BitVector::<3>::new(0b111));
    Mk_Counteren((v.subrange::<0, 32, 32>() & supported_counters))
}

/// legalize_mcounteren
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L601-604.
pub fn legalize_mcounteren(core_ctx: &mut Core, c: Counteren, v: BitVector<{
    64
}>) -> Counteren {
    let supported_counters = bitvector_concat::<29, 3, 32>(subrange_bits::<_, 29>(sys_writable_hpm_counters(core_ctx, ()), 31, 3), BitVector::<3>::new(0b111));
    Mk_Counteren((v.subrange::<0, 32, 32>() & supported_counters))
}

/// Mk_Counterin
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Counterin(v: BitVector<32>) -> Counterin {
    Counterin {
        bits: v
    }
}

/// legalize_mcountinhibit
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L620-624.
pub fn legalize_mcountinhibit(core_ctx: &mut Core, c: Counterin, v: BitVector<{
    64
}>) -> Counterin {
    let supported_counters = bitvector_concat::<29, 3, 32>(subrange_bits::<_, 29>(sys_writable_hpm_counters(core_ctx, ()), 31, 3), BitVector::<3>::new(0b101));
    Mk_Counterin((v.subrange::<0, 32, 32>() & supported_counters))
}

/// Mk_Sstatus
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Sstatus(v: BitVector<64>) -> Sstatus {
    Sstatus {
        bits: v
    }
}

/// lower_mstatus
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L697-714.
pub fn lower_mstatus(m: Mstatus) -> Sstatus {
    let s = Mk_Sstatus(zeros::<64>(64));
    _update_Sstatus_SIE(_update_Sstatus_SPIE(_update_Sstatus_SPP(_update_Sstatus_VS(_update_Sstatus_FS(_update_Sstatus_XS(_update_Sstatus_SUM(_update_Sstatus_MXR(_update_Sstatus_UXL(_update_Sstatus_SD(s, _get_Mstatus_SD(m)), _get_Mstatus_UXL(m)), _get_Mstatus_MXR(m)), _get_Mstatus_SUM(m)), _get_Mstatus_XS(m)), _get_Mstatus_FS(m)), _get_Mstatus_VS(m)), _get_Mstatus_SPP(m)), _get_Mstatus_SPIE(m)), _get_Mstatus_SIE(m))
}

/// lift_sstatus
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L716-734.
pub fn lift_sstatus(m: Mstatus, s: Sstatus) -> Mstatus {
    let dirty = ((extStatus_of_bits(_get_Sstatus_FS(s)) == ExtStatus::Dirty) || ((extStatus_of_bits(_get_Sstatus_XS(s)) == ExtStatus::Dirty) || (extStatus_of_bits(_get_Sstatus_VS(s)) == ExtStatus::Dirty)));
    _update_Mstatus_SIE(_update_Mstatus_SPIE(_update_Mstatus_SPP(_update_Mstatus_VS(_update_Mstatus_FS(_update_Mstatus_XS(_update_Mstatus_SUM(_update_Mstatus_MXR(_update_Mstatus_UXL(_update_Mstatus_SD(m, bool_to_bits(dirty)), _get_Sstatus_UXL(s)), _get_Sstatus_MXR(s)), _get_Sstatus_SUM(s)), _get_Sstatus_XS(s)), _get_Sstatus_FS(s)), _get_Sstatus_VS(s)), _get_Sstatus_SPP(s)), _get_Sstatus_SPIE(s)), _get_Sstatus_SIE(s))
}

/// legalize_sstatus
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L736-738.
pub fn legalize_sstatus(core_ctx: &mut Core, m: Mstatus, v: BitVector<{
    64
}>) -> Mstatus {
    legalize_mstatus(core_ctx, m, lift_sstatus(m, Mk_Sstatus(v.zero_extend::<64>())).bits)
}

/// Mk_Sinterrupts
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Sinterrupts(v: BitVector<64>) -> Sinterrupts {
    Sinterrupts {
        bits: v
    }
}

/// lower_mip
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L756-764.
pub fn lower_mip(m: Minterrupts, d: Minterrupts) -> Sinterrupts {
    let s: Sinterrupts = Mk_Sinterrupts(zeros::<64>(64));
    _update_Sinterrupts_SSI(_update_Sinterrupts_STI(_update_Sinterrupts_SEI(s, (_get_Minterrupts_SEI(m) & _get_Minterrupts_SEI(d))), (_get_Minterrupts_STI(m) & _get_Minterrupts_STI(d))), (_get_Minterrupts_SSI(m) & _get_Minterrupts_SSI(d)))
}

/// lower_mie
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L767-775.
pub fn lower_mie(m: Minterrupts, d: Minterrupts) -> Sinterrupts {
    let s: Sinterrupts = Mk_Sinterrupts(zeros::<64>(64));
    _update_Sinterrupts_SSI(_update_Sinterrupts_STI(_update_Sinterrupts_SEI(s, (_get_Minterrupts_SEI(m) & _get_Minterrupts_SEI(d))), (_get_Minterrupts_STI(m) & _get_Minterrupts_STI(d))), (_get_Minterrupts_SSI(m) & _get_Minterrupts_SSI(d)))
}

/// lift_sip
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L778-782.
pub fn lift_sip(o: Minterrupts, d: Minterrupts, s: Sinterrupts) -> Minterrupts {
    let m: Minterrupts = o;
    let m = if {(_get_Minterrupts_SSI(d) == BitVector::<1>::new(0b1))} {
        _update_Minterrupts_SSI(m, _get_Sinterrupts_SSI(s))
    } else {
        m
    };
    m
}

/// legalize_sip
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L784-786.
pub fn legalize_sip(m: Minterrupts, d: Minterrupts, v: BitVector<{
    64
}>) -> Minterrupts {
    lift_sip(m, d, Mk_Sinterrupts(v))
}

/// lift_sie
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L796-803.
pub fn lift_sie(o: Minterrupts, d: Minterrupts, s: Sinterrupts) -> Minterrupts {
    let m: Minterrupts = o;
    {
        let var_1 = {
            let var_3 = {
                let var_5 = if {(_get_Minterrupts_SEI(d) == BitVector::<1>::new(0b1))} {
                    _get_Sinterrupts_SEI(s)
                } else {
                    _get_Minterrupts_SEI(m)
                };
                _update_Minterrupts_SEI(m, var_5)
            };
            let var_4 = if {(_get_Minterrupts_STI(d) == BitVector::<1>::new(0b1))} {
                _get_Sinterrupts_STI(s)
            } else {
                _get_Minterrupts_STI(m)
            };
            _update_Minterrupts_STI(var_3, var_4)
        };
        let var_2 = if {(_get_Minterrupts_SSI(d) == BitVector::<1>::new(0b1))} {
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
pub fn legalize_sie(m: Minterrupts, d: Minterrupts, v: BitVector<{
    64
}>) -> Minterrupts {
    lift_sie(m, d, Mk_Sinterrupts(v))
}

/// Mk_Satp64
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Satp64(v: BitVector<64>) -> Satp64 {
    Satp64 {
        bits: v
    }
}

/// Mk_Satp32
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Satp32(v: BitVector<32>) -> Satp32 {
    Satp32 {
        bits: v
    }
}

/// legalize_satp
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L858-888.
pub fn legalize_satp(core_ctx: &mut Core, arch: Architecture, prev_value: BitVector<{
    64
}>, written_value: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    let s = Mk_Satp64(written_value);
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
pub fn get_vlenb(unit_arg: ()) -> BitVector<{
    64
}> {
    to_bits::<64>(64, (8 / 8))
}

/// _get_Vtype_vlmul
/// 
/// Generated from the Sail sources.
pub fn _get_Vtype_vlmul(v: Vtype) -> BitVector<3> {
    v.bits.subrange::<0, 3, 3>()
}

/// _get_Vtype_vsew
/// 
/// Generated from the Sail sources.
pub fn _get_Vtype_vsew(v: Vtype) -> BitVector<3> {
    v.bits.subrange::<3, 6, 3>()
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
pub fn pmpAddrMatchType_of_bits(bs: BitVector<2>) -> PmpAddrMatchType {
    match bs {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {PmpAddrMatchType::OFF}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {PmpAddrMatchType::TOR}
        b__2 if {(b__2 == BitVector::<2>::new(0b10))} => {PmpAddrMatchType::NA4}
        _ => {PmpAddrMatchType::NAPOT}
        _ => {panic!("Unreachable code")}
    }
}

/// pmpAddrMatchType_to_bits
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L24-31.
pub fn pmpAddrMatchType_to_bits(bs: PmpAddrMatchType) -> BitVector<2> {
    match bs {
        PmpAddrMatchType::OFF => {BitVector::<2>::new(0b00)}
        PmpAddrMatchType::TOR => {BitVector::<2>::new(0b01)}
        PmpAddrMatchType::NA4 => {BitVector::<2>::new(0b10)}
        PmpAddrMatchType::NAPOT => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// Mk_Pmpcfg_ent
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_Pmpcfg_ent(v: BitVector<8>) -> Pmpcfg_ent {
    Pmpcfg_ent {
        bits: v
    }
}

/// pmpReadCfgReg
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L48-67.
pub fn pmpReadCfgReg(core_ctx: &mut Core, n: i128) -> BitVector<{
    64
}> {
    assert!((((n as usize) % (2 as usize)) == 0), "Unexpected pmp config reg read");
    bitvector_concat::<8, 56, 64>(core_ctx.pmpcfg_n[(((n * 4) + 7) as usize)].bits, bitvector_concat::<8, 48, 56>(core_ctx.pmpcfg_n[(((n * 4) + 6) as usize)].bits, bitvector_concat::<8, 40, 48>(core_ctx.pmpcfg_n[(((n * 4) + 5) as usize)].bits, bitvector_concat::<8, 32, 40>(core_ctx.pmpcfg_n[(((n * 4) + 4) as usize)].bits, bitvector_concat::<8, 24, 32>(core_ctx.pmpcfg_n[(((n * 4) + 3) as usize)].bits, bitvector_concat::<8, 16, 24>(core_ctx.pmpcfg_n[(((n * 4) + 2) as usize)].bits, bitvector_concat::<8, 8, 16>(core_ctx.pmpcfg_n[(((n * 4) + 1) as usize)].bits, core_ctx.pmpcfg_n[(((n * 4) + 0) as usize)].bits)))))))
}

/// pmpReadAddrReg
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L69-89.
pub fn pmpReadAddrReg(core_ctx: &mut Core, n: i128) -> BitVector<{
    64
}> {
    let G = sys_pmp_grain(core_ctx, ());
    let match_type = _get_Pmpcfg_ent_A(core_ctx.pmpcfg_n[(n as usize)]);
    let addr = core_ctx.pmpaddr_n[(n as usize)];
    match bitvector_access(match_type, 1) {
        true if {(G >= 2)} => {{
            let mask: xlenbits = ones::<64>(min_int((G - 1), 64)).zero_extend::<64>();
            (addr | mask)
        }}
        false if {(G >= 1)} => {{
            let mask: xlenbits = ones::<64>(min_int(G, 64)).zero_extend::<64>();
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
    (_get_Pmpcfg_ent_L(cfg) == BitVector::<1>::new(0b1))
}

/// pmpTORLocked
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L95-96.
pub fn pmpTORLocked(cfg: Pmpcfg_ent) -> bool {
    ((_get_Pmpcfg_ent_L(cfg) == BitVector::<1>::new(0b1)) && (pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(cfg)) == PmpAddrMatchType::TOR))
}

/// pmpWriteCfg
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L98-117.
pub fn pmpWriteCfg(core_ctx: &mut Core, n: i128, cfg: Pmpcfg_ent, v: BitVector<8>) -> Pmpcfg_ent {
    if {pmpLocked(cfg)} {
        cfg
    } else {
        let cfg = Mk_Pmpcfg_ent((v & BitVector::<8>::new(0b10011111)));
        let cfg = if {((_get_Pmpcfg_ent_W(cfg) == BitVector::<1>::new(0b1)) && (_get_Pmpcfg_ent_R(cfg) == BitVector::<1>::new(0b0)))} {
            _update_Pmpcfg_ent_R(_update_Pmpcfg_ent_W(_update_Pmpcfg_ent_X(cfg, BitVector::<1>::new(0b0)), BitVector::<1>::new(0b0)), BitVector::<1>::new(0b0))
        } else {
            cfg
        };
        let cfg = if {((sys_pmp_grain(core_ctx, ()) >= 1) && (pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(cfg)) == PmpAddrMatchType::NA4))} {
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
pub fn pmpWriteCfgReg(core_ctx: &mut Core, n: i128, v: BitVector<{
    64
}>) {
    assert!((((n as usize) % (2 as usize)) == 0), "Unexpected pmp config reg write");
    for i in 0..=7 {
        let idx = ((n * 4) + i);
        core_ctx.pmpcfg_n[(idx as usize)] = pmpWriteCfg(core_ctx, idx, core_ctx.pmpcfg_n[(idx as usize)], subrange_bits(v, ((8 * i) + 7), (8 * i)))
    }
}

/// pmpWriteAddr
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L136-139.
pub fn pmpWriteAddr(locked: bool, tor_locked: bool, reg: BitVector<{
    64
}>, v: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    if {(locked || tor_locked)} {
        reg
    } else {
        v.subrange::<0, 54, 54>().zero_extend::<64>()
    }
}

/// pmpWriteAddrReg
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L141-148.
pub fn pmpWriteAddrReg(core_ctx: &mut Core, n: i128, v: BitVector<{
    64
}>) {
    core_ctx.pmpaddr_n[(n as usize)] = {
        let var_1 = if {((n + 1) < 64)} {
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
pub fn pmpCheckRWX(ent: Pmpcfg_ent, acc: AccessType<()>) -> bool {
    match acc {
        AccessType::Read(_) => {(_get_Pmpcfg_ent_R(ent) == BitVector::<1>::new(0b1))}
        AccessType::Write(_) => {(_get_Pmpcfg_ent_W(ent) == BitVector::<1>::new(0b1))}
        AccessType::ReadWrite(_) => {((_get_Pmpcfg_ent_R(ent) == BitVector::<1>::new(0b1)) && (_get_Pmpcfg_ent_W(ent) == BitVector::<1>::new(0b1)))}
        AccessType::InstructionFetch(()) => {(_get_Pmpcfg_ent_X(ent) == BitVector::<1>::new(0b1))}
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
pub fn pmpMatchAddr(core_ctx: &mut Core, physaddr::Physaddr(addr): physaddr, width: BitVector<{
    64
}>, ent: Pmpcfg_ent, pmpaddr: BitVector<{
    64
}>, prev_pmpaddr: BitVector<{
    64
}>) -> pmpAddrMatch {
    let addr = addr.unsigned();
    let width = width.unsigned();
    match pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(ent)) {
        PmpAddrMatchType::OFF => {pmpAddrMatch::PMP_NoMatch}
        PmpAddrMatchType::TOR => {{
            if {_operator_biggerequal_u_(prev_pmpaddr, pmpaddr)} {
                pmpAddrMatch::PMP_NoMatch
            } else {
                pmpRangeMatch((((prev_pmpaddr.unsigned() as i128) * (4 as i128)) as u128), (((pmpaddr.unsigned() as i128) * (4 as i128)) as u128), (addr as u128), (width as u128))
            }
        }}
        PmpAddrMatchType::NA4 => {{
            assert!((sys_pmp_grain(core_ctx, ()) < 1), "NA4 cannot be selected when PMP grain G >= 1.");
            let begin = ((pmpaddr.unsigned() as i128) * (4 as i128));
            pmpRangeMatch((begin as u128), ((begin + 4) as u128), (addr as u128), (width as u128))
        }}
        PmpAddrMatchType::NAPOT => {{
            let mask = (pmpaddr ^ (pmpaddr + 1));
            let begin_words = (pmpaddr & !(mask)).unsigned();
            let end_words = ((begin_words + mask.unsigned()) + 1);
            pmpRangeMatch(((begin_words * 4) as u128), ((end_words * 4) as u128), (addr as u128), (width as u128))
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
pub fn pmpMatchEntry(core_ctx: &mut Core, addr: physaddr, width: BitVector<{
    64
}>, acc: AccessType<()>, _priv_: Privilege, ent: Pmpcfg_ent, pmpaddr: BitVector<{
    64
}>, prev_pmpaddr: BitVector<{
    64
}>) -> pmpMatch {
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
pub fn accessToFault(acc: AccessType<()>) -> ExceptionType {
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
pub fn pmpCheck(core_ctx: &mut Core, addr: physaddr, width: i128, acc: AccessType<()>, _priv_: Privilege) -> Option<ExceptionType> {
    let width: xlenbits = to_bits::<64>(64, width);
    for i in 0..=63 {
        let prev_pmpaddr = if {(i > 0)} {
            pmpReadAddrReg(core_ctx, (i - 1))
        } else {
            zeros::<64>(64)
        };
        match {
            let var_1 = pmpReadAddrReg(core_ctx, i);
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

/// lock_highest_priority_pmp
/// 
/// Generated from the Sail sources at `riscv_pmp_control.sail` L120-136.
pub fn lock_highest_priority_pmp(core_ctx: &mut Core, unit_arg: ()) -> bool {
    match pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(core_ctx.pmpcfg_n[(0 as usize)])) {
        PmpAddrMatchType::NAPOT => {{
            core_ctx.pmpcfg_n[0].bits = core_ctx.pmpcfg_n[0].bits.set_subrange::<7, 8, 1>(BitVector::<1>::new(0b1));
            return true;
        }}
        PmpAddrMatchType::OFF => {{
            match pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(core_ctx.pmpcfg_n[(1 as usize)])) {
                PmpAddrMatchType::TOR => {{
                    core_ctx.pmpcfg_n[1].bits = core_ctx.pmpcfg_n[1].bits.set_subrange::<7, 8, 1>(BitVector::<1>::new(0b1));
                    return true;
                }}
                _ => {{
                    return false;
                }}
                _ => {panic!("Unreachable code")}
            }
        }}
        _ => {{
            return false;
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// unlock_all_pmps
/// 
/// Generated from the Sail sources at `riscv_pmp_control.sail` L138-149.
pub fn unlock_all_pmps(core_ctx: &mut Core, unit_arg: ()) {
    for i in 0..=63 {
        core_ctx.pmpcfg_n[(i as usize)] = _update_Pmpcfg_ent_L(core_ctx.pmpcfg_n[(i as usize)], BitVector::<1>::new(0b0))
    }
}

/// get_highest_priority_pmp_sa
/// 
/// Generated from the Sail sources at `riscv_pmp_control.sail` L151-177.
pub fn get_highest_priority_pmp_sa(core_ctx: &mut Core, unit_arg: ()) -> Option<BitVector<{
    64
}>> {
    match pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(core_ctx.pmpcfg_n[(0 as usize)])) {
        PmpAddrMatchType::NAPOT => {{
            let mask = (pmpReadAddrReg(core_ctx, 0) ^ (pmpReadAddrReg(core_ctx, 0) + 1));
            let begin_words = (pmpReadAddrReg(core_ctx, 0) & !(mask)).unsigned();
            let sa_nat = (begin_words * 4);
            return Some(to_bits::<64>(64, sa_nat));
        }}
        PmpAddrMatchType::OFF => {{
            match pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(core_ctx.pmpcfg_n[(1 as usize)])) {
                PmpAddrMatchType::TOR => {{
                    return Some(to_bits::<64>(64, ((pmpReadAddrReg(core_ctx, 0).unsigned() as i128) * (4 as i128))));
                }}
                _ => {{
                    return None;
                }}
                _ => {panic!("Unreachable code")}
            }
        }}
        _ => {{
            return None;
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// reset_pmp
/// 
/// Generated from the Sail sources at `riscv_pmp_control.sail` L179-190.
pub fn reset_pmp(core_ctx: &mut Core, unit_arg: ()) {
    assert!(((sys_pmp_count(core_ctx, ()) == 0) || (((sys_pmp_count(core_ctx, ()) == 16) || ((sys_pmp_count(core_ctx, ()) == 64) as bool)) as bool)), "sys_pmp_count() must be 0, 16, or 64");
    for i in 0..=63 {
        core_ctx.pmpcfg_n[(i as usize)] = _update_Pmpcfg_ent_L(_update_Pmpcfg_ent_A(core_ctx.pmpcfg_n[(i as usize)], pmpAddrMatchType_to_bits(PmpAddrMatchType::OFF)), BitVector::<1>::new(0b0))
    }
}

/// ext_check_CSR
/// 
/// Generated from the Sail sources at `riscv_ext_regs.sail` L18.
pub const fn ext_check_CSR(csrno: BitVector<12>, p: Privilege, isWrite: bool) -> bool {
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
pub fn ext_control_check_addr(pc: BitVector<{
    64
}>) -> Ext_ControlAddr_Check<()> {
    Ext_ControlAddr_Check::Ext_ControlAddr_OK(virtaddr::Virtaddr(pc))
}

/// ext_control_check_pc
/// 
/// Generated from the Sail sources at `riscv_addr_checks.sail` L44-45.
pub fn ext_control_check_pc(pc: BitVector<{
    64
}>) -> Ext_ControlAddr_Check<()> {
    Ext_ControlAddr_Check::Ext_ControlAddr_OK(virtaddr::Virtaddr(pc))
}

pub type ext_data_addr_error = ();

pub type vreglenbits = BitVector<vlenmax>;

pub type vregtype = vreglenbits;

/// dirty_v_context
/// 
/// Generated from the Sail sources at `riscv_vext_regs.sail` L94-99.
pub fn dirty_v_context(core_ctx: &mut Core, unit_arg: ()) {
    assert!(hartSupports(core_ctx, extension::Ext_V), "riscv_vext_regs.sail:95.28-95.29");
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<9, 11, 2>(extStatus_to_bits(ExtStatus::Dirty));
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<63, 64, 1>(BitVector::<1>::new(0b1))
}

/// _get_Vcsr_vxrm
/// 
/// Generated from the Sail sources.
pub fn _get_Vcsr_vxrm(v: Vcsr) -> BitVector<2> {
    v.bits.subrange::<1, 3, 2>()
}

/// _get_Vcsr_vxsat
/// 
/// Generated from the Sail sources.
pub fn _get_Vcsr_vxsat(v: Vcsr) -> BitVector<1> {
    v.bits.subrange::<0, 1, 1>()
}

/// ext_write_vcsr
/// 
/// Generated from the Sail sources at `riscv_vext_regs.sail` L233-237.
pub fn ext_write_vcsr(core_ctx: &mut Core, vxrm_val: BitVector<2>, vxsat_val: BitVector<1>) {
    core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange::<1, 3, 2>(vxrm_val);
    core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange::<0, 1, 1>(vxsat_val);
    dirty_v_context(core_ctx, ())
}

/// set_vstart
/// 
/// Generated from the Sail sources at `riscv_vext_control.sail` L15-20.
pub fn set_vstart(core_ctx: &mut Core, value: BitVector<16>) {
    dirty_v_context(core_ctx, ());
    core_ctx.vstart = value.subrange::<0, 3, 3>().zero_extend::<16>()
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
pub const fn handle_trap_extension(p: Privilege, pc: BitVector<{
    64
}>, u: Option<()>) {
    ()
}

/// prepare_trap_vector
/// 
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L21-31.
pub fn prepare_trap_vector(core_ctx: &mut Core, p: Privilege, cause: Mcause) -> BitVector<{
    64
}> {
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
pub fn get_xepc(core_ctx: &mut Core, p: Privilege) -> BitVector<{
    64
}> {
    match p {
        Privilege::Machine => {{
            let var_1 = core_ctx.mepc;
            align_pc(core_ctx, var_1)
        }}
        Privilege::Supervisor => {{
            let var_2 = core_ctx.sepc;
            align_pc(core_ctx, var_2)
        }}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sys_exceptions.sail", 45, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

/// set_xepc
/// 
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L49-57.
pub fn set_xepc(core_ctx: &mut Core, p: Privilege, value: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    let target = legalize_xepc(core_ctx, value);
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
pub fn prepare_xret_target(core_ctx: &mut Core, p: Privilege) -> BitVector<{
    64
}> {
    get_xepc(core_ctx, p)
}

/// get_mtvec
/// 
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L65-66.
pub fn get_mtvec(core_ctx: &mut Core, unit_arg: ()) -> BitVector<{
    64
}> {
    core_ctx.mtvec.bits
}

/// get_stvec
/// 
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L68-69.
pub fn get_stvec(core_ctx: &mut Core, unit_arg: ()) -> BitVector<{
    64
}> {
    core_ctx.stvec.bits
}

/// set_mtvec
/// 
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L71-74.
pub fn set_mtvec(core_ctx: &mut Core, value: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    core_ctx.mtvec = {
        let var_1 = core_ctx.mtvec;
        legalize_tvec(var_1, value)
    };
    core_ctx.mtvec.bits
}

/// set_stvec
/// 
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L76-79.
pub fn set_stvec(core_ctx: &mut Core, value: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    core_ctx.stvec = {
        let var_1 = core_ctx.stvec;
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
pub fn Mk_HpmEvent(v: BitVector<64>) -> HpmEvent {
    HpmEvent {
        bits: v
    }
}

/// _get_HpmEvent_MINH
/// 
/// Generated from the Sail sources.
pub fn _get_HpmEvent_MINH(v: HpmEvent) -> BitVector<1> {
    v.bits.subrange::<62, 63, 1>()
}

/// _update_HpmEvent_MINH
/// 
/// Generated from the Sail sources.
pub fn _update_HpmEvent_MINH(v: HpmEvent, x: BitVector<1>) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 62, 62, x)
    }
}

/// _update_CountSmcntrpmf_MINH
/// 
/// Generated from the Sail sources.
pub fn _update_CountSmcntrpmf_MINH(v: CountSmcntrpmf, x: BitVector<1>) -> CountSmcntrpmf {
    CountSmcntrpmf {
        bits: update_subrange_bits(v.bits, 62, 62, x)
    }
}

/// _get_CountSmcntrpmf_MINH
/// 
/// Generated from the Sail sources.
pub fn _get_CountSmcntrpmf_MINH(v: CountSmcntrpmf) -> BitVector<1> {
    v.bits.subrange::<62, 63, 1>()
}

/// _get_HpmEvent_OF
/// 
/// Generated from the Sail sources.
pub fn _get_HpmEvent_OF(v: HpmEvent) -> BitVector<1> {
    v.bits.subrange::<63, 64, 1>()
}

/// _update_HpmEvent_OF
/// 
/// Generated from the Sail sources.
pub fn _update_HpmEvent_OF(v: HpmEvent, x: BitVector<1>) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 63, 63, x)
    }
}

/// _get_HpmEvent_SINH
/// 
/// Generated from the Sail sources.
pub fn _get_HpmEvent_SINH(v: HpmEvent) -> BitVector<1> {
    v.bits.subrange::<61, 62, 1>()
}

/// _update_HpmEvent_SINH
/// 
/// Generated from the Sail sources.
pub fn _update_HpmEvent_SINH(v: HpmEvent, x: BitVector<1>) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 61, 61, x)
    }
}

/// _update_CountSmcntrpmf_SINH
/// 
/// Generated from the Sail sources.
pub fn _update_CountSmcntrpmf_SINH(v: CountSmcntrpmf, x: BitVector<1>) -> CountSmcntrpmf {
    CountSmcntrpmf {
        bits: update_subrange_bits(v.bits, 61, 61, x)
    }
}

/// _get_CountSmcntrpmf_SINH
/// 
/// Generated from the Sail sources.
pub fn _get_CountSmcntrpmf_SINH(v: CountSmcntrpmf) -> BitVector<1> {
    v.bits.subrange::<61, 62, 1>()
}

/// _get_HpmEvent_UINH
/// 
/// Generated from the Sail sources.
pub fn _get_HpmEvent_UINH(v: HpmEvent) -> BitVector<1> {
    v.bits.subrange::<60, 61, 1>()
}

/// _update_HpmEvent_UINH
/// 
/// Generated from the Sail sources.
pub fn _update_HpmEvent_UINH(v: HpmEvent, x: BitVector<1>) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 60, 60, x)
    }
}

/// _update_CountSmcntrpmf_UINH
/// 
/// Generated from the Sail sources.
pub fn _update_CountSmcntrpmf_UINH(v: CountSmcntrpmf, x: BitVector<1>) -> CountSmcntrpmf {
    CountSmcntrpmf {
        bits: update_subrange_bits(v.bits, 60, 60, x)
    }
}

/// _get_CountSmcntrpmf_UINH
/// 
/// Generated from the Sail sources.
pub fn _get_CountSmcntrpmf_UINH(v: CountSmcntrpmf) -> BitVector<1> {
    v.bits.subrange::<60, 61, 1>()
}

/// _update_HpmEvent_VSINH
/// 
/// Generated from the Sail sources.
pub fn _update_HpmEvent_VSINH(v: HpmEvent, x: BitVector<1>) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 59, 59, x)
    }
}

/// _update_HpmEvent_VUINH
/// 
/// Generated from the Sail sources.
pub fn _update_HpmEvent_VUINH(v: HpmEvent, x: BitVector<1>) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 58, 58, x)
    }
}

/// _get_HpmEvent_event
/// 
/// Generated from the Sail sources.
pub fn _get_HpmEvent_event(v: HpmEvent) -> BitVector<32> {
    v.bits.subrange::<0, 32, 32>()
}

/// _update_HpmEvent_event
/// 
/// Generated from the Sail sources.
pub fn _update_HpmEvent_event(v: HpmEvent, x: BitVector<32>) -> HpmEvent {
    HpmEvent {
        bits: update_subrange_bits(v.bits, 31, 0, x)
    }
}

pub type hpmidx = i128;

/// hpmidx_from_bits
/// 
/// Generated from the Sail sources at `riscv_zihpm.sail` L189-193.
pub fn hpmidx_from_bits(b: BitVector<5>) -> i128 {
    let index = b.unsigned();
    assert!((index >= 3), "unreachable HPM index");
    index
}

/// legalize_hpmevent
/// 
/// Generated from the Sail sources at `riscv_zihpm.sail` L195-205.
pub fn legalize_hpmevent(core_ctx: &mut Core, v: HpmEvent) -> HpmEvent {
    {
        let var_1 = {
            let var_2 = {
                let var_3 = {
                    let var_4 = {
                        let var_6 = {
                            let var_8 = {
                                let var_10 = if {currentlyEnabled(core_ctx, extension::Ext_Sscofpmf)} {
                                    _get_HpmEvent_OF(v)
                                } else {
                                    BitVector::<1>::new(0b0)
                                };
                                _update_HpmEvent_OF(Mk_HpmEvent(zeros::<64>(64)), var_10)
                            };
                            let var_9 = if {currentlyEnabled(core_ctx, extension::Ext_Sscofpmf)} {
                                _get_HpmEvent_MINH(v)
                            } else {
                                BitVector::<1>::new(0b0)
                            };
                            _update_HpmEvent_MINH(var_8, var_9)
                        };
                        let var_7 = if {(currentlyEnabled(core_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(core_ctx, extension::Ext_S))} {
                            _get_HpmEvent_SINH(v)
                        } else {
                            BitVector::<1>::new(0b0)
                        };
                        _update_HpmEvent_SINH(var_6, var_7)
                    };
                    let var_5 = if {(currentlyEnabled(core_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(core_ctx, extension::Ext_U))} {
                        _get_HpmEvent_UINH(v)
                    } else {
                        BitVector::<1>::new(0b0)
                    };
                    _update_HpmEvent_UINH(var_4, var_5)
                };
                _update_HpmEvent_VSINH(var_3, BitVector::<1>::new(0b0))
            };
            _update_HpmEvent_VUINH(var_2, BitVector::<1>::new(0b0))
        };
        _update_HpmEvent_event(var_1, _get_HpmEvent_event(v))
    }
}

/// read_mhpmcounter
/// 
/// Generated from the Sail sources at `riscv_zihpm.sail` L207.
pub fn read_mhpmcounter(core_ctx: &mut Core, index: i128) -> BitVector<{
    64
}> {
    subrange_bits::<_, 64>(core_ctx.mhpmcounter[(index as usize)], 63, 0)
}

/// read_mhpmevent
/// 
/// Generated from the Sail sources at `riscv_zihpm.sail` L209.
pub fn read_mhpmevent(core_ctx: &mut Core, index: i128) -> BitVector<{
    64
}> {
    core_ctx.mhpmevent[(index as usize)].bits.subrange::<0, 64, 64>()
}

/// write_mhpmcounter
/// 
/// Generated from the Sail sources at `riscv_zihpm.sail` L212-213.
pub fn write_mhpmcounter(core_ctx: &mut Core, index: i128, value: BitVector<{
    64
}>) {
    if {(bitvector_access(sys_writable_hpm_counters(core_ctx, ()), index) == true)} {
        core_ctx.mhpmcounter[(index as usize)] = core_ctx.mhpmcounter[(index as usize)].set_subrange::<0, 64, 64>(value)
    } else {
        ()
    }
}

/// write_mhpmevent
/// 
/// Generated from the Sail sources at `riscv_zihpm.sail` L218-224.
pub fn write_mhpmevent(core_ctx: &mut Core, index: i128, value: BitVector<{
    64
}>) {
    if {(bitvector_access(sys_writable_hpm_counters(core_ctx, ()), index) == true)} {
        core_ctx.mhpmevent[(index as usize)] = legalize_hpmevent(core_ctx, Mk_HpmEvent(value))
    } else {
        ()
    }
}

/// get_scountovf
/// 
/// Generated from the Sail sources at `riscv_sscofpmf.sail` L60-76.
pub fn get_scountovf(core_ctx: &mut Core, _priv_: Privilege) -> BitVector<32> {
    let overflow = bitvector_concat::<1, 31, 32>(_get_HpmEvent_OF(core_ctx.mhpmevent[(31 as usize)]), bitvector_concat::<1, 30, 31>(_get_HpmEvent_OF(core_ctx.mhpmevent[(30 as usize)]), bitvector_concat::<1, 29, 30>(_get_HpmEvent_OF(core_ctx.mhpmevent[(29 as usize)]), bitvector_concat::<1, 28, 29>(_get_HpmEvent_OF(core_ctx.mhpmevent[(28 as usize)]), bitvector_concat::<1, 27, 28>(_get_HpmEvent_OF(core_ctx.mhpmevent[(27 as usize)]), bitvector_concat::<1, 26, 27>(_get_HpmEvent_OF(core_ctx.mhpmevent[(26 as usize)]), bitvector_concat::<1, 25, 26>(_get_HpmEvent_OF(core_ctx.mhpmevent[(25 as usize)]), bitvector_concat::<1, 24, 25>(_get_HpmEvent_OF(core_ctx.mhpmevent[(24 as usize)]), bitvector_concat::<1, 23, 24>(_get_HpmEvent_OF(core_ctx.mhpmevent[(23 as usize)]), bitvector_concat::<1, 22, 23>(_get_HpmEvent_OF(core_ctx.mhpmevent[(22 as usize)]), bitvector_concat::<1, 21, 22>(_get_HpmEvent_OF(core_ctx.mhpmevent[(21 as usize)]), bitvector_concat::<1, 20, 21>(_get_HpmEvent_OF(core_ctx.mhpmevent[(20 as usize)]), bitvector_concat::<1, 19, 20>(_get_HpmEvent_OF(core_ctx.mhpmevent[(19 as usize)]), bitvector_concat::<1, 18, 19>(_get_HpmEvent_OF(core_ctx.mhpmevent[(18 as usize)]), bitvector_concat::<1, 17, 18>(_get_HpmEvent_OF(core_ctx.mhpmevent[(17 as usize)]), bitvector_concat::<1, 16, 17>(_get_HpmEvent_OF(core_ctx.mhpmevent[(16 as usize)]), bitvector_concat::<1, 15, 16>(_get_HpmEvent_OF(core_ctx.mhpmevent[(15 as usize)]), bitvector_concat::<1, 14, 15>(_get_HpmEvent_OF(core_ctx.mhpmevent[(14 as usize)]), bitvector_concat::<1, 13, 14>(_get_HpmEvent_OF(core_ctx.mhpmevent[(13 as usize)]), bitvector_concat::<1, 12, 13>(_get_HpmEvent_OF(core_ctx.mhpmevent[(12 as usize)]), bitvector_concat::<1, 11, 12>(_get_HpmEvent_OF(core_ctx.mhpmevent[(11 as usize)]), bitvector_concat::<1, 10, 11>(_get_HpmEvent_OF(core_ctx.mhpmevent[(10 as usize)]), bitvector_concat::<1, 9, 10>(_get_HpmEvent_OF(core_ctx.mhpmevent[(9 as usize)]), bitvector_concat::<1, 8, 9>(_get_HpmEvent_OF(core_ctx.mhpmevent[(8 as usize)]), bitvector_concat::<1, 7, 8>(_get_HpmEvent_OF(core_ctx.mhpmevent[(7 as usize)]), bitvector_concat::<1, 6, 7>(_get_HpmEvent_OF(core_ctx.mhpmevent[(6 as usize)]), bitvector_concat::<1, 5, 6>(_get_HpmEvent_OF(core_ctx.mhpmevent[(5 as usize)]), bitvector_concat::<1, 4, 5>(_get_HpmEvent_OF(core_ctx.mhpmevent[(4 as usize)]), bitvector_concat::<1, 3, 4>(_get_HpmEvent_OF(core_ctx.mhpmevent[(3 as usize)]), BitVector::<3>::new(0b000))))))))))))))))))))))))))))));
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
pub fn opst_code_forwards(arg_hashtag_: seed_opst) -> BitVector<2> {
    match arg_hashtag_ {
        seed_opst::BIST => {BitVector::<2>::new(0b00)}
        seed_opst::WAIT => {BitVector::<2>::new(0b01)}
        seed_opst::ES16 => {BitVector::<2>::new(0b10)}
        seed_opst::DEAD => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

/// read_seed_csr
/// 
/// Generated from the Sail sources at `riscv_zkr_control.sail` L33-38.
pub fn read_seed_csr(unit_arg: ()) -> BitVector<{
    64
}> {
    let reserved_bits: BitVector<6> = BitVector::<6>::new(0b000000);
    let custom_bits: BitVector<8> = BitVector::<8>::new(0b00000000);
    let seed: BitVector<16> = get_16_random_bits(());
    bitvector_concat::<2, 30, 32>(opst_code_forwards(seed_opst::ES16), bitvector_concat::<6, 24, 30>(reserved_bits, bitvector_concat::<8, 16, 24>(custom_bits, seed))).zero_extend::<64>()
}

/// write_seed_csr
/// 
/// Generated from the Sail sources at `riscv_zkr_control.sail` L41.
pub fn write_seed_csr(unit_arg: ()) -> BitVector<{
    64
}> {
    zeros::<64>(64)
}

pub type bits_rm = BitVector<3>;

pub type bits_fflags = BitVector<5>;

pub type bits_H = BitVector<16>;

pub type bits_S = BitVector<32>;

pub type bits_D = BitVector<64>;

pub type bits_W = BitVector<32>;

pub type bits_WU = BitVector<32>;

pub type bits_L = BitVector<64>;

pub type bits_LU = BitVector<64>;

/// fregno
/// 
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L56.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fregno {
    Fregno(i128)
}

/// dirty_fd_context
/// 
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L109-114.
pub fn dirty_fd_context(core_ctx: &mut Core, unit_arg: ()) {
    assert!(hartSupports(core_ctx, extension::Ext_F), "riscv_fdext_regs.sail:110.28-110.29");
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<13, 15, 2>(extStatus_to_bits(ExtStatus::Dirty));
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<63, 64, 1>(BitVector::<1>::new(0b1))
}

/// dirty_fd_context_if_present
/// 
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L116-119.
pub fn dirty_fd_context_if_present(core_ctx: &mut Core, unit_arg: ()) {
    assert!({
        let var_1 = hartSupports(core_ctx, extension::Ext_F);
        let var_2 = hartSupports(core_ctx, extension::Ext_Zfinx);
        neq_bool(var_1, var_2)
    }, "riscv_fdext_regs.sail:117.55-117.56");
    if {hartSupports(core_ctx, extension::Ext_F)} {
        dirty_fd_context(core_ctx, ())
    } else {
        ()
    }
}

/// _get_Fcsr_FFLAGS
/// 
/// Generated from the Sail sources.
pub fn _get_Fcsr_FFLAGS(v: Fcsr) -> BitVector<5> {
    v.bits.subrange::<0, 5, 5>()
}

/// _get_Fcsr_FRM
/// 
/// Generated from the Sail sources.
pub fn _get_Fcsr_FRM(v: Fcsr) -> BitVector<3> {
    v.bits.subrange::<5, 8, 3>()
}

/// write_fcsr
/// 
/// Generated from the Sail sources at `riscv_fdext_regs.sail` L392-396.
pub fn write_fcsr(core_ctx: &mut Core, frm: BitVector<3>, fflags: BitVector<5>) {
    core_ctx.fcsr.bits = core_ctx.fcsr.bits.set_subrange::<5, 8, 3>(frm);
    core_ctx.fcsr.bits = core_ctx.fcsr.bits.set_subrange::<0, 5, 5>(fflags);
    dirty_fd_context_if_present(core_ctx, ())
}

/// Mk_CountSmcntrpmf
/// 
/// Generated from the Sail sources at `` L1.
pub fn Mk_CountSmcntrpmf(v: BitVector<64>) -> CountSmcntrpmf {
    CountSmcntrpmf {
        bits: v
    }
}

/// legalize_smcntrpmf
/// 
/// Generated from the Sail sources at `riscv_smcntrpmf.sail` L11-21.
pub fn legalize_smcntrpmf(core_ctx: &mut Core, c: CountSmcntrpmf, value: BitVector<64>) -> CountSmcntrpmf {
    let v = Mk_CountSmcntrpmf(value);
    {
        let var_1 = {
            let var_3 = if {currentlyEnabled(core_ctx, extension::Ext_S)} {
                _get_CountSmcntrpmf_SINH(v)
            } else {
                BitVector::<1>::new(0b0)
            };
            _update_CountSmcntrpmf_SINH(_update_CountSmcntrpmf_MINH(c, _get_CountSmcntrpmf_MINH(v)), var_3)
        };
        let var_2 = if {currentlyEnabled(core_ctx, extension::Ext_U)} {
            _get_CountSmcntrpmf_UINH(v)
        } else {
            BitVector::<1>::new(0b0)
        };
        _update_CountSmcntrpmf_UINH(var_1, var_2)
    }
}

/// csrAccess
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L13.
pub fn csrAccess(csr: BitVector<12>) -> BitVector<2> {
    csr.subrange::<10, 12, 2>()
}

/// csrPriv
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L14.
pub fn csrPriv(csr: BitVector<12>) -> BitVector<2> {
    csr.subrange::<8, 10, 2>()
}

/// check_CSR_priv
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L17-18.
pub fn check_CSR_priv(csr: BitVector<12>, p: Privilege) -> bool {
    _operator_biggerequal_u_(privLevel_to_bits(p), csrPriv(csr))
}

/// check_CSR_access
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L21-22.
pub fn check_CSR_access(csr: BitVector<12>, isWrite: bool) -> bool {
    !((isWrite && (csrAccess(csr) == BitVector::<2>::new(0b11))))
}

/// check_TVM_SATP
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L24-25.
pub fn check_TVM_SATP(core_ctx: &mut Core, csr: BitVector<12>, p: Privilege) -> bool {
    !(((csr == BitVector::<12>::new(0b000110000000)) && ((p == Privilege::Supervisor) && ({
        let var_1 = core_ctx.mstatus;
        _get_Mstatus_TVM(var_1)
    } == BitVector::<1>::new(0b1)))))
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
pub fn check_Counteren(core_ctx: &mut Core, csr: BitVector<12>, p: Privilege) -> bool {
    if {(_operator_smaller_u_(csr, BitVector::<12>::new(0b110000000000)) || _operator_smaller_u_(BitVector::<12>::new(0b110000011111), csr))} {
        return true;
    } else {
        ()
    };
    let index = csr.subrange::<0, 5, 5>().unsigned();
    {
        let var_1 = bitvector_access(core_ctx.mcounteren.bits, index);
        let var_2 = bitvector_access(core_ctx.scounteren.bits, index);
        feature_enabled_for_priv(core_ctx, p, var_1, var_2)
    }
}

/// check_Stimecmp
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L46-51.
pub fn check_Stimecmp(core_ctx: &mut Core, csr: BitVector<12>, p: Privilege) -> bool {
    if {((csr != BitVector::<12>::new(0b000101001101)) && (csr != BitVector::<12>::new(0b000101011101)))} {
        return true;
    } else {
        ()
    };
    ((p == Privilege::Machine) || ((p == Privilege::Supervisor) && (({
        let var_2 = core_ctx.mcounteren;
        _get_Counteren_TM(var_2)
    } == BitVector::<1>::new(0b1)) && ({
        let var_1 = core_ctx.menvcfg;
        _get_MEnvcfg_STCE(var_1)
    } == BitVector::<1>::new(0b1)))))
}

/// check_seed_CSR
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L56-69.
pub fn check_seed_CSR(csr: BitVector<12>, p: Privilege, isWrite: bool) -> bool {
    if {!((csr == BitVector::<12>::new(0b000000010101)))} {
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
pub fn is_CSR_defined(core_ctx: &mut Core, merge_hashtag_var: BitVector<12>) -> bool {
    match merge_hashtag_var {
        b__0 if {(b__0 == BitVector::<12>::new(0b001100000001))} => {true}
        b__1 if {(b__1 == BitVector::<12>::new(0b001100000000))} => {true}
        b__2 if {(b__2 == BitVector::<12>::new(0b001100010000))} => {false}
        b__3 if {(b__3 == BitVector::<12>::new(0b001100001010))} => {currentlyEnabled(core_ctx, extension::Ext_U)}
        b__4 if {(b__4 == BitVector::<12>::new(0b001100011010))} => {false}
        b__5 if {(b__5 == BitVector::<12>::new(0b000100001010))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__6 if {(b__6 == BitVector::<12>::new(0b001100000100))} => {true}
        b__7 if {(b__7 == BitVector::<12>::new(0b001101000100))} => {true}
        b__8 if {(b__8 == BitVector::<12>::new(0b001100000010))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__9 if {(b__9 == BitVector::<12>::new(0b001100010010))} => {false}
        b__10 if {(b__10 == BitVector::<12>::new(0b001100000011))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__11 if {(b__11 == BitVector::<12>::new(0b001101000010))} => {true}
        b__12 if {(b__12 == BitVector::<12>::new(0b001101000011))} => {true}
        b__13 if {(b__13 == BitVector::<12>::new(0b001101000000))} => {true}
        b__14 if {(b__14 == BitVector::<12>::new(0b000100000110))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__15 if {(b__15 == BitVector::<12>::new(0b001100000110))} => {currentlyEnabled(core_ctx, extension::Ext_U)}
        b__16 if {(b__16 == BitVector::<12>::new(0b001100100000))} => {true}
        b__17 if {(b__17 == BitVector::<12>::new(0b111100010001))} => {true}
        b__18 if {(b__18 == BitVector::<12>::new(0b111100010010))} => {true}
        b__19 if {(b__19 == BitVector::<12>::new(0b111100010011))} => {true}
        b__20 if {(b__20 == BitVector::<12>::new(0b111100010100))} => {true}
        b__21 if {(b__21 == BitVector::<12>::new(0b111100010101))} => {true}
        b__22 if {(b__22 == BitVector::<12>::new(0b000100000000))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__23 if {(b__23 == BitVector::<12>::new(0b000101000100))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__24 if {(b__24 == BitVector::<12>::new(0b000100000100))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__25 if {(b__25 == BitVector::<12>::new(0b000101000000))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__26 if {(b__26 == BitVector::<12>::new(0b000101000010))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__27 if {(b__27 == BitVector::<12>::new(0b000101000011))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__28 if {(b__28 == BitVector::<12>::new(0b011110100000))} => {true}
        v__3824 if {(v__3824.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111010))} => {let idx: BitVector<4> = v__3824.subrange::<0, 4, 4>();
        ((sys_pmp_count(core_ctx, ()) > (4 * idx.unsigned())) && (bitvector_access(idx, 0) == false))}
        v__3826 if {(v__3826.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111011))} => {let idx: BitVector<4> = v__3826.subrange::<0, 4, 4>();
        (sys_pmp_count(core_ctx, ()) > bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b00), idx).unsigned())}
        v__3828 if {(v__3828.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111100))} => {let idx: BitVector<4> = v__3828.subrange::<0, 4, 4>();
        (sys_pmp_count(core_ctx, ()) > bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b01), idx).unsigned())}
        v__3830 if {(v__3830.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111101))} => {let idx: BitVector<4> = v__3830.subrange::<0, 4, 4>();
        (sys_pmp_count(core_ctx, ()) > bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b10), idx).unsigned())}
        v__3832 if {(v__3832.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111110))} => {let idx: BitVector<4> = v__3832.subrange::<0, 4, 4>();
        (sys_pmp_count(core_ctx, ()) > bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b11), idx).unsigned())}
        b__29 if {(b__29 == BitVector::<12>::new(0b000000001000))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__30 if {(b__30 == BitVector::<12>::new(0b000000001001))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__31 if {(b__31 == BitVector::<12>::new(0b000000001010))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__32 if {(b__32 == BitVector::<12>::new(0b000000001111))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__33 if {(b__33 == BitVector::<12>::new(0b110000100000))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__34 if {(b__34 == BitVector::<12>::new(0b110000100001))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__35 if {(b__35 == BitVector::<12>::new(0b110000100010))} => {currentlyEnabled(core_ctx, extension::Ext_V)}
        b__36 if {(b__36 == BitVector::<12>::new(0b000100000101))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__37 if {(b__37 == BitVector::<12>::new(0b000101000001))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        b__38 if {(b__38 == BitVector::<12>::new(0b001100000101))} => {true}
        b__39 if {(b__39 == BitVector::<12>::new(0b001101000001))} => {true}
        v__3834 if {let index_var_1: BitVector<5> = v__3834.subrange::<0, 5, 5>();
        ((v__3834.subrange::<5, 12, 7>() == BitVector::<7>::new(0b0011001)) && ((index_var_1.unsigned() >= 3) as bool))} => {currentlyEnabled(core_ctx, extension::Ext_Zihpm)}
        v__3836 if {let index_var_2: BitVector<5> = v__3836.subrange::<0, 5, 5>();
        ((v__3836.subrange::<5, 12, 7>() == BitVector::<7>::new(0b1011000)) && ((index_var_2.unsigned() >= 3) as bool))} => {currentlyEnabled(core_ctx, extension::Ext_Zihpm)}
        v__3838 if {let index_var_3: BitVector<5> = v__3838.subrange::<0, 5, 5>();
        ((v__3838.subrange::<5, 12, 7>() == BitVector::<7>::new(0b1011100)) && ((index_var_3.unsigned() >= 3) as bool))} => {false}
        v__3840 if {let index_var_4: BitVector<5> = v__3840.subrange::<0, 5, 5>();
        ((v__3840.subrange::<5, 12, 7>() == BitVector::<7>::new(0b1100000)) && ((index_var_4.unsigned() >= 3) as bool))} => {(currentlyEnabled(core_ctx, extension::Ext_Zihpm) && currentlyEnabled(core_ctx, extension::Ext_U))}
        v__3842 if {let index_var_5: BitVector<5> = v__3842.subrange::<0, 5, 5>();
        ((v__3842.subrange::<5, 12, 7>() == BitVector::<7>::new(0b1100100)) && ((index_var_5.unsigned() >= 3) as bool))} => {false}
        v__3844 if {let index_var_6: BitVector<5> = v__3844.subrange::<0, 5, 5>();
        ((v__3844.subrange::<5, 12, 7>() == BitVector::<7>::new(0b0111001)) && ((index_var_6.unsigned() >= 3) as bool))} => {false}
        b__40 if {(b__40 == BitVector::<12>::new(0b110110100000))} => {(currentlyEnabled(core_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(core_ctx, extension::Ext_S))}
        b__41 if {(b__41 == BitVector::<12>::new(0b000000010101))} => {currentlyEnabled(core_ctx, extension::Ext_Zkr)}
        b__42 if {(b__42 == BitVector::<12>::new(0b110000000000))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__43 if {(b__43 == BitVector::<12>::new(0b110000000001))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__44 if {(b__44 == BitVector::<12>::new(0b110000000010))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__45 if {(b__45 == BitVector::<12>::new(0b110010000000))} => {false}
        b__46 if {(b__46 == BitVector::<12>::new(0b110010000001))} => {false}
        b__47 if {(b__47 == BitVector::<12>::new(0b110010000010))} => {false}
        b__48 if {(b__48 == BitVector::<12>::new(0b101100000000))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__49 if {(b__49 == BitVector::<12>::new(0b101100000010))} => {currentlyEnabled(core_ctx, extension::Ext_Zicntr)}
        b__50 if {(b__50 == BitVector::<12>::new(0b101110000000))} => {false}
        b__51 if {(b__51 == BitVector::<12>::new(0b101110000010))} => {false}
        b__52 if {(b__52 == BitVector::<12>::new(0b000000000001))} => {(currentlyEnabled(core_ctx, extension::Ext_F) || currentlyEnabled(core_ctx, extension::Ext_Zfinx))}
        b__53 if {(b__53 == BitVector::<12>::new(0b000000000010))} => {(currentlyEnabled(core_ctx, extension::Ext_F) || currentlyEnabled(core_ctx, extension::Ext_Zfinx))}
        b__54 if {(b__54 == BitVector::<12>::new(0b000000000011))} => {(currentlyEnabled(core_ctx, extension::Ext_F) || currentlyEnabled(core_ctx, extension::Ext_Zfinx))}
        b__55 if {(b__55 == BitVector::<12>::new(0b001100100001))} => {currentlyEnabled(core_ctx, extension::Ext_Smcntrpmf)}
        b__56 if {(b__56 == BitVector::<12>::new(0b011100100001))} => {false}
        b__57 if {(b__57 == BitVector::<12>::new(0b001100100010))} => {currentlyEnabled(core_ctx, extension::Ext_Smcntrpmf)}
        b__58 if {(b__58 == BitVector::<12>::new(0b011100100010))} => {false}
        b__59 if {(b__59 == BitVector::<12>::new(0b000101001101))} => {(currentlyEnabled(core_ctx, extension::Ext_S) && currentlyEnabled(core_ctx, extension::Ext_Sstc))}
        b__60 if {(b__60 == BitVector::<12>::new(0b000101011101))} => {false}
        b__61 if {(b__61 == BitVector::<12>::new(0b000110000000))} => {currentlyEnabled(core_ctx, extension::Ext_S)}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// check_CSR
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L71-81.
pub fn check_CSR(core_ctx: &mut Core, csr: BitVector<12>, p: Privilege, isWrite: bool) -> bool {
    (is_CSR_defined(core_ctx, csr) && (check_CSR_priv(csr, p) && (check_CSR_access(csr, isWrite) && (check_TVM_SATP(core_ctx, csr, p) && (check_Counteren(core_ctx, csr, p) && (check_Stimecmp(core_ctx, csr, p) && check_seed_CSR(csr, p, isWrite)))))))
}

/// exception_delegatee
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L104-111.
pub fn exception_delegatee(core_ctx: &mut Core, e: ExceptionType, p: Privilege) -> Privilege {
    let idx = num_of_ExceptionType(e);
    let _super_ = {
        let var_1 = bitvector_access(core_ctx.medeleg.bits, idx);
        bit_to_bool(var_1)
    };
    let deleg = if {(currentlyEnabled(core_ctx, extension::Ext_S) && _super_)} {
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
pub fn findPendingInterrupt(ip: BitVector<{
    64
}>) -> Option<InterruptType> {
    let ip = Mk_Minterrupts(ip);
    if {(_get_Minterrupts_MEI(ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_M_External)
    } else if {(_get_Minterrupts_MSI(ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_M_Software)
    } else if {(_get_Minterrupts_MTI(ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_M_Timer)
    } else if {(_get_Minterrupts_SEI(ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_S_External)
    } else if {(_get_Minterrupts_SSI(ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_S_Software)
    } else if {(_get_Minterrupts_STI(ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_S_Timer)
    } else {
        None
    }
}

/// getPendingSet
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L135-148.
pub fn getPendingSet(core_ctx: &mut Core, _priv_: Privilege) -> Option<(BitVector<{
    64
}>, Privilege)> {
    assert!((currentlyEnabled(core_ctx, extension::Ext_S) || (core_ctx.mideleg.bits == zeros::<64>(64))), "riscv_sys_control.sail:137.58-137.59");
    let pending_m = (core_ctx.mip.bits & (core_ctx.mie.bits & !(core_ctx.mideleg.bits)));
    let pending_s = (core_ctx.mip.bits & (core_ctx.mie.bits & core_ctx.mideleg.bits));
    let mIE = (((_priv_ == Privilege::Machine) && ({
        let var_2 = core_ctx.mstatus;
        _get_Mstatus_MIE(var_2)
    } == BitVector::<1>::new(0b1))) || ((_priv_ == Privilege::Supervisor) || (_priv_ == Privilege::User)));
    let sIE = (((_priv_ == Privilege::Supervisor) && ({
        let var_1 = core_ctx.mstatus;
        _get_Mstatus_SIE(var_1)
    } == BitVector::<1>::new(0b1))) || (_priv_ == Privilege::User));
    if {(mIE && (pending_m != zeros::<64>(64)))} {
        Some((pending_m, Privilege::Machine))
    } else if {(sIE && (pending_s != zeros::<64>(64)))} {
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
pub fn tval(excinfo: Option<BitVector<{
    64
}>>) -> BitVector<{
    64
}> {
    match excinfo {
        Some(e) => {e}
        None => {zeros::<64>(64)}
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
pub fn trap_handler(core_ctx: &mut Core, del_priv: Privilege, intr: bool, c: BitVector<8>, pc: BitVector<{
    64
}>, info: Option<BitVector<{
    64
}>>, ext: Option<()>) -> BitVector<{
    64
}> {
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
            track_trap(core_ctx, del_priv);
            {
                let var_5 = core_ctx.mcause;
                prepare_trap_vector(core_ctx, del_priv, var_5)
            }
        }}
        Privilege::Supervisor => {{
            assert!(currentlyEnabled(core_ctx, extension::Ext_S), "no supervisor mode present for delegation");
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
            core_ctx.mstatus.bits = {
                let var_8 = match core_ctx.cur_privilege {
                    Privilege::User => {BitVector::<1>::new(0b0)}
                    Privilege::Supervisor => {BitVector::<1>::new(0b1)}
                    Privilege::Machine => {panic!("{}, l {}: {}", "riscv_sys_control.sail", 260, "invalid privilege for s-mode trap")}
                    _ => {panic!("Unreachable code")}
                };
                core_ctx.mstatus.bits.set_subrange::<8, 9, 1>(var_8)
            };
            core_ctx.stval = tval(info);
            core_ctx.sepc = pc;
            core_ctx.cur_privilege = del_priv;
            track_trap(core_ctx, del_priv);
            {
                let var_9 = core_ctx.scause;
                prepare_trap_vector(core_ctx, del_priv, var_9)
            }
        }}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sys_control.sail", 273, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

/// exception_handler
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L277-321.
pub fn exception_handler(core_ctx: &mut Core, cur_priv: Privilege, ctl: ctl_result, pc: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    match (cur_priv, ctl) {
        (_, ctl_result::CTL_TRAP(e)) => {{
            let del_priv = exception_delegatee(core_ctx, e.trap, cur_priv);
            trap_handler(core_ctx, del_priv, false, exceptionType_to_bits(e.trap), pc, e.excinfo, e.ext)
        }}
        (_, ctl_result::CTL_MRET(())) => {{
            let prev_priv = core_ctx.cur_privilege;
            core_ctx.mstatus.bits = {
                let var_1 = {
                    let var_2 = core_ctx.mstatus;
                    _get_Mstatus_MPIE(var_2)
                };
                core_ctx.mstatus.bits.set_subrange::<3, 4, 1>(var_1)
            };
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<7, 8, 1>(BitVector::<1>::new(0b1));
            core_ctx.cur_privilege = {
                let var_3 = {
                    let var_4 = core_ctx.mstatus;
                    _get_Mstatus_MPP(var_4)
                };
                privLevel_of_bits(var_3)
            };
            core_ctx.mstatus.bits = {
                let var_5 = {
                    let var_6 = if {currentlyEnabled(core_ctx, extension::Ext_U)} {
                        Privilege::User
                    } else {
                        Privilege::Machine
                    };
                    privLevel_to_bits(var_6)
                };
                core_ctx.mstatus.bits.set_subrange::<11, 13, 2>(var_5)
            };
            if {(core_ctx.cur_privilege != Privilege::Machine)} {
                core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<17, 18, 1>(BitVector::<1>::new(0b0))
            } else {
                ()
            };
            prepare_xret_target(core_ctx, Privilege::Machine)
        }}
        (_, ctl_result::CTL_SRET(())) => {{
            let prev_priv = core_ctx.cur_privilege;
            core_ctx.mstatus.bits = {
                let var_7 = {
                    let var_8 = core_ctx.mstatus;
                    _get_Mstatus_SPIE(var_8)
                };
                core_ctx.mstatus.bits.set_subrange::<1, 2, 1>(var_7)
            };
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<5, 6, 1>(BitVector::<1>::new(0b1));
            core_ctx.cur_privilege = if {({
                let var_9 = core_ctx.mstatus;
                _get_Mstatus_SPP(var_9)
            } == BitVector::<1>::new(0b1))} {
                Privilege::Supervisor
            } else {
                Privilege::User
            };
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<8, 9, 1>(BitVector::<1>::new(0b0));
            if {(core_ctx.cur_privilege != Privilege::Machine)} {
                core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<17, 18, 1>(BitVector::<1>::new(0b0))
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
    let var_1 = {
        let var_2 = core_ctx.PC;
        trap_handler(core_ctx, del_priv, true, interruptType_to_bits(i), var_2, None, None)
    };
    set_next_pc(core_ctx, var_1)
}

/// reset_misa
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L341-360.
pub fn reset_misa(core_ctx: &mut Core, unit_arg: ()) {
    core_ctx.misa.bits = {
        let var_1 = {
            let var_2 = hartSupports(core_ctx, extension::Ext_A);
            bool_to_bits(var_2)
        };
        core_ctx.misa.bits.set_subrange::<0, 1, 1>(var_1)
    };
    core_ctx.misa.bits = {
        let var_3 = {
            let var_4 = hartSupports(core_ctx, extension::Ext_C);
            bool_to_bits(var_4)
        };
        core_ctx.misa.bits.set_subrange::<2, 3, 1>(var_3)
    };
    core_ctx.misa.bits = {
        let var_5 = {
            let var_6 = hartSupports(core_ctx, extension::Ext_B);
            bool_to_bits(var_6)
        };
        core_ctx.misa.bits.set_subrange::<1, 2, 1>(var_5)
    };
    core_ctx.misa.bits = core_ctx.misa.bits.set_subrange::<8, 9, 1>(BitVector::<1>::new(0b1));
    core_ctx.misa.bits = {
        let var_7 = {
            let var_8 = hartSupports(core_ctx, extension::Ext_M);
            bool_to_bits(var_8)
        };
        core_ctx.misa.bits.set_subrange::<12, 13, 1>(var_7)
    };
    core_ctx.misa.bits = {
        let var_9 = {
            let var_10 = hartSupports(core_ctx, extension::Ext_U);
            bool_to_bits(var_10)
        };
        core_ctx.misa.bits.set_subrange::<20, 21, 1>(var_9)
    };
    core_ctx.misa.bits = {
        let var_11 = {
            let var_12 = hartSupports(core_ctx, extension::Ext_S);
            bool_to_bits(var_12)
        };
        core_ctx.misa.bits.set_subrange::<18, 19, 1>(var_11)
    };
    core_ctx.misa.bits = {
        let var_13 = {
            let var_14 = hartSupports(core_ctx, extension::Ext_V);
            bool_to_bits(var_14)
        };
        core_ctx.misa.bits.set_subrange::<21, 22, 1>(var_13)
    };
    if {(hartSupports(core_ctx, extension::Ext_F) && hartSupports(core_ctx, extension::Ext_Zfinx))} {
        panic!("{}, l {}: {}", "riscv_sys_control.sail", 352, "F and Zfinx cannot both be enabled!")
    } else {
        ()
    };
    core_ctx.misa.bits = {
        let var_15 = {
            let var_16 = hartSupports(core_ctx, extension::Ext_F);
            bool_to_bits(var_16)
        };
        core_ctx.misa.bits.set_subrange::<5, 6, 1>(var_15)
    };
    core_ctx.misa.bits = {
        let var_17 = {
            let var_18 = hartSupports(core_ctx, extension::Ext_D);
            bool_to_bits(var_18)
        };
        core_ctx.misa.bits.set_subrange::<3, 4, 1>(var_17)
    }
}

/// reset_sys
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L364-417.
pub fn reset_sys(core_ctx: &mut Core, unit_arg: ()) {
    core_ctx.cur_privilege = Privilege::Machine;
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<3, 4, 1>(BitVector::<1>::new(0b0));
    core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<17, 18, 1>(BitVector::<1>::new(0b0));
    reset_misa(core_ctx, ());
    cancel_reservation(());
    core_ctx.mcause.bits = zeros::<64>(64);
    reset_pmp(core_ctx, ());
    core_ctx.vstart = zeros::<16>(16);
    core_ctx.vl = zeros::<64>(64);
    core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange::<1, 3, 2>(BitVector::<2>::new(0b00));
    core_ctx.vcsr.bits = core_ctx.vcsr.bits.set_subrange::<0, 1, 1>(BitVector::<1>::new(0b0));
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange::<63, 64, 1>(BitVector::<1>::new(0b1));
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange::<8, 63, 55>(zeros::<55>(55));
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange::<7, 8, 1>(BitVector::<1>::new(0b0));
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange::<6, 7, 1>(BitVector::<1>::new(0b0));
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange::<3, 6, 3>(BitVector::<3>::new(0b000));
    core_ctx.vtype.bits = core_ctx.vtype.bits.set_subrange::<0, 3, 3>(BitVector::<3>::new(0b000))
}

pub type MemoryOpResult<A> = result<A, ExceptionType>;

/// plat_cache_block_size_exp
/// 
/// Generated from the Sail sources at `riscv_platform.sail` L31.
pub fn plat_cache_block_size_exp(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    core_ctx.config.platform.cache_block_size_exp
}

pub const MSIP_BASE: physaddrbits = BitVector::<20>::new(0b00000000000000000000).zero_extend::<64>();

pub const MTIMECMP_BASE: physaddrbits = BitVector::<20>::new(0b00000100000000000000).zero_extend::<64>();

pub const MTIMECMP_BASE_HI: physaddrbits = BitVector::<20>::new(0b00000100000000000100).zero_extend::<64>();

pub const MTIME_BASE: physaddrbits = BitVector::<20>::new(0b00001011111111111000).zero_extend::<64>();

pub const MTIME_BASE_HI: physaddrbits = BitVector::<20>::new(0b00001011111111111100).zero_extend::<64>();

/// phys_access_check
/// 
/// Generated from the Sail sources at `riscv_mem.sail` L99-103.
pub fn phys_access_check(core_ctx: &mut Core, t: AccessType<()>, p: Privilege, paddr: physaddr, width: i128) -> Option<ExceptionType> {
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

pub const RETIRE_SUCCESS: ExecutionResult = ExecutionResult::Retire_Success(());

pub type pte_flags_bits = BitVector<8>;

pub type pte_ext_bits = BitVector<10>;

pub const default_sv32_ext_pte: pte_ext_bits = zeros::<10>(10);

/// PTE_Check
/// 
/// Generated from the Sail sources at `riscv_vmem_pte.sail` L94-97.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PTE_Check {
    PTE_Check_Success(ext_ptw),
    PTE_Check_Failure((ext_ptw, ext_ptw_fail))
}

pub const tlb_vpn_bits: i128 = 45;

pub const tlb_ppn_bits: i128 = 44;

/// TLB_Entry
/// 
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L25-33.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct TLB_Entry {
    pub asid: asidbits,
    pub global: bool,
    pub vpn: BitVector<tlb_vpn_bits>,
    pub levelMask: BitVector<tlb_vpn_bits>,
    pub ppn: BitVector<tlb_ppn_bits>,
    pub pte: BitVector<64>,
    pub pteAddr: physaddr,
}

pub const num_tlb_entries: i128 = 64;

pub type tlb_index_range = i128;

/// flush_TLB_Entry
/// 
/// Generated from the Sail sources at `riscv_vmem_tlb.sail` L98-113.
pub fn flush_TLB_Entry(ent: TLB_Entry, asid: Option<BitVector<16>>, vaddr: Option<BitVector<{
    64
}>>) -> bool {
    let asid_matches: bool = match asid {
        Some(asid) => {((ent.asid == asid) && !(ent.global))}
        None => {true}
        _ => {panic!("Unreachable code")}
    };
    let addr_matches: bool = match vaddr {
        Some(vaddr) => {{
            let vaddr: BitVector<64> = sign_extend(64, vaddr);
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
pub fn flush_TLB(core_ctx: &mut Core, asid: Option<BitVector<16>>, addr: Option<BitVector<{
    64
}>>) {
    for i in 0..=(core_ctx.tlb.len() - 1) {
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

/// read_CSR
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L134.
pub fn read_CSR(core_ctx: &mut Core, merge_hashtag_var: BitVector<12>) -> BitVector<{
    64
}> {
    match merge_hashtag_var {
        b__0 if {(b__0 == BitVector::<12>::new(0b001100000001))} => {core_ctx.misa.bits}
        b__1 if {(b__1 == BitVector::<12>::new(0b001100000000))} => {core_ctx.mstatus.bits.subrange::<0, 64, 64>()}
        b__3 if {(b__3 == BitVector::<12>::new(0b001100001010))} => {core_ctx.menvcfg.bits.subrange::<0, 64, 64>()}
        b__5 if {(b__5 == BitVector::<12>::new(0b000100001010))} => {core_ctx.senvcfg.bits.subrange::<0, 64, 64>()}
        b__6 if {(b__6 == BitVector::<12>::new(0b001100000100))} => {core_ctx.mie.bits}
        b__7 if {(b__7 == BitVector::<12>::new(0b001101000100))} => {core_ctx.mip.bits}
        b__8 if {(b__8 == BitVector::<12>::new(0b001100000010))} => {core_ctx.medeleg.bits.subrange::<0, 64, 64>()}
        b__10 if {(b__10 == BitVector::<12>::new(0b001100000011))} => {core_ctx.mideleg.bits}
        b__11 if {(b__11 == BitVector::<12>::new(0b001101000010))} => {core_ctx.mcause.bits}
        b__12 if {(b__12 == BitVector::<12>::new(0b001101000011))} => {core_ctx.mtval}
        b__13 if {(b__13 == BitVector::<12>::new(0b001101000000))} => {core_ctx.mscratch}
        b__14 if {(b__14 == BitVector::<12>::new(0b000100000110))} => {core_ctx.scounteren.bits.zero_extend::<64>()}
        b__15 if {(b__15 == BitVector::<12>::new(0b001100000110))} => {core_ctx.mcounteren.bits.zero_extend::<64>()}
        b__16 if {(b__16 == BitVector::<12>::new(0b001100100000))} => {core_ctx.mcountinhibit.bits.zero_extend::<64>()}
        b__17 if {(b__17 == BitVector::<12>::new(0b111100010001))} => {core_ctx.mvendorid.zero_extend::<64>()}
        b__18 if {(b__18 == BitVector::<12>::new(0b111100010010))} => {core_ctx.marchid}
        b__19 if {(b__19 == BitVector::<12>::new(0b111100010011))} => {core_ctx.mimpid}
        b__20 if {(b__20 == BitVector::<12>::new(0b111100010100))} => {core_ctx.mhartid}
        b__21 if {(b__21 == BitVector::<12>::new(0b111100010101))} => {core_ctx.mconfigptr}
        b__22 if {(b__22 == BitVector::<12>::new(0b000100000000))} => {{
            let var_1 = core_ctx.mstatus;
            lower_mstatus(var_1)
        }.bits.subrange::<0, 64, 64>()}
        b__23 if {(b__23 == BitVector::<12>::new(0b000101000100))} => {{
            let var_2 = core_ctx.mip;
            let var_3 = core_ctx.mideleg;
            lower_mip(var_2, var_3)
        }.bits}
        b__24 if {(b__24 == BitVector::<12>::new(0b000100000100))} => {{
            let var_4 = core_ctx.mie;
            let var_5 = core_ctx.mideleg;
            lower_mie(var_4, var_5)
        }.bits}
        b__25 if {(b__25 == BitVector::<12>::new(0b000101000000))} => {core_ctx.sscratch}
        b__26 if {(b__26 == BitVector::<12>::new(0b000101000010))} => {core_ctx.scause.bits}
        b__27 if {(b__27 == BitVector::<12>::new(0b000101000011))} => {core_ctx.stval}
        b__28 if {(b__28 == BitVector::<12>::new(0b011110100000))} => {!(core_ctx.tselect)}
        v__3846 if {let idx_var_6: BitVector<4> = v__3846.subrange::<0, 4, 4>();
        ((v__3846.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111010)) && (bitvector_access(idx_var_6, 0) == false))} => {let idx: BitVector<4> = v__3846.subrange::<0, 4, 4>();
        pmpReadCfgReg(core_ctx, idx.unsigned())}
        v__3848 if {(v__3848.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111011))} => {let idx: BitVector<4> = v__3848.subrange::<0, 4, 4>();
        pmpReadAddrReg(core_ctx, bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b00), idx).unsigned())}
        v__3850 if {(v__3850.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111100))} => {let idx: BitVector<4> = v__3850.subrange::<0, 4, 4>();
        pmpReadAddrReg(core_ctx, bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b01), idx).unsigned())}
        v__3852 if {(v__3852.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111101))} => {let idx: BitVector<4> = v__3852.subrange::<0, 4, 4>();
        pmpReadAddrReg(core_ctx, bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b10), idx).unsigned())}
        v__3854 if {(v__3854.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111110))} => {let idx: BitVector<4> = v__3854.subrange::<0, 4, 4>();
        pmpReadAddrReg(core_ctx, bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b11), idx).unsigned())}
        b__29 if {(b__29 == BitVector::<12>::new(0b000000001000))} => {core_ctx.vstart.zero_extend::<64>()}
        b__30 if {(b__30 == BitVector::<12>::new(0b000000001001))} => {{
            let var_7 = core_ctx.vcsr;
            _get_Vcsr_vxsat(var_7)
        }.zero_extend::<64>()}
        b__31 if {(b__31 == BitVector::<12>::new(0b000000001010))} => {{
            let var_8 = core_ctx.vcsr;
            _get_Vcsr_vxrm(var_8)
        }.zero_extend::<64>()}
        b__32 if {(b__32 == BitVector::<12>::new(0b000000001111))} => {core_ctx.vcsr.bits.zero_extend::<64>()}
        b__33 if {(b__33 == BitVector::<12>::new(0b110000100000))} => {core_ctx.vl}
        b__34 if {(b__34 == BitVector::<12>::new(0b110000100001))} => {core_ctx.vtype.bits}
        b__35 if {(b__35 == BitVector::<12>::new(0b110000100010))} => {get_vlenb(())}
        b__36 if {(b__36 == BitVector::<12>::new(0b000100000101))} => {get_stvec(core_ctx, ())}
        b__37 if {(b__37 == BitVector::<12>::new(0b000101000001))} => {get_xepc(core_ctx, Privilege::Supervisor)}
        b__38 if {(b__38 == BitVector::<12>::new(0b001100000101))} => {get_mtvec(core_ctx, ())}
        b__39 if {(b__39 == BitVector::<12>::new(0b001101000001))} => {get_xepc(core_ctx, Privilege::Machine)}
        v__3856 if {let index_var_9: BitVector<5> = v__3856.subrange::<0, 5, 5>();
        ((v__3856.subrange::<5, 12, 7>() == BitVector::<7>::new(0b0011001)) && ((index_var_9.unsigned() >= 3) as bool))} => {let index: BitVector<5> = v__3856.subrange::<0, 5, 5>();
        read_mhpmevent(core_ctx, hpmidx_from_bits(index))}
        v__3858 if {let index_var_10: BitVector<5> = v__3858.subrange::<0, 5, 5>();
        ((v__3858.subrange::<5, 12, 7>() == BitVector::<7>::new(0b1011000)) && ((index_var_10.unsigned() >= 3) as bool))} => {let index: BitVector<5> = v__3858.subrange::<0, 5, 5>();
        read_mhpmcounter(core_ctx, hpmidx_from_bits(index))}
        v__3862 if {let index_var_11: BitVector<5> = v__3862.subrange::<0, 5, 5>();
        ((v__3862.subrange::<5, 12, 7>() == BitVector::<7>::new(0b1100000)) && ((index_var_11.unsigned() >= 3) as bool))} => {let index: BitVector<5> = v__3862.subrange::<0, 5, 5>();
        read_mhpmcounter(core_ctx, hpmidx_from_bits(index))}
        b__40 if {(b__40 == BitVector::<12>::new(0b110110100000))} => {{
            let var_12 = core_ctx.cur_privilege;
            get_scountovf(core_ctx, var_12)
        }.zero_extend::<64>()}
        b__41 if {(b__41 == BitVector::<12>::new(0b000000010101))} => {read_seed_csr(())}
        b__42 if {(b__42 == BitVector::<12>::new(0b110000000000))} => {subrange_bits::<_, 64>(core_ctx.mcycle, 63, 0)}
        b__43 if {(b__43 == BitVector::<12>::new(0b110000000001))} => {subrange_bits::<_, 64>(core_ctx.mtime, 63, 0)}
        b__44 if {(b__44 == BitVector::<12>::new(0b110000000010))} => {subrange_bits::<_, 64>(core_ctx.minstret, 63, 0)}
        b__48 if {(b__48 == BitVector::<12>::new(0b101100000000))} => {subrange_bits::<_, 64>(core_ctx.mcycle, 63, 0)}
        b__49 if {(b__49 == BitVector::<12>::new(0b101100000010))} => {subrange_bits::<_, 64>(core_ctx.minstret, 63, 0)}
        b__52 if {(b__52 == BitVector::<12>::new(0b000000000001))} => {{
            let var_13 = core_ctx.fcsr;
            _get_Fcsr_FFLAGS(var_13)
        }.zero_extend::<64>()}
        b__53 if {(b__53 == BitVector::<12>::new(0b000000000010))} => {{
            let var_14 = core_ctx.fcsr;
            _get_Fcsr_FRM(var_14)
        }.zero_extend::<64>()}
        b__54 if {(b__54 == BitVector::<12>::new(0b000000000011))} => {core_ctx.fcsr.bits.zero_extend::<64>()}
        b__55 if {(b__55 == BitVector::<12>::new(0b001100100001))} => {core_ctx.mcyclecfg.bits.subrange::<0, 64, 64>()}
        b__57 if {(b__57 == BitVector::<12>::new(0b001100100010))} => {core_ctx.minstretcfg.bits.subrange::<0, 64, 64>()}
        b__59 if {(b__59 == BitVector::<12>::new(0b000101001101))} => {subrange_bits::<_, 64>(core_ctx.stimecmp, 63, 0)}
        b__61 if {(b__61 == BitVector::<12>::new(0b000110000000))} => {core_ctx.satp}
        csr => {{
            panic!("{}, l {}: {}", "riscv_csr_end.sail", 17, format!("{}{}", "Read from CSR that does not exist: ", bits_str(csr)))
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// write_CSR
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L135.
pub fn write_CSR(core_ctx: &mut Core, merge_hashtag_var: BitVector<12>, missing_arg_0: BitVector<{
    64
}>) -> BitVector<{
    64
}> {
    match (merge_hashtag_var, missing_arg_0) {
        (b__0, value) if {(b__0 == BitVector::<12>::new(0b001100000001))} => {{
            core_ctx.misa = {
                let var_1 = core_ctx.misa;
                legalize_misa(core_ctx, var_1, value)
            };
            core_ctx.misa.bits
        }}
        (b__1, value) if {(b__1 == BitVector::<12>::new(0b001100000000))} => {{
            core_ctx.mstatus = {
                let var_2 = core_ctx.mstatus;
                legalize_mstatus(core_ctx, var_2, value)
            };
            core_ctx.mstatus.bits
        }}
        (b__5, value) if {(b__5 == BitVector::<12>::new(0b001100001010))} => {{
            core_ctx.menvcfg = {
                let var_3 = core_ctx.menvcfg;
                legalize_menvcfg(core_ctx, var_3, value)
            };
            core_ctx.menvcfg.bits
        }}
        (b__7, value) if {(b__7 == BitVector::<12>::new(0b000100001010))} => {{
            core_ctx.senvcfg = {
                let var_4 = core_ctx.senvcfg;
                legalize_senvcfg(core_ctx, var_4, value.zero_extend::<64>())
            };
            core_ctx.senvcfg.bits.subrange::<0, 64, 64>()
        }}
        (b__8, value) if {(b__8 == BitVector::<12>::new(0b001100000100))} => {{
            core_ctx.mie = {
                let var_5 = core_ctx.mie;
                legalize_mie(core_ctx, var_5, value)
            };
            core_ctx.mie.bits
        }}
        (b__9, value) if {(b__9 == BitVector::<12>::new(0b001101000100))} => {{
            core_ctx.mip = {
                let var_6 = core_ctx.mip;
                legalize_mip(core_ctx, var_6, value)
            };
            core_ctx.mip.bits
        }}
        (b__10, value) if {(b__10 == BitVector::<12>::new(0b001100000010))} => {{
            core_ctx.medeleg = {
                let var_7 = core_ctx.medeleg;
                legalize_medeleg(var_7, value)
            };
            core_ctx.medeleg.bits
        }}
        (b__13, value) if {(b__13 == BitVector::<12>::new(0b001100000011))} => {{
            core_ctx.mideleg = {
                let var_8 = core_ctx.mideleg;
                legalize_mideleg(var_8, value)
            };
            core_ctx.mideleg.bits
        }}
        (b__14, value) if {(b__14 == BitVector::<12>::new(0b001101000010))} => {{
            core_ctx.mcause.bits = value;
            core_ctx.mcause.bits
        }}
        (b__15, value) if {(b__15 == BitVector::<12>::new(0b001101000011))} => {{
            core_ctx.mtval = value;
            core_ctx.mtval
        }}
        (b__16, value) if {(b__16 == BitVector::<12>::new(0b001101000000))} => {{
            core_ctx.mscratch = value;
            core_ctx.mscratch
        }}
        (b__17, value) if {(b__17 == BitVector::<12>::new(0b000100000110))} => {{
            core_ctx.scounteren = {
                let var_9 = core_ctx.scounteren;
                legalize_scounteren(core_ctx, var_9, value)
            };
            core_ctx.scounteren.bits.zero_extend::<64>()
        }}
        (b__18, value) if {(b__18 == BitVector::<12>::new(0b001100000110))} => {{
            core_ctx.mcounteren = {
                let var_10 = core_ctx.mcounteren;
                legalize_mcounteren(core_ctx, var_10, value)
            };
            core_ctx.mcounteren.bits.zero_extend::<64>()
        }}
        (b__19, value) if {(b__19 == BitVector::<12>::new(0b001100100000))} => {{
            core_ctx.mcountinhibit = {
                let var_11 = core_ctx.mcountinhibit;
                legalize_mcountinhibit(core_ctx, var_11, value)
            };
            core_ctx.mcountinhibit.bits.zero_extend::<64>()
        }}
        (b__20, value) if {(b__20 == BitVector::<12>::new(0b000100000000))} => {{
            core_ctx.mstatus = {
                let var_12 = core_ctx.mstatus;
                legalize_sstatus(core_ctx, var_12, value)
            };
            core_ctx.mstatus.bits.subrange::<0, 64, 64>()
        }}
        (b__21, value) if {(b__21 == BitVector::<12>::new(0b000101000100))} => {{
            core_ctx.mip = {
                let var_13 = core_ctx.mip;
                let var_14 = core_ctx.mideleg;
                legalize_sip(var_13, var_14, value)
            };
            core_ctx.mip.bits
        }}
        (b__22, value) if {(b__22 == BitVector::<12>::new(0b000100000100))} => {{
            core_ctx.mie = {
                let var_15 = core_ctx.mie;
                let var_16 = core_ctx.mideleg;
                legalize_sie(var_15, var_16, value)
            };
            core_ctx.mie.bits
        }}
        (b__23, value) if {(b__23 == BitVector::<12>::new(0b000101000000))} => {{
            core_ctx.sscratch = value;
            core_ctx.sscratch
        }}
        (b__24, value) if {(b__24 == BitVector::<12>::new(0b000101000010))} => {{
            core_ctx.scause.bits = value;
            core_ctx.scause.bits
        }}
        (b__25, value) if {(b__25 == BitVector::<12>::new(0b000101000011))} => {{
            core_ctx.stval = value;
            core_ctx.stval
        }}
        (b__26, value) if {(b__26 == BitVector::<12>::new(0b011110100000))} => {{
            core_ctx.tselect = value;
            core_ctx.tselect
        }}
        (v__3868, value) if {let idx_var_17: BitVector<4> = v__3868.subrange::<0, 4, 4>();
        ((v__3868.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111010)) && (bitvector_access(idx_var_17, 0) == false))} => {let idx: BitVector<4> = v__3868.subrange::<0, 4, 4>();
        let idx = idx.unsigned();
        pmpWriteCfgReg(core_ctx, idx, value);
        pmpReadCfgReg(core_ctx, idx)}
        (v__3870, value) if {(v__3870.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111011))} => {let idx: BitVector<4> = v__3870.subrange::<0, 4, 4>();
        let idx = bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b00), idx).unsigned();
        pmpWriteAddrReg(core_ctx, idx, value);
        pmpReadAddrReg(core_ctx, idx)}
        (v__3872, value) if {(v__3872.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111100))} => {let idx: BitVector<4> = v__3872.subrange::<0, 4, 4>();
        let idx = bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b01), idx).unsigned();
        pmpWriteAddrReg(core_ctx, idx, value);
        pmpReadAddrReg(core_ctx, idx)}
        (v__3874, value) if {(v__3874.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111101))} => {let idx: BitVector<4> = v__3874.subrange::<0, 4, 4>();
        let idx = bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b10), idx).unsigned();
        pmpWriteAddrReg(core_ctx, idx, value);
        pmpReadAddrReg(core_ctx, idx)}
        (v__3876, value) if {(v__3876.subrange::<4, 12, 8>() == BitVector::<8>::new(0b00111110))} => {let idx: BitVector<4> = v__3876.subrange::<0, 4, 4>();
        let idx = bitvector_concat::<2, 4, 6>(BitVector::<2>::new(0b11), idx).unsigned();
        pmpWriteAddrReg(core_ctx, idx, value);
        pmpReadAddrReg(core_ctx, idx)}
        (b__27, value) if {(b__27 == BitVector::<12>::new(0b000000001000))} => {{
            set_vstart(core_ctx, value.subrange::<0, 16, 16>());
            core_ctx.vstart.zero_extend::<64>()
        }}
        (b__28, value) if {(b__28 == BitVector::<12>::new(0b000000001001))} => {{
            {
                let var_18 = {
                    let var_19 = core_ctx.vcsr;
                    _get_Vcsr_vxrm(var_19)
                };
                ext_write_vcsr(core_ctx, var_18, value.subrange::<0, 1, 1>())
            };
            {
                let var_20 = core_ctx.vcsr;
                _get_Vcsr_vxsat(var_20)
            }.zero_extend::<64>()
        }}
        (b__29, value) if {(b__29 == BitVector::<12>::new(0b000000001010))} => {{
            {
                let var_21 = {
                    let var_22 = core_ctx.vcsr;
                    _get_Vcsr_vxsat(var_22)
                };
                ext_write_vcsr(core_ctx, value.subrange::<0, 2, 2>(), var_21)
            };
            {
                let var_23 = core_ctx.vcsr;
                _get_Vcsr_vxrm(var_23)
            }.zero_extend::<64>()
        }}
        (b__30, value) if {(b__30 == BitVector::<12>::new(0b000000001111))} => {{
            ext_write_vcsr(core_ctx, value.subrange::<1, 3, 2>(), value.subrange::<0, 1, 1>());
            core_ctx.vcsr.bits.zero_extend::<64>()
        }}
        (b__31, value) if {(b__31 == BitVector::<12>::new(0b000100000101))} => {{
            set_stvec(core_ctx, value)
        }}
        (b__32, value) if {(b__32 == BitVector::<12>::new(0b000101000001))} => {{
            set_xepc(core_ctx, Privilege::Supervisor, value)
        }}
        (b__33, value) if {(b__33 == BitVector::<12>::new(0b001100000101))} => {{
            set_mtvec(core_ctx, value)
        }}
        (b__34, value) if {(b__34 == BitVector::<12>::new(0b001101000001))} => {{
            set_xepc(core_ctx, Privilege::Machine, value)
        }}
        (v__3878, value) if {let index_var_24: BitVector<5> = v__3878.subrange::<0, 5, 5>();
        ((v__3878.subrange::<5, 12, 7>() == BitVector::<7>::new(0b0011001)) && ((index_var_24.unsigned() >= 3) as bool))} => {let index: BitVector<5> = v__3878.subrange::<0, 5, 5>();
        let index = hpmidx_from_bits(index);
        write_mhpmevent(core_ctx, index, value);
        read_mhpmevent(core_ctx, index)}
        (v__3880, value) if {let index_var_25: BitVector<5> = v__3880.subrange::<0, 5, 5>();
        ((v__3880.subrange::<5, 12, 7>() == BitVector::<7>::new(0b1011000)) && ((index_var_25.unsigned() >= 3) as bool))} => {let index: BitVector<5> = v__3880.subrange::<0, 5, 5>();
        let index = hpmidx_from_bits(index);
        write_mhpmcounter(core_ctx, index, value);
        read_mhpmcounter(core_ctx, index)}
        (b__35, value) if {(b__35 == BitVector::<12>::new(0b000000010101))} => {write_seed_csr(())}
        (b__36, value) if {(b__36 == BitVector::<12>::new(0b101100000000))} => {{
            core_ctx.mcycle = core_ctx.mcycle.set_subrange::<0, 64, 64>(value);
            value
        }}
        (b__37, value) if {(b__37 == BitVector::<12>::new(0b101100000010))} => {{
            core_ctx.minstret = core_ctx.minstret.set_subrange::<0, 64, 64>(value);
            core_ctx.minstret_increment = false;
            value
        }}
        (b__40, value) if {(b__40 == BitVector::<12>::new(0b000000000001))} => {{
            {
                let var_26 = {
                    let var_27 = core_ctx.fcsr;
                    _get_Fcsr_FRM(var_27)
                };
                write_fcsr(core_ctx, var_26, value.subrange::<0, 5, 5>())
            };
            {
                let var_28 = core_ctx.fcsr;
                _get_Fcsr_FFLAGS(var_28)
            }.zero_extend::<64>()
        }}
        (b__41, value) if {(b__41 == BitVector::<12>::new(0b000000000010))} => {{
            {
                let var_29 = {
                    let var_30 = core_ctx.fcsr;
                    _get_Fcsr_FFLAGS(var_30)
                };
                write_fcsr(core_ctx, value.subrange::<0, 3, 3>(), var_29)
            };
            {
                let var_31 = core_ctx.fcsr;
                _get_Fcsr_FRM(var_31)
            }.zero_extend::<64>()
        }}
        (b__42, value) if {(b__42 == BitVector::<12>::new(0b000000000011))} => {{
            write_fcsr(core_ctx, value.subrange::<5, 8, 3>(), value.subrange::<0, 5, 5>());
            core_ctx.fcsr.bits.zero_extend::<64>()
        }}
        (b__43, value) if {(b__43 == BitVector::<12>::new(0b001100100001))} => {{
            core_ctx.mcyclecfg = {
                let var_32 = core_ctx.mcyclecfg;
                legalize_smcntrpmf(core_ctx, var_32, value)
            };
            core_ctx.mcyclecfg.bits
        }}
        (b__46, value) if {(b__46 == BitVector::<12>::new(0b001100100010))} => {{
            core_ctx.minstretcfg = {
                let var_33 = core_ctx.minstretcfg;
                legalize_smcntrpmf(core_ctx, var_33, value)
            };
            core_ctx.minstretcfg.bits.subrange::<0, 64, 64>()
        }}
        (b__49, value) if {(b__49 == BitVector::<12>::new(0b000101001101))} => {{
            core_ctx.stimecmp = core_ctx.stimecmp.set_subrange::<0, 64, 64>(value);
            subrange_bits::<_, 64>(core_ctx.stimecmp, 63, 0)
        }}
        (b__51, value) if {(b__51 == BitVector::<12>::new(0b000110000000))} => {{
            core_ctx.satp = {
                let var_34 = cur_architecture(core_ctx, ());
                let var_35 = core_ctx.satp;
                legalize_satp(core_ctx, var_34, var_35, value)
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
pub fn doCSR(core_ctx: &mut Core, csr: BitVector<12>, rs1_val: BitVector<{
    64
}>, rd: regidx, op: csrop, is_CSR_Write: bool) -> ExecutionResult {
    if {!({
        let var_1 = core_ctx.cur_privilege;
        check_CSR(core_ctx, csr, var_1, is_CSR_Write)
    })} {
        ExecutionResult::Illegal_Instruction(())
    } else if {!(true)} {
        ExecutionResult::Ext_CSR_Check_Failure(())
    } else {
        let is_CSR_Read = !(((op == csrop::CSRRW) && (rd == zreg)));
        let csr_val: xlenbits = if {is_CSR_Read} {
            read_CSR(core_ctx, csr)
        } else {
            zeros::<64>(64)
        };
        if {is_CSR_Write} {
            let new_val: xlenbits = match op {
                csrop::CSRRW => {rs1_val}
                csrop::CSRRS => {(csr_val | rs1_val)}
                csrop::CSRRC => {(csr_val & !(rs1_val))}
                _ => {panic!("Unreachable code")}
            };
            let final_val = write_CSR(core_ctx, csr, new_val);
            
        } else {
            csr_id_read_callback(csr, csr_val)
        };
        wX_bits(core_ctx, rd, csr_val);
        RETIRE_SUCCESS
    }
}

/// execute
/// 
/// Generated from the Sail sources at `riscv_insts_base.sail` L26-33.
pub fn execute(core_ctx: &mut Core, merge_hashtag_var: ast) -> ExecutionResult {
    match merge_hashtag_var {
        ast::UTYPE((imm, rd, op)) => {{
            let off: xlenbits = sign_extend(64, bitvector_concat::<20, 12, 32>(imm, BitVector::<12>::new(0b000000000000)));
            {
                let var_1 = match op {
                    uop::LUI => {off}
                    uop::AUIPC => {get_arch_pc(core_ctx, ()).wrapped_add(off)}
                    _ => {panic!("Unreachable code")}
                };
                wX_bits(core_ctx, rd, var_1)
            };
            RETIRE_SUCCESS
        }}
        ast::JAL((imm, rd)) => {{
            let target = core_ctx.PC.wrapped_add(sign_extend(64, imm));
            match ext_control_check_pc(target) {
                Ext_ControlAddr_Check::Ext_ControlAddr_Error(e) => {ExecutionResult::Ext_ControlAddr_Check_Failure(e)}
                Ext_ControlAddr_Check::Ext_ControlAddr_OK(target) => {{
                    let target_bits = bits_of_virtaddr(target);
                    if {(bit_to_bool(bitvector_access(target_bits, 1)) && !(currentlyEnabled(core_ctx, extension::Ext_Zca)))} {
                        ExecutionResult::Memory_Exception((target, ExceptionType::E_Fetch_Addr_Align(())))
                    } else {
                        {
                            let var_2 = get_next_pc(core_ctx, ());
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
                    let var_3 = rX_bits(core_ctx, rs1);
                    let var_4 = rX_bits(core_ctx, rs2);
                    _operator_smaller_s_(var_3, var_4)
                }}
                bop::BGE => {{
                    let var_5 = rX_bits(core_ctx, rs1);
                    let var_6 = rX_bits(core_ctx, rs2);
                    _operator_biggerequal_s_(var_5, var_6)
                }}
                bop::BLTU => {{
                    let var_7 = rX_bits(core_ctx, rs1);
                    let var_8 = rX_bits(core_ctx, rs2);
                    _operator_smaller_u_(var_7, var_8)
                }}
                bop::BGEU => {{
                    let var_9 = rX_bits(core_ctx, rs1);
                    let var_10 = rX_bits(core_ctx, rs2);
                    _operator_biggerequal_u_(var_9, var_10)
                }}
                _ => {panic!("Unreachable code")}
            };
            if {taken} {
                let target = core_ctx.PC.wrapped_add(sign_extend(64, imm));
                match ext_control_check_pc(target) {
                    Ext_ControlAddr_Check::Ext_ControlAddr_Error(e) => {ExecutionResult::Ext_ControlAddr_Check_Failure(e)}
                    Ext_ControlAddr_Check::Ext_ControlAddr_OK(target) => {{
                        let target_bits = bits_of_virtaddr(target);
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
                let var_11 = match op {
                    iop::ADDI => {rX_bits(core_ctx, rs1).wrapped_add(immext)}
                    iop::SLTI => {{
                        let var_12 = {
                            let var_13 = rX_bits(core_ctx, rs1);
                            _operator_smaller_s_(var_13, immext)
                        };
                        bool_to_bits(var_12)
                    }.zero_extend::<64>()}
                    iop::SLTIU => {{
                        let var_14 = {
                            let var_15 = rX_bits(core_ctx, rs1);
                            _operator_smaller_u_(var_15, immext)
                        };
                        bool_to_bits(var_14)
                    }.zero_extend::<64>()}
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
                let var_16 = match op {
                    sop::SLLI => {shift_bits_left(rX_bits(core_ctx, rs1), shamt)}
                    sop::SRLI => {shift_bits_right(rX_bits(core_ctx, rs1), shamt)}
                    sop::SRAI => {{
                        let var_17 = rX_bits(core_ctx, rs1);
                        shift_bits_right_arith(var_17, shamt)
                    }}
                    _ => {panic!("Unreachable code")}
                };
                wX_bits(core_ctx, rd, var_16)
            };
            RETIRE_SUCCESS
        }}
        ast::RTYPE((rs2, rs1, rd, op)) => {{
            {
                let var_18 = match op {
                    rop::ADD => {{
                        let var_19 = rX_bits(core_ctx, rs2);
                        rX_bits(core_ctx, rs1).wrapped_add(var_19)
                    }}
                    rop::SLT => {{
                        let var_20 = {
                            let var_21 = rX_bits(core_ctx, rs1);
                            let var_22 = rX_bits(core_ctx, rs2);
                            _operator_smaller_s_(var_21, var_22)
                        };
                        bool_to_bits(var_20)
                    }.zero_extend::<64>()}
                    rop::SLTU => {{
                        let var_23 = {
                            let var_24 = rX_bits(core_ctx, rs1);
                            let var_25 = rX_bits(core_ctx, rs2);
                            _operator_smaller_u_(var_24, var_25)
                        };
                        bool_to_bits(var_23)
                    }.zero_extend::<64>()}
                    rop::AND => {(rX_bits(core_ctx, rs1) & rX_bits(core_ctx, rs2))}
                    rop::OR => {(rX_bits(core_ctx, rs1) | rX_bits(core_ctx, rs2))}
                    rop::XOR => {(rX_bits(core_ctx, rs1) ^ rX_bits(core_ctx, rs2))}
                    rop::SLL => {shift_bits_left(rX_bits(core_ctx, rs1), subrange_bits::<_, 6>(rX_bits(core_ctx, rs2), 5, 0))}
                    rop::SRL => {shift_bits_right(rX_bits(core_ctx, rs1), subrange_bits::<_, 6>(rX_bits(core_ctx, rs2), 5, 0))}
                    rop::SUB => {sub_vec(rX_bits(core_ctx, rs1), rX_bits(core_ctx, rs2))}
                    rop::SRA => {{
                        let var_26 = rX_bits(core_ctx, rs1);
                        let var_27 = subrange_bits::<_, 6>(rX_bits(core_ctx, rs2), 5, 0);
                        shift_bits_right_arith(var_26, var_27)
                    }}
                    _ => {panic!("Unreachable code")}
                };
                wX_bits(core_ctx, rd, var_18)
            };
            RETIRE_SUCCESS
        }}
        ast::LOAD((imm, rs1, rd, is_unsigned, width, aq, rl)) => {todo!("Unsupported: 'LOAD'")}
        ast::STORE((imm, rs2, rs1, width, aq, rl)) => {todo!("Unsupported: 'STORE'")}
        ast::ADDIW((imm, rs1, rd)) => {{
            let result = rX_bits(core_ctx, rs1).wrapped_add(sign_extend(64, imm));
            wX_bits(core_ctx, rd, sign_extend(64, result.subrange::<0, 32, 32>()));
            RETIRE_SUCCESS
        }}
        ast::RTYPEW((rs2, rs1, rd, op)) => {{
            let rs1_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs1), 31, 0);
            let rs2_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs2), 31, 0);
            let result: BitVector<32> = match op {
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
            let rs1_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs1), 31, 0);
            let result: BitVector<32> = match op {
                sopw::SLLIW => {shift_bits_left(rs1_val, shamt)}
                sopw::SRLIW => {shift_bits_right(rs1_val, shamt)}
                sopw::SRAIW => {shift_bits_right_arith(rs1_val, shamt)}
                _ => {panic!("Unreachable code")}
            };
            wX_bits(core_ctx, rd, sign_extend(64, result));
            RETIRE_SUCCESS
        }}
        ast::FENCE_TSO((pred, succ)) => {{
            match (pred, succ) {
                (v__3816, v__3817) if {((v__3816.subrange::<0, 2, 2>() == BitVector::<2>::new(0b11)) && (v__3817.subrange::<0, 2, 2>() == BitVector::<2>::new(0b11)))} => {()}
                (v__3820, v__3821) if {((v__3820.subrange::<0, 2, 2>() == BitVector::<2>::new(0b00)) && (v__3821.subrange::<0, 2, 2>() == BitVector::<2>::new(0b00)))} => {()}
                _ => {{
                    panic!("Unsupported function: 'print'")
                }}
                _ => {panic!("Unreachable code")}
            };
            RETIRE_SUCCESS
        }}
        ast::ENTER_ANCHOR((rs1, rs2)) => {{
            if {(core_ctx.cur_privilege != Privilege::Machine)} {
                ExecutionResult::Illegal_Instruction(())
            } else {
                match get_highest_priority_pmp_sa(core_ctx, ()) {
                    Some(pmp0_sa) => {{
                        set_next_pc(core_ctx, pmp0_sa);
                        core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<3, 4, 1>(BitVector::<1>::new(0b0));
                        unlock_all_pmps(core_ctx, ());
                        RETIRE_SUCCESS
                    }}
                    None => {ExecutionResult::Illegal_Instruction(())}
                    _ => {panic!("Unreachable code")}
                }
            }
        }}
        ast::EXIT_ANCHOR((rs1, rs2)) => {{
            if {(core_ctx.cur_privilege != Privilege::Machine)} {
                ExecutionResult::Illegal_Instruction(())
            } else {
                if {lock_highest_priority_pmp(core_ctx, ())} {
                    {
                        let var_28 = rX_bits(core_ctx, rs1);
                        set_next_pc(core_ctx, var_28)
                    };
                    RETIRE_SUCCESS
                } else {
                    ExecutionResult::Illegal_Instruction(())
                }
            }
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
                    let var_29 = {
                        let var_30 = core_ctx.cur_privilege;
                        let var_31 = core_ctx.PC;
                        exception_handler(core_ctx, var_30, ctl_result::CTL_MRET(()), var_31)
                    };
                    set_next_pc(core_ctx, var_29)
                };
                RETIRE_SUCCESS
            }
        }}
        ast::SRET(()) => {{
            let sret_illegal: bool = match core_ctx.cur_privilege {
                Privilege::User => {true}
                Privilege::Supervisor => {(!(currentlyEnabled(core_ctx, extension::Ext_S)) || ({
                    let var_35 = core_ctx.mstatus;
                    _get_Mstatus_TSR(var_35)
                } == BitVector::<1>::new(0b1)))}
                Privilege::Machine => {!(currentlyEnabled(core_ctx, extension::Ext_S))}
                _ => {panic!("Unreachable code")}
            };
            if {sret_illegal} {
                ExecutionResult::Illegal_Instruction(())
            } else if {!(true)} {
                ExecutionResult::Ext_XRET_Priv_Failure(())
            } else {
                {
                    let var_32 = {
                        let var_33 = core_ctx.cur_privilege;
                        let var_34 = core_ctx.PC;
                        exception_handler(core_ctx, var_33, ctl_result::CTL_SRET(()), var_34)
                    };
                    set_next_pc(core_ctx, var_32)
                };
                RETIRE_SUCCESS
            }
        }}
        ast::EBREAK(()) => {ExecutionResult::Memory_Exception(({
            let var_36 = core_ctx.PC;
            virtaddr::Virtaddr(var_36)
        }, ExceptionType::E_Breakpoint(())))}
        ast::WFI(()) => {match core_ctx.cur_privilege {
            Privilege::Machine => {ExecutionResult::Wait_For_Interrupt(())}
            Privilege::Supervisor => {if {({
                let var_37 = core_ctx.mstatus;
                _get_Mstatus_TW(var_37)
            } == BitVector::<1>::new(0b1))} {
                ExecutionResult::Illegal_Instruction(())
            } else {
                ExecutionResult::Wait_For_Interrupt(())
            }}
            Privilege::User => {ExecutionResult::Illegal_Instruction(())}
            _ => {panic!("Unreachable code")}
        }}
        ast::SFENCE_VMA((rs1, rs2)) => {{
            let addr = if {(rs1 != zreg)} {
                Some(rX_bits(core_ctx, rs1))
            } else {
                None
            };
            let asid = if {(rs2 != zreg)} {
                Some(subrange_bits::<_, 16>(rX_bits(core_ctx, rs2), 15, 0))
            } else {
                None
            };
            match core_ctx.cur_privilege {
                Privilege::User => {ExecutionResult::Illegal_Instruction(())}
                Privilege::Supervisor => {match {
                    let var_38 = core_ctx.mstatus;
                    _get_Mstatus_TVM(var_38)
                } {
                    b__0 if {(b__0 == BitVector::<1>::new(0b1))} => {ExecutionResult::Illegal_Instruction(())}
                    _ => {{
                        flush_TLB(core_ctx, asid, addr);
                        RETIRE_SUCCESS
                    }}
                    _ => {panic!("Unreachable code")}
                }}
                Privilege::Machine => {{
                    flush_TLB(core_ctx, asid, addr);
                    RETIRE_SUCCESS
                }}
                _ => {panic!("Unreachable code")}
            }
        }}
        ast::FENCEI(()) => {{
            RETIRE_SUCCESS
        }}
        ast::LOADRES((aq, rl, rs1, width, rd)) => {todo!("Unsupported: 'LOADRES'")}
        ast::STORECON((aq, rl, rs2, rs1, width, rd)) => {todo!("Unsupported: 'STORECON'")}
        ast::AMO((op, aq, rl, rs2, rs1, width, rd)) => {todo!("Unsupported: 'AMO'")}
        ast::MUL((rs2, rs1, rd, mul_op)) => {{
            let rs1_val = rX_bits(core_ctx, rs1);
            let rs2_val = rX_bits(core_ctx, rs2);
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
            let result_wide = to_bits::<128>(128, (rs1_int * rs2_int));
            let result = if {mul_op.high} {
                result_wide.subrange::<64, 128, 64>()
            } else {
                result_wide.subrange::<0, 64, 64>()
            };
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::DIV((rs2, rs1, rd, s)) => {{
            let rs1_val = rX_bits(core_ctx, rs1);
            let rs2_val = rX_bits(core_ctx, rs2);
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
            wX_bits(core_ctx, rd, to_bits::<64>(64, q__quote));
            RETIRE_SUCCESS
        }}
        ast::REM((rs2, rs1, rd, s)) => {{
            let rs1_val = rX_bits(core_ctx, rs1);
            let rs2_val = rX_bits(core_ctx, rs2);
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
            wX_bits(core_ctx, rd, to_bits::<64>(64, r));
            RETIRE_SUCCESS
        }}
        ast::MULW((rs2, rs1, rd)) => {{
            let rs1_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs1), 31, 0);
            let rs2_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs2), 31, 0);
            let rs1_int: i128 = rs1_val.signed();
            let rs2_int: i128 = rs2_val.signed();
            let result32 = subrange_bits::<_, 32>(to_bits::<64>(64, (rs1_int * rs2_int)), 31, 0);
            let result: xlenbits = sign_extend(64, result32);
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::DIVW((rs2, rs1, rd, s)) => {{
            let rs1_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs1), 31, 0);
            let rs2_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs2), 31, 0);
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
            wX_bits(core_ctx, rd, sign_extend(64, to_bits::<32>(32, q__quote)));
            RETIRE_SUCCESS
        }}
        ast::REMW((rs2, rs1, rd, s)) => {{
            let rs1_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs1), 31, 0);
            let rs2_val = subrange_bits::<_, 32>(rX_bits(core_ctx, rs2), 31, 0);
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
            wX_bits(core_ctx, rd, sign_extend(64, to_bits::<32>(32, r)));
            RETIRE_SUCCESS
        }}
        ast::CSRReg((csr, rs1, rd, op)) => {{
            let var_39 = rX_bits(core_ctx, rs1);
            doCSR(core_ctx, csr, var_39, rd, op, ((op == csrop::CSRRW) || (rs1 != zreg)))
        }}
        ast::CSRImm((csr, imm, rd, op)) => {doCSR(core_ctx, csr, imm.zero_extend::<64>(), rd, op, ((op == csrop::CSRRW) || (imm != zeros::<5>(5))))}
        ast::FENCE_RESERVED((fm, pred, succ, rs, rd)) => {RETIRE_SUCCESS}
        ast::FENCEI_RESERVED((imm, rs, rd)) => {RETIRE_SUCCESS}
        ast::LOAD_FP((imm, rs1, rd, width)) => {todo!("Unsupported: 'LOAD_FP'")}
        ast::STORE_FP((imm, rs2, rs1, width)) => {todo!("Unsupported: 'STORE_FP'")}
        ast::SINVAL_VMA((rs1, rs2)) => {{
            execute(core_ctx, ast::SFENCE_VMA((rs1, rs2)))
        }}
        ast::SFENCE_W_INVAL(()) => {{
            if {(core_ctx.cur_privilege == Privilege::User)} {
                ExecutionResult::Illegal_Instruction(())
            } else {
                RETIRE_SUCCESS
            }
        }}
        ast::SFENCE_INVAL_IR(()) => {{
            if {(core_ctx.cur_privilege == Privilege::User)} {
                ExecutionResult::Illegal_Instruction(())
            } else {
                RETIRE_SUCCESS
            }
        }}
        ast::SLLIUW((shamt, rs1, rd)) => {{
            let rs1_val = rX_bits(core_ctx, rs1);
            let result: xlenbits = shift_bits_left(rs1_val.subrange::<0, 32, 32>().zero_extend::<64>(), shamt);
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::ZBA_RTYPEUW((rs2, rs1, rd, op)) => {{
            let rs1_val = rX_bits(core_ctx, rs1);
            let rs2_val = rX_bits(core_ctx, rs2);
            let shamt: BitVector<2> = match op {
                bropw_zba::ADDUW => {BitVector::<2>::new(0b00)}
                bropw_zba::SH1ADDUW => {BitVector::<2>::new(0b01)}
                bropw_zba::SH2ADDUW => {BitVector::<2>::new(0b10)}
                bropw_zba::SH3ADDUW => {BitVector::<2>::new(0b11)}
                _ => {panic!("Unreachable code")}
            };
            let result: xlenbits = shift_bits_left(rs1_val.subrange::<0, 32, 32>().zero_extend::<64>(), shamt).wrapped_add(rs2_val);
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::ZBA_RTYPE((rs2, rs1, rd, op)) => {{
            let rs1_val = rX_bits(core_ctx, rs1);
            let rs2_val = rX_bits(core_ctx, rs2);
            let shamt: BitVector<2> = match op {
                brop_zba::SH1ADD => {BitVector::<2>::new(0b01)}
                brop_zba::SH2ADD => {BitVector::<2>::new(0b10)}
                brop_zba::SH3ADD => {BitVector::<2>::new(0b11)}
                _ => {panic!("Unreachable code")}
            };
            let result: xlenbits = shift_bits_left(rs1_val, shamt).wrapped_add(rs2_val);
            wX_bits(core_ctx, rd, result);
            RETIRE_SUCCESS
        }}
        ast::RORIW((shamt, rs1, rd)) => {todo!("Unsupported: 'RORIW'")}
        ast::RORI((shamt, rs1, rd)) => {todo!("Unsupported: 'RORI'")}
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
        ast::JALR((imm, rs1, rd)) => {{
            let t: xlenbits = rX_bits(core_ctx, rs1).wrapped_add(sign_extend(64, imm));
            match ext_control_check_addr(t) {
                Ext_ControlAddr_Check::Ext_ControlAddr_Error(e) => {ExecutionResult::Ext_ControlAddr_Check_Failure(e)}
                Ext_ControlAddr_Check::Ext_ControlAddr_OK(addr) => {{
                    let target = bitvector_update(bits_of_virtaddr(addr), 0, false);
                    if {(bit_to_bool(bitvector_access(target, 1)) && !(currentlyEnabled(core_ctx, extension::Ext_Zca)))} {
                        ExecutionResult::Memory_Exception((addr, ExceptionType::E_Fetch_Addr_Align(())))
                    } else {
                        {
                            let var_45 = get_next_pc(core_ctx, ());
                            wX_bits(core_ctx, rd, var_45)
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
