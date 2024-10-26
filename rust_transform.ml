(** Rust Transformations **)
(** This module transforms raw Tust code generated from Sail into a valid Rust module. **)

open Rust_gen

module StringSet = Set.Make(String)


(* ——————————————————————————— BitVec transformation ———————————————————————————— *)

type expr_type_transform = {
    exp : rs_exp -> rs_exp;
    lexp : rs_lexp -> rs_lexp;
    pexp : rs_pexp -> rs_pexp;
    typ : rs_type -> rs_type;
}

let lexp_to_exp (lexp: rs_lexp) : rs_exp =
    match lexp with
        | RsLexpId id -> RsId id
        | _ -> RsId "LexpToExpTodo" 

let is_bitvec_lit (pexp: rs_pexp) : bool =
    match pexp with
        | RsPexp (RsPatLit RsLitHex _, _) -> true
        | RsPexp (RsPatLit RsLitBin _, _) -> true
        | _ -> false

let bitvec_transform_lexp (lexp: rs_lexp) : rs_lexp = lexp

let bitvec_transform_exp (exp: rs_exp) : rs_exp =
    match exp with
        | RsApp (RsId "subrange_bits", [RsField (bitvec, "bits"); RsLit RsLitNum r_end; RsLit RsLitNum r_start]) ->
            let r_end = Int64.add r_end Int64.one in
            let subrange_call =
                Printf.sprintf "subrange::<%Lu, %Lu, %Lu>"
                    r_start
                    r_end
                    (Int64.sub r_end r_start)
            in
            RsApp (RsField (bitvec, subrange_call), [])
        | RsAssign (RsLexpIndexRange (RsLexpField (lexp, "bits"), RsLit RsLitNum r_end, RsLit RsLitNum r_start), exp) ->
            let r_end = Int64.add r_end Int64.one in
            let set_subrange =
                Printf.sprintf "set_subrange::<%Lu, %Lu, %Lu>"
                    r_start
                    r_end
                    (Int64.sub r_end r_start)
            in
            RsAssign (lexp, RsMethodApp (lexp_to_exp lexp, set_subrange, [exp]))
        | RsLit (RsLitBin n) ->
            let bitvec = Printf.sprintf "BitVector::new(%s)" n in
            RsId bitvec
        | RsLit (RsLitHex n) ->
            let bitvec = Printf.sprintf "BitVector::new(%s)" n in
            RsId bitvec
        | RsMatch (exp, pat::pats) when is_bitvec_lit pat ->
            RsMatch (RsMethodApp (exp, "bits", []), pat::pats)
        | _ -> exp

and uint_to_bitvector (n: int) : rs_type =
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

let rec bitvec_transform_pexp (pexp: rs_pexp) : rs_pexp = pexp

let rec bitvec_transform_type (typ: rs_type) : rs_type =
    match typ with
        | RsTypGenericParam ("bitvector", t) -> RsTypGenericParam ("BitVector", t)
        | RsTypGenericParam ("bits", t) -> RsTypGenericParam ("BitVector", t)
        
        (* TODO: once we resolve type aliasing we can remove those manual conversions *)
        | RsTypId "regbits" -> RsTypGenericParam ("BitVector", [RsTypParamNum 5])

        (* Otherwise keep as is *)
        | _ -> typ

let bitvec_transform = {
    exp = bitvec_transform_exp;
    lexp = bitvec_transform_lexp;
    pexp = bitvec_transform_pexp;
    typ = bitvec_transform_type;
}

(* ——————————————————————————— Nested Blocks remover ———————————————————————————— *)


let nested_block_remover_lexp (lexp: rs_lexp) : rs_lexp = lexp

let nested_block_remover_exp (exp: rs_exp) : rs_exp = 
    match exp with
        | RsIf (c, RsBlock e1, RsBlock e2) -> RsIf (c, RsInstrList e1, RsInstrList e2)
        | RsIf (c, RsBlock e1, e2) -> RsIf (c, RsInstrList e1, e2)
        | RsIf (c, e1, RsBlock e2) -> RsIf (c, e1, RsInstrList e2)
        | _ -> exp 

let rec nested_block_remover_pexp (pexp: rs_pexp) : rs_pexp = pexp

let rec nested_block_remover_type (typ: rs_type) : rs_type = typ

let nested_block_remover = {
    exp = nested_block_remover_exp;
    lexp = nested_block_remover_lexp;
    pexp = nested_block_remover_pexp;
    typ = nested_block_remover_type;
}


