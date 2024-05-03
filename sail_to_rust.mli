open Libsail
open Ast
open Ast_defs
open Rust_gen
open Call_set

val sail_to_rust : 'a ast -> Call_set.SSet.t -> rs_program
