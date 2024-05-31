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
        | RsApp (app, args) -> transform_app app args
        | RsMethodApp (exp, id, args) ->
            (RsMethodApp (
                (transform_exp exp),
                id,
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
        | RsIndex (exp1, exp2) ->
            (RsIndex (
                (transform_exp exp1),
                (transform_exp exp2)))
        | RsBinop (exp1, binop, exp2) ->
            (RsBinop (
                (transform_exp exp1),
                binop,
                (transform_exp exp2)))
        | RsTodo -> RsTodo

and transform_app (fn: string) (args: rs_exp list) : rs_exp =
    let args = List.map transform_exp args in
    match (fn, args) with
        | ("plain_vector_access", vector::item::[]) -> (RsIndex (vector, item))
        | ("neq_int", left::right::[]) -> (RsBinop (left, RsBinopNeq, right))
        | ("neq_bits", left::right::[]) -> (RsBinop (left, RsBinopNeq, right))
        | ("eq_int", left::right::[]) -> (RsBinop (left, RsBinopEq, right))
        | ("eq_bool", left::right::[]) -> (RsBinop (left, RsBinopEq, right))
        | ("eq_bits", left::right::[]) -> (RsBinop (left, RsBinopEq, right))
        | ("eq_anything", left::right::[]) -> (RsBinop (left, RsBinopEq, right))
        | ("or_vec", left::right::[]) -> (RsBinop (left, RsBinopOr, right))
        | ("and_vec", left::right::[]) -> (RsBinop (left, RsBinopAnd, right))
        | ("xor_vec", left::right::[]) -> (RsBinop (left, RsBinopXor, right))
        | ("add_bits", left::right::[]) -> (RsMethodApp (left, "wrapped_add", [right]))
        | ("and_bool", left::right::[]) -> (RsBinop (left, RsBinopLAnd, right))
        | ("or_bool", left::right::[]) -> (RsBinop (left, RsBinopLOr, right))
        | _ -> (RsApp (fn, args))

and transform_pexp (pexp: rs_pexp) : rs_pexp =
    match pexp with
        | RsPexp (pat, exp) ->
            (RsPexp (
                (transform_pat pat),
                (transform_exp exp)))
        | RsPexpWhen (pat, exp1, exp2) ->
            (RsPexpWhen (
                (transform_pat pat),
                (transform_exp exp1),
                (transform_exp exp2)))
 
 

let transform_fn (fn: rs_fn) : rs_fn =
    let (name, exp) = fn in
    (name, (transform_exp exp))

let rust_transform (RsProg fns) : rs_program =
    RsProg (List.map transform_fn fns)
