open Libsail
open Ast
open Ast_defs

module SSet : (Set.S with type elt = string)

val get_call_set: 'a ast -> SSet.t -> SSet.t
