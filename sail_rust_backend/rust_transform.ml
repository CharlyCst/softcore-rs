(** Rust Transformations **)
(** This module transforms raw Rust code generated from Sail into a valid Rust module. **)

open Context
open Rust_gen
open Libsail

module SSet = Call_set.SSet
module SMap = Call_set.SMap
module Big_int = Libsail.Ast_util.Big_int

(* ————————————————————————— List of external expressions —————————————————————————— *)

let external_func: SSet.t = SSet.of_list (["subrange_bits";"not_implemented"; "print_output"; "format!"; "assert!"; "panic!"; "dec_str"; "hex_str"; "update_subrange_bits"; "zero_extend_16"; "zero_extend_63";"zero_extend_64";"sign_extend"; "sail_ones"; "min_int"; "__exit"; "signed"; "lteq_int"; "sail_branch_announce"; "bitvector_length"; "bits_str"; "print_reg"; "bitvector_access"; "get_16_random_bits"; "sys_enable_writable_fiom"; "bitvector_concat"; "print_platform"; "cancel_reservation"; "sys_enable_writable_misa"; "sys_enable_rvc"; "sys_enable_fdext"; "plat_mtval_has_illegal_inst_bits"; "truncate"; "subrange_bits"; "sys_enable_zfinx"; "gt_int"; "internal_error"; "bitvector_update"; "hex_bits_12_forwards"; "hex_bits_12_backwards" ; "sail_zeros"; "parse_hex_bits"; "get_slice_int"])

(* ————————————————————————— Transform Expressions —————————————————————————— *)

let id_exp (ctx: context) (exp: rs_exp) : rs_exp = exp

let id_lexp (ctx: context) (lexp: rs_lexp) : rs_lexp = lexp

let id_pexp (ctx: context) (pexp: rs_pexp) : rs_pexp = pexp

let id_typ (ctx: context) (typ: rs_type) : rs_type = typ

let id_pat (ctx: context) (pat: rs_pat) : rs_pat = pat

let id_obj (ctx: context) (obj: rs_obj) : rs_obj = obj

type expr_type_transform = {
    exp : context -> rs_exp -> rs_exp;
    lexp : context -> rs_lexp -> rs_lexp;
    pexp : context -> rs_pexp -> rs_pexp;
    typ : context -> rs_type -> rs_type;
    pat: context -> rs_pat -> rs_pat;
    obj: context -> rs_obj -> rs_obj;
}

let rec transform_pat (ct: expr_type_transform) (ctx: context) (pat: rs_pat): rs_pat = 
    let pat = ct.pat ctx pat in 
    match pat with 
        | RsPatId "None" -> RsPatNone
        | RsPatApp (RsPatId "Some", args) -> (
            match args with
                | arg :: [] -> RsPatSome(transform_pat ct ctx arg) 
                | _ -> failwith "This case should be unreachable, revise assumption"
        )
        | RsPatType (typ, pat) -> RsPatType(transform_type ct ctx typ, transform_pat ct ctx pat)
        | RsPatTuple (pat_list) -> RsPatTuple(List.map (transform_pat ct ctx) pat_list)
        | RsPatWildcard -> RsPatWildcard
        | RsPatLit l -> RsPatLit l
        | RsPatId id -> RsPatId id
        | RsPatApp (RsPatId "None", args) -> RsPatNone
        | RsPatApp (name, args) -> RsPatApp(transform_pat ct ctx name, List.map (fun p -> transform_pat ct ctx p) args)
        | RsPatSome pat -> RsPatSome (transform_pat ct ctx pat) 
        | RsPatNone -> RsPatNone
        | RsPatTodo text -> RsPatTodo text

and transform_lexp (ct: expr_type_transform) (ctx: context) (lexp: rs_lexp): rs_lexp =
    let lexp = ct.lexp ctx lexp in
    match lexp with
        | RsLexpId id -> RsLexpId id
        | RsLexpField (lexp, id) ->
            (RsLexpField (
                (transform_exp ct ctx lexp),
                id))
        | RsLexpIndex (lexp, exp) ->
            (RsLexpIndex (
                (transform_lexp ct ctx lexp),
                (transform_exp ct ctx exp)))
        | RsLexpIndexRange (lexp, range_start, range_end) ->
            (RsLexpIndexRange (
                (transform_lexp ct ctx lexp),
                (transform_exp ct ctx range_start),
                (transform_exp ct ctx range_end)))
        | RsLexpTodo -> RsLexpTodo

and transform_exp (ct: expr_type_transform) (ctx: context) (exp: rs_exp) : rs_exp =
    let exp = ct.exp ctx exp in
    match exp with
        | RsLet (pat, exp, next) ->
            (RsLet (
                (transform_pat ct ctx pat),
                (transform_exp ct ctx exp),
                (transform_exp ct ctx next)))
        | RsApp (app, generics, args) -> transform_app ct ctx app generics args
        | RsStaticApp (app, method_name, args) -> RsStaticApp(transform_type ct ctx app, method_name, (List.map (transform_exp ct ctx) args))
        | RsMethodApp {exp; name; generics; args} ->
            (RsMethodApp {
                exp = transform_exp ct ctx exp;
                name = name;
                generics = generics;
                args = List.map (transform_exp ct ctx) args
            })
        | RsId id -> RsId id
        | RsLit lit -> RsLit lit
        | RsField (exp, field) -> RsField ((transform_exp ct ctx exp), field)
        | RsBlock exps -> RsBlock (List.map (transform_exp ct ctx) exps)
        | RsConstBlock exps -> RsConstBlock (List.map (transform_exp ct ctx) exps)
        | RsInstrList exps -> RsInstrList (List.map (transform_exp ct ctx) exps)
        | RsIf (cond, exp_true, exp_false) ->
            (RsIf (
                (transform_exp ct ctx cond),
                (transform_exp ct ctx exp_true),
                (transform_exp ct ctx exp_false)))
        | RsMatch (exp, pexps) ->
            (RsMatch (
                (transform_exp ct ctx exp),
                (List.map (transform_pexp ct ctx) pexps)))
        | RsTuple exps ->
            (RsTuple
                (List.map (transform_exp ct ctx) exps))
        | RsArray exps ->
            (RsArray
                (List.map (transform_exp ct ctx) exps))
        | RsAssign (lexp, exp) ->
            (RsAssign (
                (transform_lexp ct ctx lexp),
                (transform_exp ct ctx exp)))
        | RsIndex (exp1, exp2) ->
            (RsIndex (
                (transform_exp ct ctx exp1),
                (transform_exp ct ctx exp2)))
        | RsBinop (exp1, binop, exp2) ->
            (RsBinop (
                (transform_exp ct ctx exp1),
                binop,
                (transform_exp ct ctx exp2)))
        | RsUnop (unop, exp) -> RsUnop(unop, transform_exp ct ctx exp)
        | RsAs (exp, typ) ->
            (RsAs (
                (transform_exp ct ctx exp),
                (transform_type ct ctx typ)))
        | RsSome(exp) -> RsSome (transform_exp ct ctx exp)
        | RsNone -> RsNone
        | RsPathSeparator (t1, t2) -> RsPathSeparator (t1,t2)
        | RsFor (var, start, until, body) ->  RsFor (var, start, until, transform_exp ct ctx body)
        | RsStruct (typ, entries) -> RsStruct(transform_type ct ctx typ, List.map (fun (s,e) -> (s, transform_exp ct ctx e)) entries)
        | RsStructAssign (e1, name, e2) -> RsStructAssign(transform_exp ct ctx e1, name, transform_exp ct ctx e2)
        | RsReturn exp -> RsReturn (transform_exp ct ctx exp)
        | RsTodo str -> RsTodo str