(* ——————————————————————————— Native functions transformation ———————————————————————————— *)

(* TODO: This list is probably incomplete and we might want to add extra fields in the future *)
let native_func_transform_exp (exp : rs_exp) : rs_exp = 
    match exp with
    | RsApp (RsId "add_atom", [b1;b2]) -> RsBinop(b1,RsBinopAdd,b2)
    | RsApp (RsId "sub_atom", [b1;b2]) -> RsBinop(b1,RsBinopSub,b2)
    | RsApp (RsId "negate_atom", _) -> RsId "BUILTIN_atom_negate_TODO" 
    | RsApp (RsId "mult_atom", _) -> RsId "BUILTIN_atommult_TODO" 
    | RsApp (RsId "ediv_int", _) -> RsId "BUILTIN_atom_ediv_TODO" 
    | RsApp (RsId "emod_int", _) -> RsId "BUILTIN_atom_emod_int_TODO" 
    | RsApp (RsId "abs_int_atom", _) -> RsId "BUILTIN_atom_abs_int_TODO" 
    | RsApp (RsId "not_vec", [v]) -> RsUnop(RsUnopNeg, v)
    | RsApp (RsId "eq_bit", _) -> RsId "BUILTIN_eq_bit_TODO"
    | RsApp (RsId "eq_bool", _) -> RsId "BUILTIN_eq_bool_TODO"
    | RsApp (RsId "eq_string", _) -> RsId "BUILTIN_eq_string_TODO"
    | RsApp (RsId "eq_int", _) -> RsId "BUILTIN_eq_int_TODO"
    | RsApp (RsId "not", [b]) -> RsUnop(RsUnopNeg, b)
    | RsApp (RsId "lt", _) -> RsId "BUILTIN_lt_TODO"
    | RsApp (RsId "lteq", _) -> RsId "BUILTIN_lteq_TODO"
    | RsApp (RsId "gt", _) -> RsId "BUILTIN_gt_TODO"
    | RsApp (RsId "gteq", _) -> RsId "BUILTIN_gteq_TODO"
    | RsApp (RsId "add_int", _) -> RsId "BUILTIN_add_int_TODO"
    | RsApp (RsId "sub_int", _) -> RsId "BUILTIN_sub_int_TODO"
    | RsApp (RsId "mult_int", _) -> RsId "BUILTIN_mult_int_TODO"
    | RsApp (RsId "neg_int", _) -> RsId "BUILTIN_neg_int_TODO"
    | RsApp (RsId "abs_int", _) -> RsId "BUILTIN_abs_int_TODO"
    | RsApp (RsId "max_int", _) -> RsId "BUILTIN_max_int_TODO"
    | RsApp (RsId "min_int", _) -> RsId "BUILTIN_min_int_TODO"
    | RsApp (RsId "tdiv_int", _) -> RsId "BUILTIN_tdiv_int_TODO"
    | RsApp (RsId "tmod_int", _) -> RsId "BUILTIN_tmod_int_TODO"
    | RsApp (RsId "pow2", _) -> RsId "BUILTIN_pow2_TODO"
    | RsApp (RsId "zeros", _) -> RsId "BUILTIN_zeros_TODO"
    | RsApp (RsId "ones", _) -> RsId "BUILTIN_ones_TODO"
    | RsApp (RsId "zero_extend", _) -> RsId "BUILTIN_zero_extend_TODO"
    | RsApp (RsId "sign_extend", _) -> RsId "BUILTIN_sign_extend_TODO"
    | RsApp (RsId "sail_signed", _) -> RsId "BUILTIN_sail_signed_TODO"
    | RsApp (RsId "sail_unsigned", _) -> RsId "BUILTIN_sail_unsigned_TODO"
    | RsApp (RsId "slice", _) -> RsId "BUILTIN_slice_TODO"
    | RsApp (RsId "slice_inc", _) -> RsId "BUILTIN_slice_inc_TODO"
    | RsApp (RsId "add_bits", _) -> RsId "BUILTIN_add_bits_TODO"
    | RsApp (RsId "add_bits_int", [b1;b2]) -> RsBinop(b1, RsBinopAdd, b2)
    | RsApp (RsId "sub_bits", _) -> RsId "BUILTIN_sub_bits_TODO"
    | RsApp (RsId "sub_bits_int", _) -> RsId "BUILTIN_sub_bits_int_TODO"
    | RsApp (RsId "append", _) -> RsId "BUILTIN_append_TODO"
    | RsApp (RsId "get_slice_int", _) -> RsId "BUILTIN_get_slice_int_TODO"
    | RsApp (RsId "eq_bits", _) -> RsId "BUILTIN_eq_bits_TODO"
    | RsApp (RsId "neq_bits", _) -> RsId "BUILTIN_neq_bits_TODO"
    | RsApp (RsId "not_bits", _) -> RsId "BUILTIN_not_bits_TODO"
    | RsApp (RsId "sail_truncate", _) -> RsId "BUILTIN_sail_truncate_TODO"
    | RsApp (RsId "sail_truncateLSB", _) -> RsId "BUILTIN_sail_truncateLSB_TODO"
    | RsApp (RsId "shiftl", _) -> RsId "BUILTIN_shiftl_TODO"
    | RsApp (RsId "shiftr", _) -> RsId "BUILTIN_shiftr_TODO"
    | RsApp (RsId "arith_shiftr", _) -> RsId "BUILTIN_arith_shiftr_TODO"
    | RsApp (RsId "and_bits", _) -> RsId "BUILTIN_and_bits_TODO"
    | RsApp (RsId "or_bits", _) -> RsId "BUILTIN_or_bits_TODO"
    | RsApp (RsId "xor_bits", _) -> RsId "BUILTIN_xor_bits_TODO"
    | RsApp (RsId "vector_init", _) -> RsId "BUILTIN_vector_init_TODO"
    | RsApp (RsId "vector_access", _) -> RsId "BUILTIN_vector_access_TODO"
    | RsApp (RsId "vector_access_inc", _) -> RsId "BUILTIN_vector_access_inc_TODO"
    | RsApp (RsId "vector_subrange", _) -> RsId "BUILTIN_vector_subrange_TODO"
    | RsApp (RsId "vector_subrange_inc", _) -> RsId "BUILTIN_vector_subrange_inc_TODO"
    | RsApp (RsId "vector_update", _) -> RsId "BUILTIN_vector_update_TODO"
    | RsApp (RsId "vector_update_inc", _) -> RsId "BUILTIN_vector_update_inc_TODO"
    | RsApp (RsId "vector_update_subrange", _) -> RsId "BUILTIN_vector_update_subrange_TODO"
    | RsApp (RsId "vector_update_subrange_inc", _) -> RsId "BUILTIN_vector_update_subrange_inc_TODO"
    | RsApp (RsId "length", _) -> RsId "BUILTIN_length_TODO"
    | RsApp (RsId "replicate_bits", _) -> RsId "BUILTIN_replicate_bits_TODO"
    | RsApp (RsId "count_leading_zeros", _) -> RsId "BUILTIN_count_leading_zeros_TODO"
    | RsApp (RsId "eq_real", _) -> RsId "BUILTIN_eq_real_TODO"
    | RsApp (RsId "neg_real", _) -> RsId "BUILTIN_neg_real_TODO"
    | RsApp (RsId "add_real", _) -> RsId "BUILTIN_add_real_TODO"
    | RsApp (RsId "sub_real", _) -> RsId "BUILTIN_sub_real_TODO"
    | RsApp (RsId "mult_real", _) -> RsId "BUILTIN_mult_real_TODO"
    | RsApp (RsId "div_real", _) -> RsId "BUILTIN_div_real_TODO"
    | RsApp (RsId "lt_real", _) -> RsId "BUILTIN_lt_real_TODO"
    | RsApp (RsId "gt_real", _) -> RsId "BUILTIN_gt_real_TODO"
    | RsApp (RsId "lteq_real", _) -> RsId "BUILTIN_lteq_real_TODO"
    | RsApp (RsId "gteq_real", _) -> RsId "BUILTIN_gteq_real_TODO"
    | RsApp (RsId "concat_str", _) -> RsId "BUILTIN_concat_str_TODO"
    | RsApp (RsId "print_bits", _) -> RsId "BUILTIN_print_bits_TODO"
    | RsApp (RsId "string_of_bits", _) -> RsId "BUILTIN_string_of_bits_TODO"
    | RsApp (RsId "dec_str", _) -> RsId "BUILTIN_dec_str_TODO"
    | RsApp (RsId "hex_str", _) -> RsId "BUILTIN_hex_str_TODO"
    | RsApp (RsId "hex_str_upper", _) -> RsId "BUILTIN_hex_str_upper_TODO"
    | RsApp (RsId "sail_assert", _) -> RsId "BUILTIN_sail_assert_TODO"
    | RsApp (RsId "reg_deref", _) -> RsId "BUILTIN_reg_deref_TODO"
    | RsApp (RsId "sail_cons", _) -> RsId "BUILTIN_sail_cons_TODO"
    | RsApp (RsId "eq_anything", _) -> RsId "BUILTIN_eq_anything_TODO"
    | RsApp (RsId "id", _) -> RsId "BUILTIN_id_TODO"
    | RsApp (RsId "gteq_int", [e1; e2]) ->  RsBinop (e1, RsBinopGe, e2)
    | RsApp (RsId "lt_int", [e1; e2]) -> RsBinop (e1, RsBinopLt, e2)
    | _ -> exp
  

