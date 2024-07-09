(** Rust Transformations **)
(** This module transforms raw Tust code generated from Sail into a valid Rust module. **)

open Rust_gen

(* ————————————————————————— Transform Expressions —————————————————————————— *)

let rec transform_pat (pat: rs_pat) : rs_pat =
    pat

and transform_lexp (lexp: rs_lexp) : rs_lexp =
    match lexp with
        | RsLexpId id -> RsLexpId id
        | RsLexpField (lexp, id) ->
            (RsLexpField (
                (transform_lexp lexp),
                id))
        | RsLexpIndex (lexp, exp) ->
            (RsLexpIndex (
                (transform_lexp lexp),
                (transform_exp exp)))
        | RsLexpIndexRange (lexp, range_start, range_end) ->
            (RsLexpIndexRange (
                (transform_lexp lexp),
                (transform_exp range_start),
                (transform_exp range_end)))
        | RsLexpTodo -> RsLexpTodo

and transform_exp (exp: rs_exp) : rs_exp =
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
        | RsField (exp, field) -> RsField ((transform_exp exp), field)
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
        | RsAs (exp, typ) ->
            (RsAs (
                (transform_exp exp),
                (transform_type typ)))
        | RsTodo -> RsTodo

and transform_app (fn: string) (args: rs_exp list) : rs_exp =
    let args = List.map transform_exp args in
    match (fn, args) with
        (* Built-in elementary operations *)
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

        (* Custom RISC-V bit extension functions *)
        | ("EXTZ", (RsLit (RsLitNum n))::value::[]) ->
            (match n with
                | 8L -> RsAs (value, RsTypId "u8")
                | 16L -> RsAs (value, RsTypId "u16")
                | 32L -> RsAs (value, RsTypId "u32")
                | 64L -> RsAs (value, RsTypId "u64")
                | _ -> RsAs (value, RsTypId "InvalidUSigned")
            )
        (* Unsigned is used for array indexing *)
        | ("unsigned", value::[]) -> RsAs (value, RsTypId "usize")

        (* Otherwise keep as is *)
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

and transform_type (typ: rs_type) : rs_type =
    match typ with
        | RsTypId "unit" -> RsTypUnit
        | RsTypGenericParam ("bitvector", [RsTypParamNum n]) -> bitvector_to_uint n
        | RsTypGenericParam ("bits", [RsTypParamNum n]) -> bitvector_to_uint n

        (* TODO: once we resolve type aliasing we can remove those manual conversions *)
        | RsTypId "regbits" -> RsTypId "u8"

        (* Otherwise keep as is *)
        | _ -> typ

(* ———————————————————————— Transform Rust Programs ————————————————————————— *)

let transform_fn (fn: rs_fn) : rs_fn =
    let (args, ret) = fn.signature in
    let args = List.map transform_type args in
    let ret = transform_type ret in
    {
        name = fn.name;
        args = fn.args;
        signature = (args, ret);
        body = transform_exp fn.body;
    }

let rust_transform (RsProg fns) : rs_program =
    RsProg (List.map transform_fn fns)
