open Libsail
open Ast
open Ast_defs
open Rust_gen
open Call_set

type expr_type_transform = {
    exp : rs_exp -> rs_exp;
    lexp: rs_lexp -> rs_lexp;
    typ : rs_type -> rs_type;
}

type func_transform = {
    func : rs_fn -> rs_fn
}

val bitvec_transform: expr_type_transform

val rust_transform_expr: expr_type_transform -> rs_program -> rs_program
val rust_transform_func: func_transform -> rs_program -> rs_program

val rust_remove_type_bits: rs_program -> rs_program