and transform_app (ct: expr_type_transform) (ctx: context) (fn: rs_exp) (generics: string list) (args: rs_exp list) : rs_exp =
    let args = List.map (transform_exp ct ctx) args in
    match (fn, args) with
        (* Built-in elementary operations *)
        | (RsId "plain_vector_access", [vector; item]) -> (RsIndex (vector, RsAs (item, usize_typ)))
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
        | (RsId "add_bits", [left; right]) -> (mk_method_app left "wrapped_add" [right])
        | (RsId "and_bool", [left; right]) -> (RsBinop (left, RsBinopLAnd, right))
        | (RsId "or_bool", [left; right]) -> (RsBinop (left, RsBinopLOr, right))
        | (RsId "Some", [exp]) -> RsSome(exp)
        | (RsId "None", _) -> RsNone 

        (* Unsigned is used for array indexing *)
        | (RsId "unsigned", value::[]) -> mk_method_app value "as_usize" []

        (* Otherwise keep as is *)
        | _ -> (RsApp (fn, generics, args))

and transform_pexp (ct: expr_type_transform) (ctx: context) (pexp: rs_pexp) : rs_pexp =
    let pexp = ct.pexp ctx pexp in
    match pexp with
        | RsPexp (pat, exp) ->
            (RsPexp (
                (transform_pat ct ctx pat),
                (transform_exp ct ctx exp)))
        | RsPexpWhen (pat, exp1, exp2) ->
            (RsPexpWhen (
                (transform_pat ct ctx pat),
                (transform_exp ct ctx exp1),
                (transform_exp ct ctx exp2)))
 
(* ———————————————————————————— Transform Types ————————————————————————————— *)
 
and transform_type_param (ct: expr_type_transform) (ctx: context) (param: rs_type_param) : rs_type_param =
    match param with
        | RsTypParamTyp typ -> RsTypParamTyp (transform_type ct ctx typ)
        | RsTypParamNum n -> RsTypParamNum n

and transform_type (ct: expr_type_transform) (ctx: context) (typ: rs_type) : rs_type =
    let typ = ct.typ ctx typ in
    match typ with
        | RsTypId "unit" -> RsTypUnit
        | RsTypId id -> RsTypId id
        | RsTypUnit -> RsTypUnit
        | RsTypTodo e -> RsTypTodo e
        | RsTypTuple types -> RsTypTuple (List.map (transform_type ct ctx) types)
        | RsTypGeneric typ -> RsTypGeneric typ
        (* TODO: Maybe there is a bug here *)
        | RsTypGenericParam (typ, e::params) when typ = "option" -> RsTypOption (transform_type_param ct ctx e)
        | RsTypGenericParam (typ, params) -> RsTypGenericParam (typ, (List.map (transform_type_param ct ctx) params))
        | RsTypArray (typ, size) -> RsTypArray (transform_type_param ct ctx typ, transform_type_param ct ctx size)
        | RsTypOption param -> RsTypOption (transform_type_param ct ctx param)
        

(* ———————————————————————— Expression and Type transformer ————————————————————————— *)

let transform_fn (ct: expr_type_transform) (ctx: context) (fn: rs_fn) : rs_fn =
    let {generics; args; ret} = fn.signature in
    let args = List.map (transform_type ct ctx) args in
    let ret = transform_type ct ctx ret in
    {
        fn with
        args = (List.map (ct.pat ctx) fn.args);
        signature = {generics; args; ret};
        body = transform_exp ct ctx fn.body;
    }

let transform_alias (ct: expr_type_transform) (ctx: context) (alias: rs_alias) : rs_alias = {
        new_typ = alias.new_typ; 
        generics = alias.generics;
        old_type = transform_type ct ctx alias.old_type
    }

let transform_obj (ct: expr_type_transform) (ctx: context) (obj: rs_obj) : rs_obj = 
    let obj = ct.obj ctx obj in
    match obj with
        | RsFn fn -> RsFn (transform_fn ct ctx fn)
        | RsAlias alias -> RsAlias (transform_alias ct ctx alias)
        | RsStruct s ->
            RsStruct { s with
                fields = (List.map (fun (a,b) -> (a, transform_type ct ctx b)) s.fields)
            }
        | RsEnum enum -> RsEnum { enum with
            fields = (List.map (fun (name, typ) -> match typ with
                | None -> (name, None) 
                | Some typ -> (name, Some (transform_type ct ctx typ)))
            enum.fields)
        }
        | RsConst const -> RsConst { const with
                value = transform_exp ct ctx const.value;
                typ = transform_type ct ctx const.typ;
            }
        | _ -> obj

let rust_transform_expr (ct: expr_type_transform) (ctx: context) (RsProg objs) : rs_program =
    RsProg (List.map (transform_obj ct ctx) objs)

(* ———————————————————————— Function transformer ————————————————————————— *)

type func_transform = {
    func : context -> rs_fn -> rs_fn
}

let transform_obj_func (ct: func_transform) (ctx: context) (obj: rs_obj) : rs_obj =
    match obj with
    | RsFn fn -> RsFn (ct.func ctx fn) 
    | _ -> obj

let rust_transform_func (ct: func_transform) (ctx: context) (RsProg objs) : rs_program =
    RsProg (List.map (transform_obj_func ct ctx) objs)

(* ——————————————————————————— BitVec transformation ———————————————————————————— *)

let is_bitvec_lit (pexp: rs_pexp) : bool =
    match pexp with
        | RsPexp (RsPatLit RsLitHex _, _) -> true
        | RsPexp (RsPatLit RsLitBin _, _) -> true
        | _ -> false

let bitvec_transform_match_tuple (exp: rs_exp list) (patterns: rs_pat list) : rs_exp =
    assert(List.length exp = List.length patterns);
    RsTuple (List.map2 (fun e p ->
      match p with
      | RsPatLit (RsLitHex _) -> mk_method_app e "bits" []
      | RsPatLit (RsLitBin _) -> mk_method_app e "bits" []
      | _ -> e
    ) exp patterns)
     
  
let parse_first_tuple_entry(values: rs_pexp list) : rs_pat list = 
    match values with 
        | RsPexp(RsPatTuple t, _) :: rest -> t
        | RsPexpWhen(RsPatTuple t, _, _) :: rest -> t
        | _ -> failwith "Code should be unreachable"
  
let bitvec_transform_exp (ctx: context) (exp: rs_exp) : rs_exp =
    let one = Big_int.of_int 1 in
    match exp with
        | RsApp (RsId "subrange_bits", generics, [RsField (bitvec, "bits"); RsLit RsLitNum r_end; RsLit RsLitNum r_start]) ->
            let r_end = Big_int.add r_end (Big_int.of_int 1) in
            let r_size = Big_int.sub r_end r_start in
            RsMethodApp {
                exp = RsField (bitvec, "bits");
                name = "subrange";
                generics = [Big_int.to_string r_start; Big_int.to_string r_end; Big_int.to_string r_size];
                args = [];
            }
        | RsApp (RsId "subrange_bits", generics, [RsId id; RsLit RsLitNum r_end; RsLit RsLitNum r_start]) ->
            let r_end = Big_int.add r_end (Big_int.of_int 1) in
            let r_size = Big_int.sub r_end r_start in
            RsMethodApp {
                exp = RsId id;
                name = "subrange";
                generics = [Big_int.to_string r_start; Big_int.to_string r_end; Big_int.to_string r_size];
                args = [];
            }
        | RsAssign (RsLexpIndexRange (RsLexpField (fexp, "bits"), RsLit RsLitNum r_end, RsLit RsLitNum r_start), exp) ->
            let r_end = Big_int.add r_end one in
            let r_size = Big_int.sub r_end r_start in
            let method_app = {
                exp = RsField (fexp, "bits");
                name = "set_subrange";
                generics = [Big_int.to_string r_start; Big_int.to_string r_end; Big_int.to_string r_size];
                args = [exp];
            } in
            RsAssign (RsLexpField (fexp, "bits"), RsMethodApp method_app)
        | RsApp (RsId "zero_extend", generics, RsLit RsLitNum size :: [e] ) ->  
            RsMethodApp {
                exp = e;
                name = "zero_extend";
                generics = [Big_int.to_string size];
                args = [];
            }
        | RsMatch (exp, pat::pats) when is_bitvec_lit pat ->
            let method_app = {
                exp = exp;
                name = "bits";
                generics = [];
                args = [];
            } in
            RsMatch (RsMethodApp method_app, pat::pats)
        | RsMatch (RsTuple exp_tuple, patterns) -> RsMatch (bitvec_transform_match_tuple exp_tuple (parse_first_tuple_entry patterns), patterns)
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

