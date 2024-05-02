open Libsail
open Ast
open Ast_defs
open Rust_gen

val sail_to_rust : 'a ast -> rs_program
