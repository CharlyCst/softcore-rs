(** Compute the set of functions transitively called from a given entry point **)

open Libsail
open Ast
open Ast_util
open Ast_defs
open Type_check
module SSet = Set.Make (String)
module SMap = Map.Make (String)

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

let rec exp_call_set (texp : tannot exp) (ctx : sail_ctx) : sail_ctx =
  let (E_aux (exp, aux)) = texp in
  match exp with
  | E_block exp_list -> List.fold_left fold_set ctx exp_list
  | E_id id -> ctx
  | E_lit lit -> ctx
  | E_typ (typ, exp) -> exp_call_set exp ctx
  | E_app (id, exp_list) ->
    let ctx = add_fn (string_of_id id) ctx in
    List.fold_left fold_set ctx exp_list
  | E_app_infix (exp1, id, exp2) ->
    ctx_union (exp_call_set exp1 ctx) (exp_call_set exp2 ctx)
  | E_tuple exp_list -> List.fold_left fold_set ctx exp_list
  | E_if (exp1, exp2, exp3) ->
    let s = exp_call_set exp1 ctx in
    let s = exp_call_set exp2 s in
    let s = exp_call_set exp3 s in
    s
  | E_loop (loop, measure, exp1, exp2) ->
    ctx_union (exp_call_set exp1 ctx) (exp_call_set exp2 ctx)
  | E_for (id, exp1, exp2, exp3, order, exp4) ->
    let s = ctx_union (exp_call_set exp1 ctx) (exp_call_set exp2 ctx) in
    let s = ctx_union (exp_call_set exp3 s) s in
    ctx_union (exp_call_set exp4 s) s
  | E_vector exp_list -> List.fold_left fold_set ctx exp_list
  | E_vector_access (exp1, exp2) ->
    ctx_union (exp_call_set exp1 ctx) (exp_call_set exp2 ctx)
  | E_vector_subrange (exp1, exp2, exp3) -> ctx
  | E_vector_update (exp1, exp2, exp3) -> ctx
  | E_vector_update_subrange (exp1, exp2, exp3, exp4) -> ctx
  | E_vector_append (exp1, exp2) -> ctx
  | E_list exp_list -> List.fold_left fold_set ctx exp_list
  | E_cons (exp1, exp2) -> ctx
  | E_struct fexp_list -> ctx
  | E_struct_update (exp, fexp_list) -> ctx
  | E_field (exp, id) -> exp_call_set exp ctx
  | E_match (exp, pexp_list) ->
    let s = exp_call_set exp ctx in
    let fold_set_pexp s pexp = ctx_union s (pexp_call_set pexp s) in
    List.fold_left fold_set_pexp s pexp_list
  | E_let (LB_aux (LB_val (let_var, let_exp), _), exp) ->
    let s = exp_call_set let_exp ctx in
    exp_call_set exp s
  | E_assign (lexp, exp) -> exp_call_set exp ctx
  | E_sizeof nexp -> ctx
  | E_return exp -> exp_call_set exp ctx
  | E_exit exp -> exp_call_set exp ctx
  | E_ref id -> ctx
  | E_throw exp -> ctx
  | E_try (exp, pexp_list) -> ctx
  | E_assert (exp1, exp2) -> ctx_union (exp_call_set exp1 ctx) (exp_call_set exp2 ctx)
  | E_var (lexp, exp1, exp2) -> ctx
  | E_internal_plet (pat, exp1, exp2) -> ctx
  | E_internal_return exp -> ctx
  | E_internal_value value -> ctx
  | E_internal_assume (n_constraint, exp) -> ctx
  | E_constraint n_constraint -> ctx
  | E_config cfgs ->
    let typ = typ_of texp in
    let cfg = String.concat "." cfgs in
    add_config cfg typ ctx

