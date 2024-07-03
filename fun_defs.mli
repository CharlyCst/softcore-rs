open Libsail
open Ast
open Ast_defs

module SMap : (Map.S with type key = string)

type arg = string * string
type defmap = arg list SMap.t

val get_fun_defs: 'a ast -> defmap
val print_all_fun_defs: defmap -> unit