let rec bitvec_transform_type (ctx: context) (typ: rs_type) : rs_type =
    match typ with
        | RsTypGenericParam ("bitvector", t) -> RsTypGenericParam ("BitVector", t)
        | RsTypGenericParam ("bits", t) -> RsTypGenericParam ("BitVector", t)
        (* TODO: This violate the fact that vector or bits != bitvector. Change it in the future *)
        | RsTypGenericParam ("vector", t) -> RsTypGenericParam ("BitVector", t)
        
        (* TODO: once we resolve type aliasing we can remove those manual conversions *)
        | RsTypId "regbits" -> RsTypGenericParam ("BitVector", [RsTypParamNum (mk_num 5)])

        (* Otherwise keep as is *)
        | _ -> typ

let bitvec_transform = {
    exp = bitvec_transform_exp;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = bitvec_transform_type;
    pat = id_pat;
    obj = id_obj;
}

(* —————————————————————————— Expression Optimizer —————————————————————————— *)

(** Simplifies rust expression by applying basic optimisations.

 For now, this mostly includes arithmetic operators.**)
let rec simplify_rs_exp (ctx: context) (rs_exp: rs_exp) : rs_exp =
    match rs_exp with
        | RsBinop (RsLit (RsLitNum a), RsBinopAdd, RsLit (RsLitNum b)) -> 
            RsLit (RsLitNum (Big_int.add a b))
        | RsBinop (RsLit (RsLitNum a), RsBinopSub, RsLit (RsLitNum b)) -> 
            RsLit (RsLitNum (Big_int.sub a b))
        | RsBinop (RsLit (RsLitNum a), RsBinopMult, RsLit (RsLitNum b)) -> 
            RsLit (RsLitNum (Big_int.mul a b))
        | RsApp (RsPathSeparator (int_typ, RsTypId "pow"), [], [RsLit (RsLitNum n); RsAs (RsLit (RsLitNum m), _)])
        | RsStaticApp (int_typ, "pow", [RsLit (RsLitNum n); RsAs (RsLit (RsLitNum m), _)])->
            mk_big_num (Big_int.pow_int n (Big_int.to_int m))
        | RsBlock exps ->
            let is_not_unit exp = match exp with
                | RsLit RsLitUnit -> false
                | _ -> true
                in
            RsBlock (List.filter is_not_unit exps)
        | RsId id when SMap.mem id ctx.defs.num_constants ->
            let n = SMap.find id ctx.defs.num_constants in
            mk_big_num n
        | _ -> rs_exp


let expression_optimizer = {
    exp = simplify_rs_exp;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = id_typ;
    pat = id_pat;
    obj = id_obj;
}

(* ——————————————————————————— Nested Blocks remover ———————————————————————————— *)


(** Sail often generates blocks in constructs such as if statements, for which
    we already have blocks. This transformation removes the nested blocks. **)
let nested_block_remover_exp (ctx: context) (exp: rs_exp) : rs_exp = 
    match exp with
        | RsIf (c, RsBlock e1, RsBlock e2) -> RsIf (c, RsInstrList e1, RsInstrList e2)
        | RsIf (c, RsBlock e1, e2) -> RsIf (c, RsInstrList e1, e2)
        | RsIf (c, e1, RsBlock e2) -> RsIf (c, e1, RsInstrList e2)
        | RsFor (var, start, until, RsBlock b) -> RsFor(var, start, until, RsInstrList b)
        | _ -> exp 

let nested_block_remover = {
    exp = nested_block_remover_exp;
    lexp = id_lexp;
    pexp = id_pexp;
    typ =  id_typ;
    pat = id_pat;
    obj = id_obj;
}

(* ——————————————————————————— Native functions transformation ———————————————————————————— *)

let unsupported_fun: SSet.t = SSet.of_list ([
    (* Used only for side effects, not necessary in the Rust back-end *)
    "csr_name_write_callback";
    "csr_id_write_callback";
    "csr_full_write_callback";
    "long_csr_write_callback";
])

