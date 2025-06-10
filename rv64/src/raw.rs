#![allow(warnings)]

use softcore_prelude::*;

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
    pub pmpcfg_n: [Pmpcfg_ent;64],
    pub pmpaddr_n: [xlenbits;64],
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
    pub mhpmevent: [HpmEvent;32],
    pub mhpmcounter: [BitVector<64>;32],
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
    pub tlb: [Option<TLB_Entry>;num_tlb_entries],
    pub satp: xlenbits,
    pub hart_state: HartState,
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {
    pub extensions: ConfigExtensions,
    pub memory: ConfigMemory,
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
    pub grain: usize,
}

pub type xlenbits = BitVector<xlen>;

/// virtaddr
/// 
/// Generated from the Sail sources at `prelude_mem_addrtype.sail` L17.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum virtaddr {
    Virtaddr(xlenbits)
}

/// hex_bits_backwards
/// 
/// Generated from the Sail sources at `sail/lib/hex_bits.sail` L65.
pub fn hex_bits_backwards<const N: usize>(n: usize, str: &'static str) -> BitVector<N> {
    parse_hex_bits(n, str)
}

/// hex_bits_12_backwards_matches
/// 
/// Generated from the Sail sources.
pub fn hex_bits_12_backwards_matches(arg_hashtag_: &'static str) -> bool {
    match arg_hashtag_ {
        s => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

/// get_config_print_platform
/// 
/// Generated from the Sail sources at `prelude.sail` L75.
pub fn get_config_print_platform(unit_arg: ()) -> bool {
    false
}

/// zeros
/// 
/// Generated from the Sail sources at `prelude.sail` L84.
pub fn zeros<const N: usize>(n: usize) -> BitVector<N> {
    sail_zeros(n)
}

/// ones
/// 
/// Generated from the Sail sources at `prelude.sail` L87.
pub fn ones<const N: usize>(n: usize) -> BitVector<N> {
    sail_ones(n)
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

/// bool_to_bits
/// 
/// Generated from the Sail sources at `prelude.sail` L112.
pub fn bool_to_bits(x: bool) -> BitVector<1> {
    bool_bits_forwards(x)
}

/// to_bits
/// 
/// Generated from the Sail sources at `prelude.sail` L117.
pub fn to_bits<const L: usize>(l: usize, n: usize) -> BitVector<L> {
    get_slice_int(l, n, 0)
}

/// (operator >=_u)
/// 
/// Generated from the Sail sources at `prelude.sail` L144.
pub fn _operator_biggerequal_u_<const N: usize>(x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.as_usize() >= y.as_usize())
}

pub const max_mem_access: usize = 4096;

pub type mem_access_width = usize;

/// exception
/// 
/// Generated from the Sail sources at `riscv_errors.sail` L11-14.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_not_implemented(&'static str),
    Error_internal_error(())
}

pub const log2_xlen: usize = 6;

pub const log2_xlen_bytes: usize = 3;

pub const xlen_bytes: usize = 8;

pub const xlen: usize = 64;

pub const asidlen: usize = 16;

pub type asidbits = BitVector<asidlen>;

pub type flenbits = BitVector<flen>;

pub const flen_bytes: usize = 8;

pub const flen: usize = 64;

pub const vlenmax: usize = 65536;

pub const VLEN: usize = usize::pow(2, get_vlen_pow(core_ctx, ()));

pub type physaddrbits = BitVector<physaddrbits_len>;

pub const physaddrbits_len: usize = 64;

/// physaddr
/// 
/// Generated from the Sail sources at `prelude_mem_addrtype.sail` L16.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum physaddr {
    Physaddr(physaddrbits)
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
pub struct Mem_read_request<const N: usize, const VASIZE: usize, PA, TS, ARCH_AK> {
    pub access_kind: Access_kind<ARCH_AK>,
    pub va: Option<BitVector<VASIZE>>,
    pub pa: PA,
    pub translation: TS,
    pub size: usize,
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
        extension::Ext_Zcf => {((core_ctx.config.extensions.Zcf.supported as bool) && (xlen == 32))}
        extension::Ext_Zcmop => {core_ctx.config.extensions.Zcmop.supported}
        extension::Ext_C => {(hartSupports(core_ctx, extension::Ext_Zca) && ((hartSupports(core_ctx, extension::Ext_Zcf) || (!(hartSupports(core_ctx, extension::Ext_F)) || (xlen != 32))) && (hartSupports(core_ctx, extension::Ext_Zcd) || !(hartSupports(core_ctx, extension::Ext_D)))))}
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
        extension::Ext_Sv32 => {((core_ctx.config.extensions.Sv32.supported as bool) && (xlen == 32))}
        extension::Ext_Sv39 => {((core_ctx.config.extensions.Sv39.supported as bool) && (xlen == 64))}
        extension::Ext_Sv48 => {((core_ctx.config.extensions.Sv48.supported as bool) && (xlen == 64))}
        extension::Ext_Sv57 => {((core_ctx.config.extensions.Sv57.supported as bool) && (xlen == 64))}
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

pub const xlen_max_unsigned: usize = (usize::pow(2, xlen) - 1);

pub const xlen_max_signed: usize = (usize::pow(2, (xlen - 1)) - 1);

pub const xlen_min_signed: usize = (0 - usize::pow(2, (xlen - 1)));

pub type half = BitVector<16>;

pub type word = BitVector<32>;

pub type instbits = BitVector<32>;

pub const pagesize_bits: usize = 12;

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
    Regno(usize)
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

/// privLevel_to_bits
/// 
/// Generated from the Sail sources at `riscv_types.sail` L72.
pub fn privLevel_to_bits(p: Privilege) -> BitVector<2> {
    privLevel_bits_forwards(p)
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

/// rivvfunct6
/// 
/// Generated from the Sail sources at `riscv_vreg_type.sail` L94.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rivvfunct6 {
    IVV_VWREDSUMU,
    IVV_VWREDSUM
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
    Vregidx(BitVector<5>)
}

/// zvkfunct6
/// 
/// Generated from the Sail sources at `riscv_zvk_utils.sail` L24.
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
    C_NOP(()),
    C_ADDI4SPN((cregidx, BitVector<8>)),
    C_LW((BitVector<5>, cregidx, cregidx)),
    C_LD((BitVector<5>, cregidx, cregidx)),
    C_SW((BitVector<5>, cregidx, cregidx)),
    C_SD((BitVector<5>, cregidx, cregidx)),
    C_ADDI((BitVector<6>, regidx)),
    C_JAL(BitVector<11>),
    C_ADDIW((BitVector<6>, regidx)),
    C_LI((BitVector<6>, regidx)),
    C_ADDI16SP(BitVector<6>),
    C_LUI((BitVector<6>, regidx)),
    C_SRLI((BitVector<6>, cregidx)),
    C_SRAI((BitVector<6>, cregidx)),
    C_ANDI((BitVector<6>, cregidx)),
    C_SUB((cregidx, cregidx)),
    C_XOR((cregidx, cregidx)),
    C_OR((cregidx, cregidx)),
    C_AND((cregidx, cregidx)),
    C_SUBW((cregidx, cregidx)),
    C_ADDW((cregidx, cregidx)),
    C_J(BitVector<11>),
    C_BEQZ((BitVector<8>, cregidx)),
    C_BNEZ((BitVector<8>, cregidx)),
    C_SLLI((BitVector<6>, regidx)),
    C_LWSP((BitVector<6>, regidx)),
    C_LDSP((BitVector<6>, regidx)),
    C_SWSP((BitVector<6>, regidx)),
    C_SDSP((BitVector<6>, regidx)),
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
    CSRImm((csreg, BitVector<5>, regidx, csrop)),
    C_NOP_HINT(BitVector<6>),
    C_ADDI_HINT(regidx),
    C_LI_HINT(BitVector<6>),
    C_LUI_HINT(BitVector<6>),
    C_MV_HINT(regidx),
    C_ADD_HINT(regidx),
    C_SLLI_HINT((BitVector<6>, regidx)),
    C_SRLI_HINT(cregidx),
    C_SRAI_HINT(cregidx),
    FENCE_RESERVED((BitVector<4>, BitVector<4>, BitVector<4>, regidx, regidx)),
    FENCEI_RESERVED((BitVector<12>, regidx, regidx)),
    LOAD_FP((BitVector<12>, regidx, fregidx, word_width)),
    STORE_FP((BitVector<12>, fregidx, regidx, word_width)),
    F_MADD_TYPE_S((fregidx, fregidx, fregidx, rounding_mode, fregidx, f_madd_op_S)),
    F_BIN_RM_TYPE_S((fregidx, fregidx, rounding_mode, fregidx, f_bin_rm_op_S)),
    F_UN_RM_FF_TYPE_S((fregidx, rounding_mode, fregidx, f_un_rm_ff_op_S)),
    F_UN_RM_FX_TYPE_S((fregidx, rounding_mode, regidx, f_un_rm_fx_op_S)),
    F_UN_RM_XF_TYPE_S((regidx, rounding_mode, fregidx, f_un_rm_xf_op_S)),
    F_BIN_TYPE_F_S((fregidx, fregidx, fregidx, f_bin_op_f_S)),
    F_BIN_TYPE_X_S((fregidx, fregidx, regidx, f_bin_op_x_S)),
    F_UN_TYPE_F_S((regidx, fregidx, f_un_op_f_S)),
    F_UN_TYPE_X_S((fregidx, regidx, f_un_op_x_S)),
    C_FLWSP((BitVector<6>, fregidx)),
    C_FSWSP((BitVector<6>, fregidx)),
    C_FLW((BitVector<5>, cregidx, cregidx)),
    C_FSW((BitVector<5>, cregidx, cregidx)),
    F_MADD_TYPE_D((fregidx, fregidx, fregidx, rounding_mode, fregidx, f_madd_op_D)),
    F_BIN_RM_TYPE_D((fregidx, fregidx, rounding_mode, fregidx, f_bin_rm_op_D)),
    F_UN_RM_FF_TYPE_D((fregidx, rounding_mode, fregidx, f_un_rm_ff_op_D)),
    F_UN_RM_XF_TYPE_D((regidx, rounding_mode, fregidx, f_un_rm_xf_op_D)),
    F_UN_RM_FX_TYPE_D((fregidx, rounding_mode, regidx, f_un_rm_fx_op_D)),
    F_BIN_F_TYPE_D((fregidx, fregidx, fregidx, f_bin_f_op_D)),
    F_BIN_X_TYPE_D((fregidx, fregidx, regidx, f_bin_x_op_D)),
    F_UN_X_TYPE_D((fregidx, regidx, f_un_x_op_D)),
    F_UN_F_TYPE_D((regidx, fregidx, f_un_f_op_D)),
    C_FLDSP((BitVector<6>, fregidx)),
    C_FSDSP((BitVector<6>, fregidx)),
    C_FLD((BitVector<5>, cregidx, cregidx)),
    C_FSD((BitVector<5>, cregidx, cregidx)),
    SINVAL_VMA((regidx, regidx)),
    SFENCE_W_INVAL(()),
    SFENCE_INVAL_IR(()),
    SLLIUW((BitVector<6>, regidx, regidx)),
    ZBA_RTYPEUW((regidx, regidx, regidx, bropw_zba)),
    ZBA_RTYPE((regidx, regidx, regidx, brop_zba)),
    RORIW((BitVector<5>, regidx, regidx)),
    RORI((BitVector<6>, regidx, regidx)),
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
    ZBS_IOP((BitVector<6>, regidx, regidx, biop_zbs)),
    ZBS_RTYPE((regidx, regidx, regidx, brop_zbs)),
    C_LBU((BitVector<2>, cregidx, cregidx)),
    C_LHU((BitVector<2>, cregidx, cregidx)),
    C_LH((BitVector<2>, cregidx, cregidx)),
    C_SB((BitVector<2>, cregidx, cregidx)),
    C_SH((BitVector<2>, cregidx, cregidx)),
    C_ZEXT_B(cregidx),
    C_SEXT_B(cregidx),
    C_ZEXT_H(cregidx),
    C_SEXT_H(cregidx),
    C_ZEXT_W(cregidx),
    C_NOT(cregidx),
    C_MUL((cregidx, cregidx)),
    F_BIN_RM_TYPE_H((fregidx, fregidx, rounding_mode, fregidx, f_bin_rm_op_H)),
    F_MADD_TYPE_H((fregidx, fregidx, fregidx, rounding_mode, fregidx, f_madd_op_H)),
    F_BIN_F_TYPE_H((fregidx, fregidx, fregidx, f_bin_f_op_H)),
    F_BIN_X_TYPE_H((fregidx, fregidx, regidx, f_bin_x_op_H)),
    F_UN_RM_FF_TYPE_H((fregidx, rounding_mode, fregidx, f_un_rm_ff_op_H)),
    F_UN_RM_FX_TYPE_H((fregidx, rounding_mode, regidx, f_un_rm_fx_op_H)),
    F_UN_RM_XF_TYPE_H((regidx, rounding_mode, fregidx, f_un_rm_xf_op_H)),
    F_UN_F_TYPE_H((regidx, fregidx, f_un_f_op_H)),
    F_UN_X_TYPE_H((fregidx, regidx, f_un_x_op_H)),
    FLI_H((BitVector<5>, fregidx)),
    FLI_S((BitVector<5>, fregidx)),
    FLI_D((BitVector<5>, fregidx)),
    FMINM_H((fregidx, fregidx, fregidx)),
    FMAXM_H((fregidx, fregidx, fregidx)),
    FMINM_S((fregidx, fregidx, fregidx)),
    FMAXM_S((fregidx, fregidx, fregidx)),
    FMINM_D((fregidx, fregidx, fregidx)),
    FMAXM_D((fregidx, fregidx, fregidx)),
    FROUND_H((fregidx, rounding_mode, fregidx)),
    FROUNDNX_H((fregidx, rounding_mode, fregidx)),
    FROUND_S((fregidx, rounding_mode, fregidx)),
    FROUNDNX_S((fregidx, rounding_mode, fregidx)),
    FROUND_D((fregidx, rounding_mode, fregidx)),
    FROUNDNX_D((fregidx, rounding_mode, fregidx)),
    FMVH_X_D((fregidx, regidx)),
    FMVP_D_X((regidx, regidx, fregidx)),
    FLEQ_H((fregidx, fregidx, regidx)),
    FLTQ_H((fregidx, fregidx, regidx)),
    FLEQ_S((fregidx, fregidx, regidx)),
    FLTQ_S((fregidx, fregidx, regidx)),
    FLEQ_D((fregidx, fregidx, regidx)),
    FLTQ_D((fregidx, fregidx, regidx)),
    FCVTMOD_W_D((fregidx, regidx)),
    SHA256SIG0((regidx, regidx)),
    SHA256SIG1((regidx, regidx)),
    SHA256SUM0((regidx, regidx)),
    SHA256SUM1((regidx, regidx)),
    AES32ESMI((BitVector<2>, regidx, regidx, regidx)),
    AES32ESI((BitVector<2>, regidx, regidx, regidx)),
    AES32DSMI((BitVector<2>, regidx, regidx, regidx)),
    AES32DSI((BitVector<2>, regidx, regidx, regidx)),
    SHA512SIG0L((regidx, regidx, regidx)),
    SHA512SIG0H((regidx, regidx, regidx)),
    SHA512SIG1L((regidx, regidx, regidx)),
    SHA512SIG1H((regidx, regidx, regidx)),
    SHA512SUM0R((regidx, regidx, regidx)),
    SHA512SUM1R((regidx, regidx, regidx)),
    AES64KS1I((BitVector<4>, regidx, regidx)),
    AES64KS2((regidx, regidx, regidx)),
    AES64IM((regidx, regidx)),
    AES64ESM((regidx, regidx, regidx)),
    AES64ES((regidx, regidx, regidx)),
    AES64DSM((regidx, regidx, regidx)),
    AES64DS((regidx, regidx, regidx)),
    SHA512SIG0((regidx, regidx)),
    SHA512SIG1((regidx, regidx)),
    SHA512SUM0((regidx, regidx)),
    SHA512SUM1((regidx, regidx)),
    SM3P0((regidx, regidx)),
    SM3P1((regidx, regidx)),
    SM4ED((BitVector<2>, regidx, regidx, regidx)),
    SM4KS((BitVector<2>, regidx, regidx, regidx)),
    ZBKB_RTYPE((regidx, regidx, regidx, brop_zbkb)),
    ZBKB_PACKW((regidx, regidx, regidx)),
    ZIP((regidx, regidx)),
    UNZIP((regidx, regidx)),
    BREV8((regidx, regidx)),
    XPERM8((regidx, regidx, regidx)),
    XPERM4((regidx, regidx, regidx)),
    ZICOND_RTYPE((regidx, regidx, regidx, zicondop)),
    VSETVLI((BitVector<1>, BitVector<1>, BitVector<3>, BitVector<3>, regidx, regidx)),
    VSETVL((regidx, regidx, regidx)),
    VSETIVLI((BitVector<1>, BitVector<1>, BitVector<3>, BitVector<3>, BitVector<5>, regidx)),
    VVTYPE((vvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    NVSTYPE((nvsfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    NVTYPE((nvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    MASKTYPEV((vregidx, vregidx, vregidx)),
    MOVETYPEV((vregidx, vregidx)),
    VXTYPE((vxfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    NXSTYPE((nxsfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    NXTYPE((nxfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    VXSG((vxsgfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    MASKTYPEX((vregidx, regidx, vregidx)),
    MOVETYPEX((regidx, vregidx)),
    VITYPE((vifunct6, BitVector<1>, vregidx, BitVector<5>, vregidx)),
    NISTYPE((nisfunct6, BitVector<1>, vregidx, BitVector<5>, vregidx)),
    NITYPE((nifunct6, BitVector<1>, vregidx, BitVector<5>, vregidx)),
    VISG((visgfunct6, BitVector<1>, vregidx, BitVector<5>, vregidx)),
    MASKTYPEI((vregidx, BitVector<5>, vregidx)),
    MOVETYPEI((vregidx, BitVector<5>)),
    VMVRTYPE((vregidx, BitVector<5>, vregidx)),
    MVVTYPE((mvvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    MVVMATYPE((mvvmafunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    WVVTYPE((wvvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    WVTYPE((wvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    WMVVTYPE((wmvvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    VEXT2TYPE((vext2funct6, BitVector<1>, vregidx, vregidx)),
    VEXT4TYPE((vext4funct6, BitVector<1>, vregidx, vregidx)),
    VEXT8TYPE((vext8funct6, BitVector<1>, vregidx, vregidx)),
    VMVXS((vregidx, regidx)),
    MVVCOMPRESS((vregidx, vregidx, vregidx)),
    MVXTYPE((mvxfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    MVXMATYPE((mvxmafunct6, BitVector<1>, vregidx, regidx, vregidx)),
    WVXTYPE((wvxfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    WXTYPE((wxfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    WMVXTYPE((wmvxfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    VMVSX((regidx, vregidx)),
    FVVTYPE((fvvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    FVVMATYPE((fvvmafunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    FWVVTYPE((fwvvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    FWVVMATYPE((fwvvmafunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    FWVTYPE((fwvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    VFUNARY0((BitVector<1>, vregidx, vfunary0, vregidx)),
    VFWUNARY0((BitVector<1>, vregidx, vfwunary0, vregidx)),
    VFNUNARY0((BitVector<1>, vregidx, vfnunary0, vregidx)),
    VFUNARY1((BitVector<1>, vregidx, vfunary1, vregidx)),
    VFMVFS((vregidx, fregidx)),
    FVFTYPE((fvffunct6, BitVector<1>, vregidx, fregidx, vregidx)),
    FVFMATYPE((fvfmafunct6, BitVector<1>, vregidx, fregidx, vregidx)),
    FWVFTYPE((fwvffunct6, BitVector<1>, vregidx, fregidx, vregidx)),
    FWVFMATYPE((fwvfmafunct6, BitVector<1>, fregidx, vregidx, vregidx)),
    FWFTYPE((fwffunct6, BitVector<1>, vregidx, fregidx, vregidx)),
    VFMERGE((vregidx, fregidx, vregidx)),
    VFMV((fregidx, vregidx)),
    VFMVSF((fregidx, vregidx)),
    VLSEGTYPE((BitVector<3>, BitVector<1>, regidx, vlewidth, vregidx)),
    VLSEGFFTYPE((BitVector<3>, BitVector<1>, regidx, vlewidth, vregidx)),
    VSSEGTYPE((BitVector<3>, BitVector<1>, regidx, vlewidth, vregidx)),
    VLSSEGTYPE((BitVector<3>, BitVector<1>, regidx, regidx, vlewidth, vregidx)),
    VSSSEGTYPE((BitVector<3>, BitVector<1>, regidx, regidx, vlewidth, vregidx)),
    VLUXSEGTYPE((BitVector<3>, BitVector<1>, vregidx, regidx, vlewidth, vregidx)),
    VLOXSEGTYPE((BitVector<3>, BitVector<1>, vregidx, regidx, vlewidth, vregidx)),
    VSUXSEGTYPE((BitVector<3>, BitVector<1>, vregidx, regidx, vlewidth, vregidx)),
    VSOXSEGTYPE((BitVector<3>, BitVector<1>, vregidx, regidx, vlewidth, vregidx)),
    VLRETYPE((BitVector<3>, regidx, vlewidth, vregidx)),
    VSRETYPE((BitVector<3>, regidx, vregidx)),
    VMTYPE((regidx, vregidx, vmlsop)),
    MMTYPE((mmfunct6, vregidx, vregidx, vregidx)),
    VCPOP_M((BitVector<1>, vregidx, regidx)),
    VFIRST_M((BitVector<1>, vregidx, regidx)),
    VMSBF_M((BitVector<1>, vregidx, vregidx)),
    VMSIF_M((BitVector<1>, vregidx, vregidx)),
    VMSOF_M((BitVector<1>, vregidx, vregidx)),
    VIOTA_M((BitVector<1>, vregidx, vregidx)),
    VID_V((BitVector<1>, vregidx)),
    VVMTYPE((vvmfunct6, vregidx, vregidx, vregidx)),
    VVMCTYPE((vvmcfunct6, vregidx, vregidx, vregidx)),
    VVMSTYPE((vvmsfunct6, vregidx, vregidx, vregidx)),
    VVCMPTYPE((vvcmpfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    VXMTYPE((vxmfunct6, vregidx, regidx, vregidx)),
    VXMCTYPE((vxmcfunct6, vregidx, regidx, vregidx)),
    VXMSTYPE((vxmsfunct6, vregidx, regidx, vregidx)),
    VXCMPTYPE((vxcmpfunct6, BitVector<1>, vregidx, regidx, vregidx)),
    VIMTYPE((vimfunct6, vregidx, BitVector<5>, vregidx)),
    VIMCTYPE((vimcfunct6, vregidx, BitVector<5>, vregidx)),
    VIMSTYPE((vimsfunct6, vregidx, BitVector<5>, vregidx)),
    VICMPTYPE((vicmpfunct6, BitVector<1>, vregidx, BitVector<5>, vregidx)),
    FVVMTYPE((fvvmfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    FVFMTYPE((fvfmfunct6, BitVector<1>, vregidx, fregidx, vregidx)),
    RIVVTYPE((rivvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    RMVVTYPE((rmvvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    RFVVTYPE((rfvvfunct6, BitVector<1>, vregidx, vregidx, vregidx)),
    ZICBOM((cbop_zicbom, regidx)),
    ZICBOZ(regidx),
    VANDN_VV((BitVector<1>, vregidx, vregidx, vregidx)),
    VANDN_VX((BitVector<1>, vregidx, regidx, vregidx)),
    VBREV_V((BitVector<1>, vregidx, vregidx)),
    VBREV8_V((BitVector<1>, vregidx, vregidx)),
    VREV8_V((BitVector<1>, vregidx, vregidx)),
    VCLZ_V((BitVector<1>, vregidx, vregidx)),
    VCTZ_V((BitVector<1>, vregidx, vregidx)),
    VCPOP_V((BitVector<1>, vregidx, vregidx)),
    VROL_VV((BitVector<1>, vregidx, vregidx, vregidx)),
    VROL_VX((BitVector<1>, vregidx, regidx, vregidx)),
    VROR_VV((BitVector<1>, vregidx, vregidx, vregidx)),
    VROR_VX((BitVector<1>, vregidx, regidx, vregidx)),
    VROR_VI((BitVector<1>, vregidx, BitVector<5>, vregidx)),
    VWSLL_VV((BitVector<1>, vregidx, vregidx, vregidx)),
    VWSLL_VX((BitVector<1>, vregidx, regidx, vregidx)),
    VWSLL_VI((BitVector<1>, vregidx, BitVector<5>, vregidx)),
    VCLMUL_VV((BitVector<1>, vregidx, vregidx, vregidx)),
    VCLMUL_VX((BitVector<1>, vregidx, regidx, vregidx)),
    VCLMULH_VV((BitVector<1>, vregidx, vregidx, vregidx)),
    VCLMULH_VX((BitVector<1>, vregidx, regidx, vregidx)),
    VSHA2MS_VV((vregidx, vregidx, vregidx)),
    ZVKSHA2TYPE((zvkfunct6, vregidx, vregidx, vregidx)),
    VSM3ME_VV((vregidx, vregidx, vregidx)),
    VSM3C_VI((vregidx, BitVector<5>, vregidx)),
    ZIMOP_MOP_R((BitVector<5>, regidx, regidx)),
    ZIMOP_MOP_RR((BitVector<3>, regidx, regidx, regidx)),
    ZCMOP(BitVector<3>)
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

pub type csrRW = BitVector<2>;

pub type level_range<const V: usize> = usize;

pub type ext_access_type = ();

pub const Data: ext_access_type = ();

pub const default_write_acc: ext_access_type = Data;

/// trap_callback
/// 
/// Generated from the Sail sources at `riscv_callbacks.sail` L32.
pub fn trap_callback(unit_arg: ()) {
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

pub type regtype = xlenbits;

pub const zero_reg: regtype = zeros(64);

pub type fregtype = flenbits;

pub const zero_freg: fregtype = zeros(64);

/// Misa
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L49-78.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Misa {
    pub bits: BitVector<{
    (usize::pow(2, 3) * 8)
}>,
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
    (usize::pow(2, 3) * 8)
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
    (usize::pow(2, 3) * 8)
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
    (usize::pow(2, 3) * 8)
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
    (usize::pow(2, 3) * 8)
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
    (usize::pow(2, 3) * 8)
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
    (usize::pow(2, 3) * 8)
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

/// _get_Misa_C
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_C(v: Misa) -> BitVector<1> {
    v.bits.subrange::<2, 3, 1>()
}

/// _get_Misa_D
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_D(v: Misa) -> BitVector<1> {
    v.bits.subrange::<3, 4, 1>()
}

/// _get_Misa_F
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_F(v: Misa) -> BitVector<1> {
    v.bits.subrange::<5, 6, 1>()
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

/// _get_Misa_U
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_U(v: Misa) -> BitVector<1> {
    v.bits.subrange::<20, 21, 1>()
}

/// _get_Misa_V
/// 
/// Generated from the Sail sources.
pub fn _get_Misa_V(v: Misa) -> BitVector<1> {
    v.bits.subrange::<21, 22, 1>()
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

/// sys_pmp_grain
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L97.
pub fn sys_pmp_grain(core_ctx: &mut Core, unit_arg: ()) -> usize {
    core_ctx.config.memory.pmp.grain
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
        } == BitVector::<1>::new(0b1)) && (({
            let var_7 = core_ctx.mstatus;
            _get_Mstatus_FS(var_7)
        } != BitVector::<2>::new(0b00)) && (flen >= 64))))}
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
        extension::Ext_Zdinx => {(hartSupports(core_ctx, extension::Ext_Zdinx) && (flen >= 64))}
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

/// _get_Mstatus_MIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_MIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<3, 4, 1>()
}

/// _get_Mstatus_SIE
/// 
/// Generated from the Sail sources.
pub fn _get_Mstatus_SIE(v: Mstatus) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
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

/// _get_Minterrupts_MSI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_MSI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<3, 4, 1>()
}

/// _get_Minterrupts_MTI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_MTI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_Minterrupts_SEI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_SEI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<9, 10, 1>()
}

/// _get_Minterrupts_SSI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_SSI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<1, 2, 1>()
}

/// _get_Minterrupts_STI
/// 
/// Generated from the Sail sources.
pub fn _get_Minterrupts_STI(v: Minterrupts) -> BitVector<1> {
    v.bits.subrange::<5, 6, 1>()
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
    (usize::pow(2, 3) * 8)
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

/// agtype
/// 
/// Generated from the Sail sources at `riscv_sys_regs.sail` L985.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum agtype {
    UNDISTURBED,
    AGNOSTIC
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

/// pmpReadAddrReg
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L69-89.
pub fn pmpReadAddrReg(core_ctx: &mut Core, n: usize) -> BitVector<{
    (usize::pow(2, 3) * 8)
}> {
    let G = sys_pmp_grain(core_ctx, ());
    let match_type = _get_Pmpcfg_ent_A(core_ctx.pmpcfg_n[n]);
    let addr = core_ctx.pmpaddr_n[n];
    match bitvector_access(match_type, 1) {
        true if {(G >= 2)} => {{
            let mask: xlenbits = ones::<64>(min_int((G - 1), xlen)).zero_extend::<64>();
            (addr | mask)
        }}
        false if {(G >= 1)} => {{
            let mask: xlenbits = ones::<64>(min_int(G, xlen)).zero_extend::<64>();
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
pub fn pmpWriteCfg(core_ctx: &mut Core, n: usize, cfg: Pmpcfg_ent, v: BitVector<8>) -> Pmpcfg_ent {
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
pub fn pmpWriteCfgReg(core_ctx: &mut Core, n: usize, v: BitVector<{
    (usize::pow(2, 3) * 8)
}>) {
    if {(xlen == 32)} {
        for i in 0..=3 {
            let idx = ((n * 4) + i);
            core_ctx.pmpcfg_n[idx] = pmpWriteCfg(core_ctx, idx, core_ctx.pmpcfg_n[idx], subrange_bits(v, ((8 * i) + 7), (8 * i)))
        }
    } else {
        assert!(((n % 2) == 0), "Unexpected pmp config reg write");
        for i in 0..=7 {
            let idx = ((n * 4) + i);
            core_ctx.pmpcfg_n[idx] = pmpWriteCfg(core_ctx, idx, core_ctx.pmpcfg_n[idx], subrange_bits(v, ((8 * i) + 7), (8 * i)))
        }
    }
}

/// pmpWriteAddr
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L136-139.
pub fn pmpWriteAddr(locked: bool, tor_locked: bool, reg: BitVector<{
    (usize::pow(2, 3) * 8)
}>, v: BitVector<{
    (usize::pow(2, 3) * 8)
}>) -> BitVector<{
    (usize::pow(2, 3) * 8)
}> {
    if {(xlen == 32)} {
        if {(locked || tor_locked)} {
            reg
        } else {
            v
        }
    } else {
        if {(locked || tor_locked)} {
            reg
        } else {
            v.subrange::<0, 54, 54>().zero_extend::<64>()
        }
    }
}

/// pmpWriteAddrReg
/// 
/// Generated from the Sail sources at `riscv_pmp_regs.sail` L141-148.
pub fn pmpWriteAddrReg(core_ctx: &mut Core, n: usize, v: BitVector<{
    (usize::pow(2, 3) * 8)
}>) {
    core_ctx.pmpaddr_n[n] = {
        let var_1 = if {((n + 1) < 64)} {
            pmpTORLocked(core_ctx.pmpcfg_n[(n + 1)])
        } else {
            false
        };
        pmpWriteAddr(pmpLocked(core_ctx.pmpcfg_n[n]), var_1, core_ctx.pmpaddr_n[n], v)
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
    (usize::pow(2, 3) * 8)
}>, ent: Pmpcfg_ent, pmpaddr: BitVector<{
    (usize::pow(2, 3) * 8)
}>, prev_pmpaddr: BitVector<{
    (usize::pow(2, 3) * 8)
}>) -> pmpAddrMatch {
    let addr = addr.as_usize();
    let width = width.as_usize();
    match pmpAddrMatchType_of_bits(_get_Pmpcfg_ent_A(ent)) {
        PmpAddrMatchType::OFF => {pmpAddrMatch::PMP_NoMatch}
        PmpAddrMatchType::TOR => {{
            if {_operator_biggerequal_u_(prev_pmpaddr, pmpaddr)} {
                pmpAddrMatch::PMP_NoMatch
            } else {
                pmpRangeMatch(((prev_pmpaddr.as_usize() * 4) as u128), ((pmpaddr.as_usize() * 4) as u128), (addr as u128), (width as u128))
            }
        }}
        PmpAddrMatchType::NA4 => {{
            assert!((sys_pmp_grain(core_ctx, ()) < 1), "NA4 cannot be selected when PMP grain G >= 1.");
            let begin = (pmpaddr.as_usize() * 4);
            pmpRangeMatch((begin as u128), ((begin + 4) as u128), (addr as u128), (width as u128))
        }}
        PmpAddrMatchType::NAPOT => {{
            let mask = (pmpaddr ^ (pmpaddr + 1));
            let begin_words = (pmpaddr & !(mask)).as_usize();
            let end_words = ((begin_words + mask.as_usize()) + 1);
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
    (usize::pow(2, 3) * 8)
}>, acc: AccessType<()>, _priv_: Privilege, ent: Pmpcfg_ent, pmpaddr: BitVector<{
    (usize::pow(2, 3) * 8)
}>, prev_pmpaddr: BitVector<{
    (usize::pow(2, 3) * 8)
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
pub fn pmpCheck<const N: usize>(core_ctx: &mut Core, addr: physaddr, width: usize, acc: AccessType<()>, _priv_: Privilege) -> Option<ExceptionType> {
    let width: xlenbits = to_bits(xlen, width);
    for i in 0..=63 {
        let prev_pmpaddr = if {gt_int(i, 0)} {
            pmpReadAddrReg(core_ctx, (i - 1))
        } else {
            zeros(64)
        };
        match {
            let var_1 = pmpReadAddrReg(core_ctx, i);
            pmpMatchEntry(core_ctx, addr, width, acc, _priv_, core_ctx.pmpcfg_n[i], var_1, prev_pmpaddr)
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

pub type ext_data_addr_error = ();

pub type vreglenbits = BitVector<vlenmax>;

pub type vregtype = vreglenbits;

/// maskfunct3
/// 
/// Generated from the Sail sources at `riscv_vreg_type.sail` L108.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum maskfunct3 {
    VV_VMERGE,
    VI_VMERGE,
    VX_VMERGE
}

/// vregno
/// 
/// Generated from the Sail sources at `riscv_vext_regs.sail` L10.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregno {
    Vregno(usize)
}

pub const zvreg: vregidx = vregidx::Vregidx(BitVector::<5>::new(0b00000));

pub type ext_exception = ();

/// handle_trap_extension
/// 
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L18.
pub fn handle_trap_extension(p: Privilege, pc: BitVector<{
    (usize::pow(2, 3) * 8)
}>, u: Option<()>) {
    ()
}

/// prepare_trap_vector
/// 
/// Generated from the Sail sources at `riscv_sys_exceptions.sail` L21-31.
pub fn prepare_trap_vector(core_ctx: &mut Core, p: Privilege, cause: Mcause) -> BitVector<{
    (usize::pow(2, 3) * 8)
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

/// sync_exception
/// 
/// Generated from the Sail sources at `riscv_sync_exception.sail` L11-15.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct sync_exception {
    pub trap: ExceptionType,
    pub excinfo: Option<xlenbits>,
    pub ext: Option<ext_exception>,
}

pub type hpmidx = usize;

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
    Fregno(usize)
}

/// findPendingInterrupt
/// 
/// Generated from the Sail sources at `riscv_sys_control.sail` L116-125.
pub fn findPendingInterrupt(ip: BitVector<{
    (usize::pow(2, 3) * 8)
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
    (usize::pow(2, 3) * 8)
}>, Privilege)> {
    assert!((currentlyEnabled(core_ctx, extension::Ext_S) || (core_ctx.mideleg.bits == zeros(64))), "riscv_sys_control.sail:137.58-137.59");
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
pub fn tval(excinfo: Option<BitVector<{
    (usize::pow(2, 3) * 8)
}>>) -> BitVector<{
    (usize::pow(2, 3) * 8)
}> {
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
pub fn trap_handler(core_ctx: &mut Core, del_priv: Privilege, intr: bool, c: BitVector<8>, pc: BitVector<{
    (usize::pow(2, 3) * 8)
}>, info: Option<BitVector<{
    (usize::pow(2, 3) * 8)
}>>, ext: Option<()>) -> BitVector<{
    (usize::pow(2, 3) * 8)
}> {
    trap_callback(());
    if {get_config_print_platform(())} {
        print_platform(format!("{}{}", "handling ", format!("{}{}", if {intr} {
            "int_hashtag_"
        } else {
            "exc_hashtag_"
        }, format!("{}{}", bits_str(c), format!("{}{}", " at priv ", format!("{}{}", privLevel_to_str(del_priv), format!("{}{}", " with tval ", bits_str(tval(info)))))))))
    } else {
        ()
    };
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
            handle_trap_extension(del_priv, pc, ext);
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
            core_ctx.mstatus.bits = core_ctx.mstatus.bits.set_subrange::<8, 9, 1>(match core_ctx.cur_privilege {
                Privilege::User => {BitVector::<1>::new(0b0)}
                Privilege::Supervisor => {BitVector::<1>::new(0b1)}
                Privilege::Machine => {panic!("{}, l {}: {}", "riscv_sys_control.sail", 260, "invalid privilege for s-mode trap")}
                _ => {panic!("Unreachable code")}
            });
            core_ctx.stval = tval(info);
            core_ctx.sepc = pc;
            core_ctx.cur_privilege = del_priv;
            handle_trap_extension(del_priv, pc, ext);
            track_trap(core_ctx, del_priv);
            {
                let var_8 = core_ctx.scause;
                prepare_trap_vector(core_ctx, del_priv, var_8)
            }
        }}
        Privilege::User => {panic!("{}, l {}: {}", "riscv_sys_control.sail", 273, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

pub type MemoryOpResult<A> = result<A, ExceptionType>;

pub const MSIP_BASE: physaddrbits = BitVector::<20>::new(0b00000000000000000000).zero_extend::<64>();

pub const MTIMECMP_BASE: physaddrbits = BitVector::<20>::new(0b00000100000000000000).zero_extend::<64>();

pub const MTIMECMP_BASE_HI: physaddrbits = BitVector::<20>::new(0b00000100000000000100).zero_extend::<64>();

pub const MTIME_BASE: physaddrbits = BitVector::<20>::new(0b00001011111111111000).zero_extend::<64>();

pub const MTIME_BASE_HI: physaddrbits = BitVector::<20>::new(0b00001011111111111100).zero_extend::<64>();

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

pub const default_sv32_ext_pte: pte_ext_bits = zeros(10);

/// PTE_Check
/// 
/// Generated from the Sail sources at `riscv_vmem_pte.sail` L94-97.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PTE_Check {
    PTE_Check_Success(ext_ptw),
    PTE_Check_Failure((ext_ptw, ext_ptw_fail))
}

pub const tlb_vpn_bits: usize = 45;

pub const tlb_ppn_bits: usize = 44;

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

pub const num_tlb_entries: usize = 64;

pub type tlb_index_range = usize;

/// TR_Result
/// 
/// Generated from the Sail sources at `riscv_vmem.sail` L212-215.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TR_Result<PADDR, FAILURE> {
    TR_Address((PADDR, ext_ptw)),
    TR_Failure((FAILURE, ext_ptw))
}

pub const sm4_sbox_table: [BitVector<8>;256] = [BitVector::<8>::new(0b11010110), BitVector::<8>::new(0b10010000), BitVector::<8>::new(0b11101001), BitVector::<8>::new(0b11111110), BitVector::<8>::new(0b11001100), BitVector::<8>::new(0b11100001), BitVector::<8>::new(0b00111101), BitVector::<8>::new(0b10110111), BitVector::<8>::new(0b00010110), BitVector::<8>::new(0b10110110), BitVector::<8>::new(0b00010100), BitVector::<8>::new(0b11000010), BitVector::<8>::new(0b00101000), BitVector::<8>::new(0b11111011), BitVector::<8>::new(0b00101100), BitVector::<8>::new(0b00000101), BitVector::<8>::new(0b00101011), BitVector::<8>::new(0b01100111), BitVector::<8>::new(0b10011010), BitVector::<8>::new(0b01110110), BitVector::<8>::new(0b00101010), BitVector::<8>::new(0b10111110), BitVector::<8>::new(0b00000100), BitVector::<8>::new(0b11000011), BitVector::<8>::new(0b10101010), BitVector::<8>::new(0b01000100), BitVector::<8>::new(0b00010011), BitVector::<8>::new(0b00100110), BitVector::<8>::new(0b01001001), BitVector::<8>::new(0b10000110), BitVector::<8>::new(0b00000110), BitVector::<8>::new(0b10011001), BitVector::<8>::new(0b10011100), BitVector::<8>::new(0b01000010), BitVector::<8>::new(0b01010000), BitVector::<8>::new(0b11110100), BitVector::<8>::new(0b10010001), BitVector::<8>::new(0b11101111), BitVector::<8>::new(0b10011000), BitVector::<8>::new(0b01111010), BitVector::<8>::new(0b00110011), BitVector::<8>::new(0b01010100), BitVector::<8>::new(0b00001011), BitVector::<8>::new(0b01000011), BitVector::<8>::new(0b11101101), BitVector::<8>::new(0b11001111), BitVector::<8>::new(0b10101100), BitVector::<8>::new(0b01100010), BitVector::<8>::new(0b11100100), BitVector::<8>::new(0b10110011), BitVector::<8>::new(0b00011100), BitVector::<8>::new(0b10101001), BitVector::<8>::new(0b11001001), BitVector::<8>::new(0b00001000), BitVector::<8>::new(0b11101000), BitVector::<8>::new(0b10010101), BitVector::<8>::new(0b10000000), BitVector::<8>::new(0b11011111), BitVector::<8>::new(0b10010100), BitVector::<8>::new(0b11111010), BitVector::<8>::new(0b01110101), BitVector::<8>::new(0b10001111), BitVector::<8>::new(0b00111111), BitVector::<8>::new(0b10100110), BitVector::<8>::new(0b01000111), BitVector::<8>::new(0b00000111), BitVector::<8>::new(0b10100111), BitVector::<8>::new(0b11111100), BitVector::<8>::new(0b11110011), BitVector::<8>::new(0b01110011), BitVector::<8>::new(0b00010111), BitVector::<8>::new(0b10111010), BitVector::<8>::new(0b10000011), BitVector::<8>::new(0b01011001), BitVector::<8>::new(0b00111100), BitVector::<8>::new(0b00011001), BitVector::<8>::new(0b11100110), BitVector::<8>::new(0b10000101), BitVector::<8>::new(0b01001111), BitVector::<8>::new(0b10101000), BitVector::<8>::new(0b01101000), BitVector::<8>::new(0b01101011), BitVector::<8>::new(0b10000001), BitVector::<8>::new(0b10110010), BitVector::<8>::new(0b01110001), BitVector::<8>::new(0b01100100), BitVector::<8>::new(0b11011010), BitVector::<8>::new(0b10001011), BitVector::<8>::new(0b11111000), BitVector::<8>::new(0b11101011), BitVector::<8>::new(0b00001111), BitVector::<8>::new(0b01001011), BitVector::<8>::new(0b01110000), BitVector::<8>::new(0b01010110), BitVector::<8>::new(0b10011101), BitVector::<8>::new(0b00110101), BitVector::<8>::new(0b00011110), BitVector::<8>::new(0b00100100), BitVector::<8>::new(0b00001110), BitVector::<8>::new(0b01011110), BitVector::<8>::new(0b01100011), BitVector::<8>::new(0b01011000), BitVector::<8>::new(0b11010001), BitVector::<8>::new(0b10100010), BitVector::<8>::new(0b00100101), BitVector::<8>::new(0b00100010), BitVector::<8>::new(0b01111100), BitVector::<8>::new(0b00111011), BitVector::<8>::new(0b00000001), BitVector::<8>::new(0b00100001), BitVector::<8>::new(0b01111000), BitVector::<8>::new(0b10000111), BitVector::<8>::new(0b11010100), BitVector::<8>::new(0b00000000), BitVector::<8>::new(0b01000110), BitVector::<8>::new(0b01010111), BitVector::<8>::new(0b10011111), BitVector::<8>::new(0b11010011), BitVector::<8>::new(0b00100111), BitVector::<8>::new(0b01010010), BitVector::<8>::new(0b01001100), BitVector::<8>::new(0b00110110), BitVector::<8>::new(0b00000010), BitVector::<8>::new(0b11100111), BitVector::<8>::new(0b10100000), BitVector::<8>::new(0b11000100), BitVector::<8>::new(0b11001000), BitVector::<8>::new(0b10011110), BitVector::<8>::new(0b11101010), BitVector::<8>::new(0b10111111), BitVector::<8>::new(0b10001010), BitVector::<8>::new(0b11010010), BitVector::<8>::new(0b01000000), BitVector::<8>::new(0b11000111), BitVector::<8>::new(0b00111000), BitVector::<8>::new(0b10110101), BitVector::<8>::new(0b10100011), BitVector::<8>::new(0b11110111), BitVector::<8>::new(0b11110010), BitVector::<8>::new(0b11001110), BitVector::<8>::new(0b11111001), BitVector::<8>::new(0b01100001), BitVector::<8>::new(0b00010101), BitVector::<8>::new(0b10100001), BitVector::<8>::new(0b11100000), BitVector::<8>::new(0b10101110), BitVector::<8>::new(0b01011101), BitVector::<8>::new(0b10100100), BitVector::<8>::new(0b10011011), BitVector::<8>::new(0b00110100), BitVector::<8>::new(0b00011010), BitVector::<8>::new(0b01010101), BitVector::<8>::new(0b10101101), BitVector::<8>::new(0b10010011), BitVector::<8>::new(0b00110010), BitVector::<8>::new(0b00110000), BitVector::<8>::new(0b11110101), BitVector::<8>::new(0b10001100), BitVector::<8>::new(0b10110001), BitVector::<8>::new(0b11100011), BitVector::<8>::new(0b00011101), BitVector::<8>::new(0b11110110), BitVector::<8>::new(0b11100010), BitVector::<8>::new(0b00101110), BitVector::<8>::new(0b10000010), BitVector::<8>::new(0b01100110), BitVector::<8>::new(0b11001010), BitVector::<8>::new(0b01100000), BitVector::<8>::new(0b11000000), BitVector::<8>::new(0b00101001), BitVector::<8>::new(0b00100011), BitVector::<8>::new(0b10101011), BitVector::<8>::new(0b00001101), BitVector::<8>::new(0b01010011), BitVector::<8>::new(0b01001110), BitVector::<8>::new(0b01101111), BitVector::<8>::new(0b11010101), BitVector::<8>::new(0b11011011), BitVector::<8>::new(0b00110111), BitVector::<8>::new(0b01000101), BitVector::<8>::new(0b11011110), BitVector::<8>::new(0b11111101), BitVector::<8>::new(0b10001110), BitVector::<8>::new(0b00101111), BitVector::<8>::new(0b00000011), BitVector::<8>::new(0b11111111), BitVector::<8>::new(0b01101010), BitVector::<8>::new(0b01110010), BitVector::<8>::new(0b01101101), BitVector::<8>::new(0b01101100), BitVector::<8>::new(0b01011011), BitVector::<8>::new(0b01010001), BitVector::<8>::new(0b10001101), BitVector::<8>::new(0b00011011), BitVector::<8>::new(0b10101111), BitVector::<8>::new(0b10010010), BitVector::<8>::new(0b10111011), BitVector::<8>::new(0b11011101), BitVector::<8>::new(0b10111100), BitVector::<8>::new(0b01111111), BitVector::<8>::new(0b00010001), BitVector::<8>::new(0b11011001), BitVector::<8>::new(0b01011100), BitVector::<8>::new(0b01000001), BitVector::<8>::new(0b00011111), BitVector::<8>::new(0b00010000), BitVector::<8>::new(0b01011010), BitVector::<8>::new(0b11011000), BitVector::<8>::new(0b00001010), BitVector::<8>::new(0b11000001), BitVector::<8>::new(0b00110001), BitVector::<8>::new(0b10001000), BitVector::<8>::new(0b10100101), BitVector::<8>::new(0b11001101), BitVector::<8>::new(0b01111011), BitVector::<8>::new(0b10111101), BitVector::<8>::new(0b00101101), BitVector::<8>::new(0b01110100), BitVector::<8>::new(0b11010000), BitVector::<8>::new(0b00010010), BitVector::<8>::new(0b10111000), BitVector::<8>::new(0b11100101), BitVector::<8>::new(0b10110100), BitVector::<8>::new(0b10110000), BitVector::<8>::new(0b10001001), BitVector::<8>::new(0b01101001), BitVector::<8>::new(0b10010111), BitVector::<8>::new(0b01001010), BitVector::<8>::new(0b00001100), BitVector::<8>::new(0b10010110), BitVector::<8>::new(0b01110111), BitVector::<8>::new(0b01111110), BitVector::<8>::new(0b01100101), BitVector::<8>::new(0b10111001), BitVector::<8>::new(0b11110001), BitVector::<8>::new(0b00001001), BitVector::<8>::new(0b11000101), BitVector::<8>::new(0b01101110), BitVector::<8>::new(0b11000110), BitVector::<8>::new(0b10000100), BitVector::<8>::new(0b00011000), BitVector::<8>::new(0b11110000), BitVector::<8>::new(0b01111101), BitVector::<8>::new(0b11101100), BitVector::<8>::new(0b00111010), BitVector::<8>::new(0b11011100), BitVector::<8>::new(0b01001101), BitVector::<8>::new(0b00100000), BitVector::<8>::new(0b01111001), BitVector::<8>::new(0b11101110), BitVector::<8>::new(0b01011111), BitVector::<8>::new(0b00111110), BitVector::<8>::new(0b11010111), BitVector::<8>::new(0b11001011), BitVector::<8>::new(0b00111001), BitVector::<8>::new(0b01001000)];

pub const aes_sbox_fwd_table: [BitVector<8>;256] = [BitVector::<8>::new(0b01100011), BitVector::<8>::new(0b01111100), BitVector::<8>::new(0b01110111), BitVector::<8>::new(0b01111011), BitVector::<8>::new(0b11110010), BitVector::<8>::new(0b01101011), BitVector::<8>::new(0b01101111), BitVector::<8>::new(0b11000101), BitVector::<8>::new(0b00110000), BitVector::<8>::new(0b00000001), BitVector::<8>::new(0b01100111), BitVector::<8>::new(0b00101011), BitVector::<8>::new(0b11111110), BitVector::<8>::new(0b11010111), BitVector::<8>::new(0b10101011), BitVector::<8>::new(0b01110110), BitVector::<8>::new(0b11001010), BitVector::<8>::new(0b10000010), BitVector::<8>::new(0b11001001), BitVector::<8>::new(0b01111101), BitVector::<8>::new(0b11111010), BitVector::<8>::new(0b01011001), BitVector::<8>::new(0b01000111), BitVector::<8>::new(0b11110000), BitVector::<8>::new(0b10101101), BitVector::<8>::new(0b11010100), BitVector::<8>::new(0b10100010), BitVector::<8>::new(0b10101111), BitVector::<8>::new(0b10011100), BitVector::<8>::new(0b10100100), BitVector::<8>::new(0b01110010), BitVector::<8>::new(0b11000000), BitVector::<8>::new(0b10110111), BitVector::<8>::new(0b11111101), BitVector::<8>::new(0b10010011), BitVector::<8>::new(0b00100110), BitVector::<8>::new(0b00110110), BitVector::<8>::new(0b00111111), BitVector::<8>::new(0b11110111), BitVector::<8>::new(0b11001100), BitVector::<8>::new(0b00110100), BitVector::<8>::new(0b10100101), BitVector::<8>::new(0b11100101), BitVector::<8>::new(0b11110001), BitVector::<8>::new(0b01110001), BitVector::<8>::new(0b11011000), BitVector::<8>::new(0b00110001), BitVector::<8>::new(0b00010101), BitVector::<8>::new(0b00000100), BitVector::<8>::new(0b11000111), BitVector::<8>::new(0b00100011), BitVector::<8>::new(0b11000011), BitVector::<8>::new(0b00011000), BitVector::<8>::new(0b10010110), BitVector::<8>::new(0b00000101), BitVector::<8>::new(0b10011010), BitVector::<8>::new(0b00000111), BitVector::<8>::new(0b00010010), BitVector::<8>::new(0b10000000), BitVector::<8>::new(0b11100010), BitVector::<8>::new(0b11101011), BitVector::<8>::new(0b00100111), BitVector::<8>::new(0b10110010), BitVector::<8>::new(0b01110101), BitVector::<8>::new(0b00001001), BitVector::<8>::new(0b10000011), BitVector::<8>::new(0b00101100), BitVector::<8>::new(0b00011010), BitVector::<8>::new(0b00011011), BitVector::<8>::new(0b01101110), BitVector::<8>::new(0b01011010), BitVector::<8>::new(0b10100000), BitVector::<8>::new(0b01010010), BitVector::<8>::new(0b00111011), BitVector::<8>::new(0b11010110), BitVector::<8>::new(0b10110011), BitVector::<8>::new(0b00101001), BitVector::<8>::new(0b11100011), BitVector::<8>::new(0b00101111), BitVector::<8>::new(0b10000100), BitVector::<8>::new(0b01010011), BitVector::<8>::new(0b11010001), BitVector::<8>::new(0b00000000), BitVector::<8>::new(0b11101101), BitVector::<8>::new(0b00100000), BitVector::<8>::new(0b11111100), BitVector::<8>::new(0b10110001), BitVector::<8>::new(0b01011011), BitVector::<8>::new(0b01101010), BitVector::<8>::new(0b11001011), BitVector::<8>::new(0b10111110), BitVector::<8>::new(0b00111001), BitVector::<8>::new(0b01001010), BitVector::<8>::new(0b01001100), BitVector::<8>::new(0b01011000), BitVector::<8>::new(0b11001111), BitVector::<8>::new(0b11010000), BitVector::<8>::new(0b11101111), BitVector::<8>::new(0b10101010), BitVector::<8>::new(0b11111011), BitVector::<8>::new(0b01000011), BitVector::<8>::new(0b01001101), BitVector::<8>::new(0b00110011), BitVector::<8>::new(0b10000101), BitVector::<8>::new(0b01000101), BitVector::<8>::new(0b11111001), BitVector::<8>::new(0b00000010), BitVector::<8>::new(0b01111111), BitVector::<8>::new(0b01010000), BitVector::<8>::new(0b00111100), BitVector::<8>::new(0b10011111), BitVector::<8>::new(0b10101000), BitVector::<8>::new(0b01010001), BitVector::<8>::new(0b10100011), BitVector::<8>::new(0b01000000), BitVector::<8>::new(0b10001111), BitVector::<8>::new(0b10010010), BitVector::<8>::new(0b10011101), BitVector::<8>::new(0b00111000), BitVector::<8>::new(0b11110101), BitVector::<8>::new(0b10111100), BitVector::<8>::new(0b10110110), BitVector::<8>::new(0b11011010), BitVector::<8>::new(0b00100001), BitVector::<8>::new(0b00010000), BitVector::<8>::new(0b11111111), BitVector::<8>::new(0b11110011), BitVector::<8>::new(0b11010010), BitVector::<8>::new(0b11001101), BitVector::<8>::new(0b00001100), BitVector::<8>::new(0b00010011), BitVector::<8>::new(0b11101100), BitVector::<8>::new(0b01011111), BitVector::<8>::new(0b10010111), BitVector::<8>::new(0b01000100), BitVector::<8>::new(0b00010111), BitVector::<8>::new(0b11000100), BitVector::<8>::new(0b10100111), BitVector::<8>::new(0b01111110), BitVector::<8>::new(0b00111101), BitVector::<8>::new(0b01100100), BitVector::<8>::new(0b01011101), BitVector::<8>::new(0b00011001), BitVector::<8>::new(0b01110011), BitVector::<8>::new(0b01100000), BitVector::<8>::new(0b10000001), BitVector::<8>::new(0b01001111), BitVector::<8>::new(0b11011100), BitVector::<8>::new(0b00100010), BitVector::<8>::new(0b00101010), BitVector::<8>::new(0b10010000), BitVector::<8>::new(0b10001000), BitVector::<8>::new(0b01000110), BitVector::<8>::new(0b11101110), BitVector::<8>::new(0b10111000), BitVector::<8>::new(0b00010100), BitVector::<8>::new(0b11011110), BitVector::<8>::new(0b01011110), BitVector::<8>::new(0b00001011), BitVector::<8>::new(0b11011011), BitVector::<8>::new(0b11100000), BitVector::<8>::new(0b00110010), BitVector::<8>::new(0b00111010), BitVector::<8>::new(0b00001010), BitVector::<8>::new(0b01001001), BitVector::<8>::new(0b00000110), BitVector::<8>::new(0b00100100), BitVector::<8>::new(0b01011100), BitVector::<8>::new(0b11000010), BitVector::<8>::new(0b11010011), BitVector::<8>::new(0b10101100), BitVector::<8>::new(0b01100010), BitVector::<8>::new(0b10010001), BitVector::<8>::new(0b10010101), BitVector::<8>::new(0b11100100), BitVector::<8>::new(0b01111001), BitVector::<8>::new(0b11100111), BitVector::<8>::new(0b11001000), BitVector::<8>::new(0b00110111), BitVector::<8>::new(0b01101101), BitVector::<8>::new(0b10001101), BitVector::<8>::new(0b11010101), BitVector::<8>::new(0b01001110), BitVector::<8>::new(0b10101001), BitVector::<8>::new(0b01101100), BitVector::<8>::new(0b01010110), BitVector::<8>::new(0b11110100), BitVector::<8>::new(0b11101010), BitVector::<8>::new(0b01100101), BitVector::<8>::new(0b01111010), BitVector::<8>::new(0b10101110), BitVector::<8>::new(0b00001000), BitVector::<8>::new(0b10111010), BitVector::<8>::new(0b01111000), BitVector::<8>::new(0b00100101), BitVector::<8>::new(0b00101110), BitVector::<8>::new(0b00011100), BitVector::<8>::new(0b10100110), BitVector::<8>::new(0b10110100), BitVector::<8>::new(0b11000110), BitVector::<8>::new(0b11101000), BitVector::<8>::new(0b11011101), BitVector::<8>::new(0b01110100), BitVector::<8>::new(0b00011111), BitVector::<8>::new(0b01001011), BitVector::<8>::new(0b10111101), BitVector::<8>::new(0b10001011), BitVector::<8>::new(0b10001010), BitVector::<8>::new(0b01110000), BitVector::<8>::new(0b00111110), BitVector::<8>::new(0b10110101), BitVector::<8>::new(0b01100110), BitVector::<8>::new(0b01001000), BitVector::<8>::new(0b00000011), BitVector::<8>::new(0b11110110), BitVector::<8>::new(0b00001110), BitVector::<8>::new(0b01100001), BitVector::<8>::new(0b00110101), BitVector::<8>::new(0b01010111), BitVector::<8>::new(0b10111001), BitVector::<8>::new(0b10000110), BitVector::<8>::new(0b11000001), BitVector::<8>::new(0b00011101), BitVector::<8>::new(0b10011110), BitVector::<8>::new(0b11100001), BitVector::<8>::new(0b11111000), BitVector::<8>::new(0b10011000), BitVector::<8>::new(0b00010001), BitVector::<8>::new(0b01101001), BitVector::<8>::new(0b11011001), BitVector::<8>::new(0b10001110), BitVector::<8>::new(0b10010100), BitVector::<8>::new(0b10011011), BitVector::<8>::new(0b00011110), BitVector::<8>::new(0b10000111), BitVector::<8>::new(0b11101001), BitVector::<8>::new(0b11001110), BitVector::<8>::new(0b01010101), BitVector::<8>::new(0b00101000), BitVector::<8>::new(0b11011111), BitVector::<8>::new(0b10001100), BitVector::<8>::new(0b10100001), BitVector::<8>::new(0b10001001), BitVector::<8>::new(0b00001101), BitVector::<8>::new(0b10111111), BitVector::<8>::new(0b11100110), BitVector::<8>::new(0b01000010), BitVector::<8>::new(0b01101000), BitVector::<8>::new(0b01000001), BitVector::<8>::new(0b10011001), BitVector::<8>::new(0b00101101), BitVector::<8>::new(0b00001111), BitVector::<8>::new(0b10110000), BitVector::<8>::new(0b01010100), BitVector::<8>::new(0b10111011), BitVector::<8>::new(0b00010110)];

pub const aes_sbox_inv_table: [BitVector<8>;256] = [BitVector::<8>::new(0b01010010), BitVector::<8>::new(0b00001001), BitVector::<8>::new(0b01101010), BitVector::<8>::new(0b11010101), BitVector::<8>::new(0b00110000), BitVector::<8>::new(0b00110110), BitVector::<8>::new(0b10100101), BitVector::<8>::new(0b00111000), BitVector::<8>::new(0b10111111), BitVector::<8>::new(0b01000000), BitVector::<8>::new(0b10100011), BitVector::<8>::new(0b10011110), BitVector::<8>::new(0b10000001), BitVector::<8>::new(0b11110011), BitVector::<8>::new(0b11010111), BitVector::<8>::new(0b11111011), BitVector::<8>::new(0b01111100), BitVector::<8>::new(0b11100011), BitVector::<8>::new(0b00111001), BitVector::<8>::new(0b10000010), BitVector::<8>::new(0b10011011), BitVector::<8>::new(0b00101111), BitVector::<8>::new(0b11111111), BitVector::<8>::new(0b10000111), BitVector::<8>::new(0b00110100), BitVector::<8>::new(0b10001110), BitVector::<8>::new(0b01000011), BitVector::<8>::new(0b01000100), BitVector::<8>::new(0b11000100), BitVector::<8>::new(0b11011110), BitVector::<8>::new(0b11101001), BitVector::<8>::new(0b11001011), BitVector::<8>::new(0b01010100), BitVector::<8>::new(0b01111011), BitVector::<8>::new(0b10010100), BitVector::<8>::new(0b00110010), BitVector::<8>::new(0b10100110), BitVector::<8>::new(0b11000010), BitVector::<8>::new(0b00100011), BitVector::<8>::new(0b00111101), BitVector::<8>::new(0b11101110), BitVector::<8>::new(0b01001100), BitVector::<8>::new(0b10010101), BitVector::<8>::new(0b00001011), BitVector::<8>::new(0b01000010), BitVector::<8>::new(0b11111010), BitVector::<8>::new(0b11000011), BitVector::<8>::new(0b01001110), BitVector::<8>::new(0b00001000), BitVector::<8>::new(0b00101110), BitVector::<8>::new(0b10100001), BitVector::<8>::new(0b01100110), BitVector::<8>::new(0b00101000), BitVector::<8>::new(0b11011001), BitVector::<8>::new(0b00100100), BitVector::<8>::new(0b10110010), BitVector::<8>::new(0b01110110), BitVector::<8>::new(0b01011011), BitVector::<8>::new(0b10100010), BitVector::<8>::new(0b01001001), BitVector::<8>::new(0b01101101), BitVector::<8>::new(0b10001011), BitVector::<8>::new(0b11010001), BitVector::<8>::new(0b00100101), BitVector::<8>::new(0b01110010), BitVector::<8>::new(0b11111000), BitVector::<8>::new(0b11110110), BitVector::<8>::new(0b01100100), BitVector::<8>::new(0b10000110), BitVector::<8>::new(0b01101000), BitVector::<8>::new(0b10011000), BitVector::<8>::new(0b00010110), BitVector::<8>::new(0b11010100), BitVector::<8>::new(0b10100100), BitVector::<8>::new(0b01011100), BitVector::<8>::new(0b11001100), BitVector::<8>::new(0b01011101), BitVector::<8>::new(0b01100101), BitVector::<8>::new(0b10110110), BitVector::<8>::new(0b10010010), BitVector::<8>::new(0b01101100), BitVector::<8>::new(0b01110000), BitVector::<8>::new(0b01001000), BitVector::<8>::new(0b01010000), BitVector::<8>::new(0b11111101), BitVector::<8>::new(0b11101101), BitVector::<8>::new(0b10111001), BitVector::<8>::new(0b11011010), BitVector::<8>::new(0b01011110), BitVector::<8>::new(0b00010101), BitVector::<8>::new(0b01000110), BitVector::<8>::new(0b01010111), BitVector::<8>::new(0b10100111), BitVector::<8>::new(0b10001101), BitVector::<8>::new(0b10011101), BitVector::<8>::new(0b10000100), BitVector::<8>::new(0b10010000), BitVector::<8>::new(0b11011000), BitVector::<8>::new(0b10101011), BitVector::<8>::new(0b00000000), BitVector::<8>::new(0b10001100), BitVector::<8>::new(0b10111100), BitVector::<8>::new(0b11010011), BitVector::<8>::new(0b00001010), BitVector::<8>::new(0b11110111), BitVector::<8>::new(0b11100100), BitVector::<8>::new(0b01011000), BitVector::<8>::new(0b00000101), BitVector::<8>::new(0b10111000), BitVector::<8>::new(0b10110011), BitVector::<8>::new(0b01000101), BitVector::<8>::new(0b00000110), BitVector::<8>::new(0b11010000), BitVector::<8>::new(0b00101100), BitVector::<8>::new(0b00011110), BitVector::<8>::new(0b10001111), BitVector::<8>::new(0b11001010), BitVector::<8>::new(0b00111111), BitVector::<8>::new(0b00001111), BitVector::<8>::new(0b00000010), BitVector::<8>::new(0b11000001), BitVector::<8>::new(0b10101111), BitVector::<8>::new(0b10111101), BitVector::<8>::new(0b00000011), BitVector::<8>::new(0b00000001), BitVector::<8>::new(0b00010011), BitVector::<8>::new(0b10001010), BitVector::<8>::new(0b01101011), BitVector::<8>::new(0b00111010), BitVector::<8>::new(0b10010001), BitVector::<8>::new(0b00010001), BitVector::<8>::new(0b01000001), BitVector::<8>::new(0b01001111), BitVector::<8>::new(0b01100111), BitVector::<8>::new(0b11011100), BitVector::<8>::new(0b11101010), BitVector::<8>::new(0b10010111), BitVector::<8>::new(0b11110010), BitVector::<8>::new(0b11001111), BitVector::<8>::new(0b11001110), BitVector::<8>::new(0b11110000), BitVector::<8>::new(0b10110100), BitVector::<8>::new(0b11100110), BitVector::<8>::new(0b01110011), BitVector::<8>::new(0b10010110), BitVector::<8>::new(0b10101100), BitVector::<8>::new(0b01110100), BitVector::<8>::new(0b00100010), BitVector::<8>::new(0b11100111), BitVector::<8>::new(0b10101101), BitVector::<8>::new(0b00110101), BitVector::<8>::new(0b10000101), BitVector::<8>::new(0b11100010), BitVector::<8>::new(0b11111001), BitVector::<8>::new(0b00110111), BitVector::<8>::new(0b11101000), BitVector::<8>::new(0b00011100), BitVector::<8>::new(0b01110101), BitVector::<8>::new(0b11011111), BitVector::<8>::new(0b01101110), BitVector::<8>::new(0b01000111), BitVector::<8>::new(0b11110001), BitVector::<8>::new(0b00011010), BitVector::<8>::new(0b01110001), BitVector::<8>::new(0b00011101), BitVector::<8>::new(0b00101001), BitVector::<8>::new(0b11000101), BitVector::<8>::new(0b10001001), BitVector::<8>::new(0b01101111), BitVector::<8>::new(0b10110111), BitVector::<8>::new(0b01100010), BitVector::<8>::new(0b00001110), BitVector::<8>::new(0b10101010), BitVector::<8>::new(0b00011000), BitVector::<8>::new(0b10111110), BitVector::<8>::new(0b00011011), BitVector::<8>::new(0b11111100), BitVector::<8>::new(0b01010110), BitVector::<8>::new(0b00111110), BitVector::<8>::new(0b01001011), BitVector::<8>::new(0b11000110), BitVector::<8>::new(0b11010010), BitVector::<8>::new(0b01111001), BitVector::<8>::new(0b00100000), BitVector::<8>::new(0b10011010), BitVector::<8>::new(0b11011011), BitVector::<8>::new(0b11000000), BitVector::<8>::new(0b11111110), BitVector::<8>::new(0b01111000), BitVector::<8>::new(0b11001101), BitVector::<8>::new(0b01011010), BitVector::<8>::new(0b11110100), BitVector::<8>::new(0b00011111), BitVector::<8>::new(0b11011101), BitVector::<8>::new(0b10101000), BitVector::<8>::new(0b00110011), BitVector::<8>::new(0b10001000), BitVector::<8>::new(0b00000111), BitVector::<8>::new(0b11000111), BitVector::<8>::new(0b00110001), BitVector::<8>::new(0b10110001), BitVector::<8>::new(0b00010010), BitVector::<8>::new(0b00010000), BitVector::<8>::new(0b01011001), BitVector::<8>::new(0b00100111), BitVector::<8>::new(0b10000000), BitVector::<8>::new(0b11101100), BitVector::<8>::new(0b01011111), BitVector::<8>::new(0b01100000), BitVector::<8>::new(0b01010001), BitVector::<8>::new(0b01111111), BitVector::<8>::new(0b10101001), BitVector::<8>::new(0b00011001), BitVector::<8>::new(0b10110101), BitVector::<8>::new(0b01001010), BitVector::<8>::new(0b00001101), BitVector::<8>::new(0b00101101), BitVector::<8>::new(0b11100101), BitVector::<8>::new(0b01111010), BitVector::<8>::new(0b10011111), BitVector::<8>::new(0b10010011), BitVector::<8>::new(0b11001001), BitVector::<8>::new(0b10011100), BitVector::<8>::new(0b11101111), BitVector::<8>::new(0b10100000), BitVector::<8>::new(0b11100000), BitVector::<8>::new(0b00111011), BitVector::<8>::new(0b01001101), BitVector::<8>::new(0b10101110), BitVector::<8>::new(0b00101010), BitVector::<8>::new(0b11110101), BitVector::<8>::new(0b10110000), BitVector::<8>::new(0b11001000), BitVector::<8>::new(0b11101011), BitVector::<8>::new(0b10111011), BitVector::<8>::new(0b00111100), BitVector::<8>::new(0b10000011), BitVector::<8>::new(0b01010011), BitVector::<8>::new(0b10011001), BitVector::<8>::new(0b01100001), BitVector::<8>::new(0b00010111), BitVector::<8>::new(0b00101011), BitVector::<8>::new(0b00000100), BitVector::<8>::new(0b01111110), BitVector::<8>::new(0b10111010), BitVector::<8>::new(0b01110111), BitVector::<8>::new(0b11010110), BitVector::<8>::new(0b00100110), BitVector::<8>::new(0b11100001), BitVector::<8>::new(0b01101001), BitVector::<8>::new(0b00010100), BitVector::<8>::new(0b01100011), BitVector::<8>::new(0b01010101), BitVector::<8>::new(0b00100001), BitVector::<8>::new(0b00001100), BitVector::<8>::new(0b01111101)];

pub type nfields = usize;

/// cbie
/// 
/// Generated from the Sail sources at `riscv_insts_zicbom.sail` L38.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum cbie {
    CBIE_ILLEGAL,
    CBIE_EXEC_FLUSH,
    CBIE_EXEC_INVAL
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
