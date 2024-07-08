open Libsail
open Ast
open Ast_defs

open Rust_gen

module SMap : (Map.S with type key = string)

type defmap = rs_fn_type SMap.t
type unionmap = rs_type SMap.t
type defs = {
    funs : defmap;
    unions: unionmap;
}

val get_defs: 'a ast -> defs
val print_all_defs: defs -> unit