let native_func_transform_lexp (lexp: rs_lexp) : rs_lexp = lexp 

let native_func_transform_pexp (pexp: rs_pexp) : rs_pexp = pexp

let native_func_transform_type (typ: rs_type) : rs_type = typ

let native_func_transform = {
    exp = native_func_transform_exp;
    lexp = native_func_transform_lexp;
    pexp = native_func_transform_pexp;
    typ = native_func_transform_type;
}

(* ————————————————————————— Transform Expressions —————————————————————————— *)

let rec transform_pat (ct: expr_type_transform) (pat: rs_pat): rs_pat =
    pat

and transform_lexp (ct: expr_type_transform) (lexp: rs_lexp): rs_lexp =
    let lexp = ct.lexp lexp in
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

and transform_exp (ct: expr_type_transform) (exp: rs_exp) : rs_exp =
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
        | RsInstrList exps -> RsInstrList (List.map (transform_exp ct) exps)
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
        | RsUnop (unop, exp) -> RsUnop(unop, transform_exp ct exp)
        | RsAs (exp, typ) ->
            (RsAs (
                (transform_exp ct exp),
                (transform_type ct typ)))
        | RsSome(exp) -> RsSome (transform_exp ct exp)
        | RsNone -> RsNone
        | RsTodo -> RsTodo

