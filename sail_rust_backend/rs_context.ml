open Rs_ast

(** Map from type constructors to their types *)
type unionmap = rs_type SMap.t

(** Map from type alias to the type they point to *)
type aliasmap = rs_type SMap.t

(** Map for function definitions *)
type funmap = rs_fn SMap.t

(** Map for structure definitions *)
type structmap = rs_struct SMap.t

(** Map for numerical constants *)
type nummap = Libsail.Ast_util.Big_int.num SMap.t

type inline_fun = rs_exp SMap.t

type defs =
  { unions : unionmap
  ; aliasmap : aliasmap
  ; structmap : structmap
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
  match SMap.find_opt fun_id ctx.defs.funmap with
  | Some fn -> Some fn.signature
  | None ->
    (match SMap.find_opt fun_id ctx.arch.external_func with
     | Some (Some t) -> Some t
     | _ -> None)
;;

let ctx_fun (fun_id : string) (ctx : context) : rs_fn option =
  SMap.find_opt fun_id ctx.defs.funmap
;;

let ctx_union_type (union_id : string) (ctx : context) : rs_type option =
  SMap.find_opt union_id ctx.defs.unions
;;

(** Unfold a type definition, resolving type aliases *)
let rec ctx_type (t : rs_type) (ctx : context) : rs_type =
  match t with
  | RsTypId type_id ->
    (match SMap.find_opt type_id ctx.defs.aliasmap with
     | Some t -> ctx_type t ctx
     | None -> t)
  | _ -> t
;;

let ctx_field_type (t : rs_type) (field : string) (ctx : context) : rs_type option =
  match ctx_type t ctx with
  | RsTypId type_id ->
    (match SMap.find_opt type_id ctx.defs.structmap with
     | Some s -> List.find_opt (fun (f, t) -> f = field) s.fields |> Option.map snd
     | None -> None)
  | _ -> None
;;
