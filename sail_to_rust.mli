open Libsail
open Ast
open Ast_defs
open Rust_gen

val gather_registers_list: 'a ast -> string list
val sail_to_rust : 'a ast -> Context.context -> rs_program
val process_enum_entries : 'a Libsail.Ast.def list -> (string * string) list