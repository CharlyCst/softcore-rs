open Libsail
open Ast
open Ast_util
open Ast_defs

open Rust_gen

module SSet = Set.Make(String)
module SMap = Map.Make(String)

type defmap = rs_fn_type SMap.t

let defmap_union (a: defmap) (b: defmap) : defmap =
    let select key elt_a elt_b = Some(elt_a) in
    SMap.union select a b

let extract_type (Typ_aux (typ, _)): rs_type =
    match typ with
        | _ -> RsTypTodo

let extract_types (TypSchm_aux (typeschm, _)) : rs_fn_type =
    (* We ignore the type quantifier for now, there is no `forall` on most types of interest *)
    let TypSchm_ts (type_quant, typ) = typeschm in
    let Typ_aux (typ, _) = typ in
    match typ with 
        | Typ_fn (args, ret) -> ((List.map extract_type args), extract_type ret)
        | _ -> ([RsTypTodo], RsTypTodo)

let val_fun_def (val_spec: val_spec_aux) : defmap =
    let map = SMap.empty in
    let VS_val_spec (typeschm, id, extern) = val_spec in
    print_string (string_of_id id);
    print_string " - ";
    print_endline (string_of_typschm typeschm);
    SMap.add (string_of_id id) (extract_types typeschm) map

let node_fun_def (DEF_aux (def, annot)) : defmap =
    match def with
        | DEF_val (VS_aux (val_spec, annot)) -> val_fun_def val_spec
        | DEF_register (DEC_aux (dec_spec, annot)) -> SMap.empty
        | DEF_scattered (SD_aux (scattered, annot)) -> SMap.empty
        | DEF_fundef (FD_aux (fundef, annot)) -> SMap.empty 
        | DEF_impl funcl -> SMap.empty
        | _ -> SMap.empty

let rec defs_fun_defs(defs: 'a def list) : defmap =
    match defs with
        | h :: t -> defmap_union (node_fun_def h) (defs_fun_defs t)
        | [] -> SMap.empty

let get_fun_defs (ast: 'a ast) : defmap =
    defs_fun_defs ast.defs

let print_all_fun_defs (defs: defmap) =
    let print_one key (args, ret) =
        Printf.printf "  %s(%s) -> %s\n"
            key
            (String.concat ", " (List.map string_of_rs_type args))
            (string_of_rs_type ret)
    in
    print_endline "Fun defs:";
    SMap.iter print_one defs
    
