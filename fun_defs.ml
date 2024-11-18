open Libsail
open Ast
open Ast_util
open Ast_defs

open Rust_gen


(* ————————————————————————————————— Types —————————————————————————————————— *)

module SSet = Set.Make(String)
module SMap = Map.Make(String)

type defmap = rs_fn_type SMap.t
type unionmap = rs_type SMap.t
type defs = {
    funs : defmap;
    unions: unionmap;
}

(* ——————————————————————————————— Type Utils ——————————————————————————————— *)

let map_union (a: 'a SMap.t) (b: 'a SMap.t) : 'a SMap.t =
    let select key elt_a elt_b = Some(elt_a) in
    SMap.union select a b

let defs_empty = {
    funs = SMap.empty;
    unions = SMap.empty;
}

let defs_merge (a: defs) (b: defs) : defs =
    {
        funs = map_union a.funs b.funs;
        unions = map_union a.unions b.unions;
    }

let defs_add_union (defs: defs) (union: unionmap) : defs =
    let unions = map_union union defs.unions in
    {
        funs = defs.funs;
        unions = unions;
    }

let defs_from_union (union: unionmap) : defs = 
    {
        funs = SMap.empty;
        unions = union;
    }

let defs_from_funs (funs: defmap) : defs =
    {
        funs = funs;
        unions = SMap.empty;
    }

(* ———————————————————————— Sail Types to Rust Types ———————————————————————— *)

let capitalize_after_removal s =
    if String.length s > 1 then
      String.uppercase_ascii (String.sub s 1 (String.length s - 1))
    else
      ""
  
let get_first_two_elements lst =
    assert ((List.length lst) = 2);
    match lst with
    | first :: second :: _ -> (first, second)
    | _ -> failwith "List does not have enough elements"
  
let rec extract_type (Typ_aux (typ, _)): rs_type =
    match typ with
        | Typ_id id when string_of_id id = "unit" -> RsTypUnit
        | Typ_id id -> RsTypId (string_of_id id)
        | Typ_var (Kid_aux (Var x, _)) -> RsTypGeneric x
        | Typ_tuple types -> RsTypTuple (List.map extract_type types)
        | Typ_fn _ -> RsTypId "TodoFnType"
        | Typ_app (id, params) -> if (string_of_id id) = "vector" then 
                let (size, typ) = get_first_two_elements (List.map extract_type_arg params) in
                RsTypArray (typ,size)
            else 
                RsTypGenericParam ((string_of_id id), (List.map extract_type_arg params))
        | Typ_internal_unknown -> RsTypId "TodoUnknownType"
        | Typ_bidir (_, _) -> RsTypId "TodoBidirType"
        | Typ_exist (_, _, typ) -> extract_type typ
and extract_type_arg (A_aux (typ, _)): rs_type_param =
    match typ with
        | A_nexp exp -> extract_type_nexp exp
        | A_typ typ -> RsTypParamTyp (extract_type typ)
        | A_bool b -> RsTypParamTyp (RsTypId "TodoBoolType")
and extract_type_nexp (Nexp_aux (nexp, _)): rs_type_param =
    match nexp with
        | Nexp_constant n -> RsTypParamNum (Nat_big_num.to_int n)
        | Nexp_app (Id_aux (_, _), _) -> RsTypParamTyp (RsTypId "TodoNexpTypeApp")
        | Nexp_id id -> RsTypParamTyp (RsTypId (string_of_id id))
        | Nexp_var var -> RsTypParamTyp (RsTypId (capitalize_after_removal (string_of_kid var))) 
        | Nexp_times (_, _) -> RsTypParamTyp (RsTypId "TodoNexpTypeTimes")
        | Nexp_sum (_, _) -> RsTypParamTyp (RsTypId "TodoNexpTypeSum")
        | Nexp_minus (_, _) -> RsTypParamTyp (RsTypId "TodoNexpTypeMinus")
        | Nexp_exp _ -> RsTypParamTyp (RsTypId "TodoNexpTypeExp")
        | Nexp_neg _ -> RsTypParamTyp (RsTypId "TodoNexpTypeNeg")

(* ———————————————————————————— Value Definition ———————————————————————————— *)

let extract_types (TypSchm_aux (typeschm, _)) : rs_fn_type =
    (* We ignore the type quantifier for now, there is no `forall` on most types of interest *)
    let TypSchm_ts (type_quant, typ) = typeschm in
    let Typ_aux (typ, _) = typ in
    match typ with 
        | Typ_fn (args, ret) -> ((List.map extract_type args), extract_type ret)
        | _ -> ([RsTypTodo "todo_extract_types"], RsTypTodo "todo_extract_types")

let val_fun_def (val_spec: val_spec_aux) : defmap =
    let map = SMap.empty in
    let VS_val_spec (typeschm, id, extern) = val_spec in
    (* print_string (string_of_id id); *)
    (* print_string " - "; *)
    (* print_endline (string_of_typschm typeschm); *)
    SMap.add (string_of_id id) (extract_types typeschm) map

(* ————————————————————————————————— Union —————————————————————————————————— *)

let type_union_def (Tu_aux (union, _)) : unionmap = 
    let Tu_ty_id (typ, id) = union in
    SMap.add (string_of_id id) (extract_type typ) SMap.empty 

let rec type_union_defs (members: type_union list) : unionmap =
    match members with
        | head :: tail -> map_union (type_union_def head) (type_union_defs tail)
        | [] -> SMap.empty

let type_def_fun_def (TD_aux (typ, _)) : unionmap =
    match typ with
        | TD_abbrev (id, typquant, typ_arg) -> print_string (string_of_id id); print_endline " Abbrev"; SMap.empty
        | TD_record (id, typquant, items, _) -> print_string (string_of_id id); print_endline " Record"; SMap.empty
        | TD_variant (id, typquant, members, _) ->
            print_string (string_of_id id); print_endline " Variant";
            type_union_defs members
        | TD_enum (id, member, _) -> print_string (string_of_id id); print_endline " Enum"; SMap.empty
        | TD_bitfield _ -> print_endline "Bitfield"; SMap.empty

(* ——————————————————————— Iterating over definitions ——————————————————————— *)

let node_defs (DEF_aux (def, annot)) : defs =
    match def with
        | DEF_val (VS_aux (val_spec, annot)) -> defs_from_funs (val_fun_def val_spec)
        | DEF_register (DEC_aux (dec_spec, annot)) -> defs_empty
        | DEF_scattered (SD_aux (scattered, annot)) -> defs_empty
        | DEF_fundef (FD_aux (fundef, annot)) -> defs_empty
        | DEF_impl funcl -> defs_empty
        | DEF_type typ -> defs_from_union (type_def_fun_def typ)
        | _ -> defs_empty

let rec process_defs(defs: 'a def list) : defs =
    match defs with
        | h :: t -> defs_merge (node_defs h) (process_defs t)
        | [] -> defs_empty

let get_defs (ast: 'a ast) : defs =
    process_defs ast.defs

(* ————————————————————————————— Display Utils —————————————————————————————— *)

let print_all_defs (defs: defs) =
    let print_one_fun key (args, ret) =
        Printf.printf "  %s(%s) -> %s\n"
            key
            (String.concat ", " (List.map string_of_rs_type args))
            (string_of_rs_type ret)
    in
    let print_one_union key typ =
        Printf.printf "  %s %s\n"
            key
            (string_of_rs_type typ)
    in
    print_endline "Fun defs:";
    SMap.iter print_one_fun defs.funs;
    print_endline "Union defs:";
    SMap.iter print_one_union defs.unions;