(* TODO: This list is probably incomplete and we might want to add extra fields in the future *)
let native_func_transform_exp (ctx: context) (exp : rs_exp) : rs_exp = 
    match exp with
    | RsApp (RsId "add_atom", gens, [e1;e2]) -> RsBinop(e1,RsBinopAdd,e2)
    | RsApp (RsId "sub_atom", gens, [e1;e2]) -> RsBinop(e1,RsBinopSub,e2)
    | RsApp (RsId "negate_atom", gens, _) -> RsId "BUILTIN_atom_negate_TODO" 
    | RsApp (RsId "ediv_int", gens, _) -> RsId "BUILTIN_atom_ediv_TODO" 
    | RsApp (RsId "emod_int", gens, [e1; e2]) -> RsBinop(e1, RsBinopMod, e2)
    | RsApp (RsId "abs_int_atom", gens, _) -> RsId "BUILTIN_atom_abs_int_TODO" 
    | RsApp (RsId "not_vec", gens, [v]) -> RsUnop(RsUnopNeg, v)
    | RsApp (RsId "eq_bit", gens, [e1; e2]) -> RsBinop(e1, RsBinopEq, e2) (* TODO Is it correct to compare like that? *)
    | RsApp (RsId "eq_bool", gens, _) -> RsId "BUILTIN_eq_bool_TODO"
    | RsApp (RsId "eq_string", gens, _) -> RsId "BUILTIN_eq_string_TODO"
    | RsApp (RsId "eq_int", gens, _) -> RsId "BUILTIN_eq_int_TODO"
    | RsApp (RsId "not", gens, [b]) -> RsUnop(RsUnopNeg, b)
    | RsApp (RsId "lt", gens, _) -> RsId "BUILTIN_lt_TODO"
    | RsApp (RsId "lteq", gens, _) -> RsId "BUILTIN_lteq_TODO"
    | RsApp (RsId "lteq_int", gens, [e1; e2]) -> RsBinop (e1, RsBinopLe, e2)
    | RsApp (RsId "gt", gens, _) -> RsId "BUILTIN_gt_TODO"
    | RsApp (RsId "gteq", gens, _) -> RsId "BUILTIN_gteq_TODO"
    | RsApp (RsId "add_int", gens, _) -> RsId "BUILTIN_add_int_TODO"
    | RsApp (RsId "sub_int", gens, _) -> RsId "BUILTIN_sub_int_TODO"
    | RsApp (RsId "mult_int", gens, _) -> RsId "BUILTIN_mult_int_TODO"
    | RsApp (RsId "neg_int", gens, _) -> RsId "BUILTIN_neg_int_TODO"
    | RsApp (RsId "abs_int", gens, _) -> RsId "BUILTIN_abs_int_TODO"
    | RsApp (RsId "max_int", gens, _) -> RsId "BUILTIN_max_int_TODO"
    (*| RsApp (RsId "min_int", gens, _) -> RsId "BUILTIN_min_int_TODO" *)
    | RsApp (RsId "tdiv_int", gens, _) -> RsId "BUILTIN_tdiv_int_TODO"
    | RsApp (RsId "tmod_int", gens, _) -> RsId "BUILTIN_tmod_int_TODO"
    | RsApp (RsId "pow2", [], [n]) -> RsApp (RsPathSeparator (int_typ, RsTypId "pow"), [], [mk_num 2; RsAs(n, RsTypId "u32")])
    (* | RsApp (RsId "zeros", gens, _) -> RsId "BUILTIN_zeros_TODO" *)
    (*| RsApp (RsId "ones", gens, e) -> RsApp (RsId "ones", e) Handled by the integrated library *) 
    (* Implemented in lib.sail *)
    (*| RsApp (RsId "zero_extend", gens, e) -> RsApp (RsId "zero_extend", e)
    | RsApp (RsId "sign_extend", gens, e) -> RsApp (RsId "sign_extend", e) 
    | RsApp (RsId "sail_ones", gens, e) -> RsApp (RsId "sail_ones", e) *)
    | RsApp (RsId "sail_signed", gens, _) -> RsId "BUILTIN_sail_signed_TODO"
    | RsApp (RsId "sail_unsigned", gens, _) -> RsId "BUILTIN_sail_unsigned_TODO"
    | RsApp (RsId "slice", gens, _) -> RsId "BUILTIN_slice_TODO"
    | RsApp (RsId "slice_inc", gens, _) -> RsId "BUILTIN_slice_inc_TODO"
    | RsApp (RsId "add_bits", gens, _) -> RsId "BUILTIN_add_bits_TODO"
    | RsApp (RsId "add_bits_int", gens, [b1;b2]) -> RsBinop(b1, RsBinopAdd, b2)
    | RsApp (RsId "sub_bits", gens, _) -> RsId "BUILTIN_sub_bits_TODO"
    | RsApp (RsId "sub_bits_int", gens, _) -> RsId "BUILTIN_sub_bits_int_TODO"
    | RsApp (RsId "append", gens, _) -> RsId "BUILTIN_append_TODO"
    | RsApp (RsId "eq_bits", gens, _) -> RsId "BUILTIN_eq_bits_TODO"
    | RsApp (RsId "neq_bits", gens, _) -> RsId "BUILTIN_neq_bits_TODO"
    | RsApp (RsId "not_bits", gens, _) -> RsId "BUILTIN_not_bits_TODO"
    | RsApp (RsId "sail_truncate", gens, _) -> RsId "BUILTIN_sail_truncate_TODO"
    | RsApp (RsId "sail_truncateLSB", gens, _) -> RsId "BUILTIN_sail_truncateLSB_TODO"
    | RsApp (RsId "shiftl", gens, [e1; e2]) -> RsBinop(e1, RsBinopShiftLeft, e2)
    | RsApp (RsId "shiftr", gens, [e1; e2]) -> RsBinop(e1, RsBinopShiftRight, e2)
    | RsApp (RsId "arith_shiftr", gens, _) -> RsId "BUILTIN_arith_shiftr_TODO"
    | RsApp (RsId "and_bits", gens, _) -> RsId "BUILTIN_and_bits_TODO"
    | RsApp (RsId "or_bits", gens, _) -> RsId "BUILTIN_or_bits_TODO"
    | RsApp (RsId "xor_bits", gens, _) -> RsId "BUILTIN_xor_bits_TODO"
    | RsApp (RsId "vector_init", gens, _) -> RsId "BUILTIN_vector_init_TODO"
    | RsApp (RsId "vector_access", gens, _) -> RsId "BUILTIN_vector_access_TODO"
    | RsApp (RsId "vector_access_inc", gens, _) -> RsId "BUILTIN_vector_access_inc_TODO"
    | RsApp (RsId "vector_subrange", gens, _) -> RsId "BUILTIN_vector_subrange_TODO"
    | RsApp (RsId "vector_subrange_inc", gens, _) -> RsId "BUILTIN_vector_subrange_inc_TODO"
    | RsApp (RsId "vector_update", gens, _) -> RsId "BUILTIN_vector_update_TODO"
    | RsApp (RsId "vector_update_inc", gens, _) -> RsId "BUILTIN_vector_update_inc_TODO"
    | RsApp (RsId "vector_update_subrange", gens, _) -> RsId "BUILTIN_vector_update_subrange_TODO"
    | RsApp (RsId "vector_update_subrange_inc", gens, _) -> RsId "BUILTIN_vector_update_subrange_inc_TODO"
    | RsApp (RsId "length", gens, _) -> RsId "BUILTIN_length_TODO"
    | RsApp (RsId "replicate_bits", gens, _) -> RsId "BUILTIN_replicate_bits_TODO"
    | RsApp (RsId "count_leading_zeros", gens, _) -> RsId "BUILTIN_count_leading_zeros_TODO"
    | RsApp (RsId "eq_real", gens, _) -> RsId "BUILTIN_eq_real_TODO"
    | RsApp (RsId "neg_real", gens, _) -> RsId "BUILTIN_neg_real_TODO"
    | RsApp (RsId "add_real", gens, _) -> RsId "BUILTIN_add_real_TODO"
    | RsApp (RsId "sub_real", gens, _) -> RsId "BUILTIN_sub_real_TODO"
    | RsApp (RsId "mult_real", gens, _) -> RsId "BUILTIN_mult_real_TODO"
    | RsApp (RsId "div_real", gens, _) -> RsId "BUILTIN_div_real_TODO"
    | RsApp (RsId "lt_real", gens, _) -> RsId "BUILTIN_lt_real_TODO"
    | RsApp (RsId "gt_real", gens, _) -> RsId "BUILTIN_gt_real_TODO"
    | RsApp (RsId "lteq_real", gens, _) -> RsId "BUILTIN_lteq_real_TODO"
    | RsApp (RsId "gteq_real", gens, _) -> RsId "BUILTIN_gteq_real_TODO"
    | RsApp (RsId "concat_str", gens, [s1; s2]) -> RsApp(RsId "format!", gens, [RsId "\"{}{}\""; s1; s2]) (* There is a bug with hoisting here *)
    | RsApp (RsId "print_bits", gens, e) -> RsApp(RsId "print_output", gens, e)
    | RsApp (RsId "string_of_bits", gens, _) -> RsId "BUILTIN_string_of_bits_TODO"
    | RsApp (RsId "dec_str", gens, e) ->RsApp (RsId "dec_str", gens, e) (* Handled by an external lib *)
    | RsApp (RsId "hex_str", gens, e) -> RsApp (RsId "hex_str", gens, e) (* Handled by an external lib *)
    | RsApp (RsId "hex_str_upper", gens, _) -> RsId "BUILTIN_hex_str_upper_TODO"
    | RsApp (RsId "sail_assert", gens, _) -> RsId "BUILTIN_sail_assert_TODO"
    | RsApp (RsId "reg_deref", gens, _) -> RsId "BUILTIN_reg_deref_TODO"
    | RsApp (RsId "sail_cons", gens, _) -> RsId "BUILTIN_sail_cons_TODO"
    | RsApp (RsId "eq_anything", gens, _) -> RsId "BUILTIN_eq_anything_TODO"
    | RsApp (RsId "id", gens, _) -> RsId "BUILTIN_id_TODO"
    | RsApp (RsId "gteq_int", gens, [e1; e2]) ->  RsBinop (e1, RsBinopGe, e2)
    | RsApp (RsId "lt_int", gens, [e1; e2]) -> RsBinop (e1, RsBinopLt, e2)
    | RsApp (RsId "internal_error", gens, [file; line; message]) -> RsApp (RsId "panic!", [], [RsLit (RsLitStr "{}, l {}: {}"); file; line; message])
    | RsApp (RsId id, gens, _) when SSet.mem id unsupported_fun -> RsLit RsLitUnit
    | _ -> exp
  

let native_func_transform = {
    exp = native_func_transform_exp;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = id_typ;
    pat = id_pat;
    obj = id_obj;
}

(* ———————————————————————— Hoisting rewriting  ————————————————————————— *)

let create_variable_generator () =
    let counter = ref 0 in
    fun () ->
      counter := !counter + 1;
      Printf.sprintf "var_%d" !counter
  
let variable_generator = ref (create_variable_generator ())

let reset_variable_generator () =
    variable_generator := (create_variable_generator ())