and pexp_call_set (Pat_aux (pexp, annot)) (ctx : sail_ctx) : sail_ctx =
  match pexp with
  | Pat_exp (pat, exp) -> exp_call_set exp ctx
  | Pat_when (pat, exp1, exp2) ->
    ctx_union (exp_call_set exp1 ctx) (exp_call_set exp2 ctx)

and fold_set (ctx : sail_ctx) exp = ctx_union ctx (exp_call_set exp ctx)

(* Return the ID of an application pattern as a string, or "" otherwise. *)
let pat_app_name (P_aux (pat_aux, _)) =
  match pat_aux with
  | P_app (id, _) -> string_of_id id
  | _ -> ""
;;

let func_call_set (FCL_aux (FCL_funcl (id, pexp), annot) : tannot funcl) (ctx : sail_ctx)
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
    | Pat_exp (pat, exp) -> exp_call_set exp ctx
    | Pat_when (pat1, exp, pat2) -> exp_call_set exp ctx)
  else (
    match pexp with
    | Pat_exp (pat, exp) ->
      let name = pat_app_name pat in
      if SSet.mem name ctx.call_set then exp_call_set exp ctx else ctx
    | _ -> ctx)
;;

let rec funcl_call_set (funcl : tannot funcl list) (ctx : sail_ctx) : sail_ctx =
  match funcl with
  | h :: t -> ctx_union (func_call_set h ctx) (funcl_call_set t ctx)
  | [] -> ctx
;;

let fundef_call_set
      (FD_function (rec_opt, tannot_opt, funcl) : tannot fundef_aux)
      (ctx : sail_ctx)
  : sail_ctx
  =
  funcl_call_set funcl ctx
;;

let node_call_set (DEF_aux (def, annot)) (ctx : sail_ctx) : sail_ctx =
  match def with
  | DEF_register (DEC_aux (dec_spec, annot)) -> ctx
  | DEF_scattered (SD_aux (scattered, annot)) -> ctx
  | DEF_fundef (FD_aux (fundef, annot)) -> fundef_call_set fundef ctx
  | DEF_impl funcl -> func_call_set funcl ctx
  | DEF_let (LB_aux (LB_val (pat, exp), aux)) -> exp_call_set exp ctx
  | _ -> ctx
;;

let rec defs_call_set (defs : (tannot, env) def list) (ctx : sail_ctx) : sail_ctx =
  match defs with
  | h :: t -> ctx_union (node_call_set h ctx) (defs_call_set t ctx)
  | [] -> ctx
;;

let rec get_call_set_rec (ast : (tannot, env) ast) (ctx : sail_ctx) : sail_ctx =
  let new_ctx = defs_call_set ast.defs ctx in
  if SSet.equal new_ctx.call_set ctx.call_set
  then new_ctx
  else get_call_set_rec ast new_ctx
;;

let rec get_call_set (ast : (tannot, env) ast) : sail_ctx =
  let call_set =
    SSet.of_list
      [ (* "execute"; *)
        "CSR"
      ; "MRET"
      ; "SRET"
      ; "ITYPE"
      ; "TEST"
      ; "WFI"
      ; "EBREAK"
      ; "dispatchInterrupt"
      ; "step_interrupts_only"
      ; "SFENCE_VMA"
      ; "HFENCE_VVMA"
      ; "HFENCE_GVMA"
      ; (* Decoder *)
        "encdec_backwards"
      ; (* Registers *)
        "rX"
      ; "wX"
      ; "is_CSR_defined"
      ; (* PMP checks *)
        "pmpCheck"
      ; "pmpWriteAddrReg"
      ; "pmpWriteCfgReg"
      ; (* Trap handling *)
        "trap_handler"
      ; "exception_delegatee"
      ; "exceptionType_to_bits"
      ; (* CSRs *)
        "read_CSR"
      ; "write_CSR"
      ; "doCSR"
      ; (* System reset *)
        "reset_sys"
      ]
  in
  let sail_ctx = { call_set; config_map = SMap.empty } in
  get_call_set_rec ast sail_ctx
;;
