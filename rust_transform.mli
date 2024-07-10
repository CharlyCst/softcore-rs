open Libsail
open Ast
open Ast_defs
open Rust_gen
open Call_set

type custom_transform = {
    exp : rs_exp -> rs_exp;
    typ : rs_type -> rs_type;
}

val kani_transform: custom_transform

val rust_transform: custom_transform -> rs_program -> rs_program