let rec rename_in_exp (rn: string * string) (exp: rs_exp) : rs_exp =
    (* Helpers for renaming one or more exps *)
    let rename_in_exp exp = rename_in_exp rn exp in
    let rename_in_exps exps = List.map rename_in_exp exps in

    (* The actual renaming *)
    let (id, new_id) = rn in
    match (exp : rs_exp) with
        | RsId id' ->
            if id' = id then
                RsId new_id (* Rename! *)
            else
                RsId id' (* No renaming *)
        | RsLet (pat, cond, next) ->
            let new_ids = ids_of_pat pat in
            if SSet.mem id new_ids then
                (* The ID is being shadowed, stop renaming at that point *)
                exp
            else
                (* The ID is not shadowed, so we need to continue the renaming *)
                RsLet (pat, rename_in_exp cond, rename_in_exp next)
        | RsApp (fn, generics, args) -> RsApp (rename_in_exp fn, generics, rename_in_exps args)
        | RsMethodApp app -> RsMethodApp { app with
                exp = rename_in_exp app.exp;
                args = rename_in_exps app.args;
            }
        | RsStaticApp (typ, name, args) -> RsStaticApp (typ, name, rename_in_exps args)
        | RsLit lit -> RsLit lit
        | RsField (exp, field) -> RsField (rename_in_exp exp, field)
        | RsBlock exps -> RsBlock (rename_in_exps exps)
        | RsConstBlock exps -> RsConstBlock (rename_in_exps exps)
        | RsInstrList exps -> RsInstrList (rename_in_exps exps)
        | RsIf (cond, if_branch, else_branch) -> RsIf (rename_in_exp cond, rename_in_exp if_branch, rename_in_exp else_branch)
        | RsMatch (exp, pexps) -> RsMatch (
                rename_in_exp exp,
                List.map (rename_in_pexp rn) pexps
            )
        | RsTuple exps -> RsTuple (rename_in_exps exps)
        | RsArray exps -> RsArray (rename_in_exps exps)
        | RsAssign (lexp, exp) -> RsAssign (rename_in_lexp rn lexp, rename_in_exp exp)
        | RsIndex (exp1, exp2) -> RsIndex (rename_in_exp exp1, rename_in_exp exp2)
        | RsBinop (exp1, op, exp2) -> RsBinop (rename_in_exp exp1, op, rename_in_exp exp2)
        | RsUnop (op, exp) -> RsUnop (op, rename_in_exp exp)
        | RsAs (exp, typ) -> RsAs (rename_in_exp exp, typ)
        | RsSome exp -> RsSome (rename_in_exp exp)
        | RsNone -> RsNone
        | RsPathSeparator (typ, typ') -> RsPathSeparator (typ, typ')
        | RsFor (typ, lit, lit', exp) -> RsFor (typ, lit, lit', rename_in_exp exp)
        | RsStruct (typ, fields) -> RsStruct (
                typ,
                List.map (fun (s, exp) -> (s, rename_in_exp exp)) fields
            )
        | RsStructAssign (st, field, value) -> RsStructAssign (rename_in_exp st, field, rename_in_exp value)
        | RsReturn exp -> RsReturn (rename_in_exp exp)
        | RsTodo s -> RsTodo s
and rename_in_pexp (rn: string * string) (pexp: rs_pexp) : rs_pexp =
    let (id, new_id) = rn in
    match (pexp : rs_pexp) with
        (* First case: the ID is being shadowed, stop renaming at that point *)
        | RsPexp (pat, _)
        | RsPexpWhen (pat, _, _) when SSet.mem id (ids_of_pat pat) ->
            pexp
        (* Second case: the IS is not shadowed, continue renaming*)
        | RsPexp (pat, exp) -> RsPexp (pat, rename_in_exp rn exp)
        | RsPexpWhen (pat, cond, exp) -> RsPexpWhen (pat, rename_in_exp rn cond, rename_in_exp rn exp)
and rename_in_lexp (rn: string * string) (lexp: rs_lexp) : rs_lexp =
    let rename_in_lexp lexp = rename_in_lexp rn lexp in
    let (id, new_id) = rn in
    match (lexp : rs_lexp) with
        | RsLexpId id'->
            if id' == id then
                RsLexpId new_id (* Rename! *)
            else
                RsLexpId id' (* No renaming *)
        | RsLexpField (fexp, field) -> RsLexpField (rename_in_exp rn fexp, field)
        | RsLexpIndex (lexp, exp) -> RsLexpIndex (rename_in_lexp lexp, rename_in_exp rn exp)
        | RsLexpIndexRange (lexp, start, end') -> RsLexpIndexRange (rename_in_lexp lexp, rename_in_exp rn start, rename_in_exp rn end')
        | RsLexpTodo -> RsLexpTodo


(* Wehter an expression should be hoisted. *)
let rec should_hoist_exp (is_nested: bool) (exp: rs_exp) : bool =
    let core_ctx = RsId core_ctx in
    match exp with
        | RsApp (exp, generics, args) -> List.mem core_ctx args || should_hoist_args args
        | RsMethodApp app -> should_hoist_exp true app.exp || List.mem core_ctx app.args || should_hoist_args app.args
        | RsIf (_,_,_) -> true
        | RsField (e,e2) -> should_hoist_exp true e
        | RsBinop (e1, op, e2) -> (should_hoist_exp true e1) || (should_hoist_exp true e2)
        | e when e = core_ctx && is_nested -> true
        | _ -> false

(* Whether at least one argument should be hoisted *)
and should_hoist_args (args: rs_exp list) : bool = 
    List.fold_left (fun acc arg -> acc || should_hoist_exp false arg) false args
    
let rec hoist (exp: rs_exp list) : rs_exp list * rs_exp list = 
    match exp with
    | e :: arr  when should_hoist_exp false e -> 
        let ident = !variable_generator () in
        let (l1, l2) = hoist arr in
        (RsLet (RsPatId ident, e, RsTodo "hoist") :: l1, RsId ident :: l2)
    | e :: arr ->
        let (l1, l2) = hoist arr in
        (l1, e :: l2)
    | [] -> ([], [])

let rec generate_hoisted_block (exp: rs_exp list) (app) : rs_exp = 
    match exp with
        | RsLet (pat, exp2, _) :: arr -> RsLet(pat, exp2, generate_hoisted_block arr app)
        | [] -> app
        | _ -> failwith "Unreachable code"

let rec hoist_let_exp (exp: rs_exp) : rs_exp * ((rs_pat * rs_exp) list) =
    let hoit_let_exp_list (exps: rs_exp list) : (rs_exp list) * ((rs_pat * rs_exp) list) =
        let accumulate acc exp =
            let (exp, defs) = hoist_let_exp exp in
            acc @ [exp, defs]
        in
        let list = List.fold_left accumulate [] exps in
        let (exps, defs) = List.split list in
        (exps, List.flatten defs)
    in
    match exp with
        | RsLet (pat, exp, next) ->
            (* We generate a new ID to to avoid shadowing existing variables when hoisting the let statement. *)
            let build_new_id id =
                id ^ "_" ^ (!variable_generator ())
            in
            let (rename, pat) = match pat with
                | RsPatId id ->
                    let new_id = build_new_id id in
                    (Some (id, new_id), RsPatId new_id)
                | RsPatType (typ, RsPatId id) ->
                    let new_id = build_new_id id in
                    (Some (id, new_id), RsPatType (typ, RsPatId new_id))
                | _ ->
                    (None, pat)
            in
            begin match rename with 
                (* We decided to hoist that definition *)
                | Some(rename) ->
                    let def = (pat, exp) in
                    let next = rename_in_exp rename next in
                    let (next, defs) = hoist_let_exp next in
                    (next, def :: defs )
                (* We will not hoist that definition *)
                | None ->
                    let (next, defs) = hoist_let_exp next in
                    (RsLet (pat, exp, next), defs)
            end
        | RsApp (fn, generics, args) ->
            let (fn, defs) = hoist_let_exp fn in
            let (args, defs_args) = hoit_let_exp_list args in
            (RsApp (fn, generics, args), defs @ defs_args)
        | RsMethodApp app ->
            let (exp, defs) = hoist_let_exp app.exp in
            let (args, defs_args) = hoit_let_exp_list app.args in
            (RsMethodApp { app with exp = exp; args = args;}, defs @ defs_args)
        | RsStaticApp (typ, name, exps) -> 
            let (exps, defs) = hoit_let_exp_list exps in
            (RsStaticApp (typ, name, exps), defs)
        | RsField (exp, field) ->
            let (exp, defs) = hoist_let_exp exp in
            (RsField (exp, field), defs)
        | RsBlock exps ->
            let (exps, defs) = hoit_let_exp_list exps in
            (RsBlock (exps), defs)
        | RsInstrList exps ->
            let (exps, defs) = hoit_let_exp_list exps in
            (RsInstrList (exps), defs)
        | RsTuple exps ->
            let (exps, defs) = hoit_let_exp_list exps in
            (RsTuple (exps), defs)
        | RsIndex (exp1, exp2) ->
            let (exp1, defs1) = hoist_let_exp exp1 in
            let (exp2, defs2) = hoist_let_exp exp2 in
            (RsIndex (exp1,exp2), defs1 @ defs2)
        | RsBinop (exp1, op, exp2) -> 
            let (exp1, defs1) = hoist_let_exp exp1 in
            let (exp2, defs2) = hoist_let_exp exp2 in
            (RsBinop (exp1, op, exp2), defs1 @ defs2)
        | RsUnop (unop, exp) ->
            let (exp, defs) = hoist_let_exp exp in
            (RsUnop (unop, exp), defs)
        | RsAs (exp, typ) ->
            let (exp, defs) = hoist_let_exp exp in
            (RsAs (exp, typ), defs)
        | _ -> (exp, [])

