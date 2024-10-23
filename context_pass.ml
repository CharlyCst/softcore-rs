(** VirtContext Transformations **)
(** This module adds a virtual context to each function call and binds free variables to this context. **)

open Rust_gen
open Rust_transform

(* ———————————————————————— VirtContext transformer ————————————————————————— *)


let sail_context_inserter (func: rs_fn): rs_fn = { 
  func with 
    args = "sail_ctx" :: func.args;
    signature = (RsTypId "&mut SailCtx" :: (fst func.signature), snd func.signature)
}


let virt_context_transform = {
  func = sail_context_inserter;
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