and transform_app (ct: expr_type_transform) (fn: rs_exp) (args: rs_exp list) : rs_exp =
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
        | (RsId "neq_anything", [left; right]) -> (RsBinop (left, RsBinopNeq, right))
        | (RsId "or_vec", [left; right]) -> (RsBinop (left, RsBinopOr, right))
        | (RsId "and_vec", [left; right]) -> (RsBinop (left, RsBinopAnd, right))
        | (RsId "xor_vec", [left; right]) -> (RsBinop (left, RsBinopXor, right))
        | (RsId "add_bits", [left; right]) -> (RsMethodApp (left, "wrapped_add", [right]))
        | (RsId "and_bool", [left; right]) -> (RsBinop (left, RsBinopLAnd, right))
        | (RsId "or_bool", [left; right]) -> (RsBinop (left, RsBinopLOr, right))
        | (RsId "Some", [exp]) -> RsSome(exp)
        | (RsId "None", _) -> RsNone 

        (* Custom RISC-V bit extension functions *)
        | (RsId "EXTZ", (RsLit (RsLitNum n))::value::[]) ->
            (match n with
                | 8L -> RsAs (value, RsTypId "BitVector<8>")
                | 16L -> RsAs (value, RsTypId "BitVector<16>")
                | 32L -> RsAs (value, RsTypId "BitVector<32>")
                | 64L -> RsAs (value, RsTypId "BitVector<64>")
                | _ -> RsAs (value, RsTypId "InvalidUSigned")
            )
        (* Unsigned is used for array indexing *)
        | (RsId "unsigned", value::[]) -> RsMethodApp (value, "as_usize",[])

        (* Otherwise keep as is *)
        | _ -> (RsApp (fn, args))

and transform_pexp (ct: expr_type_transform) (pexp: rs_pexp) : rs_pexp =
    let pexp = ct.pexp pexp in
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
 
and transform_type_param (ct: expr_type_transform) (param: rs_type_param) : rs_type_param =
    match param with
        | RsTypParamTyp typ -> RsTypParamTyp (transform_type ct typ)
        | RsTypParamNum n -> RsTypParamNum n

and transform_type (ct: expr_type_transform) (typ: rs_type) : rs_type =
    let typ = ct.typ typ in
    match typ with
        | RsTypId "unit" -> RsTypUnit
        | RsTypId id -> RsTypId id
        | RsTypUnit -> RsTypUnit
        | RsTypTodo -> RsTypTodo
        | RsTypTuple types -> RsTypTuple (List.map (transform_type ct) types)
        | RsTypGeneric typ -> RsTypGeneric typ
        | RsTypGenericParam (typ, params) -> RsTypGenericParam (typ, (List.map (transform_type_param ct) params))
        | RsTypArray (typ, size) -> RsTypArray (transform_type_param ct typ, transform_type_param ct size)
        

