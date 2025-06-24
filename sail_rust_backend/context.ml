open Rust_gen

type defmap = rs_fn_type Call_set.SMap.t
type unionmap = rs_type Call_set.SMap.t
type funmap = rs_fn Call_set.SMap.t
type nummap = Libsail.Ast_util.Big_int.num Call_set.SMap.t
type inline_fun = rs_exp Call_set.SMap.t

type defs =
  { fun_typs : defmap
  ; unions : unionmap
  ; funmap : funmap
  ; constants : SSet.t
  ; num_constants : nummap
  ; inline_fun : inline_fun
  }

type context =
  { defs : defs
  ; call_set : Call_set.SSet.t
  ; config_map : Call_set.config_map
  ; registers : SSet.t
  ; enum_entries : (string * string) list
  ; mutable uses_sail_ctx : bool
  }

let ctx_fun_is_used (fun_id : string) (ctx : context) : bool =
  Call_set.SSet.mem fun_id ctx.call_set
;;

let ctx_fun_type (fun_id : string) (ctx : context) : rs_fn_type option =
  Call_set.SMap.find_opt fun_id ctx.defs.fun_typs
;;

let ctx_fun (fun_id : string) (ctx : context) : rs_fn option =
  Call_set.SMap.find_opt fun_id ctx.defs.funmap
;;

let ctx_union_type (union_id : string) (ctx : context) : rs_type option =
  Call_set.SMap.find_opt union_id ctx.defs.unions
;;
