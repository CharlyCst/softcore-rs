(** Rust Transformations **)
(** This module transforms raw Tust code generated from Sail into a valid Rust module. **)

open Rust_gen

let transform_pat (pat: rs_pat) : rs_pat =
    pat

let transform_lexp (lexp: rs_lexp) : rs_lexp =
    lexp

let rec transform_exp (exp: rs_exp) : rs_exp =
    match exp with
        | RsLet (pat, exp, next) ->
            (RsLet (
                (transform_pat pat),
                (transform_exp exp),
                (transform_exp next)))
        | RsApp (app, args) ->
            (RsApp (
                app,
                (List.map transform_exp args)))
        | RsId id -> RsId id
        | RsLit lit -> RsLit lit
        | RsBlock exps -> RsBlock (List.map transform_exp exps)
        | RsIf (cond, exp_true, exp_false) ->
            (RsIf (
                (transform_exp cond),
                (transform_exp exp_true),
                (transform_exp exp_false)))
        | RsMatch (exp, pexps) ->
            (RsMatch (
                (transform_exp exp),
                (List.map transform_pexp pexps)))
        | RsTuple exps ->
            (RsTuple
                (List.map transform_exp exps))
        | RsAssign (lexp, exp) ->
            (RsAssign (
                (transform_lexp lexp),
                (transform_exp exp)))
        | RsTodo -> RsTodo
and transform_pexp (pexp: rs_pexp) : rs_pexp =
    pexp
 

let transform_fn (fn: rs_fn) : rs_fn =
    fn

let rust_transform (RsProg fns) : rs_program =
    RsProg (List.map transform_fn fns)
