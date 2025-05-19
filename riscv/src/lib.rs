#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

pub const log2_xlen_bytes: usize = 3;

pub const xlen_bytes: usize = usize::pow(log2_xlen_bytes, 2);

pub const xlen: usize = (xlen_bytes * 8);

pub type xlenbits = BitVector<xlen>;

pub const max_mem_access: usize = 4096;

pub type mem_access_width = usize;

pub const physaddrbits_len: usize = 64;

pub const asidlen: usize = 16;

pub const log2_xlen: usize = (log2_xlen_bytes + 3);

pub type asidbits = BitVector<asidlen>;

pub const flen_bytes: usize = 8;

pub const flen: usize = (flen_bytes * 8);

pub type flenbits = BitVector<flen>;

pub const vlenmax: usize = 65536;

pub type physaddrbits = BitVector<physaddrbits_len>;

pub type mem_meta = ();

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Explicit_access_kind {
    pub variety: Access_variety,
    pub strength: Access_strength,
}

pub type RISCV_strong_access = Access_variety;

pub type exc_code = BitVector<8>;

pub type ext_ptw = ();

pub type ext_ptw_fail = ();

pub type ext_ptw_error = ();

pub type ext_exc_type = ();

pub type half = BitVector<16>;

pub type word = BitVector<32>;

pub type instbits = BitVector<32>;

pub const pagesize_bits: usize = 12;

pub type csreg = BitVector<12>;

pub type arch_xlen = BitVector<2>;

pub type priv_level = BitVector<2>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct mul_op {
    pub high: bool,
    pub signed_rs1: bool,
    pub signed_rs2: bool,
}

pub type tv_mode = BitVector<2>;

pub type ext_status = BitVector<2>;

pub type satp_mode = BitVector<4>;

pub type csrRW = BitVector<2>;

pub const is_sv_mode: usize = todo!("TodoNConstraint");

pub type level_range = usize;

pub type pte_bits = BitVector<TodoNexpOther>;

pub type ppn_bits = BitVector<TodoNexpOther>;

pub type vpn_bits = BitVector<{
    (todo!("TodoVarExpr") - pagesize_bits)
}>;

pub type ext_access_type = ();

pub type regtype = xlenbits;

pub type fregtype = flenbits;

pub type Misa = BitField<{
    (usize::pow(3, 2) * 8)
}>;

pub type CountSmcntrpmf = BitField<64>;

pub type Counteren = BitField<32>;

pub type Counterin = BitField<32>;

pub type Fcsr = BitField<32>;

pub type HpmEvent = BitField<64>;

pub type MEnvcfg = BitField<64>;

pub type Mcause = BitField<{
    (usize::pow(3, 2) * 8)
}>;

pub type Medeleg = BitField<64>;

pub type Minterrupts = BitField<{
    (usize::pow(3, 2) * 8)
}>;

pub type Mstatus = BitField<64>;

pub type Mtvec = BitField<{
    (usize::pow(3, 2) * 8)
}>;

pub type PTE_Ext = BitField<10>;

pub type PTE_Flags = BitField<8>;

pub type Pmpcfg_ent = BitField<8>;

pub type SEnvcfg = BitField<{
    (usize::pow(3, 2) * 8)
}>;

pub type Satp32 = BitField<32>;

pub type Satp64 = BitField<64>;

pub type Sinterrupts = BitField<{
    (usize::pow(3, 2) * 8)
}>;

pub type Sstatus = BitField<64>;

pub type Vcsr = BitField<3>;

pub type Vtype = BitField<{
    (usize::pow(3, 2) * 8)
}>;

pub type htif_cmd = BitField<64>;

pub type ext_fetch_addr_error = ();

pub type ext_control_addr_error = ();

pub type ext_data_addr_error = ();

pub type vreglenbits = BitVector<vlenmax>;

pub type vregtype = vreglenbits;

pub type ext_exception = ();

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct sync_exception {
    pub trap: ExceptionType,
    pub excinfo: Option<xlenbits>,
    pub ext: Option<ext_exception>,
}

pub type hpmidx = usize;

pub type bits_rm = BitVector<3>;

pub type bits_fflags = BitVector<5>;

pub type bits_H = BitVector<16>;

pub type bits_S = BitVector<32>;

pub type bits_D = BitVector<64>;

pub type bits_W = BitVector<32>;

pub type bits_WU = BitVector<32>;

pub type bits_L = BitVector<64>;

pub type bits_LU = BitVector<64>;

pub type MemoryOpResult = result<'a, ExceptionType>;

pub type pte_flags_bits = BitVector<8>;

pub type pte_ext_bits = BitVector<10>;

pub const tlb_vpn_bits: usize = (57 - pagesize_bits);

pub const tlb_ppn_bits: usize = 44;

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

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct PTW_Output {
    pub ppn: ppn_bits<V>,
    pub pte: pte_bits<V>,
    pub pteAddr: physaddr,
    pub level: level_range<V>,
    pub global: bool,
}

pub type PTW_Result = result<(PTW_Output<V>, ext_ptw), (PTW_Error, ext_ptw)>;

pub const valid_misaligned_order: usize = todo!("TodoNConstraint");

pub const nfields_range: usize = todo!("TodoNConstraint");

pub const nfields_range_pow2: usize = todo!("TodoNConstraint");

pub type nfields = atom<Q>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
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
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum virtaddr {
    Virtaddr(xlenbits),
}

