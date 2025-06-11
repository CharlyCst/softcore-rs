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
    pub config: Config,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Config {
    pub extensions: ConfigExtensions,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigExtensions {
    pub S: ConfigS,
    pub U: ConfigU,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigS {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ConfigU {
    pub supported: bool,
}

pub const xlen: i128 = 64;

pub const xlen_bytes: i128 = 8;

pub type xlenbits = BitVector<xlen>;

/// extension
/// 
/// Generated from the Sail sources at `tests/config/arch.sail` L12.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum extension {
    Ext_U,
    Ext_S
}

/// hartSupports
/// 
/// Generated from the Sail sources at `tests/config/arch.sail` L18.
pub fn hartSupports(core_ctx: &mut Core, merge_hashtag_var: extension) -> bool {
    match merge_hashtag_var {
        extension::Ext_U => {core_ctx.config.extensions.U.supported}
        extension::Ext_S => {core_ctx.config.extensions.S.supported}
        _ => {panic!("Unreachable code")}
    }
}

pub type regbits = BitVector<5>;

/// ast
/// 
/// Generated from the Sail sources at `tests/config/arch.sail` L32.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

/// execute_TEST
/// 
/// Generated from the Sail sources at `tests/config/arch.sail` L37-41.
pub fn execute_TEST(core_ctx: &mut Core) {
    if {hartSupports(core_ctx, extension::Ext_U)} {
        ()
    } else {
        ()
    }
}