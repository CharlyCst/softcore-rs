open Libsail
open Ast
open Ast_defs
open Rust_gen
open Call_set

open Rust_transform


module StringSet : sig
    type t
    val of_list : string list -> t
  end
  
val sail_context_binder_generator : StringSet.t -> expr_type_transform

val virt_context_transform: func_transform
val unit_remove_transform: func_transform

val sail_context_arg_inserter: expr_type_transform
val remove_last_unit_func_arg: expr_type_transform
