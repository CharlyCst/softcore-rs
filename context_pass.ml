(** VirtContext Transformations **)
(** This module adds a virtual context to each function call and binds free variables to this context. **)

open Rust_gen
open Rust_transform

(* ———————————————————————— VirtContext transformer ————————————————————————— *)


let id_function_modifier (func: rs_fn): rs_fn = { 
  func with 
    args = "sail_ctx" :: func.args;
    signature = (RsTypId "&mut SailCtx" :: (fst func.signature), snd func.signature)
}

let virt_context_transform = {
  func = id_function_modifier;
}