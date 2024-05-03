(** Compute the set of functions transitively called from a given entry point **)

open Libsail
open Ast
open Ast_util
open Ast_defs

module SSet = Set.Make(String)

let rec exp_call_set (exp: 'a exp) (s: SSet.t) : SSet.t =
    (* print_string "Exp "; *)
    (* print_endline (string_of_exp exp); *)
    let exp = match exp with | E_aux (exp, aux) -> exp in
    match exp with
        | E_block exp_list -> List.fold_left fold_set s exp_list
        | E_id id -> s
        | E_lit lit -> s
        | E_typ (typ, exp) -> s (*print_string (string_of_typ typ);*)
        | E_app (id, exp_list) ->
            let s = SSet.add (string_of_id id) s in
            List.fold_left fold_set s exp_list
        | E_app_infix (exp1, id, exp2) -> s
        | E_tuple (exp_list) -> s
        | E_if (exp1, exp2, exp3) ->
            let s = exp_call_set exp1 s in
            let s = exp_call_set exp2 s in
            let s = exp_call_set exp3 s in
            s
        | E_loop (loop, measure, exp1, exp2) -> s
        | E_for (id, exp1, exp2, exp3, order, exp4) -> s
        | E_vector (exp_list) -> s
        | E_vector_access (exp1, exp2) -> s
        | E_vector_subrange (exp1, exp2, exp3) -> s
        | E_vector_update (exp1, exp2, exp3) -> s
        | E_vector_update_subrange (exp1 ,exp2, exp3, exp4) -> s
        | E_vector_append (exp1 ,exp2) -> s
        | E_list (exp_list) -> s
        | E_cons (exp1, exp2) -> s
        | E_struct (fexp_list) -> s
        | E_struct_update (exp, fexp_list) -> s
        | E_field (exp, id) -> s
        | E_match (exp, pexp_list) ->
            let s = exp_call_set exp s in
            let fold_set_pexp s pexp = SSet.union s (pexp_call_set pexp s) in
            List.fold_left fold_set_pexp s pexp_list
        | E_let (LB_aux (LB_val (let_var, let_exp), _), exp) ->
            let s = exp_call_set let_exp s in
            exp_call_set exp s
        | E_assign (lexp, exp) -> s
        | E_sizeof nexp -> s
        | E_return exp -> s
        | E_exit exp -> s
        | E_ref id -> s
        | E_throw exp -> s
        | E_try (exp, pexp_list) -> s
        | E_assert (exp1, exp2) -> s
        | E_var (lexp, exp1, exp2) -> s
        | E_internal_plet (pat, exp1, exp2) -> s
        | E_internal_return exp -> s
        | E_internal_value value -> s
        | E_internal_assume (n_constraint, exp) -> s
        | E_constraint n_constraint -> s
and pexp_call_set (Pat_aux (pexp, annot)) (s: SSet.t) : SSet.t =
    match pexp with
        | Pat_exp (pat, exp) -> s
        | Pat_when (pat, exp1, exp2) -> s
and fold_set (s: SSet.t) exp =
    SSet.union s (exp_call_set exp s)

(* Return the ID of an application pattern as a string, or "" otherwise. *)
let pat_app_name (P_aux (pat_aux, _)) =
    match pat_aux with
        | P_app (id, _) -> string_of_id id
        | _ -> ""

let pexp_func_call_set (Pat_aux (pexp, annot)) (s: SSet.t) : SSet.t =
    match pexp with
        | Pat_exp (pat, exp) ->
            if SSet.mem (pat_app_name pat) s then
                exp_call_set exp s
            else s
        | Pat_when (pat1, exp, pat2) -> s (* TODO: do we need to handle that? *)

let func_call_set (FCL_aux (FCL_funcl (id, pexp), annot)) (s: SSet.t): SSet.t =
    (* print_endline (string_of_id id); *)
    pexp_func_call_set pexp s

let rec funcl_call_set (funcl: 'a funcl list) (s: SSet.t) : SSet.t =
    match funcl with
        | h :: t -> SSet.union (func_call_set h s) (funcl_call_set t s)
        | [] -> s

let fundef_call_set (FD_function (rec_opt, tannot_opt, funcl)) (s: SSet.t) : SSet.t =
    funcl_call_set funcl s

let node_call_set (DEF_aux (def, annot)) (s: SSet.t) : SSet.t =
    match def with
        | DEF_register (DEC_aux (dec_spec, annot)) -> s
        | DEF_scattered (SD_aux (scattered, annot)) -> s
        | DEF_fundef (FD_aux (fundef, annot)) -> fundef_call_set fundef s
        | _ -> s

let rec defs_call_set (defs: 'a def list) (s: SSet.t) : SSet.t =
    match defs with
        | h :: t -> SSet.union (node_call_set h s) (defs_call_set t s)
        | [] -> s

let get_call_set (ast: 'a ast) : unit =
    let set = SSet.empty in
    let set = SSet.add "CSR" set in
    let set = SSet.add "ITYPE" set in
    let set = defs_call_set ast.defs set in
    SSet.iter (Printf.printf "%s ") set;
    print_endline "Hello, world!"
