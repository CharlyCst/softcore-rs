open Libsail
open Ast
open Ast_defs

open Rust_gen

module SMap : (Map.S with type key = string)

type defmap = rs_fn_type SMap.t

val get_fun_defs: 'a ast -> defmap
val print_all_fun_defs: defmap -> unit
