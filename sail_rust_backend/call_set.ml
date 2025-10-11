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
  { ctx with config_map = SMap.add config t ctx.config_map }
;;

let ctx_union (ctx1 : sail_ctx) (ctx2 : sail_ctx) : sail_ctx =
  let choose_typ (cfg : string) (a : typ) (b : typ) : typ option =
    let ret = Some a in
    if a <> b
    then (
      Reporting.simple_warn
        (Printf.sprintf
           "Config used with different types: %s and %s"
           (string_of_typ a)
           (string_of_typ b));
      ret)
    else ret
  in
  { call_set = SSet.union ctx1.call_set ctx2.call_set
  ; config_map = SMap.union choose_typ ctx1.config_map ctx2.config_map
  }
;;

let rec exp_call_set (texp : tannot exp) (arch : arch_t) (ctx : sail_ctx) : sail_ctx =
  let (E_aux (exp, aux)) = texp in
  match exp with
  | E_block exp_list -> List.fold_left (fold_set arch) ctx exp_list
  | E_id id -> ctx
  | E_lit lit -> ctx
  | E_typ (typ, exp) -> exp_call_set exp arch ctx
  | E_app (id, exp_list) ->
    if SSet.mem (string_of_id id) arch.unsupported_func
    then ctx
    else (
      let ctx = add_fn (string_of_id id) ctx in
      List.fold_left (fold_set arch) ctx exp_list)
  | E_app_infix (exp1, id, exp2) ->
    ctx_union (exp_call_set exp1 arch ctx) (exp_call_set exp2 arch ctx)
  | E_tuple exp_list -> List.fold_left (fold_set arch) ctx exp_list
  | E_if (exp1, exp2, exp3) ->
    let s = exp_call_set exp1 arch ctx in
    let s = exp_call_set exp2 arch s in
    let s = exp_call_set exp3 arch s in
    s
  | E_loop (loop, measure, exp1, exp2) ->
    ctx_union (exp_call_set exp1 arch ctx) (exp_call_set exp2 arch ctx)
  | E_for (id, exp1, exp2, exp3, order, exp4) ->
    let s = ctx_union (exp_call_set exp1 arch ctx) (exp_call_set exp2 arch ctx) in
    let s = ctx_union (exp_call_set exp3 arch s) s in
    ctx_union (exp_call_set exp4 arch s) s
  | E_vector exp_list -> List.fold_left (fold_set arch) ctx exp_list
  | E_vector_access (exp1, exp2) ->
    ctx_union (exp_call_set exp1 arch ctx) (exp_call_set exp2 arch ctx)
  | E_vector_subrange (exp1, exp2, exp3) -> ctx
  | E_vector_update (exp1, exp2, exp3) -> ctx
  | E_vector_update_subrange (exp1, exp2, exp3, exp4) -> ctx
  | E_vector_append (exp1, exp2) -> ctx
  | E_list exp_list -> List.fold_left (fold_set arch) ctx exp_list
  | E_cons (exp1, exp2) -> ctx
  | E_struct fexp_list -> ctx
  | E_struct_update (exp, fexp_list) -> ctx
  | E_field (exp, id) -> exp_call_set exp arch ctx
  | E_match (exp, pexp_list) ->
    let s = exp_call_set exp arch ctx in
    let fold_set_pexp s pexp = ctx_union s (pexp_call_set pexp arch s) in
    List.fold_left fold_set_pexp s pexp_list
  | E_let (LB_aux (LB_val (let_var, let_exp), _), exp) ->
    let s = exp_call_set let_exp arch ctx in
    exp_call_set exp arch s
  | E_var (lexp, exp1, exp2) ->
    ctx_union (exp_call_set exp1 arch ctx) (exp_call_set exp2 arch ctx)
  | E_assign (lexp, exp) -> exp_call_set exp arch ctx
  | E_sizeof nexp -> ctx
  | E_return exp -> exp_call_set exp arch ctx
  | E_exit exp -> exp_call_set exp arch ctx
  | E_ref id -> ctx
  | E_throw exp -> ctx
  | E_try (exp, pexp_list) -> ctx
  | E_assert (exp1, exp2) ->
    ctx_union (exp_call_set exp1 arch ctx) (exp_call_set exp2 arch ctx)
  | E_internal_plet (pat, exp1, exp2) -> ctx
  | E_internal_return exp -> ctx
  | E_internal_value value -> ctx
  | E_internal_assume (n_constraint, exp) -> ctx
  | E_constraint n_constraint -> ctx
  | E_config cfgs ->
    let typ = typ_of texp in
    let cfg = String.concat "." cfgs in
    add_config cfg typ ctx

