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
    pub vtype: Vtype,
    pub vstart: BitVector,
    pub vl: xlenbits,
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
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {

}

/// Initialize all registers.
///
/// This function should be called before using a fresh core, otherwise the core might not be in a valid state.
pub fn _reset_all_registers() {
    
}

/// __id
///
/// Generated from the Sail sources at `sail/lib/flow.sail` L107.
pub fn __id(x: i128) -> i128 {
    x
}

/// result
///
/// Generated from the Sail sources at `sail/lib/result.sail` L60-63.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum result<A, B> {
    Ok(A),
    Err(B)
}

/// to_bits
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L59.
pub fn to_bits(l: i128, n: i128) -> BitVector {
    get_slice_int(l, n, 0)
}

/// zeros
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L62.
pub const fn zeros(n: i128) -> BitVector {
    sail_zeros(n)
}

/// ones
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L65.
pub const fn ones(n: i128) -> BitVector {
    sail_ones(n)
}

pub const log2_xlen_bytes: i128 = 3;

pub const log2_xlen: i128 = 6;

pub const xlen_bytes: i128 = i128::pow(2, ((3 as u32) as u32));

pub const xlen: i128 = (xlen_bytes * 8);

pub type xlenbits = BitVector;

/// Vtype
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L96-103.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Vtype {
    pub bits: BitVector,
}

/// _get_Vtype_vill
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vill(v: Vtype) -> BitVector {
    v.bits.subrange::<63, 64, 1>()
}

/// _get_Vtype_vlmul
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vlmul(v: Vtype) -> BitVector {
    v.bits.subrange::<0, 3, 3>()
}

/// _get_Vtype_vma
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vma(v: Vtype) -> BitVector {
    v.bits.subrange::<7, 8, 1>()
}

/// _get_Vtype_vsew
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vsew(v: Vtype) -> BitVector {
    v.bits.subrange::<3, 6, 3>()
}

/// _get_Vtype_vta
///
/// Generated from the Sail sources.
pub fn _get_Vtype_vta(v: Vtype) -> BitVector {
    v.bits.subrange::<6, 7, 1>()
}