let pexp_hoister (ctx: context) (pexp: rs_pexp) : rs_pexp =
    match pexp with
        | RsPexpWhen (pat, cond, exp) ->
            let (cond, defs) = hoist_let_exp cond in
            let rec build_cond exp defs =
                match defs with
                    | (pat, binding) :: tail -> RsLet (pat, binding, build_cond exp tail)
                    | [] -> exp
            in
            let cond = build_cond cond defs in
            RsPexpWhen (pat, cond, exp)
        | _ -> pexp

let expr_hoister (ctx: context) (exp: rs_exp) : rs_exp = 
    match exp with
        (* We dont need to hoist external functions & some macro might not work with hoisting (for example: format!)*)
        | RsApp (RsId name, generics, args) when should_hoist_args args && not(SSet.mem name external_func) -> let ret = hoist args in 
            RsBlock[generate_hoisted_block (fst ret) (RsApp (RsId name, generics, snd ret))]
        | RsMethodApp {exp; name; generics; args} when should_hoist_args args -> let ret = hoist args in 
            RsBlock[generate_hoisted_block (fst ret) (RsMethodApp {
                exp = exp;
                name = name;
                generics = generics;
                args = snd ret
            })]
        | RsIf (cond, if_branch, else_branch) ->
            let (cond, defs) = hoist_let_exp cond in
            let rec build_cond exp defs =
                match defs with
                    | (pat, binding) :: tail -> RsLet (pat, binding, build_cond exp tail)
                    | [] -> exp
            in
            let cond = build_cond cond defs in
            RsIf (cond, if_branch, else_branch)
        | _ -> exp