(* ———————————————————————— Expression and Type transformer ————————————————————————— *)

let transform_fn (ct: expr_type_transform) (fn: rs_fn) : rs_fn =
    let (args, ret) = fn.signature in
    let args = List.map (transform_type ct) args in
    let ret = transform_type ct ret in
    {
        name = fn.name;
        args = fn.args;
        signature = (args, ret);
        body = transform_exp ct fn.body;
    }

let transform_alias (ct: expr_type_transform) (alias: rs_alias) : rs_alias = {
        new_typ = alias.new_typ; 
        old_type = transform_type ct alias.old_type
    }

let transform_obj (ct: expr_type_transform) (obj: rs_obj) : rs_obj = 
    match obj with
        | RsFn fn -> RsFn (transform_fn ct fn)
        | RsAlias alias -> RsAlias (transform_alias ct alias)
        | _ -> obj

let rust_transform_expr (ct: expr_type_transform) (RsProg objs) : rs_program =
    RsProg (List.map (transform_obj ct) objs)


(* ———————————————————————— Function transformer ————————————————————————— *)

type func_transform = {
    func : rs_fn -> rs_fn
}

let transform_obj_func (ct: func_transform) (obj: rs_obj) : rs_obj =
    match obj with
    | RsFn fn -> RsFn (ct.func fn) 
    | _ -> obj

let rust_transform_func (ct: func_transform) (RsProg objs) : rs_program =
    RsProg (List.map (transform_obj_func ct) objs)

(* ———————————————————————— type bits = bitvector filter  ————————————————————————— *)

let filter_bits_bitvector_alias (obj: rs_obj) : rs_program = 
    match obj with
        | RsAlias { new_typ = "bits"; _ } -> RsProg []
        | _ -> RsProg[obj]

let rust_remove_type_bits (RsProg objs) : rs_program =  merge_rs_prog_list (List.map (filter_bits_bitvector_alias) objs)

(* ———————————————————————— prelude_func_filter  ————————————————————————— *)

let prelude_func: StringSet.t = StringSet.of_list (["EXTZ";"EXTS";"not"; "plain_vector_access"; "neq_int"; "neq_bits"; "eq_int"; "eq_bool"; "eq_bits"; "eq_anything"; "neq_anything"; "or_vec"; "and_vec"; "xor_vec"; "add_bits"; "and_bool"; "or_bool"])

let rust_prelude_func_filter_alias (obj: rs_obj) : rs_program = 
    match obj with
        | RsFn {name;_ } when (StringSet.mem name prelude_func)-> RsProg []
        | _ -> RsProg[obj]

let rust_prelude_func_filter (RsProg objs) : rs_program =  merge_rs_prog_list (List.map (rust_prelude_func_filter_alias) objs)

(* ———————————————————————— Operator rewriter function side  ————————————————————————— *)

open Str

let remove_illegal_operator_char str =
  let str = global_replace (regexp "=") "equal" str in
  let str = global_replace (regexp "<") "smaller" str in
  let str = global_replace (regexp ">") "bigger" str in
  let str = global_replace (regexp "(") "_" str in
  let str = global_replace (regexp ")") "_" str in
  let str = global_replace (regexp " ") "_" str in
  str  (* Return the final modified string *)

let operator_rewriter_func (func: rs_fn): rs_fn = {
  name = remove_illegal_operator_char func.name;  (* Use `func` instead of `rs_fn` *)
  args = func.args;
  signature = func.signature;
  body = func.body;
}

let operator_rewriter = {
  func = operator_rewriter_func
}

(* ———————————————————————— Operator rewriter caller side  ————————————————————————— *)

let expr_operator_rewriter (exp: rs_exp) : rs_exp = 
    match exp with
        | RsApp (RsId id, args) -> RsApp(RsId(remove_illegal_operator_char id), args)
        | _ -> exp

let lexpr_operator_rewriter (lexp: rs_lexp) : rs_lexp = lexp

let pexpr_operator_rewriter (pexp: rs_pexp) : rs_pexp = pexp 

let type_operator_rewriter (typ: rs_type) : rs_type = typ 

let expr_type_operator_rewriter = {
    exp = expr_operator_rewriter;
    lexp = lexpr_operator_rewriter;
    pexp = pexpr_operator_rewriter;
    typ = type_operator_rewriter;
}