/// get_sew_pow
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L107-116.
pub fn get_sew_pow(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    let SEW_pow: i128 = match {
        let var_1 = core_ctx.vtype;
        _get_Vtype_vsew(var_1)
    } {
        b__0 if {(b__0 == BitVector::new(3, 0b000))} => {3}
        b__1 if {(b__1 == BitVector::new(3, 0b001))} => {4}
        b__2 if {(b__2 == BitVector::new(3, 0b010))} => {5}
        b__3 if {(b__3 == BitVector::new(3, 0b011))} => {6}
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
/// Generated from the Sail sources at `tests/vec/arch.sail` L119-127.
pub fn get_sew(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    match get_sew_pow(core_ctx, ()) {
        l__67 if {(l__67 == 3)} => {8}
        l__68 if {(l__68 == 4)} => {16}
        l__69 if {(l__69 == 5)} => {32}
        l__70 if {(l__70 == 6)} => {64}
        _ => {{
            assert!(false, "invalid SEW");
            panic!("exit")
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// get_lmul_pow
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L132-143.
pub fn get_lmul_pow(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    match {
        let var_1 = core_ctx.vtype;
        _get_Vtype_vlmul(var_1)
    } {
        b__0 if {(b__0 == BitVector::new(3, 0b101))} => {-3}
        b__1 if {(b__1 == BitVector::new(3, 0b110))} => {-2}
        b__2 if {(b__2 == BitVector::new(3, 0b111))} => {-1}
        b__3 if {(b__3 == BitVector::new(3, 0b000))} => {0}
        b__4 if {(b__4 == BitVector::new(3, 0b001))} => {1}
        b__5 if {(b__5 == BitVector::new(3, 0b010))} => {2}
        b__6 if {(b__6 == BitVector::new(3, 0b011))} => {3}
        _ => {{
            assert!(false, "invalid vlmul field in vtype");
            panic!("exit")
        }}
        _ => {panic!("Unreachable code")}
    }
}

/// agtype
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L149.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum agtype {
    UNDISTURBED,
    AGNOSTIC
}

/// decode_agtype
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L152-157.
pub fn decode_agtype(ag: BitVector) -> agtype {
    match ag {
        b__0 if {(b__0 == BitVector::new(1, 0b0))} => {agtype::UNDISTURBED}
        _ => {agtype::AGNOSTIC}
        _ => {panic!("Unreachable code")}
    }
}

/// get_vtype_vma
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L160.
pub fn get_vtype_vma(core_ctx: &mut Core, unit_arg: ()) -> agtype {
    let var_1 = {
        let var_2 = core_ctx.vtype;
        _get_Vtype_vma(var_2)
    };
    decode_agtype(var_1)
}

/// get_vtype_vta
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L163.
pub fn get_vtype_vta(core_ctx: &mut Core, unit_arg: ()) -> agtype {
    let var_1 = {
        let var_2 = core_ctx.vtype;
        _get_Vtype_vta(var_2)
    };
    decode_agtype(var_1)
}

/// get_vlen_pow
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L168.
pub const fn get_vlen_pow(unit_arg: ()) -> i128 {
    9
}

pub const vlenmax: i128 = 65536;

pub const VLEN: i128 = 512;

pub type vreglenbits = BitVector;

pub type vregtype = vreglenbits;

/// vvfunct6
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L178-184.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvfunct6 {
    VV_VADD,
    VV_VSUB,
    VV_VAND,
    VV_VOR,
    VV_VXOR
}

/// vregidx
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L188.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregidx {
    Vregidx(BitVector)
}

/// vregno
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L189.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregno {
    Vregno(i128)
}

/// vregidx_to_vregno
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L190.
pub fn vregidx_to_vregno(vregidx::Vregidx(b): vregidx) -> vregno {
    vregno::Vregno(b.unsigned())
}

/// vregidx_offset
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L192.
pub fn vregidx_offset(vregidx::Vregidx(r): vregidx, o: BitVector) -> vregidx {
    vregidx::Vregidx(r.wrapped_add(o))
}

/// vregidx_bits
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L193.
pub fn vregidx_bits(vregidx::Vregidx(b): vregidx) -> BitVector {
    b
}

pub const zvreg: vregidx = vregidx::Vregidx(BitVector::new(5, 0b00000));

/// rV
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L231-266.
pub fn rV(core_ctx: &mut Core, vregno::Vregno(r): vregno) -> BitVector {
    match r {
        l__31 if {(l__31 == 0)} => {core_ctx.vr0}
        l__32 if {(l__32 == 1)} => {core_ctx.vr1}
        l__33 if {(l__33 == 2)} => {core_ctx.vr2}
        l__34 if {(l__34 == 3)} => {core_ctx.vr3}
        l__35 if {(l__35 == 4)} => {core_ctx.vr4}
        l__36 if {(l__36 == 5)} => {core_ctx.vr5}
        l__37 if {(l__37 == 6)} => {core_ctx.vr6}
        l__38 if {(l__38 == 7)} => {core_ctx.vr7}
        l__39 if {(l__39 == 8)} => {core_ctx.vr8}
        l__40 if {(l__40 == 9)} => {core_ctx.vr9}
        l__41 if {(l__41 == 10)} => {core_ctx.vr10}
        l__42 if {(l__42 == 11)} => {core_ctx.vr11}
        l__43 if {(l__43 == 12)} => {core_ctx.vr12}
        l__44 if {(l__44 == 13)} => {core_ctx.vr13}
        l__45 if {(l__45 == 14)} => {core_ctx.vr14}
        l__46 if {(l__46 == 15)} => {core_ctx.vr15}
        l__47 if {(l__47 == 16)} => {core_ctx.vr16}
        l__48 if {(l__48 == 17)} => {core_ctx.vr17}
        l__49 if {(l__49 == 18)} => {core_ctx.vr18}
        l__50 if {(l__50 == 19)} => {core_ctx.vr19}
        l__51 if {(l__51 == 20)} => {core_ctx.vr20}
        l__52 if {(l__52 == 21)} => {core_ctx.vr21}
        l__53 if {(l__53 == 22)} => {core_ctx.vr22}
        l__54 if {(l__54 == 23)} => {core_ctx.vr23}
        l__55 if {(l__55 == 24)} => {core_ctx.vr24}
        l__56 if {(l__56 == 25)} => {core_ctx.vr25}
        l__57 if {(l__57 == 26)} => {core_ctx.vr26}
        l__58 if {(l__58 == 27)} => {core_ctx.vr27}
        l__59 if {(l__59 == 28)} => {core_ctx.vr28}
        l__60 if {(l__60 == 29)} => {core_ctx.vr29}
        l__61 if {(l__61 == 30)} => {core_ctx.vr30}
        _ => {core_ctx.vr31}
        _ => {panic!("Unreachable code")}
    }
}

/// wV
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L268-311.
pub fn wV(core_ctx: &mut Core, vregno::Vregno(r): vregno, v: BitVector) {
    match r {
        l__0 if {(l__0 == 0)} => {core_ctx.vr0 = v}
        l__1 if {(l__1 == 1)} => {core_ctx.vr1 = v}
        l__2 if {(l__2 == 2)} => {core_ctx.vr2 = v}
        l__3 if {(l__3 == 3)} => {core_ctx.vr3 = v}
        l__4 if {(l__4 == 4)} => {core_ctx.vr4 = v}
        l__5 if {(l__5 == 5)} => {core_ctx.vr5 = v}
        l__6 if {(l__6 == 6)} => {core_ctx.vr6 = v}
        l__7 if {(l__7 == 7)} => {core_ctx.vr7 = v}
        l__8 if {(l__8 == 8)} => {core_ctx.vr8 = v}
        l__9 if {(l__9 == 9)} => {core_ctx.vr9 = v}
        l__10 if {(l__10 == 10)} => {core_ctx.vr10 = v}
        l__11 if {(l__11 == 11)} => {core_ctx.vr11 = v}
        l__12 if {(l__12 == 12)} => {core_ctx.vr12 = v}
        l__13 if {(l__13 == 13)} => {core_ctx.vr13 = v}
        l__14 if {(l__14 == 14)} => {core_ctx.vr14 = v}
        l__15 if {(l__15 == 15)} => {core_ctx.vr15 = v}
        l__16 if {(l__16 == 16)} => {core_ctx.vr16 = v}
        l__17 if {(l__17 == 17)} => {core_ctx.vr17 = v}
        l__18 if {(l__18 == 18)} => {core_ctx.vr18 = v}
        l__19 if {(l__19 == 19)} => {core_ctx.vr19 = v}
        l__20 if {(l__20 == 20)} => {core_ctx.vr20 = v}
        l__21 if {(l__21 == 21)} => {core_ctx.vr21 = v}
        l__22 if {(l__22 == 22)} => {core_ctx.vr22 = v}
        l__23 if {(l__23 == 23)} => {core_ctx.vr23 = v}
        l__24 if {(l__24 == 24)} => {core_ctx.vr24 = v}
        l__25 if {(l__25 == 25)} => {core_ctx.vr25 = v}
        l__26 if {(l__26 == 26)} => {core_ctx.vr26 = v}
        l__27 if {(l__27 == 27)} => {core_ctx.vr27 = v}
        l__28 if {(l__28 == 28)} => {core_ctx.vr28 = v}
        l__29 if {(l__29 == 29)} => {core_ctx.vr29 = v}
        l__30 if {(l__30 == 30)} => {core_ctx.vr30 = v}
        _ => {core_ctx.vr31 = v}
        _ => {panic!("Unreachable code")}
    };
    assert!(((0 < 512) && (512 <= 65536)), "tests/vec/arch.sail:307.43-307.44")
}

/// rV_bits
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L313.
pub fn rV_bits(core_ctx: &mut Core, i: vregidx) -> BitVector {
    rV(core_ctx, vregidx_to_vregno(i))
}

/// wV_bits
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L315-317.
pub fn wV_bits(core_ctx: &mut Core, i: vregidx, data: BitVector) {
    wV(core_ctx, vregidx_to_vregno(i), data)
}

/// read_single_vreg
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L323-334.
pub fn read_single_vreg<const N: usize>(core_ctx: &mut Core, num_elem: i128, SEW: i128, vrid: vregidx) -> [BitVector; N] {
    let bv: vregtype = rV_bits(core_ctx, vrid);
    let mut result: [BitVector; N] = [zeros(__id(SEW)); N];
    {
        assert!(((8 <= SEW) && (SEW <= 64)), "tests/vec/arch.sail:327.29-327.30");
        for i in 0..=(num_elem - 1) {
            let start_index = (i * SEW);
            result[(i as usize)] = slice(bv, start_index, SEW)
        };
        result
    }
}

/// write_single_vreg
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L338-348.
pub fn write_single_vreg<const N: usize>(core_ctx: &mut Core, num_elem: i128, SEW: i128, vrid: vregidx, v: [BitVector; N]) {
    let mut r: vregtype = zeros(65536);
    {
        assert!(((8 <= SEW) && (SEW <= 64)), "tests/vec/arch.sail:341.29-341.30");
        todo!("E_for_dec");
        wV_bits(core_ctx, vrid, r)
    }
}

/// read_vreg
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L352-386.
pub fn read_vreg<const N: usize>(core_ctx: &mut Core, num_elem: i128, SEW: i128, LMUL_pow: i128, vrid: vregidx) -> [BitVector; N] {
    let vrid_val = vregidx_bits(vrid).unsigned();
    let mut result: [BitVector; N] = [zeros(__id(SEW)); N];
    {
        let LMUL_pow_reg = if {(LMUL_pow < 0)} {
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
                result = read_single_vreg(core_ctx, result.len(), SEW, vrid)
            } else {
                let num_elem_single: i128 = quot_round_zero(512, SEW);
                assert!((__id(num_elem_single) >= 0), "tests/vec/arch.sail:369.34-369.35");
                for i_lmul in 0..=(i128::pow(2, (LMUL_pow_reg as u32)) - 1) {
                    let r_start_i: i128 = (i_lmul * __id(num_elem_single));
                    let r_end_i: i128 = ((r_start_i + __id(num_elem_single)) - 1);
                    let vrid_lmul: vregidx = vregidx_offset(vrid, to_bits(5, i_lmul));
                    let single_result: [BitVector; NUM_ELEM_SINGLE] = read_single_vreg(core_ctx, __id(num_elem_single), SEW, vrid_lmul);
                    for r_i in r_start_i..=r_end_i {
                        let s_i: i128 = (r_i - r_start_i);
                        assert!(((0 <= r_i) && (r_i < num_elem)), "tests/vec/arch.sail:377.42-377.43");
                        assert!(((0 <= s_i) && (s_i < __id(num_elem_single))), "tests/vec/arch.sail:378.50-378.51");
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
/// Generated from the Sail sources at `tests/vec/arch.sail` L390-408.
pub fn write_vreg<const N: usize>(core_ctx: &mut Core, num_elem: i128, SEW: i128, LMUL_pow: i128, vrid: vregidx, vec: [BitVector; N]) {
    let LMUL_pow_reg = if {(LMUL_pow < 0)} {
        0
    } else {
        LMUL_pow
    };
    let num_elem_single: i128 = quot_round_zero(512, SEW);
    assert!((__id(num_elem_single) >= 0), "tests/vec/arch.sail:394.30-394.31");
    for i_lmul in 0..=(i128::pow(2, (LMUL_pow_reg as u32)) - 1) {
        let mut single_vec: [BitVector; NUM_ELEM_SINGLE] = [zeros(__id(SEW)); NUM_ELEM_SINGLE];
        {
            let vrid_lmul: vregidx = vregidx_offset(vrid, to_bits(5, i_lmul));
            let r_start_i: i128 = (i_lmul * __id(num_elem_single));
            let r_end_i: i128 = ((r_start_i + __id(num_elem_single)) - 1);
            for r_i in r_start_i..=r_end_i {
                let s_i: i128 = (r_i - r_start_i);
                assert!(((0 <= r_i) && (r_i < num_elem)), "tests/vec/arch.sail:402.38-402.39");
                assert!(((0 <= s_i) && (s_i < __id(num_elem_single))), "tests/vec/arch.sail:403.46-403.47");
                single_vec[(s_i as usize)] = vec[(r_i as usize)]
            };
            write_single_vreg(core_ctx, __id(num_elem_single), SEW, vrid_lmul, single_vec)
        }
    }
}

/// get_num_elem
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L412-419.
pub fn get_num_elem(LMUL_pow: i128, SEW: i128) -> i128 {
    let LMUL_pow_reg = if {(LMUL_pow < 0)} {
        0
    } else {
        LMUL_pow
    };
    let num_elem = quot_round_zero((i128::pow(2, (LMUL_pow_reg as u32)) * 512), SEW);
    assert!((num_elem > 0), "tests/vec/arch.sail:417.21-417.22");
    num_elem
}

/// read_vmask
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L423-437.
pub fn read_vmask(core_ctx: &mut Core, num_elem: i128, vm: BitVector, vrid: vregidx) -> BitVector {
    assert!((num_elem <= 65536), "tests/vec/arch.sail:424.36-424.37");
    let vreg_val: vregtype = rV_bits(core_ctx, vrid);
    let mut result: BitVector = ones(__id(num_elem));
    {
        if {(vm == BitVector::new(1, 0b1))} {
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

/// ExecutionResult
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L441-459.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ExecutionResult {
    Retire_Success(()),
    Illegal_Instruction(())
}

/// ast
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L463.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    VVTYPE((vvfunct6, BitVector, vregidx, vregidx, vregidx))
}

/// valid_vtype
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L475-477.
pub fn valid_vtype(core_ctx: &mut Core, unit_arg: ()) -> bool {
    ({
        let var_1 = core_ctx.vtype;
        _get_Vtype_vill(var_1)
    } == BitVector::new(1, 0b0))
}

/// valid_rd_mask
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L486-488.
pub fn valid_rd_mask(rd: vregidx, vm: BitVector) -> bool {
    ((vm != BitVector::new(1, 0b0)) || (rd != zvreg))
}

/// illegal_normal
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L492-494.
pub fn illegal_normal(core_ctx: &mut Core, vd: vregidx, vm: BitVector) -> bool {
    (!(valid_vtype(core_ctx, ())) || !(valid_rd_mask(vd, vm)))
}

/// get_start_element
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L498-511.
pub fn get_start_element(core_ctx: &mut Core, unit_arg: ()) -> result<nat, ()> {
    let start_element = core_ctx.vstart.unsigned();
    let SEW_pow = get_sew_pow(core_ctx, ());
    if {(start_element > (i128::pow(2, ((12 - SEW_pow) as u32)) - 1))} {
        result::Err(())
    } else {
        result::Ok(start_element)
    }
}

/// get_end_element
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L515.
pub fn get_end_element(core_ctx: &mut Core, unit_arg: ()) -> i128 {
    (core_ctx.vl.unsigned() - 1)
}

/// init_masked_result
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L526-574.
pub fn init_masked_result<const N: usize>(core_ctx: &mut Core, num_elem: i128, SEW: i128, LMUL_pow: i128, vd_val: [BitVector; N], vm_val: BitVector) -> result<([BitVector; N], BitVector), ()> {
    let start_element: nat = match get_start_element(core_ctx, ()) {
        result::Ok(v) => {v}
        result::Err(()) => {return result::Err(());}
        _ => {panic!("Unreachable code")}
    };
    let end_element = get_end_element(core_ctx, ());
    let tail_ag: agtype = get_vtype_vta(core_ctx, ());
    let mask_ag: agtype = get_vtype_vma(core_ctx, ());
    let mut mask: BitVector = undefined_bitvector(bitvector_length(vm_val));
    {
        let mut result: [BitVector; N] = undefined_vector(bitvector_length(vm_val), undefined_bitvector(__id(SEW)));
        {
            let real_num_elem = if {(LMUL_pow >= 0)} {
                num_elem
            } else {
                (num_elem / i128::pow(2, ((0 - LMUL_pow) as u32)))
            };
            assert!((num_elem >= real_num_elem), "tests/vec/arch.sail:539.34-539.35");
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

/// set_vstart
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L578-584.
pub fn set_vstart(core_ctx: &mut Core, value: BitVector) {
    core_ctx.vstart = value.subrange::<0, 9, 9>().zero_extend(16)
}

/// execute
///
/// Generated from the Sail sources at `tests/vec/arch.sail` L590-628.
pub fn execute(core_ctx: &mut Core, funct6: ast) -> ExecutionResult {
    let SEW_pow = get_sew_pow(core_ctx, ());
    let SEW = get_sew(core_ctx, ());
    let LMUL_pow = get_lmul_pow(core_ctx, ());
    let num_elem = get_num_elem(LMUL_pow, SEW);
    if {illegal_normal(core_ctx, vd, vm)} {
        return ExecutionResult::Illegal_Instruction(());
    } else {
        ()
    };
    let n = num_elem;
    let m = SEW;
    let vm_val: BitVector = read_vmask(core_ctx, num_elem, vm, zvreg);
    let vs1_val: [BitVector; N] = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vs1);
    let vs2_val: [BitVector; N] = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vs2);
    let vd_val: [BitVector; N] = read_vreg(core_ctx, num_elem, SEW, LMUL_pow, vd);
    let (initial_result, mask): ([BitVector; N], BitVector) = match init_masked_result(core_ctx, num_elem, SEW, LMUL_pow, vd_val, vm_val) {
        result::Ok(v) => {v}
        result::Err(()) => {return ExecutionResult::Illegal_Instruction(());}
        _ => {panic!("Unreachable code")}
    };
    let mut result = initial_result;
    {
        for i in 0..=(num_elem - 1) {
            if {(bitvector_access(mask, i) == true)} {
                result[(i as usize)] = match funct6 {
                    vvfunct6::VV_VADD => {vs2_val[(i as usize)].wrapped_add(vs1_val[(i as usize)])}
                    vvfunct6::VV_VSUB => {sub_vec(vs2_val[(i as usize)], vs1_val[(i as usize)])}
                    vvfunct6::VV_VAND => {(vs2_val[(i as usize)] & vs1_val[(i as usize)])}
                    vvfunct6::VV_VOR => {(vs2_val[(i as usize)] | vs1_val[(i as usize)])}
                    vvfunct6::VV_VXOR => {(vs2_val[(i as usize)] ^ vs1_val[(i as usize)])}
                    _ => {panic!("Unreachable code")}
                }
            } else {
                ()
            }
        };
        write_vreg(core_ctx, num_elem, SEW, LMUL_pow, vd, result);
        set_vstart(core_ctx, zeros(16));
        ExecutionResult::Retire_Success(())
    }
}
