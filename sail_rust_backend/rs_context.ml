open Rs_ast

type defmap = rs_fn_type SMap.t
type unionmap = rs_type SMap.t
type funmap = rs_fn SMap.t
type nummap = Libsail.Ast_util.Big_int.num SMap.t
type inline_fun = rs_exp SMap.t

type defs =
  { fun_typs : defmap
  ; unions : unionmap
  ; funmap : funmap
  ; constants : SSet.t
  ; num_constants : nummap
  ; inline_fun : inline_fun
  }

type arch_t =
  { call_set : SSet.t
  ; external_func : rs_fn_type option SMap.t
  ; overwritten_func : rs_fn_type option SMap.t
  ; unsupported_obj : SSet.t
  ; unsupported_func : SSet.t
  ; unsupported_match : SSet.t
  }

type config_map = Libsail.Ast.typ SMap.t

type sail_ctx =
  { call_set : SSet.t
  ; config_map : config_map
  }

type context =
  { defs : defs
  ; call_set : SSet.t
  ; config_map : config_map
  ; registers : SSet.t
  ; enum_entries : (string * string) list
  ; arch : arch_t
  ; mutable uses_sail_ctx : bool
  }

let ctx_fun_is_used (fun_id : string) (ctx : context) : bool =
  SSet.mem fun_id ctx.call_set
;;

let ctx_fun_type (fun_id : string) (ctx : context) : rs_fn_type option =
  SMap.find_opt fun_id ctx.defs.fun_typs
;;

let ctx_fun (fun_id : string) (ctx : context) : rs_fn option =
  SMap.find_opt fun_id ctx.defs.funmap
;;

let ctx_union_type (union_id : string) (ctx : context) : rs_type option =
  SMap.find_opt union_id ctx.defs.unions
;;