and pexp_call_set (Pat_aux (pexp, annot)) (arch : arch_t) (ctx : sail_ctx) : sail_ctx =
  match pexp with
  | Pat_exp (P_aux (P_id id, _), _)
  | Pat_when (P_aux (P_id id, _), _, _)
  | Pat_exp (P_aux (P_app (id, _), _), _)
  | Pat_when (P_aux (P_app (id, _), _), _, _)
    when SSet.mem (string_of_id id) arch.unsupported_match -> ctx
  | Pat_exp (pat, exp) -> exp_call_set exp arch ctx
  | Pat_when (pat, exp1, exp2) ->
    ctx_union (exp_call_set exp1 arch ctx) (exp_call_set exp2 arch ctx)

and fold_set (arch : arch_t) (ctx : sail_ctx) exp =
  ctx_union ctx (exp_call_set exp arch ctx)
;;

(* Return the ID of an application pattern as a string, or "" otherwise. *)
let pat_app_name (P_aux (pat_aux, _)) =
  match pat_aux with
  | P_app (id, _) -> string_of_id id
  | _ -> ""
;;

let func_call_set
      (FCL_aux (FCL_funcl (id, pexp), annot) : tannot funcl)
      (arch : arch_t)
      (ctx : sail_ctx)
  : sail_ctx
  =
  let pexp, annot =
    match pexp with
    | Pat_aux (pexp, annot) -> pexp, annot
  in
  let name = string_of_id id in
  if SSet.mem name ctx.call_set
  then (
    match pexp with
    | Pat_exp (pat, exp) -> exp_call_set exp arch ctx
    | Pat_when (pat1, exp, pat2) -> exp_call_set exp arch ctx)
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
  | h :: t -> ctx_union (func_call_set h arch ctx) (funcl_call_set t arch ctx)
  | [] -> ctx
;;

let fundef_call_set
      (FD_function (rec_opt, tannot_opt, funcl) : tannot fundef_aux)
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

let node_call_set (DEF_aux (def, annot)) (arch : arch_t) (ctx : sail_ctx) : sail_ctx =
  match def with
  | DEF_register (DEC_aux (dec_spec, annot)) -> register_call_set dec_spec arch ctx
  | DEF_scattered (SD_aux (scattered, annot)) -> ctx
  | DEF_fundef (FD_aux (fundef, annot)) -> fundef_call_set fundef arch ctx
  | DEF_impl funcl -> func_call_set funcl arch ctx
  | DEF_let (LB_aux (LB_val (pat, exp), aux)) -> exp_call_set exp arch ctx
  | _ -> ctx
;;

let rec defs_call_set (defs : (tannot, env) def list) (arch : arch_t) (ctx : sail_ctx)
  : sail_ctx
  =
  match defs with
  | h :: t -> ctx_union (node_call_set h arch ctx) (defs_call_set t arch ctx)
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

let rec get_call_set (arch : arch_t) (ast : (tannot, env) ast) : sail_ctx =
  let call_set = arch.call_set in
  let sail_ctx = { call_set; config_map = SMap.empty } in
  get_call_set_rec arch ast sail_ctx
;;
