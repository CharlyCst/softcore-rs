(** VirtContext Transformations **)
(** This module adds a virtual context to each function call and binds free variables to this context. **)

open Rust_gen
open Rust_transform

module StringSet = Set.Make(String)

(* ———————————————————————— Enumeration binder ————————————————————————— *)

let rec enum_prefix_inserter (key : string) (lst : (string * string) list) : string =
  match lst with
  | [] -> key
  | (k, v) :: rest ->
      if k = key then v ^ "::" ^ k
      else enum_prefix_inserter key rest

let enum_binder_exp (enum_list: (string * string) list) (exp: rs_exp) : rs_exp = 
  match exp with
    | RsId id -> RsId (enum_prefix_inserter id enum_list)
    | _ -> exp

let enum_binder_lexp (enum_list: (string * string) list) (lexp: rs_lexp) : rs_lexp = 
  match lexp with
    | RsLexpId id -> RsLexpId (enum_prefix_inserter id enum_list)
    | _ -> lexp

let enum_binder_pexp (enum_list: (string * string) list) (pexp: rs_pexp) : rs_pexp = 
  match pexp with
    | RsPexp (RsPatId id, second) -> RsPexp  ((RsPatId (enum_prefix_inserter id enum_list)), second)
    | _ -> pexp

let enum_binder_type (typ: rs_type) : rs_type = typ

let enum_binder_generator (enum_list: (string * string) list) : expr_type_transform = {
    exp = enum_binder_exp enum_list;
    pexp = enum_binder_pexp enum_list;
    lexp = enum_binder_lexp enum_list;
    typ = enum_binder_type;
}


(* ———————————————————————— VirtContext transformer ————————————————————————— *)

let sail_context_inserter (func: rs_fn): rs_fn = { 
  func with 
    args = "sail_ctx" :: func.args;
    signature = (RsTypId "&mut SailVirtCtx" :: (fst func.signature), snd func.signature)
}


let virt_context_transform = {
  func = sail_context_inserter;
}

(* ———————————————————————— VirtContext binder ————————————————————————— *)


let sail_context_binder_exp (register_list: StringSet.t) (exp: rs_exp) : rs_exp = 
  match exp with
    | RsId value -> if (StringSet.mem value register_list) then (RsId ("sail_ctx." ^ value)) else RsId value
    | _ -> exp

let sail_context_binder_lexp (register_list: StringSet.t) (lexp: rs_lexp) : rs_lexp = 
  match lexp with
    | RsLexpId value -> if (StringSet.mem value register_list) then (RsLexpId ("sail_ctx." ^ value)) else RsLexpId value
    | _ -> lexp

let sail_context_binder_pexp (register_list: StringSet.t) (pexp: rs_pexp) : rs_pexp = 
  match pexp with
    | RsPexp (RsPatId value, exp) -> if (StringSet.mem value register_list) then RsPexp(RsPatId ("sail_ctx." ^ value), exp) else RsPexp (RsPatId value, exp)
    | _ -> pexp


let sail_context_binder_type (typ: rs_type) : rs_type = typ

let sail_context_binder_generator (register_list: StringSet.t): expr_type_transform = {
    exp = sail_context_binder_exp register_list;
    lexp = sail_context_binder_lexp register_list;
    pexp = sail_context_binder_pexp register_list;
    typ = sail_context_binder_type;
}

(* ———————————————————————— VirtContext argument inserter  ————————————————————————— *)


let sail_context_arg_inserter_exp (exp: rs_exp) : rs_exp = 
  match exp with 
    | RsApp (app, args) -> let args = RsId "sail_ctx" :: args in RsApp(app, args)
    | _ -> exp

let sail_context_arg_inserter_lexp (lexp: rs_lexp) : rs_lexp = lexp

let sail_context_arg_inserter_pexp (pexp: rs_pexp) : rs_pexp = pexp

let sail_context_arg_inserter_type (typ: rs_type) : rs_type = typ

let sail_context_arg_inserter: expr_type_transform = {
    exp = sail_context_arg_inserter_exp;
    lexp = sail_context_arg_inserter_lexp;
    pexp = sail_context_arg_inserter_pexp;
    typ = sail_context_arg_inserter_type;
}

(* ———————————————————————— FuncArgument remove last unit  ————————————————————————— *)


let remove_last_unit_func_arg_exp (exp: rs_exp) : rs_exp = 
  match exp with 
    | RsApp (app, args) -> let args_no_unit = List.filter (
      function 
        | RsLit _ -> false 
        | _ -> true 
    ) args in RsApp(app, args_no_unit)
    | _ -> exp

let remove_last_unit_func_arg_lexp (lexp: rs_lexp) : rs_lexp = lexp

let remove_last_unit_func_arg_pexp (pexp: rs_pexp) : rs_pexp = pexp

let remove_last_unit_func_arg_type (typ: rs_type) : rs_type = typ

let remove_last_unit_func_arg: expr_type_transform = {
    exp = remove_last_unit_func_arg_exp;
    lexp = remove_last_unit_func_arg_lexp;
    pexp = remove_last_unit_func_arg_pexp;
    typ = remove_last_unit_func_arg_type;
}

(* ———————————————————————— Remove last unit transformer ————————————————————————— *)


let unit_filter (typ: rs_type) = typ != RsTypUnit

let filter_units (names: string list) (types: rs_type list) =
  let filtered_indices = 
    List.mapi (fun i x -> if unit_filter x then Some i else None) types
    |> List.filter_map Fun.id 
  in

  let filtered_names = List.filteri (fun i _ -> List.mem i filtered_indices) names in
  let filtered_types = List.filter unit_filter types in

  (filtered_names, filtered_types)

let unit_remover (func: rs_fn): rs_fn = 
  let (arg_names, arg_types) = filter_units func.args (fst func.signature) in  
  {
    func with 
    args = arg_names;
    signature = (arg_types, snd func.signature)
  }

let unit_remove_transform = {
  func = unit_remover;
}