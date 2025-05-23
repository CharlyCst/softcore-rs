#![allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types, bindings_with_variant_name)]

use sail_prelude::*;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailVirtCtx {
    pub PC: xlenbits,
    pub config: SailConfig,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfig {
    pub extensions: SailConfigExtensions,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfigExtensions {
    pub S: SailConfigS,
    pub U: SailConfigU,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfigS {
    pub supported: bool,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct SailConfigU {
    pub supported: bool,
}

pub const xlen: usize = 64;

pub const xlen_bytes: usize = 8;

pub type xlenbits = BitVector<xlen>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum extension {
    Ext_U,
    Ext_S
}

pub fn hartSupports(sail_ctx: &mut SailVirtCtx, merge_hashtag_var: extension) -> bool {
    match merge_hashtag_var {
        extension::Ext_U => {sail_ctx.config.extensions.U.supported}
        extension::Ext_S => {sail_ctx.config.extensions.S.supported}
        _ => {panic!("Unreachable code")}
    }
}

pub type regbits = BitVector<5>;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ast {
    TEST(())
}

pub fn execute(sail_ctx: &mut SailVirtCtx, TodoArgsApp: ast) {
    if {hartSupports(sail_ctx, extension::Ext_U)} {
        ()
    } else {
        ()
    }
}