(** Rust Transformations **)
(** This module transforms raw Tust code generated from Sail into a valid Rust module. **)

open Rust_gen

(* ——————————————————————————— Custom transforms ———————————————————————————— *)

type custom_transform = {
    exp : rs_exp -> rs_exp
}

let kani_transfrom_exp (exp: rs_exp) : rs_exp =
    match exp with
        (* | RsApp (RsId "subrange_bits", [bitvec; range_end; range_start]) -> RsTodo *)
        | _ -> exp

let kani_transform = {
    exp = kani_transfrom_exp
}

(* ————————————————————————— Transform Expressions —————————————————————————— *)

let rec transform_pat (ct: custom_transform) (pat: rs_pat): rs_pat =
    pat

and transform_lexp (ct: custom_transform) (lexp: rs_lexp): rs_lexp =
    match lexp with
        | RsLexpId id -> RsLexpId id
        | RsLexpField (lexp, id) ->
            (RsLexpField (
                (transform_lexp ct lexp),
                id))
        | RsLexpIndex (lexp, exp) ->
            (RsLexpIndex (
                (transform_lexp ct lexp),
                (transform_exp ct exp)))
        | RsLexpIndexRange (lexp, range_start, range_end) ->
            (RsLexpIndexRange (
                (transform_lexp ct lexp),
                (transform_exp ct range_start),
                (transform_exp ct range_end)))
        | RsLexpTodo -> RsLexpTodo

and transform_exp (ct: custom_transform) (exp: rs_exp) : rs_exp =
    let exp = ct.exp exp in
    match exp with
        | RsLet (pat, exp, next) ->
            (RsLet (
                (transform_pat ct pat),
                (transform_exp ct exp),
                (transform_exp ct next)))
        | RsApp (app, args) -> transform_app ct app args
        | RsMethodApp (exp, id, args) ->
            (RsMethodApp (
                (transform_exp ct exp),
                id,
                (List.map (transform_exp ct) args)))
        | RsId id -> RsId id
        | RsLit lit -> RsLit lit
        | RsField (exp, field) -> RsField ((transform_exp ct exp), field)
        | RsBlock exps -> RsBlock (List.map (transform_exp ct) exps)
        | RsIf (cond, exp_true, exp_false) ->
            (RsIf (
                (transform_exp ct cond),
                (transform_exp ct exp_true),
                (transform_exp ct exp_false)))
        | RsMatch (exp, pexps) ->
            (RsMatch (
                (transform_exp ct exp),
                (List.map (transform_pexp ct) pexps)))
        | RsTuple exps ->
            (RsTuple
                (List.map (transform_exp ct) exps))
        | RsAssign (lexp, exp) ->
            (RsAssign (
                (transform_lexp ct lexp),
                (transform_exp ct exp)))
        | RsIndex (exp1, exp2) ->
            (RsIndex (
                (transform_exp ct exp1),
                (transform_exp ct exp2)))
        | RsBinop (exp1, binop, exp2) ->
            (RsBinop (
                (transform_exp ct exp1),
                binop,
                (transform_exp ct exp2)))
        | RsAs (exp, typ) ->
            (RsAs (
                (transform_exp ct exp),
                (transform_type ct typ)))
        | RsTodo -> RsTodo

and transform_app (ct: custom_transform) (fn: rs_exp) (args: rs_exp list) : rs_exp =
    let args = List.map (transform_exp ct) args in
    match (fn, args) with
        (* Built-in elementary operations *)
        | (RsId "plain_vector_access", [vector; item]) -> (RsIndex (vector, item))
        | (RsId "neq_int", [left; right]) -> (RsBinop (left, RsBinopNeq, right))
        | (RsId "neq_bits", [left; right]) -> (RsBinop (left, RsBinopNeq, right))
        | (RsId "eq_int", [left; right]) -> (RsBinop (left, RsBinopEq, right))
        | (RsId "eq_bool", [left; right]) -> (RsBinop (left, RsBinopEq, right))
        | (RsId "eq_bits", [left; right]) -> (RsBinop (left, RsBinopEq, right))
        | (RsId "eq_anything", [left; right]) -> (RsBinop (left, RsBinopEq, right))
        | (RsId "or_vec", [left; right]) -> (RsBinop (left, RsBinopOr, right))
        | (RsId "and_vec", [left; right]) -> (RsBinop (left, RsBinopAnd, right))
        | (RsId "xor_vec", [left; right]) -> (RsBinop (left, RsBinopXor, right))
        | (RsId "add_bits", [left; right]) -> (RsMethodApp (left, "wrapped_add", [right]))
        | (RsId "and_bool", [left; right]) -> (RsBinop (left, RsBinopLAnd, right))
        | (RsId "or_bool", [left; right]) -> (RsBinop (left, RsBinopLOr, right))

        (* Custom RISC-V bit extension functions *)
        | (RsId "EXTZ", (RsLit (RsLitNum n))::value::[]) ->
            (match n with
                | 8L -> RsAs (value, RsTypId "u8")
                | 16L -> RsAs (value, RsTypId "u16")
                | 32L -> RsAs (value, RsTypId "u32")
                | 64L -> RsAs (value, RsTypId "u64")
                | _ -> RsAs (value, RsTypId "InvalidUSigned")
            )
        (* Unsigned is used for array indexing *)
        | (RsId "unsigned", value::[]) -> RsAs (value, RsTypId "usize")

        (* Otherwise keep as is *)
        | _ -> (RsApp (fn, args))

and transform_pexp (ct: custom_transform) (pexp: rs_pexp) : rs_pexp =
    match pexp with
        | RsPexp (pat, exp) ->
            (RsPexp (
                (transform_pat ct pat),
                (transform_exp ct exp)))
        | RsPexpWhen (pat, exp1, exp2) ->
            (RsPexpWhen (
                (transform_pat ct pat),
                (transform_exp ct exp1),
                (transform_exp ct exp2)))
 
(* ———————————————————————————— Transform Types ————————————————————————————— *)
 
and bitvector_to_uint (n: int) : rs_type =
    if n <= 8 then
        RsTypId "u8"
    else if n <= 16 then
        RsTypId "u16"
    else if n <= 32 then
        RsTypId "u32"
    else if n <= 64 then
        RsTypId "u64"
    else
        RsTypId "InvalidBitVectorSize"

and transform_type (ct: custom_transform) (typ: rs_type) : rs_type =
    match typ with
        | RsTypId "unit" -> RsTypUnit
        | RsTypGenericParam ("bitvector", [RsTypParamNum n]) -> bitvector_to_uint n
        | RsTypGenericParam ("bits", [RsTypParamNum n]) -> bitvector_to_uint n

        (* TODO: once we resolve type aliasing we can remove those manual conversions *)
        | RsTypId "regbits" -> RsTypId "u8"

        (* Otherwise keep as is *)
        | _ -> typ

(* ———————————————————————— Transform Rust Programs ————————————————————————— *)

let transform_fn (ct: custom_transform) (fn: rs_fn) : rs_fn =
    let (args, ret) = fn.signature in
    let args = List.map (transform_type ct) args in
    let ret = transform_type ct ret in
    {
        name = fn.name;
        args = fn.args;
        signature = (args, ret);
        body = transform_exp ct fn.body;
    }

let rust_transform (ct: custom_transform) (RsProg fns) : rs_program =
    RsProg (List.map (transform_fn ct) fns)