let obj_hoister (ctx: context) (obj: rs_obj) : rs_obj =
    (* For each rust object we reset the counter
       This makes the IDs of variables more stable, as a change in a function
       doesn't rename variables in another *)
    reset_variable_generator ();
    obj

let expr_type_hoister = {
    exp = expr_hoister;
    lexp = id_lexp;
    pexp = pexp_hoister;
    typ = id_typ;
    pat = id_pat;
    obj = obj_hoister;
}

(* ——————————————————————————— Generic Rewriting ———————————————————————————— *)
(* Sail uses apostrophes in generic names, which are parsed as lifetimes in   *)
(* Rust. This transformation removes the apostrophes and format the generic   *)
(* names.                                                                     *)
(* —————————————————————————————————————————————————————————————————————————— *)

let sanitize_generic (generic: rs_generic) : rs_generic =
    let id = match generic with
        | RsGenTyp s -> s
        | RsGenConst (s, _) -> s
    in
    let new_id = sanitize_generic_id id in
    match generic with
        | RsGenTyp _ -> RsGenTyp new_id
        | RsGenConst (_, typ) -> RsGenConst (new_id, typ)

let rec rewrite_generics (ctx: context) (typ: rs_type) : rs_type =
    match (typ : rs_type) with
        | RsTypId id -> RsTypId (sanitize_generic_id id)
        | RsTypGeneric id -> RsTypGeneric (sanitize_generic_id id)
        | RsTypTuple typs -> RsTypTuple (List.map (rewrite_generics ctx) typs)
        | RsTypUnit -> RsTypUnit
        | RsTypGenericParam (typ, params) -> RsTypGenericParam (typ, List.map (rewrite_generic_params ctx) params)
        | RsTypArray (n, m) -> RsTypArray (rewrite_generic_params ctx n, rewrite_generic_params ctx m)
        | RsTypOption typ -> RsTypOption (rewrite_generic_params ctx typ)
        | RsTypTodo s -> RsTypTodo s

and rewrite_generic_params (ctx: context) (param: rs_type_param) : rs_type_param =
    match param with 
        | RsTypParamTyp typ -> RsTypParamTyp (rewrite_generics ctx typ)
        | RsTypParamNum num -> param

let rewrite_generics_obj (ctx: context) (obj: rs_obj) : rs_obj =
    match (obj : rs_obj) with
        | RsEnum enum -> RsEnum {
            enum with
            generics = List.map sanitize_generic enum.generics;
        }
        | RsStruct s -> RsStruct {
            s with
            generics = List.map sanitize_generic s.generics;
        }
        | RsAlias alias -> RsAlias {
            alias with
            generics = List.map sanitize_generic alias.generics;
        }
        | RsFn fn -> let generics = List.map sanitize_generic fn.signature.generics in
            RsFn {fn with signature = {fn.signature with generics = generics}}
        | _ -> obj


let normalize_generics = {
    exp = id_exp;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = rewrite_generics;
    pat = id_pat;
    obj = rewrite_generics_obj;
}


(* ————————————————————————— VirtContext Call Graph ————————————————————————— *)
(* Infers which function needs to have the context as argument.               *)
(* The initial Sail-to-Rust translation tracks usage of registers and         *)
(* configuration, in this pass we aditionnaly check if a function calls       *)
(* another function that needs the context.                                   *)
(* —————————————————————————————————————————————————————————————————————————— *)

let exp_virt_ctx_usage (ctx: context) (exp: rs_exp) : rs_exp = match exp with
    | RsApp (RsId fn, _, _) ->
        begin match ctx_fun fn ctx with
            | Some fn when fn.use_sail_ctx ->
                ctx.uses_sail_ctx <- true;
                exp
            | _ -> exp
        end
    | _ -> exp

let exp_virt_context_call_graph = {
    exp = exp_virt_ctx_usage;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = id_typ;
    pat = id_pat;
    obj = id_obj;
}

let is_sail_context_needed (ctx: context) (func: rs_fn): rs_fn = match func.use_sail_ctx with
    | true ->
        (* Nothing to do, we already know it uses the context *)
        func
    | false ->
        (* We need to search for any function call that needs the context *)
        ctx.uses_sail_ctx <- false; (* remove flag *)
        ignore (transform_fn exp_virt_context_call_graph ctx func);
        let ctx_func = Option.get (ctx_fun func.name ctx) in
        match ctx.uses_sail_ctx with
            | true ->
                (* We need to keep the context in sync *)
                    ctx_func.use_sail_ctx <- true;
                { func with use_sail_ctx = true; }
            | false ->
                func

let virt_context_call_graph = {
  func = is_sail_context_needed;
}

(* ———————————————————————— VirtContext transformer ————————————————————————— *)
(* Adds a virtual context as first argument to all functions.                 *)
(* —————————————————————————————————————————————————————————————————————————— *)

let sail_context_inserter (ctx: context) (func: rs_fn): rs_fn = 
    if func.use_sail_ctx then
        { 
            func with 
            args = RsPatId core_ctx :: func.args;
            signature = { func.signature with args = RsTypId "&mut Core" :: func.signature.args }
        }
    else
        func

let virt_context_transform = {
  func = sail_context_inserter;
}

(* —————————————————————————— Enum Args Namespace ——————————————————————————— *)
(* Sail does not need to namespace its enum, but Rust does. This pass adds    *)
(* the approriate namespaces to all enum arguments.                           *)
(* —————————————————————————————————————————————————————————————————————————— *)

let add_namespace_to_arg_pats (ctx: context) (func: rs_fn): rs_fn = 
  let rec get_namespace enum enum_list = match enum_list with
    | (k, v) :: tail when k = enum -> Some v
    | (k, v) :: tail -> get_namespace enum tail
    | [] -> None
  in
  (** Add the proper enum namespace to all enum pattern argument, leave other unchanged **)
  let add_namespace pat = match pat with
    | RsPatApp (RsPatId enum, args) -> begin match get_namespace enum ctx.enum_entries with
        (* There is no concept of path in patterns yet, so we do a hacky string concatenation. *)
        (* TODO: fix that by adding a path to patterns *)
        | Some path -> RsPatApp (RsPatId (path ^ "::" ^ enum), args)
        | None -> pat (* Could not find enum *)
    end
    | _ -> pat
  in

  { func with args = List.map add_namespace func.args; }

let enum_arg_namespace : func_transform = {
  func = add_namespace_to_arg_pats;
}

(* ———————————————————————————— Fix Generic Type ———————————————————————————— *)

let fix_generic_type_func (ctx: context) (func: rs_fn): rs_fn = 
    let rec get_array_type_vars (typs: rs_type list) = match typs with
        | RsTypArray (_, RsTypParamTyp (RsTypId n)) :: tail -> n :: get_array_type_vars tail
        | head :: tail -> get_array_type_vars tail
        | [] -> []
    in
    let set_array_generic_types (should_set: string list) (generic: rs_generic) =
        match generic with
            | RsGenConst (s, typ) when List.mem s should_set -> RsGenConst (s, "usize")
            | _ -> generic
    in
    let array_type_vars = (get_array_type_vars func.signature.args) @ (get_array_type_vars [func.signature.ret]) in
    let new_generics = List.map (set_array_generic_types array_type_vars) func.signature.generics in
    let signature = { func.signature with generics = new_generics } in
    { func with signature = signature }

let fix_generic_type : func_transform = {
  func = fix_generic_type_func;
}

(* ———————————————————————— Enumeration binder ————————————————————————— *)

let rec enum_prefix_inserter (key : string) (lst : (string * string) list) : string =
    match lst with
    | [] -> key
    | (k, v) :: rest ->
        if k = key then v ^ "::" ^ k
        else enum_prefix_inserter key rest
  
let enum_binder_exp (ctx: context) (exp: rs_exp) : rs_exp = 
  match exp with
    | RsId id -> RsId (enum_prefix_inserter id ctx.enum_entries)
    | RsApp (RsId id, generics, args) -> RsApp (RsId (enum_prefix_inserter id ctx.enum_entries), generics, args)
    | RsMethodApp {exp = RsId id; name; generics; args} -> RsMethodApp {
            exp = RsId (enum_prefix_inserter id ctx.enum_entries);
            name = name;
            generics = generics;
            args = args;
        }
    | _ -> exp
 
let enum_binder_lexp (ctx: context) (lexp: rs_lexp) : rs_lexp = 
  match lexp with
    | RsLexpId id -> RsLexpId (enum_prefix_inserter id ctx.enum_entries)
    | _ -> lexp
 
(*TODO: Maybe we should match RsPatId directly?*)
let enum_binder_pat (ctx: context) (pat: rs_pat) : rs_pat = 
  match pat with
    | RsPatId id -> RsPatId (enum_prefix_inserter id ctx.enum_entries)
    | _ -> pat
 
 
let enum_binder : expr_type_transform = {
    exp = enum_binder_exp;
    pexp = id_pexp;
    lexp = enum_binder_lexp;
    typ = id_typ;
    pat = enum_binder_pat;
    obj = id_obj;
}
  
(* ———————————————————————————— Const Functions ————————————————————————————— *)

(* Constant prelude functions *)
let const_prelude_func: SSet.t = SSet.of_list ([
        "sail_ones";
        "sail_zeros";
    ])

(* For now we use very simple heuristics *)
let should_be_const (body: rs_exp) : bool =
    match body with
        | RsLit _ -> true
        | RsApp (RsId id, _, _) when SSet.mem id const_prelude_func-> true 
        | _ -> false

let const_functions (ctx: context) (func: rs_fn): rs_fn = {
    func with
    const = should_be_const func.body
}

let const_fn_rewriter = {
  func = const_functions
}

(* ———————————————————————— Operator rewriter function side  ————————————————————————— *)

open Str

let remove_illegal_operator_char str =
  let str = global_replace (regexp "=") "equal" str in
  let str = global_replace (regexp "<") "smaller" str in
  let str = global_replace (regexp ">") "bigger" str in
  let str = global_replace (regexp "(") "_" str in
  let str = global_replace (regexp ")") "_" str in
  let str = global_replace (regexp " ") "_" str in
  str 

let operator_rewriter_func (ctx: context) (func: rs_fn): rs_fn = {
    func with
    name = remove_illegal_operator_char func.name;
}

let operator_rewriter = {
  func = operator_rewriter_func
}


(* ———————————————————————— Operator rewriter caller side  ————————————————————————— *)

let expr_operator_rewriter (ctx: context) (exp: rs_exp) : rs_exp = 
    match exp with
        | RsApp (RsId id, generics, args) -> RsApp(RsId(remove_illegal_operator_char id), generics, args)
        | _ -> exp

let expr_type_operator_rewriter = {
    exp = expr_operator_rewriter;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = id_typ;
    pat = id_pat;
    obj = id_obj;
}


(* ———————————————————————— type bits = bitvector filter  ————————————————————————— *)

let filter_bits_bitvector_alias (obj: rs_obj) : rs_program = 
    match obj with
        | RsAlias { new_typ = "bits"; _ } -> RsProg []
        | _ -> RsProg[obj]

let rust_remove_type_bits (RsProg objs) : rs_program =  merge_rs_prog_list (List.map (filter_bits_bitvector_alias) objs)

(* ———————————————————————— prelude_func_filter  ————————————————————————— *)

let prelude_func: SSet.t = SSet.of_list (["not"; "plain_vector_access"; "neq_int"; "neq_bits"; "eq_int"; "eq_bool"; "eq_bits"; "eq_anything"; "neq_anything"; "or_vec"; "and_vec"; "xor_vec"; "add_bits"; "and_bool"; "or_bool"; "zero_extend"; "sign_extend"; "sail_ones"; "internal_error"; "hex_bits_12_forwards"; "hex_bits_12_backwards"; "parse_hex_bits"])

let rust_prelude_func_filter_alias (obj: rs_obj) : rs_program = 
    match obj with
        | RsFn {name;_ } when (SSet.mem name prelude_func)-> RsProg []
        | _ -> RsProg[obj]

let rust_prelude_func_filter (RsProg objs) : rs_program =  merge_rs_prog_list (List.map (rust_prelude_func_filter_alias) objs)


(* ———————————————————————— Annotations and imports inserter  ————————————————————————— *)

(* todo: Is a static function good enough here? *)
let insert_annotation_imports_aux () : rs_program = RsProg[RsAttribute "allow(warnings)"; RsImport "softcore_prelude::*"]

let insert_annotation_imports (RsProg objs) : rs_program =  merge_rs_prog_list[insert_annotation_imports_aux();RsProg(objs)]

(* ———————————————————————— BasicTypes rewriter  ————————————————————————— *)

let transform_basic_types_exp (ctx: context) (exp: rs_exp) : rs_exp = 
    match exp with
        (* Reserved keywords in rust *)
        | RsId "priv" -> RsId "_priv_"
        | RsId "super" -> RsId "_super_"
        (* Conversion for `nat` type *)
        | RsApp (RsId id, generics, args)->
            let patch_arg (exp, typ) =
                match typ with
                    (* Conversion between integer types is not automatic, therefore we need to insert some casts *)
                    | RsTypId "nat" -> RsAs (exp, RsTypId "u128")
                    | _ -> exp
            in
            let args = match ctx_fun_type id ctx with
                | Some fun_def ->
                    List.map patch_arg (List.combine args fun_def.args)
                | None -> args
            in
            RsApp (RsId id, generics, args)
        | _ -> exp

(* TODO: Should we apply the same logic in lexp here? *)
let transform_basic_types_lexp (ctx: context) (lexp: rs_lexp) : rs_lexp = 
    match lexp with 
        | RsLexpId "priv" -> RsLexpId "_priv_"
        | RsLexpId "super" -> RsLexpId "_super_"
        | _ -> lexp


let transform_basic_types_pexp (ctx: context) (pexp: rs_pexp) : rs_pexp = pexp

let transform_basic_types_type (ctx: context) (typ: rs_type) : rs_type = 
  match typ with
    | RsTypId "string" -> RsTypId "&\'static str"
    | RsTypId "int" -> int_typ
    | RsTypId "bit" -> bool_typ
    (* TODO: Is this transformation legal? Should we add an assertion at some place in the code? *)
    | RsTypGenericParam ("range", _) -> int_typ
    | RsTypGenericParam ("implicit", _) -> int_typ
    | _ -> typ

let transform_basic_types_pat (ctx: context) (pat: rs_pat) : rs_pat =
    match pat with
        | RsPatId "priv" -> RsPatId "_priv_"
        | RsPatId "super" -> RsPatId "_super_"
        | _ -> pat

let transform_basic_types: expr_type_transform = {
    exp = transform_basic_types_exp;
    lexp = transform_basic_types_lexp;
    pexp = transform_basic_types_pexp;
    typ = transform_basic_types_type;
    pat = transform_basic_types_pat;
    obj = id_obj;
}

(* ———————————————————————— Wildcard inserter  ————————————————————————— *)

let add_wildcard_match_expr (ctx: context) (exp: rs_exp) : rs_exp = 
    match exp with 
        | RsMatch (exp, pexps) -> RsMatch(exp, pexps @ [RsPexp(RsPatWildcard, RsApp(RsId "panic!", [], [RsLit (RsLitStr "Unreachable code")]))]) 
        | _ -> exp

let add_wildcard_match: expr_type_transform = {
    exp = add_wildcard_match_expr;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = id_typ;
    pat = id_pat;
    obj = id_obj;
}

(* ———————————————————————— VirtContext argument inserter  ————————————————————————— *)


(* TODO: Is it correct like that? It might not be... *)
let is_enum (value: string) : bool = 
    let re = Str.regexp_string "::" in
    try
        ignore (Str.search_forward re value 0);
        true
    with Not_found -> false 

let sail_context_arg_inserter_exp (ctx: context) (exp: rs_exp) : rs_exp = 
  match exp with 
    | RsApp (RsId app_id, generics, args) when not(SSet.mem app_id external_func) && not(is_enum app_id) -> 
        begin match ctx_fun app_id ctx  with
            | Some(fn) when not fn.use_sail_ctx -> exp
            | Some(fn) -> let args = RsId core_ctx :: args in RsApp (RsId app_id, generics, args)
            | _ ->
                Reporting.simple_warn (Printf.sprintf "Could not find function '%s' in context" app_id);
                let args = RsId core_ctx :: args in RsApp (RsId app_id, generics, args)
        end
    | _ -> exp
    
let sail_context_arg_inserter: expr_type_transform = {
    exp = sail_context_arg_inserter_exp;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = id_typ;
    pat = id_pat;
    obj = id_obj;
}

(* TODO: This is a very (almost useless) basic dead code remover only for our use case. Extend it in the future *)
(* ———————————————————————— Dead code remover  ————————————————————————— *)

let filter_different_litterals (lit: Big_int.num) (pexp: rs_pexp) : bool =
    match pexp with 
        | RsPexp (RsPatTuple [e; RsPatLit(RsLitNum n)],e2) when n <> lit -> false
        | RsPexpWhen (RsPatTuple [e; RsPatLit(RsLitNum n)],e2, e3) when n <> lit -> false
        | _ -> true


let dead_code_remover_exp (ctx: context) (exp: rs_exp) : rs_exp = 
    match exp with
        | RsMatch (RsTuple [e; RsLit(RsLitNum n)], pexps) -> 
            RsMatch (RsTuple [e; RsLit(RsLitNum n)], List.filter (filter_different_litterals n) pexps)
        | RsIf (RsBinop (RsLit(RsLitNum n1),RsBinopEq, RsLit(RsLitNum n2)), then_exp, else_exp) when n1 <> n2 ->
            RsIf (RsBinop (RsLit(RsLitNum n1),RsBinopEq, RsLit(RsLitNum n2)), RsApp(RsId "panic!", [], [RsLit (RsLitStr "unreachable code")]), else_exp) 
        | _ -> exp
    
let dead_code_remover: expr_type_transform = {
    exp = dead_code_remover_exp;
    lexp = id_lexp;
    pexp = id_pexp;
    typ = id_typ;
    pat = id_pat;
    obj = id_obj;
}

(* ——————————————————————————— Remove Unsupported ——————————————————————————— *)
(* We do not yet support all sail features, so for now we allow ourselves     *)
(* to selectively drop parts of the Sail model. We hope to support all of     *)
(* those in the future.                                                       *)
(* —————————————————————————————————————————————————————————————————————————— *)

let unsupported_obj: SSet.t = SSet.of_list ([
    (* Used only for side effects, not necessary in the Rust back-end *)
    "csr_name_write_callback";
    "csr_id_write_callback";
    "csr_full_write_callback";
    "long_csr_write_callback";

    (* Depend on const generic exprs, would require monomorphisation. *)
    "Mem_write_request";
    "PTW_Output";
    "PTW_Result";
    "pte_bits";
    "ppn_bits";
    "vpn_bits";
])

let is_supported_obj (obj: rs_obj) : bool =
    match obj with
        | RsStruct s when SSet.mem s.name unsupported_obj -> false
        | RsAlias alias when SSet.mem alias.new_typ unsupported_obj -> false
        | RsFn fn when SSet.mem fn.name unsupported_obj -> false
        | _ -> true

(* ————————————————————————————— Rust Transform ————————————————————————————— *)

(** Computes the fix point of a function. **)
let rec fix_point fn rs_program ctx limit =
    let new_args = fn rs_program ctx in
    if new_args = rs_program || limit = 0 then
        new_args
    else
        fix_point fn new_args ctx (limit - 1)

let optimizer (rust_program: rs_program) (ctx: context) : rs_program =
    let get_num_constants (RsProg obj : rs_program) : (string * Big_int.num) list = 
        let rec constants obj = 
            match obj with
                | RsConst {value = RsLit (RsLitNum n); name;} :: tail ->
                    (name, n) :: constants tail
                | head :: tail -> constants tail
                | [] -> []
            in
            constants obj
    in
    let constants = get_num_constants rust_program in
    let defs = { ctx.defs with
        num_constants = SMap.of_list constants;
      } in
      let ctx = { ctx with defs = defs } in
    rust_transform_expr expression_optimizer ctx rust_program

let transform (rust_program: rs_program) (ctx: context) : rs_program =

  (* Bitvector transformations

     We must first replace the Sail native function and perform a basic pass of optimization
     to detect some bitvec patterns properly *)
  let rust_program = rust_transform_func virt_context_call_graph ctx rust_program in
  let rust_program = rust_transform_func virt_context_transform ctx rust_program in
  let rust_program = rust_transform_expr nested_block_remover ctx rust_program in
  let rust_program = rust_transform_expr native_func_transform ctx rust_program in
  let rust_program = fix_point optimizer rust_program ctx 10 in
  let rust_program = rust_transform_expr bitvec_transform ctx rust_program in
  let rust_program = rust_transform_expr normalize_generics ctx rust_program in
  let rust_program = rust_transform_func enum_arg_namespace ctx rust_program in
  let rust_program = rust_transform_func fix_generic_type ctx rust_program in
  let rust_program = rust_transform_expr enum_binder ctx rust_program in
  let rust_program = rust_remove_type_bits rust_program in
  let rust_program = rust_prelude_func_filter rust_program in
  let rust_program = insert_annotation_imports rust_program in
  let rust_program = rust_transform_expr transform_basic_types ctx rust_program in
  let rust_program = rust_transform_expr add_wildcard_match ctx rust_program in
  let rust_program = rust_transform_expr sail_context_arg_inserter ctx rust_program in
  let rust_program = rust_transform_expr expr_type_hoister ctx rust_program in 
  let rust_program = rust_transform_expr expr_type_operator_rewriter ctx rust_program in
  let rust_program = rust_transform_func const_fn_rewriter ctx rust_program in
  let rust_program = rust_transform_func operator_rewriter ctx rust_program in

  (* Optimizer: Dead code elimination *)
  let rust_program = rust_transform_expr dead_code_remover ctx rust_program in

  (* Filter unsupported items *)
  let rust_program = match rust_program with RsProg objs -> RsProg (List.filter is_supported_obj objs) in

  rust_program