pub fn hex_bits_backwards(sail_ctx: &mut SailVirtCtx, n: (usize, &'static str)) -> BitVector<N> {
    parse_hex_bits(sail_ctx, n, str)
}

pub fn hex_bits_12_backwards(sail_ctx: &mut SailVirtCtx, arg_hashtag_: &'static str) -> BitVector<12> {
    match arg_hashtag_ {
        s => {hex_bits_backwards(sail_ctx, (12, s))}
        _ => {panic!("Unreachable code")}
    }
}

pub fn hex_bits_12_backwards_matches(sail_ctx: &mut SailVirtCtx, arg_hashtag_: &'static str) -> bool {
    match arg_hashtag_ {
        s => {true}
        _ => {false}
        _ => {panic!("Unreachable code")}
    }
}

pub fn get_config_print_platform(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> bool {
    false
}

pub fn zeros<const N: usize>(sail_ctx: &mut SailVirtCtx, n: usize) -> BitVector<N> {
    sail_zeros(n)
}

pub fn ones<const N: usize>(sail_ctx: &mut SailVirtCtx, n: usize) -> BitVector<N> {
    sail_ones(n)
}

pub fn bool_bit_backwards(sail_ctx: &mut SailVirtCtx, arg_hashtag_: bool) -> bool {
    match arg_hashtag_ {
        true => {true}
        false => {false}
        _ => {panic!("Unreachable code")}
    }
}

pub fn bool_bits_forwards(sail_ctx: &mut SailVirtCtx, arg_hashtag_: bool) -> BitVector<1> {
    match arg_hashtag_ {
        true => {BitVector::<1>::new(0b1)}
        false => {BitVector::<1>::new(0b0)}
        _ => {panic!("Unreachable code")}
    }
}

pub fn bit_to_bool(sail_ctx: &mut SailVirtCtx, x: bool) -> bool {
    bool_bit_backwards(sail_ctx, x)
}

pub fn bool_to_bits(sail_ctx: &mut SailVirtCtx, x: bool) -> BitVector<1> {
    bool_bits_forwards(sail_ctx, x)
}

pub fn _operator_smaller_s_<const N: usize>(sail_ctx: &mut SailVirtCtx, x: BitVector<N>, y: BitVector<N>) -> bool {
    (signed(x) < signed(y))
}

pub fn _operator_smaller_u_<const N: usize>(sail_ctx: &mut SailVirtCtx, x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.as_usize() < y.as_usize())
}

pub fn _operator_biggerequal_u_<const N: usize>(sail_ctx: &mut SailVirtCtx, x: BitVector<N>, y: BitVector<N>) -> bool {
    (x.as_usize() >= y.as_usize())
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum exception {
    Error_not_implemented(&'static str),
    Error_internal_error(()),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum physaddr {
    Physaddr(physaddrbits),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum result {
    Ok('a),
    Err('b),
}

pub fn sail_branch_announce<const N: usize>(sail_ctx: &mut SailVirtCtx, _: atom<ADDRSIZE>, _: BitVector<ADDRSIZE>) {
    ()
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_variety {
    AV_plain,
    AV_exclusive,
    AV_atomic_rmw,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_strength {
    AS_normal,
    AS_rel_or_acq,
    AS_acq_rcpc,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Access_kind {
    AK_explicit(Explicit_access_kind),
    AK_ifetch(()),
    AK_ttw(()),
    AK_arch('arch_ak),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum write_kind {
    Write_plain,
    Write_RISCV_release,
    Write_RISCV_strong_release,
    Write_RISCV_conditional,
    Write_RISCV_conditional_release,
    Write_RISCV_conditional_strong_release,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum read_kind {
    Read_plain,
    Read_ifetch,
    Read_RISCV_acquire,
    Read_RISCV_strong_acquire,
    Read_RISCV_reserved,
    Read_RISCV_reserved_acquire,
    Read_RISCV_reserved_strong_acquire,
}

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
    Barrier_RISCV_i,
}

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
    Ext_Smcntrpmf,
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    ((todo!("E_config") as bool) && (xlen == 32))
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zca) && ((hartSupports(sail_ctx, extension::Ext_Zcf) || (!(hartSupports(sail_ctx, extension::Ext_F)) || (xlen != 32))) && (hartSupports(sail_ctx, extension::Ext_Zcd) || !(hartSupports(sail_ctx, extension::Ext_D)))))
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    false
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    false
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    ((todo!("E_config") as bool) && (xlen == 32))
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    ((todo!("E_config") as bool) && (xlen == 64))
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    ((todo!("E_config") as bool) && (xlen == 64))
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    ((todo!("E_config") as bool) && (xlen == 64))
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx) -> bool {
    todo!("E_config")
}

pub fn ext_exc_type_to_bits(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> BitVector<8> {
    BitVector::<8>::new(0b00011000)
}

pub fn num_of_ext_exc_type(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> usize {
    24
}

pub fn ext_exc_type_to_str(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> &'static str {
    "extension-exception"
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum regidx {
    Regidx(BitVector<5>),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum cregidx {
    Cregidx(BitVector<3>),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum regno {
    Regno(usize),
}

pub fn regidx_to_regno(sail_ctx: &mut SailVirtCtx, TodoArgsApp: regidx) -> regno {
    let var_1 = b.as_usize();
    regno::Regno(var_1)
}

pub fn regno_to_regidx(sail_ctx: &mut SailVirtCtx, TodoArgsApp: regno) -> regidx {
    let var_1 = to_bits(sail_ctx, 5, b);
    regidx::Regidx(var_1)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Architecture {
    RV32,
    RV64,
    RV128,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Privilege {
    User,
    Supervisor,
    Machine,
}

pub fn privLevel_bits_forwards(sail_ctx: &mut SailVirtCtx, arg_hashtag_: Privilege) -> BitVector<2> {
    match arg_hashtag_ {
        Privilege::User => {BitVector::<2>::new(0b00)}
        Privilege::Supervisor => {BitVector::<2>::new(0b01)}
        Privilege::Machine => {BitVector::<2>::new(0b11)}
        _ => {panic!("Unreachable code")}
    }
}

pub fn privLevel_bits_backwards(sail_ctx: &mut SailVirtCtx, arg_hashtag_: BitVector<2>) -> Privilege {
    match arg_hashtag_ {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {Privilege::User}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {Privilege::Supervisor}
        b__2 if {(b__2 == BitVector::<2>::new(0b11))} => {Privilege::Machine}
        _ => {internal_error("riscv_types.sail", 69, format!("{}{}", "Invalid privilege level: ", bits_str(BitVector::<2>::new(0b10))))}
        _ => {panic!("Unreachable code")}
    }
}

pub fn privLevel_to_bits(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<2> {
    privLevel_bits_forwards(sail_ctx, p)
}

pub fn privLevel_of_bits(sail_ctx: &mut SailVirtCtx, b: BitVector<2>) -> Privilege {
    privLevel_bits_backwards(sail_ctx, b)
}

pub fn privLevel_to_str(sail_ctx: &mut SailVirtCtx, p: Privilege) -> &'static str {
    match p {
        Privilege::User => {"U"}
        Privilege::Supervisor => {"S"}
        Privilege::Machine => {"M"}
        _ => {panic!("Unreachable code")}
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum AccessType {
    Read('a),
    Write('a),
    ReadWrite(('a, 'a)),
    InstructionFetch(()),
}

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
    E_Extension(ext_exc_type),
}

pub fn exceptionType_to_str(sail_ctx: &mut SailVirtCtx, e: ExceptionType) -> &'static str {
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
        ExceptionType::E_Extension(e) => {ext_exc_type_to_str(sail_ctx, e)}
        _ => {panic!("Unreachable code")}
    }
}

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
    AMOMAXU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum bop {
    BEQ,
    BNE,
    BLT,
    BGE,
    BLTU,
    BGEU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum cbop_zicbom {
    CBO_CLEAN,
    CBO_FLUSH,
    CBO_INVAL,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum csrop {
    CSRRW,
    CSRRS,
    CSRRC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_f_op_D {
    FSGNJ_D,
    FSGNJN_D,
    FSGNJX_D,
    FMIN_D,
    FMAX_D,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_f_op_H {
    FSGNJ_H,
    FSGNJN_H,
    FSGNJX_H,
    FMIN_H,
    FMAX_H,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_rm_op_D {
    FADD_D,
    FSUB_D,
    FMUL_D,
    FDIV_D,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_rm_op_H {
    FADD_H,
    FSUB_H,
    FMUL_H,
    FDIV_H,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_rm_op_S {
    FADD_S,
    FSUB_S,
    FMUL_S,
    FDIV_S,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_op_f_S {
    FSGNJ_S,
    FSGNJN_S,
    FSGNJX_S,
    FMIN_S,
    FMAX_S,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_op_x_S {
    FEQ_S,
    FLT_S,
    FLE_S,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_x_op_D {
    FEQ_D,
    FLT_D,
    FLE_D,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_bin_x_op_H {
    FEQ_H,
    FLT_H,
    FLE_H,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_madd_op_D {
    FMADD_D,
    FMSUB_D,
    FNMSUB_D,
    FNMADD_D,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_madd_op_H {
    FMADD_H,
    FMSUB_H,
    FNMSUB_H,
    FNMADD_H,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_madd_op_S {
    FMADD_S,
    FMSUB_S,
    FNMSUB_S,
    FNMADD_S,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_f_op_D {
    FMV_D_X,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_f_op_H {
    FMV_H_X,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_ff_op_D {
    FSQRT_D,
    FCVT_S_D,
    FCVT_D_S,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_ff_op_H {
    FSQRT_H,
    FCVT_H_S,
    FCVT_H_D,
    FCVT_S_H,
    FCVT_D_H,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_fx_op_D {
    FCVT_W_D,
    FCVT_WU_D,
    FCVT_L_D,
    FCVT_LU_D,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_fx_op_H {
    FCVT_W_H,
    FCVT_WU_H,
    FCVT_L_H,
    FCVT_LU_H,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_fx_op_S {
    FCVT_W_S,
    FCVT_WU_S,
    FCVT_L_S,
    FCVT_LU_S,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_xf_op_D {
    FCVT_D_W,
    FCVT_D_WU,
    FCVT_D_L,
    FCVT_D_LU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_xf_op_H {
    FCVT_H_W,
    FCVT_H_WU,
    FCVT_H_L,
    FCVT_H_LU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_xf_op_S {
    FCVT_S_W,
    FCVT_S_WU,
    FCVT_S_L,
    FCVT_S_LU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_op_f_S {
    FMV_W_X,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_op_x_S {
    FCLASS_S,
    FMV_X_W,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_x_op_D {
    FCLASS_D,
    FMV_X_D,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_x_op_H {
    FCLASS_H,
    FMV_X_H,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fregidx {
    Fregidx(BitVector<5>),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rounding_mode {
    RM_RNE,
    RM_RTZ,
    RM_RDN,
    RM_RUP,
    RM_RMM,
    RM_DYN,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvfmafunct6 {
    VF_VMADD,
    VF_VNMADD,
    VF_VMSUB,
    VF_VNMSUB,
    VF_VMACC,
    VF_VNMACC,
    VF_VMSAC,
    VF_VNMSAC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvfmfunct6 {
    VFM_VMFEQ,
    VFM_VMFLE,
    VFM_VMFLT,
    VFM_VMFNE,
    VFM_VMFGT,
    VFM_VMFGE,
}

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
    VF_VSLIDE1DOWN,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvvmafunct6 {
    FVV_VMADD,
    FVV_VNMADD,
    FVV_VMSUB,
    FVV_VNMSUB,
    FVV_VMACC,
    FVV_VNMACC,
    FVV_VMSAC,
    FVV_VNMSAC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fvvmfunct6 {
    FVVM_VMFEQ,
    FVVM_VMFLE,
    FVVM_VMFLT,
    FVVM_VMFNE,
}

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
    FVV_VMUL,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwffunct6 {
    FWF_VADD,
    FWF_VSUB,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvfmafunct6 {
    FWVF_VMACC,
    FWVF_VNMACC,
    FWVF_VMSAC,
    FWVF_VNMSAC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvffunct6 {
    FWVF_VADD,
    FWVF_VSUB,
    FWVF_VMUL,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvfunct6 {
    FWV_VADD,
    FWV_VSUB,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvvmafunct6 {
    FWVV_VMACC,
    FWVV_VNMACC,
    FWVV_VMSAC,
    FWVV_VNMSAC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fwvvfunct6 {
    FWVV_VADD,
    FWVV_VSUB,
    FWVV_VMUL,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum iop {
    ADDI,
    SLTI,
    SLTIU,
    XORI,
    ORI,
    ANDI,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum mmfunct6 {
    MM_VMAND,
    MM_VMNAND,
    MM_VMANDN,
    MM_VMXOR,
    MM_VMOR,
    MM_VMNOR,
    MM_VMORN,
    MM_VMXNOR,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum mvvmafunct6 {
    MVV_VMACC,
    MVV_VNMSAC,
    MVV_VMADD,
    MVV_VNMSUB,
}

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
    MVV_VREM,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum mvxmafunct6 {
    MVX_VMACC,
    MVX_VNMSAC,
    MVX_VMADD,
    MVX_VNMSUB,
}

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
    MVX_VREM,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nisfunct6 {
    NIS_VNSRL,
    NIS_VNSRA,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nifunct6 {
    NI_VNCLIPU,
    NI_VNCLIP,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nvsfunct6 {
    NVS_VNSRL,
    NVS_VNSRA,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nvfunct6 {
    NV_VNCLIPU,
    NV_VNCLIP,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nxsfunct6 {
    NXS_VNSRL,
    NXS_VNSRA,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum nxfunct6 {
    NX_VNCLIPU,
    NX_VNCLIP,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rfvvfunct6 {
    FVV_VFREDOSUM,
    FVV_VFREDUSUM,
    FVV_VFREDMAX,
    FVV_VFREDMIN,
    FVV_VFWREDOSUM,
    FVV_VFWREDUSUM,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rivvfunct6 {
    IVV_VWREDSUMU,
    IVV_VWREDSUM,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum rmvvfunct6 {
    MVV_VREDSUM,
    MVV_VREDAND,
    MVV_VREDOR,
    MVV_VREDXOR,
    MVV_VREDMINU,
    MVV_VREDMIN,
    MVV_VREDMAXU,
    MVV_VREDMAX,
}

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
    AND,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ropw {
    ADDW,
    SUBW,
    SLLW,
    SRLW,
    SRAW,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum sop {
    SLLI,
    SRLI,
    SRAI,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum sopw {
    SLLIW,
    SRLIW,
    SRAIW,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum word_width {
    BYTE,
    HALF,
    WORD,
    DOUBLE,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum uop {
    LUI,
    AUIPC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vext2funct6 {
    VEXT2_ZVF2,
    VEXT2_SVF2,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vext4funct6 {
    VEXT4_ZVF4,
    VEXT4_SVF4,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vext8funct6 {
    VEXT8_ZVF8,
    VEXT8_SVF8,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vfnunary0 {
    FNV_CVT_XU_F,
    FNV_CVT_X_F,
    FNV_CVT_F_XU,
    FNV_CVT_F_X,
    FNV_CVT_F_F,
    FNV_CVT_ROD_F_F,
    FNV_CVT_RTZ_XU_F,
    FNV_CVT_RTZ_X_F,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vfunary0 {
    FV_CVT_XU_F,
    FV_CVT_X_F,
    FV_CVT_F_XU,
    FV_CVT_F_X,
    FV_CVT_RTZ_XU_F,
    FV_CVT_RTZ_X_F,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vfunary1 {
    FVV_VSQRT,
    FVV_VRSQRT7,
    FVV_VREC7,
    FVV_VCLASS,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vfwunary0 {
    FWV_CVT_XU_F,
    FWV_CVT_X_F,
    FWV_CVT_F_XU,
    FWV_CVT_F_X,
    FWV_CVT_F_F,
    FWV_CVT_RTZ_XU_F,
    FWV_CVT_RTZ_X_F,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vicmpfunct6 {
    VICMP_VMSEQ,
    VICMP_VMSNE,
    VICMP_VMSLEU,
    VICMP_VMSLE,
    VICMP_VMSGTU,
    VICMP_VMSGT,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vimcfunct6 {
    VIMC_VMADC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vimsfunct6 {
    VIMS_VADC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vimfunct6 {
    VIM_VMADC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum visgfunct6 {
    VI_VSLIDEUP,
    VI_VSLIDEDOWN,
    VI_VRGATHER,
}

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
    VI_VSSRA,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vlewidth {
    VLE8,
    VLE16,
    VLE32,
    VLE64,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vmlsop {
    VLM,
    VSM,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregidx {
    Vregidx(BitVector<5>),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum zvkfunct6 {
    ZVK_VSHA2CH,
    ZVK_VSHA2CL,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvcmpfunct6 {
    VVCMP_VMSEQ,
    VVCMP_VMSNE,
    VVCMP_VMSLTU,
    VVCMP_VMSLT,
    VVCMP_VMSLEU,
    VVCMP_VMSLE,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvmcfunct6 {
    VVMC_VMADC,
    VVMC_VMSBC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvmsfunct6 {
    VVMS_VADC,
    VVMS_VSBC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvmfunct6 {
    VVM_VMADC,
    VVM_VMSBC,
}

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
    VV_VSSRA,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxcmpfunct6 {
    VXCMP_VMSEQ,
    VXCMP_VMSNE,
    VXCMP_VMSLTU,
    VXCMP_VMSLT,
    VXCMP_VMSLEU,
    VXCMP_VMSLE,
    VXCMP_VMSGTU,
    VXCMP_VMSGT,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxmcfunct6 {
    VXMC_VMADC,
    VXMC_VMSBC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxmsfunct6 {
    VXMS_VADC,
    VXMS_VSBC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxmfunct6 {
    VXM_VMADC,
    VXM_VMSBC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vxsgfunct6 {
    VX_VSLIDEUP,
    VX_VSLIDEDOWN,
    VX_VRGATHER,
}

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
    VX_VSSRA,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wmvvfunct6 {
    WMVV_VWMACCU,
    WMVV_VWMACC,
    WMVV_VWMACCSU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wmvxfunct6 {
    WMVX_VWMACCU,
    WMVX_VWMACC,
    WMVX_VWMACCUS,
    WMVX_VWMACCSU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wvfunct6 {
    WV_VADD,
    WV_VSUB,
    WV_VADDU,
    WV_VSUBU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wvvfunct6 {
    WVV_VADD,
    WVV_VSUB,
    WVV_VADDU,
    WVV_VSUBU,
    WVV_VWMUL,
    WVV_VWMULU,
    WVV_VWMULSU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wvxfunct6 {
    WVX_VADD,
    WVX_VSUB,
    WVX_VADDU,
    WVX_VSUBU,
    WVX_VWMUL,
    WVX_VWMULU,
    WVX_VWMULSU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum wxfunct6 {
    WX_VADD,
    WX_VSUB,
    WX_VADDU,
    WX_VSUBU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum brop_zba {
    SH1ADD,
    SH2ADD,
    SH3ADD,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum bropw_zba {
    ADDUW,
    SH1ADDUW,
    SH2ADDUW,
    SH3ADDUW,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum extop_zbb {
    SEXTB,
    SEXTH,
    ZEXTH,
}

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
    ROR,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum bropw_zbb {
    ROLW,
    RORW,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum brop_zbkb {
    PACK,
    PACKH,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum biop_zbs {
    BCLRI,
    BEXTI,
    BINVI,
    BSETI,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum brop_zbs {
    BCLR,
    BEXT,
    BINV,
    BSET,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum zicondop {
    CZERO_EQZ,
    CZERO_NEZ,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum f_un_rm_ff_op_S {
    FSQRT_S,
}

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
    ZCMOP(BitVector<3>),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PTW_Error {
    PTW_Invalid_Addr(()),
    PTW_Access(()),
    PTW_Invalid_PTE(()),
    PTW_No_Permission(()),
    PTW_Misaligned(()),
    PTW_PTE_Update(()),
    PTW_Ext_Error(ext_ptw_error),
}

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
    I_M_External,
}

pub fn exceptionType_to_bits(sail_ctx: &mut SailVirtCtx, e: ExceptionType) -> BitVector<8> {
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
        ExceptionType::E_Extension(e) => {ext_exc_type_to_bits(sail_ctx, e)}
        _ => {panic!("Unreachable code")}
    }
}

pub fn num_of_ExceptionType(sail_ctx: &mut SailVirtCtx, e: ExceptionType) -> usize {
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
        ExceptionType::E_Extension(e) => {num_of_ext_exc_type(sail_ctx, e)}
        _ => {panic!("Unreachable code")}
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TrapVectorMode {
    TV_Direct,
    TV_Vector,
    TV_Reserved,
}

pub fn trapVectorMode_of_bits(sail_ctx: &mut SailVirtCtx, m: BitVector<2>) -> TrapVectorMode {
    match m {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {TrapVectorMode::TV_Direct}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {TrapVectorMode::TV_Vector}
        _ => {TrapVectorMode::TV_Reserved}
        _ => {panic!("Unreachable code")}
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExtStatus {
    Off,
    Initial,
    Clean,
    Dirty,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SATPMode {
    Bare,
    Sv32,
    Sv39,
    Sv48,
    Sv57,
}

pub fn xreg_write_callback(sail_ctx: &mut SailVirtCtx, _: regidx, TodoMissingArg: BitVector<{
    (usize::pow(3, 2) * 8)
}>) {
    ()
}

pub fn csr_full_write_callback(sail_ctx: &mut SailVirtCtx, _: &'static str, TodoMissingArg: BitVector<12>, TodoMissingArg: BitVector<{
    (usize::pow(3, 2) * 8)
}>) {
    ()
}

pub fn trap_callback(sail_ctx: &mut SailVirtCtx, unit_arg: ()) {
    ()
}

pub fn csr_name_map_backwards(sail_ctx: &mut SailVirtCtx, arg_hashtag_: &'static str) -> BitVector<12> {
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
        mapping0_hashtag_ if {hex_bits_12_backwards_matches(sail_ctx, mapping0_hashtag_)} => {match hex_bits_12_backwards(sail_ctx, mapping0_hashtag_) {
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

pub fn csr_name_write_callback(sail_ctx: &mut SailVirtCtx, name: &'static str, value: BitVector<{
    (usize::pow(3, 2) * 8)
}>) {
    let csr = csr_name_map_backwards(sail_ctx, name);
    csr_full_write_callback(sail_ctx, name, csr, value)
}

pub fn long_csr_write_callback(sail_ctx: &mut SailVirtCtx, name: &'static str, name_high: &'static str, value: BitVector<64>) {
    {
        let var_1 = name;
        let var_2 = subrange_bits(value, (xlen - 1), 0);
        csr_name_write_callback(sail_ctx, var_1, var_2)
    };
    if {(xlen == 32)} {
        {
            let var_3 = name_high;
            let var_4 = value.subrange::<32, 64, 32>();
            csr_name_write_callback(sail_ctx, var_3, var_4)
        }
    } else {
        ()
    }
}

pub fn regval_from_reg(sail_ctx: &mut SailVirtCtx, r: BitVector<{
    (usize::pow(3, 2) * 8)
}>) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    r
}

pub fn regval_into_reg(sail_ctx: &mut SailVirtCtx, v: BitVector<{
    (usize::pow(3, 2) * 8)
}>) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    v
}

pub fn rX(sail_ctx: &mut SailVirtCtx, TodoArgsApp: regno) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    let v: regtype = match r {
        l__583 if {(l__583 == 0)} => {zero_reg}
        l__584 if {(l__584 == 1)} => {sail_ctx.x1}
        l__585 if {(l__585 == 2)} => {sail_ctx.x2}
        l__586 if {(l__586 == 3)} => {sail_ctx.x3}
        l__587 if {(l__587 == 4)} => {sail_ctx.x4}
        l__588 if {(l__588 == 5)} => {sail_ctx.x5}
        l__589 if {(l__589 == 6)} => {sail_ctx.x6}
        l__590 if {(l__590 == 7)} => {sail_ctx.x7}
        l__591 if {(l__591 == 8)} => {sail_ctx.x8}
        l__592 if {(l__592 == 9)} => {sail_ctx.x9}
        l__593 if {(l__593 == 10)} => {sail_ctx.x10}
        l__594 if {(l__594 == 11)} => {sail_ctx.x11}
        l__595 if {(l__595 == 12)} => {sail_ctx.x12}
        l__596 if {(l__596 == 13)} => {sail_ctx.x13}
        l__597 if {(l__597 == 14)} => {sail_ctx.x14}
        l__598 if {(l__598 == 15)} => {sail_ctx.x15}
        l__599 if {(l__599 == 16)} => {sail_ctx.x16}
        l__600 if {(l__600 == 17)} => {sail_ctx.x17}
        l__601 if {(l__601 == 18)} => {sail_ctx.x18}
        l__602 if {(l__602 == 19)} => {sail_ctx.x19}
        l__603 if {(l__603 == 20)} => {sail_ctx.x20}
        l__604 if {(l__604 == 21)} => {sail_ctx.x21}
        l__605 if {(l__605 == 22)} => {sail_ctx.x22}
        l__606 if {(l__606 == 23)} => {sail_ctx.x23}
        l__607 if {(l__607 == 24)} => {sail_ctx.x24}
        l__608 if {(l__608 == 25)} => {sail_ctx.x25}
        l__609 if {(l__609 == 26)} => {sail_ctx.x26}
        l__610 if {(l__610 == 27)} => {sail_ctx.x27}
        l__611 if {(l__611 == 28)} => {sail_ctx.x28}
        l__612 if {(l__612 == 29)} => {sail_ctx.x29}
        l__613 if {(l__613 == 30)} => {sail_ctx.x30}
        l__614 if {(l__614 == 31)} => {sail_ctx.x31}
        _ => {{
            assert!(false, "Process message");
            __exit()
        }}
        _ => {panic!("Unreachable code")}
    };
    regval_from_reg(sail_ctx, v)
}

pub fn wX(sail_ctx: &mut SailVirtCtx, Regno(r): regno, in_v: BitVector<{
    (usize::pow(3, 2) * 8)
}>) {
    let v = regval_into_reg(sail_ctx, in_v);
    match r {
        l__551 if {(l__551 == 0)} => {()}
        l__552 if {(l__552 == 1)} => {sail_ctx.x1 = v}
        l__553 if {(l__553 == 2)} => {sail_ctx.x2 = v}
        l__554 if {(l__554 == 3)} => {sail_ctx.x3 = v}
        l__555 if {(l__555 == 4)} => {sail_ctx.x4 = v}
        l__556 if {(l__556 == 5)} => {sail_ctx.x5 = v}
        l__557 if {(l__557 == 6)} => {sail_ctx.x6 = v}
        l__558 if {(l__558 == 7)} => {sail_ctx.x7 = v}
        l__559 if {(l__559 == 8)} => {sail_ctx.x8 = v}
        l__560 if {(l__560 == 9)} => {sail_ctx.x9 = v}
        l__561 if {(l__561 == 10)} => {sail_ctx.x10 = v}
        l__562 if {(l__562 == 11)} => {sail_ctx.x11 = v}
        l__563 if {(l__563 == 12)} => {sail_ctx.x12 = v}
        l__564 if {(l__564 == 13)} => {sail_ctx.x13 = v}
        l__565 if {(l__565 == 14)} => {sail_ctx.x14 = v}
        l__566 if {(l__566 == 15)} => {sail_ctx.x15 = v}
        l__567 if {(l__567 == 16)} => {sail_ctx.x16 = v}
        l__568 if {(l__568 == 17)} => {sail_ctx.x17 = v}
        l__569 if {(l__569 == 18)} => {sail_ctx.x18 = v}
        l__570 if {(l__570 == 19)} => {sail_ctx.x19 = v}
        l__571 if {(l__571 == 20)} => {sail_ctx.x20 = v}
        l__572 if {(l__572 == 21)} => {sail_ctx.x21 = v}
        l__573 if {(l__573 == 22)} => {sail_ctx.x22 = v}
        l__574 if {(l__574 == 23)} => {sail_ctx.x23 = v}
        l__575 if {(l__575 == 24)} => {sail_ctx.x24 = v}
        l__576 if {(l__576 == 25)} => {sail_ctx.x25 = v}
        l__577 if {(l__577 == 26)} => {sail_ctx.x26 = v}
        l__578 if {(l__578 == 27)} => {sail_ctx.x27 = v}
        l__579 if {(l__579 == 28)} => {sail_ctx.x28 = v}
        l__580 if {(l__580 == 29)} => {sail_ctx.x29 = v}
        l__581 if {(l__581 == 30)} => {sail_ctx.x30 = v}
        l__582 if {(l__582 == 31)} => {sail_ctx.x31 = v}
        _ => {assert!(false, "Process message")}
        _ => {panic!("Unreachable code")}
    };
    if {(r != 0)} {
        {
            let var_1 = {
                let var_3 = regno::Regno(r);
                regno_to_regidx(sail_ctx, var_3)
            };
            let var_2 = in_v;
            xreg_write_callback(sail_ctx, var_1, var_2)
        }
    } else {
        ()
    }
}

pub fn rX_bits(sail_ctx: &mut SailVirtCtx, i: regidx) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    let var_1 = regidx_to_regno(sail_ctx, i);
    rX(sail_ctx, var_1)
}

pub fn wX_bits(sail_ctx: &mut SailVirtCtx, i: regidx, data: BitVector<{
    (usize::pow(3, 2) * 8)
}>) {
    {
        let var_1 = regidx_to_regno(sail_ctx, i);
        let var_2 = data;
        wX(sail_ctx, var_1, var_2)
    }
}

pub fn set_next_pc(sail_ctx: &mut SailVirtCtx, pc: BitVector<{
    (usize::pow(3, 2) * 8)
}>) {
    sail_branch_announce(xlen, pc);
    sail_ctx.nextPC = pc
}

pub fn _get_Misa_A(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<0, 1, 1>()
}

pub fn _get_Pmpcfg_ent_A(sail_ctx: &mut SailVirtCtx, v: Pmpcfg_ent) -> BitVector<2> {
    v.subrange::<3, 5, 2>()
}

pub fn _get_Misa_B(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<1, 2, 1>()
}

pub fn _get_Misa_C(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<2, 3, 1>()
}

pub fn _get_Misa_D(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<3, 4, 1>()
}

pub fn _get_Misa_F(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<5, 6, 1>()
}

pub fn _get_Pmpcfg_ent_L(sail_ctx: &mut SailVirtCtx, v: Pmpcfg_ent) -> BitVector<1> {
    v.subrange::<7, 8, 1>()
}

pub fn _get_Misa_M(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<12, 13, 1>()
}

pub fn _get_Pmpcfg_ent_R(sail_ctx: &mut SailVirtCtx, v: Pmpcfg_ent) -> BitVector<1> {
    v.subrange::<0, 1, 1>()
}

pub fn _get_Misa_S(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<18, 19, 1>()
}

pub fn _get_Misa_U(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<20, 21, 1>()
}

pub fn _get_Misa_V(sail_ctx: &mut SailVirtCtx, v: Misa) -> BitVector<1> {
    v.subrange::<21, 22, 1>()
}

pub fn _get_Pmpcfg_ent_W(sail_ctx: &mut SailVirtCtx, v: Pmpcfg_ent) -> BitVector<1> {
    v.subrange::<1, 2, 1>()
}

pub fn _get_Pmpcfg_ent_X(sail_ctx: &mut SailVirtCtx, v: Pmpcfg_ent) -> BitVector<1> {
    v.subrange::<2, 3, 1>()
}

pub fn sys_pmp_grain(sail_ctx: &mut SailVirtCtx, unit_arg: ()) -> usize {
    todo!("E_config")
}

pub fn _get_Mstatus_FS(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<2> {
    v.subrange::<13, 15, 2>()
}

pub fn _get_Mstatus_VS(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<2> {
    v.subrange::<9, 11, 2>()
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Sstc)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_U) && (_get_Misa_U(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_S) && (_get_Misa_S(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    currentlyEnabled(sail_ctx, extension::Ext_S)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Sv32) && currentlyEnabled(sail_ctx, extension::Ext_S))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Sv39) && currentlyEnabled(sail_ctx, extension::Ext_S))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Sv48) && currentlyEnabled(sail_ctx, extension::Ext_S))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Sv57) && currentlyEnabled(sail_ctx, extension::Ext_S))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_V) && ((_get_Misa_V(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)) && (_get_Mstatus_VS(sail_ctx, sail_ctx.mstatus) != BitVector::<2>::new(0b00))))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zihpm) && currentlyEnabled(sail_ctx, extension::Ext_Zicntr))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Sscofpmf) && currentlyEnabled(sail_ctx, extension::Ext_Zihpm))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zkr)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zicntr)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_F) && ((_get_Misa_F(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)) && (_get_Mstatus_FS(sail_ctx, sail_ctx.mstatus) != BitVector::<2>::new(0b00))))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_D) && ((_get_Misa_D(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)) && ((_get_Mstatus_FS(sail_ctx, sail_ctx.mstatus) != BitVector::<2>::new(0b00)) && (flen >= 64))))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zfinx)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Smcntrpmf) && currentlyEnabled(sail_ctx, extension::Ext_Zicntr))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    false
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    false
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_C) && (_get_Misa_C(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zca) && (currentlyEnabled(sail_ctx, extension::Ext_C) || !(hartSupports(sail_ctx, extension::Ext_C))))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zifencei)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_A) && (_get_Misa_A(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zabha) && currentlyEnabled(sail_ctx, extension::Ext_Zaamo))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zalrsc) || currentlyEnabled(sail_ctx, extension::Ext_A))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zaamo) || currentlyEnabled(sail_ctx, extension::Ext_A))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_M) && (_get_Misa_M(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zmmul) || currentlyEnabled(sail_ctx, extension::Ext_M))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zfh) && currentlyEnabled(sail_ctx, extension::Ext_F))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    ((hartSupports(sail_ctx, extension::Ext_Zfhmin) && currentlyEnabled(sail_ctx, extension::Ext_F)) || currentlyEnabled(sail_ctx, extension::Ext_Zfh))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zcf) && (currentlyEnabled(sail_ctx, extension::Ext_F) && (currentlyEnabled(sail_ctx, extension::Ext_Zca) && (currentlyEnabled(sail_ctx, extension::Ext_C) || !(hartSupports(sail_ctx, extension::Ext_C))))))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zdinx) && (flen >= 64))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zcd) && (currentlyEnabled(sail_ctx, extension::Ext_D) && (currentlyEnabled(sail_ctx, extension::Ext_Zca) && (currentlyEnabled(sail_ctx, extension::Ext_C) || !(hartSupports(sail_ctx, extension::Ext_C))))))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Svinval)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_B) && (_get_Misa_B(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1)))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zba) || currentlyEnabled(sail_ctx, extension::Ext_B))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zbb) || currentlyEnabled(sail_ctx, extension::Ext_B))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zbkb)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zbc)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zbkc)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zbs) || currentlyEnabled(sail_ctx, extension::Ext_B))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zcb) && currentlyEnabled(sail_ctx, extension::Ext_Zca))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zhinx) && currentlyEnabled(sail_ctx, extension::Ext_Zfinx))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zfa) && currentlyEnabled(sail_ctx, extension::Ext_F))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zknh)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zkne)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zknd)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zksh)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zksed)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zbkx)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zicond)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zicbom)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zicboz)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zvbb) && currentlyEnabled(sail_ctx, extension::Ext_V))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    ((hartSupports(sail_ctx, extension::Ext_Zvkb) || currentlyEnabled(sail_ctx, extension::Ext_Zvbb)) && currentlyEnabled(sail_ctx, extension::Ext_V))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zvbc) && currentlyEnabled(sail_ctx, extension::Ext_V))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zvknha) && currentlyEnabled(sail_ctx, extension::Ext_V))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zvknhb) && currentlyEnabled(sail_ctx, extension::Ext_V))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zvksh) && currentlyEnabled(sail_ctx, extension::Ext_V))
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    hartSupports(sail_ctx, extension::Ext_Zimop)
}

pub fn currentlyEnabled(sail_ctx: &mut SailVirtCtx) -> bool {
    (hartSupports(sail_ctx, extension::Ext_Zcmop) && currentlyEnabled(sail_ctx, extension::Ext_Zca))
}

pub fn _get_Mstatus_MIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<3, 4, 1>()
}

pub fn _get_Mstatus_MPIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<7, 8, 1>()
}

pub fn _get_Mstatus_MPP(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<2> {
    v.subrange::<11, 13, 2>()
}

pub fn _get_Mstatus_SIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<1, 2, 1>()
}

pub fn _get_Mstatus_SPIE(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<5, 6, 1>()
}

pub fn _get_Mstatus_SPP(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<8, 9, 1>()
}

pub fn _get_Mstatus_TSR(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<22, 23, 1>()
}

pub fn _get_Mstatus_TVM(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<20, 21, 1>()
}

pub fn _get_Mstatus_TW(sail_ctx: &mut SailVirtCtx, v: Mstatus) -> BitVector<1> {
    v.subrange::<21, 22, 1>()
}

pub fn Mk_Minterrupts(sail_ctx: &mut SailVirtCtx, v: BitVector<64>) -> Minterrupts {
    Minterrupts {
        bits: v
    }
}

pub fn _get_Minterrupts_MEI(sail_ctx: &mut SailVirtCtx, v: Minterrupts) -> BitVector<1> {
    v.subrange::<11, 12, 1>()
}

pub fn _get_Minterrupts_MSI(sail_ctx: &mut SailVirtCtx, v: Minterrupts) -> BitVector<1> {
    v.subrange::<3, 4, 1>()
}

pub fn _get_Minterrupts_MTI(sail_ctx: &mut SailVirtCtx, v: Minterrupts) -> BitVector<1> {
    v.subrange::<7, 8, 1>()
}

pub fn _get_Minterrupts_SEI(sail_ctx: &mut SailVirtCtx, v: Minterrupts) -> BitVector<1> {
    v.subrange::<9, 10, 1>()
}

pub fn _get_Minterrupts_SSI(sail_ctx: &mut SailVirtCtx, v: Minterrupts) -> BitVector<1> {
    v.subrange::<1, 2, 1>()
}

pub fn _get_Minterrupts_STI(sail_ctx: &mut SailVirtCtx, v: Minterrupts) -> BitVector<1> {
    v.subrange::<5, 6, 1>()
}

pub fn _get_Mtvec_Base(sail_ctx: &mut SailVirtCtx, v: Mtvec) -> BitVector<62> {
    subrange_bits(v.bits, ((BUILTIN_pow2_TODO * 8) - 1), 2)
}

pub fn _get_Mtvec_Mode(sail_ctx: &mut SailVirtCtx, v: Mtvec) -> BitVector<2> {
    v.subrange::<0, 2, 2>()
}

pub fn _get_Mcause_Cause(sail_ctx: &mut SailVirtCtx, v: Mcause) -> BitVector<63> {
    subrange_bits(v.bits, ((BUILTIN_pow2_TODO * 8) - 2), 0)
}

pub fn _get_Mcause_IsInterrupt(sail_ctx: &mut SailVirtCtx, v: Mcause) -> BitVector<1> {
    subrange_bits(v.bits, ((BUILTIN_pow2_TODO * 8) - 1), ((BUILTIN_pow2_TODO * 8) - 1))
}

pub fn tvec_addr(sail_ctx: &mut SailVirtCtx, m: Mtvec, c: Mcause) -> Option<BitVector<{
    (usize::pow(3, 2) * 8)
}>> {
    let base: xlenbits = bitvector_concat(_get_Mtvec_Base(sail_ctx, m), BitVector::<2>::new(0b00));
    match {
        let var_4 = _get_Mtvec_Mode(sail_ctx, m);
        trapVectorMode_of_bits(sail_ctx, var_4)
    } {
        TrapVectorMode::TV_Direct => {Some(base)}
        TrapVectorMode::TV_Vector => {if {(_get_Mcause_IsInterrupt(sail_ctx, c) == BitVector::<1>::new(0b1))} {
            Some({
                let var_1 = ({
                    let var_2 = (BUILTIN_pow2_TODO * 8);
                    let var_3 = _get_Mcause_Cause(sail_ctx, c);
                    zero_extend(sail_ctx, var_2, var_3)
                } << 2);
                base.wrapped_add(var_1)
            })
        } else {
            Some(base)
        }}
        TrapVectorMode::TV_Reserved => {None}
        _ => {panic!("Unreachable code")}
    }
}

pub fn align_pc(sail_ctx: &mut SailVirtCtx, addr: BitVector<{
    (usize::pow(3, 2) * 8)
}>) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    if {(_get_Misa_C(sail_ctx, sail_ctx.misa) == BitVector::<1>::new(0b1))} {
        bitvector_update(addr, 0, false)
    } else {
        update_subrange_bits(addr, 1, 0, BUILTIN_zeros_TODO)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum agtype {
    UNDISTURBED,
    AGNOSTIC,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PmpAddrMatchType {
    OFF,
    TOR,
    NA4,
    NAPOT,
}

pub fn pmpAddrMatchType_of_bits(sail_ctx: &mut SailVirtCtx, bs: BitVector<2>) -> PmpAddrMatchType {
    match bs {
        b__0 if {(b__0 == BitVector::<2>::new(0b00))} => {PmpAddrMatchType::OFF}
        b__1 if {(b__1 == BitVector::<2>::new(0b01))} => {PmpAddrMatchType::TOR}
        b__2 if {(b__2 == BitVector::<2>::new(0b10))} => {PmpAddrMatchType::NA4}
        _ => {PmpAddrMatchType::NAPOT}
        _ => {panic!("Unreachable code")}
    }
}

pub fn pmpReadAddrReg(sail_ctx: &mut SailVirtCtx, n: usize) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    let G = sys_pmp_grain(());
    let match_type = _get_Pmpcfg_ent_A(sail_ctx, sail_ctx.pmpcfg_n[n]);
    let addr = sail_ctx.pmpaddr_n[n];
    match bitvector_access(match_type, 1) {
        true if {(G >= 2)} => {{
            let mask: xlenbits = {
                let var_1 = (BUILTIN_pow2_TODO * 8);
                let var_2 = {
                    let var_3 = min_int((G - 1), xlen);
                    ones(sail_ctx, var_3)
                };
                zero_extend(sail_ctx, var_1, var_2)
            };
            (addr | mask)
        }}
        false if {(G >= 1)} => {{
            let mask: xlenbits = {
                let var_4 = (BUILTIN_pow2_TODO * 8);
                let var_5 = {
                    let var_6 = min_int(G, xlen);
                    ones(sail_ctx, var_6)
                };
                zero_extend(sail_ctx, var_4, var_5)
            };
            (addr & !(mask))
        }}
        _ => {addr}
        _ => {panic!("Unreachable code")}
    }
}

pub fn pmpLocked(sail_ctx: &mut SailVirtCtx, cfg: Pmpcfg_ent) -> bool {
    (_get_Pmpcfg_ent_L(sail_ctx, cfg) == BitVector::<1>::new(0b1))
}

pub fn pmpCheckRWX<const N: usize>(sail_ctx: &mut SailVirtCtx, ent: Pmpcfg_ent, acc: AccessType<()>) -> bool {
    match acc {
        AccessType::Read(_) => {(_get_Pmpcfg_ent_R(sail_ctx, ent) == BitVector::<1>::new(0b1))}
        AccessType::Write(_) => {(_get_Pmpcfg_ent_W(sail_ctx, ent) == BitVector::<1>::new(0b1))}
        AccessType::ReadWrite(_) => {((_get_Pmpcfg_ent_R(sail_ctx, ent) == BitVector::<1>::new(0b1)) && (_get_Pmpcfg_ent_W(sail_ctx, ent) == BitVector::<1>::new(0b1)))}
        AccessType::InstructionFetch(()) => {(_get_Pmpcfg_ent_X(sail_ctx, ent) == BitVector::<1>::new(0b1))}
        _ => {panic!("Unreachable code")}
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum pmpAddrMatch {
    PMP_NoMatch,
    PMP_PartialMatch,
    PMP_Match,
}

pub fn pmpRangeMatch(sail_ctx: &mut SailVirtCtx, begin: nat, end_: nat, addr: nat, width: nat) -> pmpAddrMatch {
    if {(lteq_int((addr + width), begin) || lteq_int(end_, addr))} {
        pmpAddrMatch::PMP_NoMatch
    } else if {(lteq_int(begin, addr) && lteq_int((addr + width), end_))} {
        pmpAddrMatch::PMP_Match
    } else {
        pmpAddrMatch::PMP_PartialMatch
    }
}

pub fn pmpMatchAddr(sail_ctx: &mut SailVirtCtx, Physaddr(addr): physaddr, width: BitVector<{
    (usize::pow(3, 2) * 8)
}>, ent: Pmpcfg_ent, pmpaddr: BitVector<{
    (usize::pow(3, 2) * 8)
}>, prev_pmpaddr: BitVector<{
    (usize::pow(3, 2) * 8)
}>) -> pmpAddrMatch {
    let addr = addr.as_usize();
    let width = width.as_usize();
    match {
        let var_13 = _get_Pmpcfg_ent_A(sail_ctx, ent);
        pmpAddrMatchType_of_bits(sail_ctx, var_13)
    } {
        PmpAddrMatchType::OFF => {pmpAddrMatch::PMP_NoMatch}
        PmpAddrMatchType::TOR => {{
            if {_operator_biggerequal_u_(sail_ctx, prev_pmpaddr, pmpaddr)} {
                pmpAddrMatch::PMP_NoMatch
            } else {
                {
                    let var_1 = (prev_pmpaddr.as_usize() * 4);
                    let var_2 = (pmpaddr.as_usize() * 4);
                    let var_3 = addr;
                    let var_4 = width;
                    pmpRangeMatch(sail_ctx, var_1, var_2, var_3, var_4)
                }
            }
        }}
        PmpAddrMatchType::NA4 => {{
            assert!((sys_pmp_grain(()) < 1), "Process message");
            let begin = (pmpaddr.as_usize() * 4);
            {
                let var_5 = begin;
                let var_6 = (begin + 4);
                let var_7 = addr;
                let var_8 = width;
                pmpRangeMatch(sail_ctx, var_5, var_6, var_7, var_8)
            }
        }}
        PmpAddrMatchType::NAPOT => {{
            let mask = (pmpaddr ^ (pmpaddr + 1));
            let begin_words = (pmpaddr & !(mask)).as_usize();
            let end_words = ((begin_words + mask.as_usize()) + 1);
            {
                let var_9 = (begin_words * 4);
                let var_10 = (end_words * 4);
                let var_11 = addr;
                let var_12 = width;
                pmpRangeMatch(sail_ctx, var_9, var_10, var_11, var_12)
            }
        }}
        _ => {panic!("Unreachable code")}
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum pmpMatch {
    PMP_Success,
    PMP_Continue,
    PMP_Fail,
}

pub fn pmpMatchEntry<const N: usize>(sail_ctx: &mut SailVirtCtx, addr: physaddr, width: BitVector<{
    (usize::pow(3, 2) * 8)
}>, acc: AccessType<()>, _priv_: Privilege, ent: Pmpcfg_ent, pmpaddr: BitVector<{
    (usize::pow(3, 2) * 8)
}>, prev_pmpaddr: BitVector<{
    (usize::pow(3, 2) * 8)
}>) -> pmpMatch {
    match pmpMatchAddr(sail_ctx, addr, width, ent, pmpaddr, prev_pmpaddr) {
        pmpAddrMatch::PMP_NoMatch => {pmpMatch::PMP_Continue}
        pmpAddrMatch::PMP_PartialMatch => {pmpMatch::PMP_Fail}
        pmpAddrMatch::PMP_Match => {if {(pmpCheckRWX(sail_ctx, ent, acc) || ((_priv_ == Privilege::Machine) && !(pmpLocked(sail_ctx, ent))))} {
            pmpMatch::PMP_Success
        } else {
            pmpMatch::PMP_Fail
        }}
        _ => {panic!("Unreachable code")}
    }
}

pub fn accessToFault<const N: usize>(sail_ctx: &mut SailVirtCtx, acc: AccessType<()>) -> ExceptionType {
    match acc {
        AccessType::Read(_) => {ExceptionType::E_Load_Access_Fault(())}
        AccessType::Write(_) => {ExceptionType::E_SAMO_Access_Fault(())}
        AccessType::ReadWrite(_) => {ExceptionType::E_SAMO_Access_Fault(())}
        AccessType::InstructionFetch(()) => {ExceptionType::E_Fetch_Access_Fault(())}
        _ => {panic!("Unreachable code")}
    }
}

pub fn pmpCheck<const N: usize>(sail_ctx: &mut SailVirtCtx, addr: physaddr, width: usize, acc: AccessType<()>, _priv_: Privilege) -> Option<ExceptionType> {
    let width: xlenbits = to_bits(sail_ctx, xlen, width);
    for i in 0..=63 {
        let prev_pmpaddr = if {gt_int(i, 0)} {
            {
                let var_8 = (i - 1);
                pmpReadAddrReg(sail_ctx, var_8)
            }
        } else {
            BUILTIN_zeros_TODO
        };
        match {
            let var_1 = addr;
            let var_2 = width;
            let var_3 = acc;
            let var_4 = _priv_;
            let var_5 = sail_ctx.pmpcfg_n[i];
            let var_6 = pmpReadAddrReg(sail_ctx, i);
            let var_7 = prev_pmpaddr;
            pmpMatchEntry(sail_ctx, var_1, var_2, var_3, var_4, var_5, var_6, var_7)
        } {
            pmpMatch::PMP_Success => {{
                return None;
            }}
            pmpMatch::PMP_Fail => {{
                return Some(accessToFault(sail_ctx, acc));
            }}
            pmpMatch::PMP_Continue => {()}
            _ => {panic!("Unreachable code")}
        }
    };
    if {(_priv_ == Privilege::Machine)} {
        None
    } else {
        Some(accessToFault(sail_ctx, acc))
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Ext_FetchAddr_Check {
    Ext_FetchAddr_OK(virtaddr),
    Ext_FetchAddr_Error('a),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Ext_ControlAddr_Check {
    Ext_ControlAddr_OK(virtaddr),
    Ext_ControlAddr_Error('a),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Ext_DataAddr_Check {
    Ext_DataAddr_OK(virtaddr),
    Ext_DataAddr_Error('a),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Ext_PhysAddr_Check {
    Ext_PhysAddr_OK(()),
    Ext_PhysAddr_Error(ExceptionType),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum maskfunct3 {
    VV_VMERGE,
    VI_VMERGE,
    VX_VMERGE,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregno {
    Vregno(usize),
}

pub fn ext_check_xret_priv(sail_ctx: &mut SailVirtCtx, p: Privilege) -> bool {
    true
}

pub fn handle_trap_extension(sail_ctx: &mut SailVirtCtx, p: Privilege, pc: BitVector<{
    (usize::pow(3, 2) * 8)
}>, u: Option<()>) {
    ()
}

pub fn prepare_trap_vector(sail_ctx: &mut SailVirtCtx, p: Privilege, cause: Mcause) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    let tvec: Mtvec = match p {
        Privilege::Machine => {sail_ctx.mtvec}
        Privilege::Supervisor => {sail_ctx.stvec}
        Privilege::User => {internal_error("riscv_sys_exceptions.sail", 25, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    };
    match tvec_addr(sail_ctx, tvec, cause) {
        Some(epc) => {epc}
        None => {internal_error("riscv_sys_exceptions.sail", 29, "Invalid tvec mode")}
        _ => {panic!("Unreachable code")}
    }
}

pub fn get_xepc(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    match p {
        Privilege::Machine => {align_pc(sail_ctx, sail_ctx.mepc)}
        Privilege::Supervisor => {align_pc(sail_ctx, sail_ctx.sepc)}
        Privilege::User => {internal_error("riscv_sys_exceptions.sail", 45, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

pub fn prepare_xret_target(sail_ctx: &mut SailVirtCtx, p: Privilege) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    get_xepc(sail_ctx, p)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum seed_opst {
    BIST,
    ES16,
    WAIT,
    DEAD,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum fregno {
    Fregno(usize),
}

pub fn exception_delegatee(sail_ctx: &mut SailVirtCtx, e: ExceptionType, p: Privilege) -> Privilege {
    let idx = num_of_ExceptionType(sail_ctx, e);
    let _super_ = {
        let var_3 = bitvector_access(sail_ctx.medeleg.bits, idx);
        bit_to_bool(sail_ctx, var_3)
    };
    let deleg = if {(currentlyEnabled(sail_ctx, extension::Ext_S) && _super_)} {
        Privilege::Supervisor
    } else {
        Privilege::Machine
    };
    if {{
        let var_1 = privLevel_to_bits(sail_ctx, deleg);
        let var_2 = privLevel_to_bits(sail_ctx, p);
        _operator_smaller_u_(sail_ctx, var_1, var_2)
    }} {
        p
    } else {
        deleg
    }
}

pub fn findPendingInterrupt(sail_ctx: &mut SailVirtCtx, ip: BitVector<{
    (usize::pow(3, 2) * 8)
}>) -> Option<InterruptType> {
    let ip = Mk_Minterrupts(sail_ctx, ip);
    if {(_get_Minterrupts_MEI(sail_ctx, ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_M_External)
    } else if {(_get_Minterrupts_MSI(sail_ctx, ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_M_Software)
    } else if {(_get_Minterrupts_MTI(sail_ctx, ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_M_Timer)
    } else if {(_get_Minterrupts_SEI(sail_ctx, ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_S_External)
    } else if {(_get_Minterrupts_SSI(sail_ctx, ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_S_Software)
    } else if {(_get_Minterrupts_STI(sail_ctx, ip) == BitVector::<1>::new(0b1))} {
        Some(InterruptType::I_S_Timer)
    } else {
        None
    }
}

pub fn getPendingSet(sail_ctx: &mut SailVirtCtx, _priv_: Privilege) -> Option<(BitVector<{
    (usize::pow(3, 2) * 8)
}>, Privilege)> {
    assert!((currentlyEnabled(sail_ctx, extension::Ext_S) || (sail_ctx.mideleg.bits == BUILTIN_zeros_TODO)), "Process message");
    let pending_m = (sail_ctx.mip.bits & (sail_ctx.mie.bits & !(sail_ctx.mideleg.bits)));
    let pending_s = (sail_ctx.mip.bits & (sail_ctx.mie.bits & sail_ctx.mideleg.bits));
    let mIE = (((_priv_ == Privilege::Machine) && (_get_Mstatus_MIE(sail_ctx, sail_ctx.mstatus) == BitVector::<1>::new(0b1))) || ((_priv_ == Privilege::Supervisor) || (_priv_ == Privilege::User)));
    let sIE = (((_priv_ == Privilege::Supervisor) && (_get_Mstatus_SIE(sail_ctx, sail_ctx.mstatus) == BitVector::<1>::new(0b1))) || (_priv_ == Privilege::User));
    if {(mIE && (pending_m != BUILTIN_zeros_TODO))} {
        Some((pending_m, Privilege::Machine))
    } else if {(sIE && (pending_s != BUILTIN_zeros_TODO))} {
        Some((pending_s, Privilege::Supervisor))
    } else {
        None
    }
}

pub fn dispatchInterrupt(sail_ctx: &mut SailVirtCtx, _priv_: Privilege) -> Option<(InterruptType, Privilege)> {
    match getPendingSet(sail_ctx, _priv_) {
        None => {None}
        Some((ip, p)) => {match findPendingInterrupt(sail_ctx, ip) {
            None => {None}
            Some(i) => {Some((i, p))}
            _ => {panic!("Unreachable code")}
        }}
        _ => {panic!("Unreachable code")}
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ctl_result {
    CTL_TRAP(sync_exception),
    CTL_SRET(()),
    CTL_MRET(()),
}

pub fn tval(sail_ctx: &mut SailVirtCtx, excinfo: Option<BitVector<{
    (usize::pow(3, 2) * 8)
}>>) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    match excinfo {
        Some(e) => {e}
        None => {BUILTIN_zeros_TODO}
        _ => {panic!("Unreachable code")}
    }
}

pub fn track_trap(sail_ctx: &mut SailVirtCtx, p: Privilege) {
    {
        let var_1 = "mstatus";
        let var_2 = "mstatush";
        let var_3 = sail_ctx.mstatus.bits;
        long_csr_write_callback(sail_ctx, var_1, var_2, var_3)
    };
    match p {
        Privilege::Machine => {{
            {
                let var_4 = "mcause";
                let var_5 = sail_ctx.mcause.bits;
                csr_name_write_callback(sail_ctx, var_4, var_5)
            };
            csr_name_write_callback(sail_ctx, "mtval", sail_ctx.mtval);
            csr_name_write_callback(sail_ctx, "mepc", sail_ctx.mepc)
        }}
        Privilege::Supervisor => {{
            {
                let var_6 = "scause";
                let var_7 = sail_ctx.scause.bits;
                csr_name_write_callback(sail_ctx, var_6, var_7)
            };
            csr_name_write_callback(sail_ctx, "stval", sail_ctx.stval);
            csr_name_write_callback(sail_ctx, "sepc", sail_ctx.sepc)
        }}
        Privilege::User => {internal_error("riscv_sys_control.sail", 217, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

pub fn trap_handler(sail_ctx: &mut SailVirtCtx, del_priv: Privilege, intr: bool, c: BitVector<8>, pc: BitVector<{
    (usize::pow(3, 2) * 8)
}>, info: Option<BitVector<{
    (usize::pow(3, 2) * 8)
}>>, ext: Option<()>) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    trap_callback(sail_ctx, ());
    if {get_config_print_platform(sail_ctx, ())} {
        print_platform(format!("{}{}", "handling ", format!("{}{}", if {intr} {
            "int_hashtag_"
        } else {
            "exc_hashtag_"
        }, format!("{}{}", bits_str(c), format!("{}{}", " at priv ", format!("{}{}", privLevel_to_str(sail_ctx, del_priv), format!("{}{}", " with tval ", bits_str(tval(sail_ctx, info)))))))))
    } else {
        ()
    };
    match del_priv {
        Privilege::Machine => {{
            sail_ctx.mcause.bits[((BUILTIN_pow2_TODO * 8) - 1)..((BUILTIN_pow2_TODO * 8) - 1)] = bool_to_bits(sail_ctx, intr);
            sail_ctx.mcause.bits[((BUILTIN_pow2_TODO * 8) - 2)..0] = zero_extend_63(c);
            sail_ctx.mstatus = {
                let var_1 = _get_Mstatus_MIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.set_subrange::<7, 8, 1>(var_1)
            };
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<3, 4, 1>(BitVector::<1>::new(0b0));
            sail_ctx.mstatus = {
                let var_2 = privLevel_to_bits(sail_ctx, sail_ctx.cur_privilege);
                sail_ctx.mstatus.set_subrange::<11, 13, 2>(var_2)
            };
            sail_ctx.mtval = tval(sail_ctx, info);
            sail_ctx.mepc = pc;
            sail_ctx.cur_privilege = del_priv;
            handle_trap_extension(sail_ctx, del_priv, pc, ext);
            track_trap(sail_ctx, del_priv);
            prepare_trap_vector(sail_ctx, del_priv, sail_ctx.mcause)
        }}
        Privilege::Supervisor => {{
            assert!(currentlyEnabled(sail_ctx, extension::Ext_S), "Process message");
            sail_ctx.scause.bits[((BUILTIN_pow2_TODO * 8) - 1)..((BUILTIN_pow2_TODO * 8) - 1)] = bool_to_bits(sail_ctx, intr);
            sail_ctx.scause.bits[((BUILTIN_pow2_TODO * 8) - 2)..0] = zero_extend_63(c);
            sail_ctx.mstatus = {
                let var_3 = _get_Mstatus_SIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.set_subrange::<5, 6, 1>(var_3)
            };
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<1, 2, 1>(BitVector::<1>::new(0b0));
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<8, 9, 1>(match sail_ctx.cur_privilege {
                Privilege::User => {BitVector::<1>::new(0b0)}
                Privilege::Supervisor => {BitVector::<1>::new(0b1)}
                Privilege::Machine => {internal_error("riscv_sys_control.sail", 260, "invalid privilege for s-mode trap")}
                _ => {panic!("Unreachable code")}
            });
            sail_ctx.stval = tval(sail_ctx, info);
            sail_ctx.sepc = pc;
            sail_ctx.cur_privilege = del_priv;
            handle_trap_extension(sail_ctx, del_priv, pc, ext);
            track_trap(sail_ctx, del_priv);
            prepare_trap_vector(sail_ctx, del_priv, sail_ctx.scause)
        }}
        Privilege::User => {internal_error("riscv_sys_control.sail", 273, "Invalid privilege level")}
        _ => {panic!("Unreachable code")}
    }
}

pub fn exception_handler(sail_ctx: &mut SailVirtCtx, cur_priv: Privilege, ctl: ctl_result, pc: BitVector<{
    (usize::pow(3, 2) * 8)
}>) -> BitVector<{
    (usize::pow(3, 2) * 8)
}> {
    match (cur_priv, ctl) {
        (_, ctl_result::CTL_TRAP(e)) => {{
            let del_priv = {
                let var_9 = e.trap;
                let var_10 = cur_priv;
                exception_delegatee(sail_ctx, var_9, var_10)
            };
            if {get_config_print_platform(sail_ctx, ())} {
                print_platform(format!("{}{}", "trapping from ", format!("{}{}", privLevel_to_str(sail_ctx, cur_priv), format!("{}{}", " to ", format!("{}{}", privLevel_to_str(sail_ctx, del_priv), format!("{}{}", " to handle ", {
                    let var_1 = e.trap;
                    exceptionType_to_str(sail_ctx, var_1)
                }))))))
            } else {
                ()
            };
            {
                let var_2 = del_priv;
                let var_3 = false;
                let var_4 = {
                    let var_8 = e.trap;
                    exceptionType_to_bits(sail_ctx, var_8)
                };
                let var_5 = pc;
                let var_6 = e.excinfo;
                let var_7 = e.ext;
                trap_handler(sail_ctx, var_2, var_3, var_4, var_5, var_6, var_7)
            }
        }}
        (_, ctl_result::CTL_MRET(())) => {{
            let prev_priv = sail_ctx.cur_privilege;
            sail_ctx.mstatus = {
                let var_11 = _get_Mstatus_MPIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.set_subrange::<3, 4, 1>(var_11)
            };
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<7, 8, 1>(BitVector::<1>::new(0b1));
            sail_ctx.cur_privilege = {
                let var_12 = _get_Mstatus_MPP(sail_ctx, sail_ctx.mstatus);
                privLevel_of_bits(sail_ctx, var_12)
            };
            sail_ctx.mstatus = {
                let var_13 = {
                    let var_14 = if {currentlyEnabled(sail_ctx, extension::Ext_U)} {
                        Privilege::User
                    } else {
                        Privilege::Machine
                    };
                    privLevel_to_bits(sail_ctx, var_14)
                };
                sail_ctx.mstatus.set_subrange::<11, 13, 2>(var_13)
            };
            if {(sail_ctx.cur_privilege != Privilege::Machine)} {
                sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<17, 18, 1>(BitVector::<1>::new(0b0))
            } else {
                ()
            };
            {
                let var_15 = "mstatus";
                let var_16 = "mstatush";
                let var_17 = sail_ctx.mstatus.bits;
                long_csr_write_callback(sail_ctx, var_15, var_16, var_17)
            };
            if {get_config_print_platform(sail_ctx, ())} {
                print_platform(format!("{}{}", "ret-ing from ", format!("{}{}", privLevel_to_str(sail_ctx, prev_priv), format!("{}{}", " to ", privLevel_to_str(sail_ctx, sail_ctx.cur_privilege)))))
            } else {
                ()
            };
            prepare_xret_target(sail_ctx, Privilege::Machine)
        }}
        (_, ctl_result::CTL_SRET(())) => {{
            let prev_priv = sail_ctx.cur_privilege;
            sail_ctx.mstatus = {
                let var_18 = _get_Mstatus_SPIE(sail_ctx, sail_ctx.mstatus);
                sail_ctx.mstatus.set_subrange::<1, 2, 1>(var_18)
            };
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<5, 6, 1>(BitVector::<1>::new(0b1));
            sail_ctx.cur_privilege = if {(_get_Mstatus_SPP(sail_ctx, sail_ctx.mstatus) == BitVector::<1>::new(0b1))} {
                Privilege::Supervisor
            } else {
                Privilege::User
            };
            sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<8, 9, 1>(BitVector::<1>::new(0b0));
            if {(sail_ctx.cur_privilege != Privilege::Machine)} {
                sail_ctx.mstatus = sail_ctx.mstatus.set_subrange::<17, 18, 1>(BitVector::<1>::new(0b0))
            } else {
                ()
            };
            {
                let var_19 = "mstatus";
                let var_20 = "mstatush";
                let var_21 = sail_ctx.mstatus.bits;
                long_csr_write_callback(sail_ctx, var_19, var_20, var_21)
            };
            if {get_config_print_platform(sail_ctx, ())} {
                print_platform(format!("{}{}", "ret-ing from ", format!("{}{}", privLevel_to_str(sail_ctx, prev_priv), format!("{}{}", " to ", privLevel_to_str(sail_ctx, sail_ctx.cur_privilege)))))
            } else {
                ()
            };
            prepare_xret_target(sail_ctx, Privilege::Supervisor)
        }}
        _ => {panic!("Unreachable code")}
    }
}

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
    Ext_XRET_Priv_Failure(()),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PTE_Check {
    PTE_Check_Success(ext_ptw),
    PTE_Check_Failure((ext_ptw, ext_ptw_fail)),
}

pub fn flush_TLB_Entry(sail_ctx: &mut SailVirtCtx, ent: TLB_Entry, asid: Option<BitVector<16>>, vaddr: Option<BitVector<{
    (usize::pow(3, 2) * 8)
}>>) -> bool {
    let asid_matches: bool = match asid {
        Some(asid) => {((ent.asid == asid) && !(ent.global))}
        None => {true}
        _ => {panic!("Unreachable code")}
    };
    let addr_matches: bool = match vaddr {
        Some(vaddr) => {{
            let vaddr: BitVector<64> = sign_extend(64, vaddr);
            (ent.vpn == (subrange_bits(vaddr, 56, pagesize_bits) & !(ent.levelMask)))
        }}
        None => {true}
        _ => {panic!("Unreachable code")}
    };
    (asid_matches && addr_matches)
}

pub fn flush_TLB(sail_ctx: &mut SailVirtCtx, asid: Option<BitVector<16>>, addr: Option<BitVector<{
    (usize::pow(3, 2) * 8)
}>>) {
    todo!("E_for")
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum TR_Result {
    TR_Address(('paddr, ext_ptw)),
    TR_Failure(('failure, ext_ptw)),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum cbie {
    CBIE_ILLEGAL,
    CBIE_EXEC_FLUSH,
    CBIE_EXEC_INVAL,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum checked_cbop {
    CBOP_ILLEGAL,
    CBOP_ILLEGAL_VIRTUAL,
    CBOP_INVAL_FLUSH,
    CBOP_INVAL_INVAL,
}

pub fn execute_ITYPE(sail_ctx: &mut SailVirtCtx, imm: BitVector<12>, rs1: regidx, rd: regidx, op: iop) -> ExecutionResult {
    let immext: xlenbits = sign_extend((BUILTIN_pow2_TODO * 8), imm);
    wX_bits(sail_ctx, rd, match op {
        iop::ADDI => {rX_bits(sail_ctx, rs1).wrapped_add(immext)}
        iop::SLTI => {{
            let var_1 = (BUILTIN_pow2_TODO * 8);
            let var_2 = {
                let var_3 = {
                    let var_4 = rX_bits(sail_ctx, rs1);
                    let var_5 = immext;
                    _operator_smaller_s_(sail_ctx, var_4, var_5)
                };
                bool_to_bits(sail_ctx, var_3)
            };
            zero_extend(sail_ctx, var_1, var_2)
        }}
        iop::SLTIU => {{
            let var_6 = (BUILTIN_pow2_TODO * 8);
            let var_7 = {
                let var_8 = {
                    let var_9 = rX_bits(sail_ctx, rs1);
                    let var_10 = immext;
                    _operator_smaller_u_(sail_ctx, var_9, var_10)
                };
                bool_to_bits(sail_ctx, var_8)
            };
            zero_extend(sail_ctx, var_6, var_7)
        }}
        iop::ANDI => {(rX_bits(sail_ctx, rs1) & immext)}
        iop::ORI => {(rX_bits(sail_ctx, rs1) | immext)}
        iop::XORI => {(rX_bits(sail_ctx, rs1) ^ immext)}
        _ => {panic!("Unreachable code")}
    });
    RETIRE_SUCCESS
}

pub fn execute_MRET(sail_ctx: &mut SailVirtCtx) -> ExecutionResult {
    if {(sail_ctx.cur_privilege != Privilege::Machine)} {
        ExecutionResult::Illegal_Instruction(())
    } else if {!(ext_check_xret_priv(sail_ctx, Privilege::Machine))} {
        ExecutionResult::Ext_XRET_Priv_Failure(())
    } else {
        {
            let var_1 = {
                let var_2 = sail_ctx.cur_privilege;
                let var_3 = ctl_result::CTL_MRET(());
                let var_4 = sail_ctx.PC;
                exception_handler(sail_ctx, var_2, var_3, var_4)
            };
            set_next_pc(sail_ctx, var_1)
        };
        RETIRE_SUCCESS
    }
}

pub fn execute_SRET(sail_ctx: &mut SailVirtCtx) -> ExecutionResult {
    let sret_illegal: bool = match sail_ctx.cur_privilege {
        Privilege::User => {true}
        Privilege::Supervisor => {(!(currentlyEnabled(sail_ctx, extension::Ext_S)) || (_get_Mstatus_TSR(sail_ctx, sail_ctx.mstatus) == BitVector::<1>::new(0b1)))}
        Privilege::Machine => {!(currentlyEnabled(sail_ctx, extension::Ext_S))}
        _ => {panic!("Unreachable code")}
    };
    if {sret_illegal} {
        ExecutionResult::Illegal_Instruction(())
    } else if {!(ext_check_xret_priv(sail_ctx, Privilege::Supervisor))} {
        ExecutionResult::Ext_XRET_Priv_Failure(())
    } else {
        {
            let var_1 = {
                let var_2 = sail_ctx.cur_privilege;
                let var_3 = ctl_result::CTL_SRET(());
                let var_4 = sail_ctx.PC;
                exception_handler(sail_ctx, var_2, var_3, var_4)
            };
            set_next_pc(sail_ctx, var_1)
        };
        RETIRE_SUCCESS
    }
}

pub fn execute_EBREAK(sail_ctx: &mut SailVirtCtx) -> ExecutionResult {
    ExecutionResult::Memory_Exception((virtaddr::Virtaddr(sail_ctx.PC), ExceptionType::E_Breakpoint(())))
}

pub fn execute_WFI(sail_ctx: &mut SailVirtCtx) -> ExecutionResult {
    match sail_ctx.cur_privilege {
        Privilege::Machine => {ExecutionResult::Wait_For_Interrupt(())}
        Privilege::Supervisor => {if {(_get_Mstatus_TW(sail_ctx, sail_ctx.mstatus) == BitVector::<1>::new(0b1))} {
            ExecutionResult::Illegal_Instruction(())
        } else {
            ExecutionResult::Wait_For_Interrupt(())
        }}
        Privilege::User => {ExecutionResult::Illegal_Instruction(())}
        _ => {panic!("Unreachable code")}
    }
}

pub fn execute_SFENCE_VMA(sail_ctx: &mut SailVirtCtx, rs1: regidx, rs2: regidx) -> ExecutionResult {
    let addr = if {(rs1 != zreg)} {
        Some(rX_bits(sail_ctx, rs1))
    } else {
        None
    };
    let asid = if {(rs2 != zreg)} {
        Some(subrange_bits(rX_bits(sail_ctx, rs2), (asidlen - 1), 0))
    } else {
        None
    };
    match sail_ctx.cur_privilege {
        Privilege::User => {ExecutionResult::Illegal_Instruction(())}
        Privilege::Supervisor => {match _get_Mstatus_TVM(sail_ctx, sail_ctx.mstatus) {
            b__0 if {(b__0 == BitVector::<1>::new(0b1))} => {ExecutionResult::Illegal_Instruction(())}
            _ => {{
                flush_TLB(sail_ctx, asid, addr);
                RETIRE_SUCCESS
            }}
            _ => {panic!("Unreachable code")}
        }}
        Privilege::Machine => {{
            flush_TLB(sail_ctx, asid, addr);
            RETIRE_SUCCESS
        }}
        _ => {panic!("Unreachable code")}
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum HartState {
    HART_ACTIVE(()),
    HART_WAITING(instbits),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum FetchResult {
    F_Ext_Error(ext_fetch_addr_error),
    F_Base(word),
    F_RVC(half),
    F_Error((ExceptionType, xlenbits)),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Step {
    Step_Pending_Interrupt((InterruptType, Privilege)),
    Step_Ext_Fetch_Failure(ext_fetch_addr_error),
    Step_Fetch_Failure((virtaddr, ExceptionType)),
    Step_Execute((ExecutionResult, instbits)),
    Step_Waiting(()),
}