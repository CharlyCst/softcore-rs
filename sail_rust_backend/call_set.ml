(** Compute the set of functions transitively called from a given entry point **)

open Libsail
open Ast
open Ast_util
open Ast_defs
open Type_check
module SSet = Types.SSet
module SMap = Types.SMap

type arch_t = Types.arch_t
type config_map = typ SMap.t

type sail_ctx =
  { call_set : SSet.t
  ; config_map : config_map
  }

let add_fn (fn : string) (ctx : sail_ctx) : sail_ctx =
  { ctx with call_set = SSet.add fn ctx.call_set }
;;

let add_config (config : string) (t : typ) (ctx : sail_ctx) : sail_ctx =
  let res = { ctx with config_map = SMap.add config t ctx.config_map } in
  (match SMap.find_opt config ctx.config_map with
   | Some t' ->
     if t <> t'
     then
       Reporting.simple_warn
         (Printf.sprintf
            "Config used with different types: %s and %s"
            (string_of_typ t)
            (string_of_typ t'))
   | None -> ());
  res
;;

let rec exp_call_set (texp : tannot exp) (arch : arch_t) (ctx : sail_ctx) : sail_ctx =
  let (E_aux (exp, _)) = texp in
  match exp with
  | E_block exp_list -> List.fold_left (fold_set arch) ctx exp_list
  | E_id _ -> ctx
  | E_lit _ -> ctx
  | E_typ (_, exp) -> exp_call_set exp arch ctx
  | E_app (id, exp_list) ->
    let id = string_of_id id in
    if SSet.mem id arch.unsupported_func || SSet.mem id arch.overwritten_func
    then ctx
    else (
      let ctx = add_fn id ctx in
      List.fold_left (fold_set arch) ctx exp_list)
  | E_app_infix (exp1, _, exp2) -> ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch
  | E_tuple exp_list -> List.fold_left (fold_set arch) ctx exp_list
  | E_if (exp1, exp2, exp3) ->
    ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch |> exp_call_set exp3 arch
  | E_loop (_, _, exp1, exp2) -> ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch
  | E_for (_, exp1, exp2, exp3, _, exp4) ->
    ctx
    |> exp_call_set exp1 arch
    |> exp_call_set exp2 arch
    |> exp_call_set exp3 arch
    |> exp_call_set exp4 arch
  | E_vector exp_list -> List.fold_left (fold_set arch) ctx exp_list
  | E_vector_access (exp1, exp2) ->
    ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch
  | E_vector_subrange (exp1, exp2, exp3) ->
    ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch |> exp_call_set exp3 arch
  | E_vector_update (exp1, exp2, exp3) ->
    ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch |> exp_call_set exp3 arch
  | E_vector_update_subrange (exp1, exp2, exp3, exp4) ->
    ctx
    |> exp_call_set exp1 arch
    |> exp_call_set exp2 arch
    |> exp_call_set exp3 arch
    |> exp_call_set exp4 arch
  | E_vector_append (exp1, exp2) ->
    ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch
  | E_list exp_list -> List.fold_left (fold_set arch) ctx exp_list
  | E_cons (exp1, exp2) -> ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch
  | E_struct fexp_list -> List.fold_left (fun c f -> fexp_call_set f arch c) ctx fexp_list
  | E_struct_update (exp, fexp_list) ->
    List.fold_left (fun c f -> fexp_call_set f arch c) ctx fexp_list
    |> exp_call_set exp arch
  | E_field (exp, _) -> exp_call_set exp arch ctx
  | E_match (exp, pexp_list) ->
    let s = exp_call_set exp arch ctx in
    let fold_set_pexp s pexp = pexp_call_set pexp arch s in
    List.fold_left fold_set_pexp s pexp_list
  | E_let (LB_aux (LB_val (_, let_exp), _), exp) ->
    let s = exp_call_set let_exp arch ctx in
    exp_call_set exp arch s
  | E_var (_, exp1, exp2) -> ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch
  | E_assign (_, exp) -> exp_call_set exp arch ctx
  | E_sizeof _ -> ctx
  | E_return exp -> exp_call_set exp arch ctx
  | E_exit exp -> exp_call_set exp arch ctx
  | E_ref _ -> ctx
  | E_throw _ -> ctx
  | E_try (_, _) -> ctx
  | E_assert (exp1, exp2) -> ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch
  | E_internal_plet _ -> ctx
  | E_internal_return _ -> ctx
  | E_internal_value _ -> ctx
  | E_internal_assume _ -> ctx
  | E_constraint _ -> ctx
  | E_config cfgs ->
    let typ = typ_of texp in
    let cfg = String.concat "." cfgs in
    add_config cfg typ ctx

and fexp_call_set (fexp : 't fexp) (arch : arch_t) (ctx : sail_ctx) : sail_ctx =
  let (FE_aux (FE_fexp (_, e), _)) = fexp in
  exp_call_set e arch ctx

and pexp_call_set (Pat_aux (pexp, _)) (arch : arch_t) (ctx : sail_ctx) : sail_ctx =
  match pexp with
  | Pat_exp (P_aux (P_id id, _), _)
  | Pat_when (P_aux (P_id id, _), _, _)
  | Pat_exp (P_aux (P_app (id, _), _), _)
  | Pat_when (P_aux (P_app (id, _), _), _, _)
    when SSet.mem (string_of_id id) arch.unsupported_match -> ctx
  | Pat_exp (_, exp) -> exp_call_set exp arch ctx
  | Pat_when (_, exp1, exp2) -> ctx |> exp_call_set exp1 arch |> exp_call_set exp2 arch

and fold_set (arch : arch_t) (ctx : sail_ctx) exp = exp_call_set exp arch ctx

(* Return the ID of an application pattern as a string, or "" otherwise. *)
let pat_app_name (P_aux (pat_aux, _)) =
  match pat_aux with
  | P_app (id, _) -> string_of_id id
  | _ -> ""
;;

let func_call_set
      (FCL_aux (FCL_funcl (id, pexp), _) : tannot funcl)
      (arch : arch_t)
      (ctx : sail_ctx)
  : sail_ctx
  =
  let pexp, _ =
    match pexp with
    | Pat_aux (pexp, annot) -> pexp, annot
  in
  let name = string_of_id id in
  if SSet.mem name ctx.call_set
  then (
    match pexp with
    | Pat_exp (_, exp) -> exp_call_set exp arch ctx
    | Pat_when (_, exp, _) -> exp_call_set exp arch ctx)
  else (
    match pexp with
    | Pat_exp (pat, exp) ->
      let name = pat_app_name pat in
      if SSet.mem name ctx.call_set then exp_call_set exp arch ctx else ctx
    | _ -> ctx)
;;

let rec funcl_call_set (funcl : tannot funcl list) (arch : arch_t) (ctx : sail_ctx)
  : sail_ctx
  =
  match funcl with
  | h :: t -> ctx |> func_call_set h arch |> funcl_call_set t arch
  | [] -> ctx
;;

let fundef_call_set
      (FD_function (_, _, funcl) : tannot fundef_aux)
      (arch : arch_t)
      (ctx : sail_ctx)
  : sail_ctx
  =
  funcl_call_set funcl arch ctx
;;

let register_call_set (DEC_reg (_, _, exp)) (arch : arch_t) (ctx : sail_ctx) : sail_ctx =
  match exp with
  | Some exp -> exp_call_set exp arch ctx
  | None -> ctx
;;

let node_call_set (DEF_aux (def, _)) (arch : arch_t) (ctx : sail_ctx) : sail_ctx =
  match def with
  | DEF_register (DEC_aux (dec_spec, _)) -> register_call_set dec_spec arch ctx
  | DEF_scattered (SD_aux (_, _)) -> ctx
  | DEF_fundef (FD_aux (fundef, _)) -> fundef_call_set fundef arch ctx
  | DEF_impl funcl -> func_call_set funcl arch ctx
  | DEF_let (LB_aux (LB_val (_, exp), _)) -> exp_call_set exp arch ctx
  | _ -> ctx
;;

let rec defs_call_set (defs : (tannot, env) def list) (arch : arch_t) (ctx : sail_ctx)
  : sail_ctx
  =
  match defs with
  | h :: t -> ctx |> node_call_set h arch |> defs_call_set t arch
  | [] -> ctx
;;

let rec get_call_set_rec (arch : arch_t) (ast : (tannot, env) ast) (ctx : sail_ctx)
  : sail_ctx
  =
  let new_ctx = defs_call_set ast.defs arch ctx in
  if SSet.equal new_ctx.call_set ctx.call_set
  then new_ctx
  else get_call_set_rec arch ast new_ctx
;;

let get_call_set (arch : arch_t) (ast : (tannot, env) ast) : sail_ctx =
  let call_set = arch.call_set in
  let sail_ctx = { call_set; config_map = SMap.empty } in
  get_call_set_rec arch ast sail_ctx
;;
