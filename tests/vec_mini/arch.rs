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
    pub rsize: atom::<N>,
    pub rcount: atom::<N>,
    pub rv: BitStatic::<256>,
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {

}

/// Initialize all registers.
///
/// This function should be called before using a fresh core, otherwise the core might not be in a valid state.
pub fn _reset_all_registers() {
    core_ctx.rsize = 8;
    core_ctx.rcount = 1
}

/// Initialize the rsize register.
///
/// Generated from the Sail sources at `tests/vec_mini/arch.sail` L7.
pub const fn _reset_rsize() -> i128 {
    8
}

/// Initialize the rcount register.
///
/// Generated from the Sail sources at `tests/vec_mini/arch.sail` L8.
pub const fn _reset_rcount() -> i128 {
    1
}

/// __id
///
/// Generated from the Sail sources at `sail/lib/flow.sail` L107.
pub fn __id(x: i128) -> i128 {
    x
}

/// zeros
///
/// Generated from the Sail sources at `tests/vec_mini/arch.sail` L5.
pub const fn zeros(n: i128) -> BitDynamic {
    sail_zeros(n)
}

/// read
///
/// Generated from the Sail sources at `tests/vec_mini/arch.sail` L13-22.
pub fn read(core_ctx: &mut Core, elem_count: i128, elem_size: i128) -> Vec::<BitDynamic> {
    let mut result: Vec::<BitDynamic> = vec![zeros(__id(elem_size)); (__id(elem_count) as usize)];
    {
        for i in 0..=(elem_count - 1) {
            let start_index = (i * elem_size);
            result[(i as usize)] = slice(core_ctx.rv, start_index, elem_size)
        };
        result
    }
}

/// execute
///
/// Generated from the Sail sources at `tests/vec_mini/arch.sail` L25-33.
pub fn execute(core_ctx: &mut Core, unit_arg: ()) {
    let elem_size = core_ctx.rsize;
    let elem_count = core_ctx.rcount;
    let n = elem_count;
    let m = elem_size;
    let vs: Vec::<BitDynamic> = read(core_ctx, elem_count, elem_size);
    ()
}
