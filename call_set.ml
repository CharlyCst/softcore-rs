(** Compute the set of functions transitively called from a given entry point **)

open Libsail
open Ast
open Ast_util
open Ast_defs

let node_call_set node =
    let (def, annot) = match node with | DEF_aux (def, annot) -> (def, annot) in 
    match def with
        | DEF_register (DEC_aux (dec_spec, annot)) -> ()
        | DEF_scattered (SD_aux (scattered, annot)) -> ()
        | DEF_fundef (FD_aux (fundef, annot)) -> ()
        | _ -> ()

let get_call_set (ast: 'a ast) : unit =
    print_endline "Hello, world!"
