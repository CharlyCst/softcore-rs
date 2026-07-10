open Rs_ast
open Rs_ast_utils

(** Map from type constructors to their types *)
type unionmap = rs_type SMap.t

(** Map from type alias to the type they point to *)
type aliasmap = rs_type SMap.t

(** Map for function definitions *)
type funmap = rs_fn SMap.t

(** Map for structure definitions *)
type structmap = rs_struct SMap.t

(** Map for constants definitions *)
type constmap = rs_const SMap.t

(** Map for numerical constants *)
type nummap = Libsail.Ast_util.Big_int.num SMap.t

type inline_fun = rs_exp SMap.t

type defs =
  { unions : unionmap
  ; aliasmap : aliasmap
  ; structmap : structmap
  ; funmap : funmap
  ; constants : constmap
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

let ctx_const (const_id : string) (ctx : context) : rs_const option =
  SMap.find_opt const_id ctx.defs.constants
;;

let ctx_id_require_sail_ctx (id : string) (ctx : context) : bool =
  match ctx_fun id ctx with
  | Some fn -> fn.use_sail_ctx
  | None ->
    (match ctx_const id ctx with
     | Some const -> const.use_sail_ctx
     | None -> false)
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

(* Removing from context ---------------------------------------------------- *)

let ctx_remove_fn (fn : rs_fn) (ctx : context) : context =
  { ctx with defs = { ctx.defs with funmap = SMap.remove fn.name ctx.defs.funmap } }
;;

let ctx_remove_alias (alias : rs_alias) (ctx : context) : context =
  { ctx with
    defs = { ctx.defs with aliasmap = SMap.remove alias.new_typ ctx.defs.aliasmap }
  }
;;

let ctx_remove_struct (s : rs_struct) (ctx : context) : context =
  { ctx with defs = { ctx.defs with structmap = SMap.remove s.name ctx.defs.structmap } }
;;

let ctx_remove_enum (enum : rs_enum) (ctx : context) : context =
  let defs =
    List.fold_left
      (fun (defs : defs) (constructor_name, constructor_type) : defs ->
         match constructor_type with
         | Some _ ->
           let name = enum.name ^ "::" ^ constructor_name in
           { defs with funmap = SMap.remove name defs.funmap }
         | None -> defs)
      ctx.defs
      enum.fields
  in
  { ctx with defs }
;;

let ctx_remove_const (const : rs_const) (ctx : context) : context =
  { ctx with
    defs = { ctx.defs with constants = SMap.remove const.name ctx.defs.constants }
  }
;;

let ctx_remove_obj (obj : rs_obj) (ctx : context) : context =
  match obj with
  | RsFn fn -> ctx_remove_fn fn ctx
  | RsEnum enum -> ctx_remove_enum enum ctx
  | RsStruct s -> ctx_remove_struct s ctx
  | RsAlias alias -> ctx_remove_alias alias ctx
  | RsConst const -> ctx_remove_const const ctx
  | RsImport _ | RsAttribute _ | RsObjTodo _ -> ctx
;;

(* Adding to context -------------------------------------------------------- *)

let ctx_add_fn (fn : rs_fn) (ctx : context) : context =
  { ctx with defs = { ctx.defs with funmap = SMap.add fn.name fn ctx.defs.funmap } }
;;

let ctx_add_alias (alias : rs_alias) (ctx : context) : context =
  { ctx with
    defs =
      { ctx.defs with aliasmap = SMap.add alias.new_typ alias.old_type ctx.defs.aliasmap }
  }
;;

let ctx_add_struct (s : rs_struct) (ctx : context) : context =
  { ctx with defs = { ctx.defs with structmap = SMap.add s.name s ctx.defs.structmap } }
;;

let ctx_add_enum (enum : rs_enum) (ctx : context) : context =
  let defs =
    List.fold_left
      (fun (defs : defs) (constructor_name, constructor_type) : defs ->
         match constructor_type with
         | Some t ->
           (* TODO(Gurvan): fix that by adding a path to patterns? *)
           let name = enum.name ^ "::" ^ constructor_name in
           let signature =
             { generics = enum.generics
             ; args = [ t ]
             ; ret = RsTypId enum.name
             ; linked_gen_args = []
             }
           in
           let f = empty_function_from_type name signature in
           { defs with funmap = SMap.add name f defs.funmap }
         | None -> defs)
      ctx.defs
      enum.fields
  in
  { ctx with defs }
;;

let ctx_add_const (const : rs_const) (ctx : context) : context =
  { ctx with
    defs = { ctx.defs with constants = SMap.add const.name const ctx.defs.constants }
  }
;;

let ctx_add_obj (obj : rs_obj) (ctx : context) : context =
  match obj with
  | RsFn fn -> ctx_add_fn fn ctx
  | RsEnum enum -> ctx_add_enum enum ctx
  | RsStruct s -> ctx_add_struct s ctx
  | RsAlias alias -> ctx_add_alias alias ctx
  | RsConst const -> ctx_add_const const ctx
  | RsImport _ | RsAttribute _ | RsObjTodo _ -> ctx
;;
