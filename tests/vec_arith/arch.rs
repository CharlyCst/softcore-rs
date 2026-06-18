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
    pub vr1: vregtype,
    pub vr2: vregtype,
    pub vr3: vregtype,
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

/// zeros
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L25.
pub const fn zeros(n: i128) -> BitDynamic {
    sail_zeros(n)
}

/// V_LEN_ELEM
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L35.
pub const V_LEN_ELEM: i128 = 65536;

/// V_LEN_BITS
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L36.
pub const V_LEN_BITS: i128 = 16;

pub type vreglenbits = BitDynamic;

pub type vregtype = [vreglenbits; (V_LEN_ELEM as usize)];

/// vregidx
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L45.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregidx {
    Vregidx(BitDynamic)
}

/// vregno
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L46.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vregno {
    Vregno(i128)
}

/// vregidx_to_vregno
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L47.
pub fn vregidx_to_vregno(vregidx::Vregidx(b): vregidx) -> vregno {
    vregno::Vregno(b.unsigned())
}

/// rV
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L50-58.
pub fn rV(core_ctx: &mut Core, vregno::Vregno(r): vregno) -> BoundedVec::<BitDynamic, 32> {
    match r {
        l__7 if {(l__7 == 0)} => {vec![zeros(16); (({
                    65536
                } as usize) as usize)].into()}
        l__8 if {(l__8 == 1)} => {core_ctx.vr1}
        l__9 if {(l__9 == 2)} => {core_ctx.vr2}
        _ => {core_ctx.vr3}
        _ => {panic!("Unreachable code")}
    }
}

/// wV
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L60-67.
pub fn wV(core_ctx: &mut Core, vregno::Vregno(r): vregno, v: BoundedVec::<BitDynamic, 32>) {
    match r {
        l__4 if {(l__4 == 0)} => {()}
        l__5 if {(l__5 == 1)} => {core_ctx.vr1 = v}
        l__6 if {(l__6 == 2)} => {core_ctx.vr2 = v}
        _ => {core_ctx.vr3 = v}
        _ => {panic!("Unreachable code")}
    }
}

/// vvop
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L69-75.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum vvop {
    VV_VADD,
    VV_VSUB,
    VV_VAND,
    VV_VOR,
    VV_VXOR
}

/// ast
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L79.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    VVTYPE((vvop, vregidx, vregidx, vregidx))
}

/// execute
///
/// Generated from the Sail sources at `tests/vec_arith/arch.sail` L83-99.
pub fn execute(core_ctx: &mut Core, op: ast) {
    let vs1_val: BoundedVec::<BitDynamic, 32> = rV(core_ctx, vregidx_to_vregno(vs1));
    let vs2_val: BoundedVec::<BitDynamic, 32> = rV(core_ctx, vregidx_to_vregno(vs2));
    let mut vd_val: vregtype = vec![zeros(16); (({
                65536
            } as usize) as usize)].into();
    {
        {
            assert!(((0 <= 65536) && (65535 < 65536)), "tests/vec_arith/arch.sail:88.60-88.61");
            for i in 0..=(len_elem - 1) {
                vd_val[(i as usize)] = match op {
                    vvop::VV_VADD => {vs2_val[(i as usize)].wrapped_add(vs1_val[(i as usize)])}
                    vvop::VV_VSUB => {sub_vec(vs2_val[(i as usize)], vs1_val[(i as usize)])}
                    vvop::VV_VAND => {(vs2_val[(i as usize)] & vs1_val[(i as usize)])}
                    vvop::VV_VOR => {(vs2_val[(i as usize)] | vs1_val[(i as usize)])}
                    vvop::VV_VXOR => {(vs2_val[(i as usize)] ^ vs1_val[(i as usize)])}
                    _ => {panic!("Unreachable code")}
                }
            };
            wV(core_ctx, vregidx_to_vregno(vd), vd_val)
        }
    }
}
