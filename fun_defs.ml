open Libsail
open Ast
open Ast_util
open Ast_defs

module SSet = Set.Make(String)
module SMap = Map.Make(String)

type arg = string * string

type defmap = arg list SMap.t

let defmap_union (a: defmap) (b: defmap) : defmap =
    let select key elt_a elt_b = Some(elt_a) in
    SMap.union select a b

let node_fun_defs (DEF_aux (def, annot)) : defmap =
    match def with
        | DEF_register (DEC_aux (dec_spec, annot)) -> SMap.empty
        | DEF_scattered (SD_aux (scattered, annot)) -> SMap.empty
        | DEF_fundef (FD_aux (fundef, annot)) -> SMap.empty 
        | DEF_impl funcl -> SMap.empty
        | _ -> SMap.empty

let rec defs_fun_defs(defs: 'a def list) : defmap =
    match defs with
        | h :: t -> defmap_union (node_fun_defs h) (defs_fun_defs t)
        | [] -> SMap.empty

let get_fun_defs (ast: 'a ast) : defmap =
    let m = SMap.empty in
    m

let print_all_fun_defs (defs: defmap) =
    let print_one key args =
        Printf.printf "  %s"
            key
    in
    print_endline "Fun defs:";
    SMap.iter print_one defs
    
