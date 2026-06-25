(** Rust Transformations **)

(** This module transforms raw Rust code generated from Sail into a valid Rust module. **)

open Rs_ast
open Rs_context
open Rs_ast_utils
open Rs_to_string
open Libsail

(* ————————————————————————— Transform Expressions —————————————————————————— *)

let id_exp (_ctx : context) (exp : rs_exp) : rs_exp_aux = exp.e_exp
let id_lexp (_ctx : context) (lexp : rs_lexp) : rs_lexp = lexp
let id_pexp (_ctx : context) (pexp : rs_pexp) : rs_pexp = pexp
let id_typ (_ctx : context) (typ : rs_type) : rs_type = typ
let id_pat (_ctx : context) (pat : rs_pat) : rs_pat = pat
let id_obj (_ctx : context) (obj : rs_obj) : rs_obj = obj

type expr_type_transform =
  { exp_aux : context -> rs_exp -> rs_exp_aux
  ; lexp : context -> rs_lexp -> rs_lexp
  ; pexp : context -> rs_pexp -> rs_pexp
  ; typ : context -> rs_type -> rs_type
  ; pat : context -> rs_pat -> rs_pat
  ; obj : context -> rs_obj -> rs_obj
  }

let id_expr_type_transform : expr_type_transform =
  { exp_aux = id_exp
  ; lexp = id_lexp
  ; pexp = id_pexp
  ; typ = id_typ
  ; pat = id_pat
  ; obj = id_obj
  }
;;

let rec transform_pat (ct : expr_type_transform) (ctx : context) (pat : rs_pat) : rs_pat =
  let pat = ct.pat ctx pat in
  match pat with
  | RsPatId "None" -> RsPatNone
  | RsPatApp (RsPatId "Some", args) ->
    (match args with
     | arg :: [] -> RsPatSome (transform_pat ct ctx arg)
     | _ -> failwith "This case should be unreachable, revise assumption")
  | RsPatType (typ, pat) -> RsPatType (transform_type ct ctx typ, transform_pat ct ctx pat)
  | RsPatTuple pat_list -> RsPatTuple (List.map (transform_pat ct ctx) pat_list)
  | RsPatWildcard -> RsPatWildcard
  | RsPatLit l -> RsPatLit l
  | RsPatId id -> RsPatId id
  | RsPatApp (RsPatId "None", _args) -> RsPatNone
  | RsPatApp (name, args) ->
    RsPatApp (transform_pat ct ctx name, List.map (fun p -> transform_pat ct ctx p) args)
  | RsPatSome pat -> RsPatSome (transform_pat ct ctx pat)
  | RsPatNone -> RsPatNone
  | RsPatTodo text -> RsPatTodo text

and transform_lexp (ct : expr_type_transform) (ctx : context) (lexp : rs_lexp) : rs_lexp =
  let lexp = ct.lexp ctx lexp in
  match lexp with
  | RsLexpId id -> RsLexpId id
  | RsLexpTyp (id, typ) -> RsLexpTyp (id, transform_type ct ctx typ)
  | RsLexpField (lexp, id) -> RsLexpField (transform_exp ct ctx lexp, id)
  | RsLexpIndex (lexp, exp) ->
    RsLexpIndex (transform_lexp ct ctx lexp, transform_exp ct ctx exp)
  | RsLexpIndexRange (lexp, range_start, range_end) ->
    RsLexpIndexRange
      ( transform_lexp ct ctx lexp
      , transform_exp ct ctx range_start
      , transform_exp ct ctx range_end )
  | RsLexpBitVectorAccess (lexp, exp) ->
    RsLexpBitVectorAccess (transform_lexp ct ctx lexp, transform_exp ct ctx exp)
  | RsLexpTodo -> RsLexpTodo

and transform_exp_aux (ct : expr_type_transform) (ctx : context) (exp : rs_exp)
  : rs_exp_aux
  =
  let exp = ct.exp_aux ctx exp in
  match exp with
  | RsLet (pat, exp, next) ->
    RsLet (transform_pat ct ctx pat, transform_exp ct ctx exp, transform_exp ct ctx next)
  | RsLetMut (pat, exp, next) ->
    RsLetMut
      (transform_pat ct ctx pat, transform_exp ct ctx exp, transform_exp ct ctx next)
  | RsApp (app, generics, args) -> transform_app ct ctx app generics args
  | RsStaticApp (app, method_name, args) ->
    RsStaticApp
      (transform_type ct ctx app, method_name, List.map (transform_exp ct ctx) args)
  | RsMethodApp { exp; name; generics; args } ->
    RsMethodApp
      { exp = transform_exp ct ctx exp
      ; name
      ; generics
      ; args = List.map (transform_exp ct ctx) args
      }
  | RsId id -> RsId id
  | RsLit lit -> RsLit lit
  | RsField (exp, field) -> RsField (transform_exp ct ctx exp, field)
  | RsBlock exps -> RsBlock (List.map (transform_exp ct ctx) exps)
  | RsConstBlock exps -> RsConstBlock (List.map (transform_exp ct ctx) exps)
  | RsInstrList exps -> RsInstrList (List.map (transform_exp ct ctx) exps)
  | RsIf (cond, exp_true, exp_false) ->
    RsIf
      ( transform_exp ct ctx cond
      , transform_exp ct ctx exp_true
      , transform_exp ct ctx exp_false )
  | RsMatch (exp, pexps) ->
    RsMatch (transform_exp ct ctx exp, List.map (transform_pexp ct ctx) pexps)
  | RsTuple exps -> RsTuple (List.map (transform_exp ct ctx) exps)
  | RsArray exps -> RsArray (List.map (transform_exp ct ctx) exps)
  | RsArraySize (exp, size) ->
    RsArraySize (transform_exp ct ctx exp, transform_exp ct ctx size)
  | RsVec exps -> RsVec (List.map (transform_exp ct ctx) exps)
  | RsVecSize (exp, size) ->
    RsVecSize (transform_exp ct ctx exp, transform_exp ct ctx size)
  | RsAssign (lexp, exp) -> RsAssign (transform_lexp ct ctx lexp, transform_exp ct ctx exp)
  | RsIndex (exp1, exp2) -> RsIndex (transform_exp ct ctx exp1, transform_exp ct ctx exp2)
  | RsBinop (exp1, binop, exp2) ->
    RsBinop (transform_exp ct ctx exp1, binop, transform_exp ct ctx exp2)
  | RsUnop (unop, exp) -> RsUnop (unop, transform_exp ct ctx exp)
  | RsAs (exp, typ) -> RsAs (transform_exp ct ctx exp, transform_type ct ctx typ)
  | RsSome exp -> RsSome (transform_exp ct ctx exp)
  | RsNone -> RsNone
  | RsPathSeparator (t1, t2) -> RsPathSeparator (t1, t2)
  | RsFor (var, start, until, body) ->
    RsFor
      ( var
      , transform_exp ct ctx start
      , transform_exp ct ctx until
      , transform_exp ct ctx body )
  | RsForRev (var, start, until, body) ->
    RsForRev
      ( var
      , transform_exp ct ctx start
      , transform_exp ct ctx until
      , transform_exp ct ctx body )
  | RsStruct (typ, entries) ->
    RsStruct
      ( transform_type ct ctx typ
      , List.map (fun (s, e) -> s, transform_exp ct ctx e) entries )
  | RsStructAssign (e1, name, e2) ->
    RsStructAssign (transform_exp ct ctx e1, name, transform_exp ct ctx e2)
  | RsReturn exp -> RsReturn (transform_exp ct ctx exp)
  | RsTodo str -> RsTodo str
  | RsBorrow exp -> RsBorrow (transform_exp ct ctx exp)

and transform_exp (ct : expr_type_transform) (ctx : context) (exp : rs_exp) : rs_exp =
  { e_annot = transform_type_annot ct ctx exp.e_annot
  ; e_exp = transform_exp_aux ct ctx exp
  }

and transform_app
      (ct : expr_type_transform)
      (ctx : context)
      (fn : rs_exp)
      (generics : string list)
      (args : rs_exp list)
  : rs_exp_aux
  =
  let args = List.map (transform_exp ct ctx) args in
  match fn.e_exp, args with
  (* Built-in elementary operations *)
  (* TODO(Gurvan): This does not make much sense to be here, it should be in its
     own pass instead of being done again each time *)
  | RsId "plain_vector_access", [ vector; item ] ->
    RsIndex (vector, mk_as item rs_type_usize)
  | RsId "neq_int", [ left; right ] -> RsBinop (left, RsBinopNeq, right)
  | RsId "neq_bits", [ left; right ] -> RsBinop (left, RsBinopNeq, right)
  | RsId "eq_int", [ left; right ] -> RsBinop (left, RsBinopEq, right)
  | RsId "eq_bool", [ left; right ] -> RsBinop (left, RsBinopEq, right)
  | RsId "eq_bits", [ left; right ] -> RsBinop (left, RsBinopEq, right)
  | RsId "eq_anything", [ left; right ] -> RsBinop (left, RsBinopEq, right)
  | RsId "neq_anything", [ left; right ] -> RsBinop (left, RsBinopNeq, right)
  | RsId "or_vec", [ left; right ] -> RsBinop (left, RsBinopOr, right)
  | RsId "and_vec", [ left; right ] -> RsBinop (left, RsBinopAnd, right)
  | RsId "xor_vec", [ left; right ] -> RsBinop (left, RsBinopXor, right)
  | RsId "add_bits", [ left; right ] -> mk_method_app left "wrapped_add" [ right ]
  | RsId "and_bool", [ left; right ] -> RsBinop (left, RsBinopLAnd, right)
  | RsId "or_bool", [ left; right ] -> RsBinop (left, RsBinopLOr, right)
  | RsId "Some", [ exp ] -> RsSome exp
  | RsId "None", _ -> RsNone
  | RsId "unsigned", value :: [] -> mk_method_app value "unsigned" []
  | RsId "signed", value :: [] -> mk_method_app value "signed" []
  (* Otherwise keep as is *)
  | _ -> RsApp (fn, generics, args)

and transform_pexp (ct : expr_type_transform) (ctx : context) (pexp : rs_pexp) : rs_pexp =
  let pexp = ct.pexp ctx pexp in
  match pexp with
  | RsPexp (pat, exp) -> RsPexp (transform_pat ct ctx pat, transform_exp ct ctx exp)
  | RsPexpWhen (pat, exp1, exp2) ->
    RsPexpWhen
      (transform_pat ct ctx pat, transform_exp ct ctx exp1, transform_exp ct ctx exp2)

(* ———————————————————————————— Transform Types ————————————————————————————— *)

and transform_type_param
      (ct : expr_type_transform)
      (ctx : context)
      (param : rs_type_param)
  : rs_type_param
  =
  match param with
  | RsTypParamTyp typ -> RsTypParamTyp (transform_type ct ctx typ)
  | RsTypParamNum n -> RsTypParamNum n

and transform_type (ct : expr_type_transform) (ctx : context) (typ : rs_type) : rs_type =
  let typ = ct.typ ctx typ in
  match typ with
  | RsTypId "unit" -> RsTypUnit
  | RsTypId id -> RsTypId id
  | RsTypUnit -> RsTypUnit
  | RsTypTodo e -> RsTypTodo e
  | RsTypTuple types -> RsTypTuple (List.map (transform_type ct ctx) types)
  | RsTypGeneric typ -> RsTypGeneric typ
  (* TODO: Maybe there is a bug here *)
  | RsTypGenericParam (typ, e :: _params) when typ = "option" ->
    RsTypOption (transform_type_param ct ctx e)
  | RsTypGenericParam (typ, params) ->
    RsTypGenericParam (typ, List.map (transform_type_param ct ctx) params)
  | RsTypArray (typ, size) ->
    RsTypArray (transform_type_param ct ctx typ, transform_type_param ct ctx size)
  | RsTypOption param -> RsTypOption (transform_type_param ct ctx param)
  | RsTypBorrow t -> RsTypBorrow (transform_type ct ctx t)

and transform_type_annot (ct : expr_type_transform) (ctx : context) (typ : rs_type option)
  : rs_type option
  =
  match typ with
  | Some t -> Some (transform_type ct ctx t)
  | None -> None
;;

(* ———————————————————————— Expression and Type transformer ————————————————————————— *)

let transform_fn (ct : expr_type_transform) (ctx : context) (fn : rs_fn) : rs_fn =
  let { generics; args; ret; linked_gen_args = _ } = fn.signature in
  let args = List.map (transform_type ct ctx) args in
  let ret = transform_type ct ctx ret in
  { fn with
    args = List.map (ct.pat ctx) fn.args
  ; signature = mk_fn_typ_gen args ret generics
  ; body = transform_exp ct ctx fn.body
  }
;;

let transform_alias (ct : expr_type_transform) (ctx : context) (alias : rs_alias)
  : rs_alias
  =
  { new_typ = alias.new_typ
  ; generics = alias.generics
  ; old_type = transform_type ct ctx alias.old_type
  }
;;

let transform_obj (ct : expr_type_transform) (ctx : context) (obj : rs_obj) : rs_obj =
  let obj = ct.obj ctx obj in
  match obj with
  | RsFn fn -> RsFn (transform_fn ct ctx fn)
  | RsAlias alias -> RsAlias (transform_alias ct ctx alias)
  | RsStruct s ->
    RsStruct
      { s with fields = List.map (fun (a, b) -> a, transform_type ct ctx b) s.fields }
  | RsEnum enum ->
    RsEnum
      { enum with
        fields =
          List.map
            (fun (name, typ) ->
               match typ with
               | None -> name, None
               | Some typ -> name, Some (transform_type ct ctx typ))
            enum.fields
      }
  | RsConst const ->
    RsConst
      { const with
        value = transform_exp ct ctx const.value
      ; typ = transform_type ct ctx const.typ
      }
  | _ -> obj
;;

let rust_transform_expr (ct : expr_type_transform) (ctx : context) (RsProg objs)
  : rs_program
  =
  RsProg (List.map (transform_obj ct ctx) objs)
;;

(* ———————————————————————— Function transformer ————————————————————————— *)

type func_transform = { func : context -> rs_fn -> rs_fn }

let transform_obj_func (ct : func_transform) (ctx : context) (obj : rs_obj) : rs_obj =
  match obj with
  | RsFn fn -> RsFn (ct.func ctx fn)
  | _ -> obj
;;

let rust_transform_func (ct : func_transform) (ctx : context) (RsProg objs) : rs_program =
  RsProg (List.map (transform_obj_func ct ctx) objs)
;;

(* ——————————————————————————————— Utils ——————————————————————————————————— *)

(* TODO(Gurvan): This might break with scoping issues, but Sail generally forbid
   shadowing *)
let update_context_constants (ctx : context) (RsProg objs : rs_program) : context =
  let update_constants (defs : defs) (obj : rs_obj) : defs =
    match obj with
    | RsConst const -> { defs with constants = SSet.add const.name defs.constants }
    | RsFn f when f.const -> { defs with constants = SSet.add f.name defs.constants }
    | _ -> defs
  in
  { ctx with defs = List.fold_left update_constants ctx.defs objs }
;;

(* TODO: This could just update all context and not only functions *)
let update_context_fn_type (ctx : context) (RsProg objs : rs_program) : context =
  let add_obj (defs : defs) (obj : rs_obj) : defs =
    match obj with
    | RsFn f -> { defs with funmap = SMap.add f.name f defs.funmap }
    | _ -> defs
  in
  { ctx with defs = List.fold_left add_obj { ctx.defs with funmap = SMap.empty } objs }
;;

let is_const_rs_typ_id (ctx : context) (x : string) : bool =
  match x with
  (* TODO(Gurvan): We should have a cleaner way to figure out Rust's built-ins *)
  | "usize" | "i128" | "i64" -> true
  | _ ->
    (* TODO(Gurvan): In some case it could still be a a const, check `ctx` *)
    false
;;

let rec is_const_rs_exp (ctx : context) (e : rs_exp) : bool =
  match e.e_exp with
  | RsLit _ | RsConstBlock _ -> true
  | RsAs (e, typ) -> is_const_rs_exp ctx e && is_const_rs_typ ctx typ
  | RsId x -> SSet.mem x ctx.defs.constants
  | RsVec _ | RsVecSize _ -> false
  | e -> false

and is_const_rs_typ (ctx : context) (typ : rs_type) : bool =
  match typ with
  | RsTypId x -> is_const_rs_typ_id ctx x
  | RsTypTuple params -> List.for_all (is_const_rs_typ ctx) params
  | RsTypGeneric x -> assert false (* TODO *)
  | RsTypGenericParam (x, params) -> assert false (* TODO *)
  | RsTypTodo _ -> assert false (* TODO *)
  | RsTypArray (typ1, typ2) ->
    is_const_rs_typ_param ctx typ1 && is_const_rs_typ_param ctx typ2
  | RsTypOption param -> is_const_rs_typ_param ctx param
  | RsTypUnit -> true
  | RsTypBorrow t -> is_const_rs_typ ctx t

and is_const_rs_typ_param (ctx : context) (param : rs_type_param) : bool =
  match param with
  | RsTypParamTyp t -> is_const_rs_typ ctx t
  | RsTypParamNum e -> is_const_rs_exp ctx e
;;

(* ——————————————————————————— BitVec transformation ———————————————————————————— *)

let is_bitvec_lit (pexp : rs_pexp) : bool =
  match pexp with
  | RsPexp (RsPatLit (RsLitHex _), _) -> true
  | RsPexp (RsPatLit (RsLitBin _), _) -> true
  | _ -> false
;;

let bitvec_transform_match_tuple (exp : rs_exp list) (patterns : rs_pat list) : rs_exp =
  assert (List.length exp = List.length patterns);
  { e_annot = None
  ; e_exp =
      RsTuple
        (List.map2
           (fun e p ->
              match p with
              | RsPatLit (RsLitHex _) ->
                { e_annot = None; e_exp = mk_method_app e "bits" [] }
              | RsPatLit (RsLitBin _) ->
                { e_annot = None; e_exp = mk_method_app e "bits" [] }
              | _ -> e)
           exp
           patterns)
  }
;;

let parse_first_tuple_entry (values : rs_pexp list) : rs_pat list =
  match values with
  | RsPexp (RsPatTuple t, _) :: _rest -> t
  | RsPexpWhen (RsPatTuple t, _, _) :: _rest -> t
  | _ ->
    Reporting.simple_warn
      ("Unexpected patterns: | "
       ^ String.concat " | " (List.map (string_of_rs_pexp 0) values));
    failwith "Code should be unreachable"
;;

(* TODO(Gurvan):
  - Try to add a `into` everywhere `exp` is of type BitVec.
  -
*)
let bitvec_transform_exp (ctx : context) (exp : rs_exp) : rs_exp_aux =
  let e_exp =
    match exp.e_exp with
    | RsApp
        ( { e_annot = _; e_exp = RsId "subrange_bits" }
        , _generics
        , [ { e_annot = tvec; e_exp = RsField (bitvec, "bits") }
          ; { e_annot = _; e_exp = RsLit (RsLitNum r_end) }
          ; { e_annot = _; e_exp = RsLit (RsLitNum r_start) }
          ] ) ->
      let r_end = Big_int.add r_end (Big_int.of_int 1) in
      let r_size = Big_int.sub r_end r_start in
      RsMethodApp
        { exp = { e_annot = tvec; e_exp = RsField (bitvec, "bits") }
        ; name = "subrange"
        ; generics =
            [ Big_int.to_string r_start
            ; Big_int.to_string r_end
            ; Big_int.to_string r_size
            ]
        ; args = []
        }
    | RsApp
        ( { e_annot = _; e_exp = RsId "subrange_bits" }
        , _generics
        , [ { e_annot = _; e_exp = RsId id }
          ; { e_annot = _; e_exp = RsLit (RsLitNum r_end) }
          ; { e_annot = _; e_exp = RsLit (RsLitNum r_start) }
          ] ) ->
      let r_end = Big_int.add r_end (Big_int.of_int 1) in
      let r_size = Big_int.sub r_end r_start in
      RsMethodApp
        { exp = { e_annot = None; e_exp = RsId id }
        ; name = "subrange"
        ; generics =
            [ Big_int.to_string r_start
            ; Big_int.to_string r_end
            ; Big_int.to_string r_size
            ]
        ; args = []
        }
    | RsAssign (RsLexpIndexRange (lexp, r_end, r_start), exp) ->
      let method_app =
        { exp = lexp_to_exp lexp
        ; name = "set_subrange"
        ; generics = []
        ; args = [ exp; r_end; r_start ]
        }
      in
      RsAssign (lexp, { exp with e_exp = RsMethodApp method_app })
    | RsApp ({ e_annot = _; e_exp = RsId "zero_extend" }, _generics, [ size; e ])
    | RsApp ({ e_annot = _; e_exp = RsId "sail_zero_extend" }, _generics, [ e; size ]) ->
      (* if is_const_rs_exp ctx size then
      RsMethodApp { exp = e; name = "zero_extend"; generics = [ const_exp_to_generic ctx size ]; args = [ ] }
    else *)
      RsMethodApp { exp = e; name = "zero_extend_dyn"; generics = []; args = [ size ] }
    | RsMatch (exp, pat :: pats) when is_bitvec_lit pat ->
      let method_app = { exp; name = "bits"; generics = []; args = [] } in
      RsMatch ({ e_annot = None; e_exp = RsMethodApp method_app }, pat :: pats)
    | RsMatch ({ e_annot = _; e_exp = RsTuple exp_tuple }, patterns) ->
      RsMatch
        ( bitvec_transform_match_tuple exp_tuple (parse_first_tuple_entry patterns)
        , patterns )
    | RsAssign (RsLexpBitVectorAccess (lexp, exp_idx), exp) ->
      RsAssign
        ( lexp
        , { e_annot = None
          ; e_exp =
              RsMethodApp
                { exp = lexp_to_exp lexp
                ; name = "set_bit"
                ; generics = []
                ; args = [ exp_idx; exp ]
                }
          } )
    | _ -> exp.e_exp
  in
  e_exp
;;

let bitvec_transform_type (ctx : context) (typ : rs_type) : rs_type =
  match typ with
  | RsTypGenericParam ("bitvector", t)
  | RsTypGenericParam ("bits", t)
  (* TODO: This violate the fact that vector or bits != bitvector. Change it in the future *)
  | RsTypGenericParam ("vector", t) ->
    (* TODO(Gurvan): Uncomment to try back bitVector *)
    (* if List.for_all (is_const_rs_typ_param ctx) t
    then RsTypGenericParam ("BitStatic", t)
    else *)
    RsTypId "BitDynamic"
  (* TODO(Gurvan): Should we uncomment the following? once we resolve type aliasing we can remove those manual conversions *)
  (* | RsTypId "regbits" -> RsTypId "BitDynamic" *)
  (* Otherwise keep as is *)
  | _ -> typ
;;

let use_dynamic_bitvec (ctx : context) (rust_program : rs_program) : rs_program =
  let ctx = update_context_constants ctx rust_program in
  rust_transform_expr
    { id_expr_type_transform with
      exp_aux = bitvec_transform_exp
    ; typ = bitvec_transform_type
    }
    ctx
    rust_program
;;

(* ———————————————————— Dynamic BitVectors Arguments ——————————————————————— *)
(* TODO(Gurvan): This is the same ugly fix that we use for vec! vs array *)

let rec is_bitvec_type (ctx : context) (typ : rs_type) =
  match typ with
  | RsTypId "BitDynamic" | RsTypGenericParam ("BitStatic", _) -> true
  | RsTypId x ->
    (match ctx_type x ctx with
     | Some t -> is_bitvec_type ctx t
     | None ->
       Reporting.simple_warn
         (Format.sprintf "Couldn't find if type '%s' is a BitVector type" x);
       false)
  | _ -> false
;;

let cast_bitvec (ctx : context) (typ : rs_type) (e : rs_exp) : rs_exp =
  if is_bitvec_type ctx typ
  then
    { e_annot = Some typ
    ; e_exp = RsMethodApp { exp = e; name = "into"; generics = []; args = [] }
    }
  else e
;;

let use_dynamic_bitvec_exp_in_app (ctx : context) (e : rs_exp) : rs_exp_aux =
  match e.e_exp with
  | RsLet ((RsPatType (t, _) as p), e1, e2) -> RsLet (p, cast_bitvec ctx t e1, e2)
  | RsLet (p, ({ e_annot = Some t1; e_exp = _ } as e1), e2) ->
    RsLet (p, cast_bitvec ctx t1 e1, e2)
  | RsMethodApp { exp; name; generics = _; args } ->
    Reporting.simple_warn
      (Format.sprintf
         "Couldn't find type of method app '%s', argument might be incorrect"
         (string_of_rs_exp 0 e));
    e.e_exp
  | RsStaticApp (t, name, args) ->
    Reporting.simple_warn
      (Format.sprintf
         "Couldn't find type of static app '%s', arguments might be incorrect"
         (string_of_rs_exp 0 e));
    e.e_exp
  | RsApp (({ e_annot = _; e_exp = RsId id } as e_id), generics, args) ->
    (match ctx_fun_type id ctx with
     | Some signature ->
       (try RsApp (e_id, generics, List.map2 (cast_bitvec ctx) signature.args args) with
        | Invalid_argument _ ->
          Format.eprintf "ERROR for %s\n" id;
          List.iter (fun t -> Format.eprintf "%s\n" (string_of_rs_type t)) signature.args;
          Format.eprintf "vs\n";
          List.iter (fun e -> Format.eprintf "%s\n" (string_of_rs_exp 0 e)) args;
          assert false)
     | None ->
       Reporting.simple_warn
         (Format.sprintf
            "Couldn't find type of function '%s', arguments might be incorrect"
            id);
       e.e_exp)
  | RsApp (e, generics, args) ->
    Reporting.simple_warn
      (Format.sprintf
         "Couldn't find type of app '%s', arguments might be incorrect"
         (string_of_rs_exp 0 e));
    e.e_exp
  | _ -> e.e_exp
;;

let use_dynamic_bitvec_exp (ctx : context) (e : rs_exp) : rs_exp_aux =
  let e_exp = use_dynamic_bitvec_exp_in_app ctx e in
  e_exp
;;

let use_dynamic_bitvec_args (ctx : context) (rust_program : rs_program) : rs_program =
  let ctx = update_context_fn_type ctx rust_program in
  rust_transform_expr
    { id_expr_type_transform with exp_aux = use_dynamic_bitvec_exp }
    ctx
    rust_program
;;

(* —————————————————————————— Expression Optimizer —————————————————————————— *)

(** Try to find the matching branch for match expression with a known integer to match. **)
let rec find_match_branch_opt (n : Big_int.num) (branches : rs_pexp list) =
  match branches with
  | branch :: tail ->
    (match branch with
     | RsPexp (RsPatLit (RsLitNum m), exp) when Big_int.equal n m -> Some exp
     | RsPexpWhen
         ( RsPatId id
         , { e_annot = _
           ; e_exp =
               RsBinop
                 ( { e_annot = _; e_exp = RsId id' }
                 , RsBinopEq
                 , { e_annot = _; e_exp = RsLit (RsLitNum m) } )
           }
         , exp )
       when id = id' && Big_int.equal n m -> Some exp
     | _ -> find_match_branch_opt n tail)
  | [] -> None
;;

(** Simplifies rust expression by applying basic optimisations.

 For now, this mostly includes arithmetic operators.**)
let simplify_rs_exp_aux (ctx : context) (rs_exp : rs_exp) : rs_exp_aux =
  match rs_exp.e_exp with
  | RsBinop
      ( { e_annot = _; e_exp = RsLit (RsLitNum a) }
      , RsBinopAdd
      , { e_annot = _; e_exp = RsLit (RsLitNum b) } ) ->
    RsLit (RsLitNum (Big_int.add a b))
  | RsBinop
      ( { e_annot = _; e_exp = RsLit (RsLitNum a) }
      , RsBinopSub
      , { e_annot = _; e_exp = RsLit (RsLitNum b) } ) ->
    RsLit (RsLitNum (Big_int.sub a b))
  | RsBinop
      ( { e_annot = _; e_exp = RsLit (RsLitNum a) }
      , RsBinopMult
      , { e_annot = _; e_exp = RsLit (RsLitNum b) } ) ->
    RsLit (RsLitNum (Big_int.mul a b))
  | RsBinop
      ( { e_annot = _; e_exp = RsLit (RsLitNum a) }
      , RsBinopEq
      , { e_annot = _; e_exp = RsLit (RsLitNum b) } ) ->
    if Big_int.equal a b then RsLit RsLitTrue else RsLit RsLitFalse
  | RsBinop
      ( { e_annot = _; e_exp = RsLit (RsLitNum a) }
      , RsBinopGe
      , { e_annot = _; e_exp = RsLit (RsLitNum b) } ) ->
    if Big_int.greater_equal a b then RsLit RsLitTrue else RsLit RsLitFalse
  (* NOTE: here we assume there is no side effects in the condition
           checks. is it is expected that some expression should be performed
           for their side effects, then this will introduce logic bugs. We
           could imagine tracking function purity in the future to work around
           that limitation. *)
  | RsBinop ({ e_annot = _; e_exp = RsLit RsLitTrue }, RsBinopLAnd, exp)
  | RsBinop (exp, RsBinopLAnd, { e_annot = _; e_exp = RsLit RsLitTrue }) -> exp.e_exp
  (* NOTE: here we assume there is no side effects in the condition
           checks. is it is expected that some expression should be performed
           for their side effects, then this will introduce logic bugs. We
           could imagine tracking function purity in the future to work around
           that limitation. *)
  | RsBinop ({ e_annot = _; e_exp = RsLit RsLitFalse }, RsBinopLAnd, _exp)
  | RsBinop (_exp, RsBinopLAnd, { e_annot = _; e_exp = RsLit RsLitFalse }) ->
    RsLit RsLitFalse
  (* NOTE: here we assume there is no side effects in the condition
           checks. is it is expected that some expression should be performed
           for their side effects, then this will introduce logic bugs. We
           could imagine tracking function purity in the future to work around
           that limitation. *)
  | RsBinop ({ e_annot = _; e_exp = RsLit RsLitTrue }, RsBinopLOr, _exp)
  | RsBinop (_exp, RsBinopLOr, { e_annot = _; e_exp = RsLit RsLitTrue }) ->
    RsLit RsLitTrue
  | RsBinop ({ e_annot = _; e_exp = RsLit RsLitFalse }, RsBinopLOr, exp)
  | RsBinop (exp, RsBinopLOr, { e_annot = _; e_exp = RsLit RsLitFalse }) -> exp.e_exp
  | RsIf ({ e_annot = _; e_exp = RsLit RsLitTrue }, if_branch, _else_branch) ->
    if_branch.e_exp
  | RsIf ({ e_annot = _; e_exp = RsLit RsLitFalse }, _if_branch, else_branch) ->
    else_branch.e_exp
  | RsMatch ({ e_annot = _; e_exp = RsLit (RsLitNum n) }, branches) ->
    (match find_match_branch_opt n branches with
     | Some exp -> exp.e_exp
     | None -> rs_exp.e_exp)
  | RsMatch (exp, branches) ->
    let can_be_taken (branch : rs_pexp) =
      match branch with
      | RsPexpWhen (_, { e_annot = _; e_exp = RsLit RsLitFalse }, _) -> false
      | _ -> true
    in
    let branches = List.filter can_be_taken branches in
    (match branches with
     | [] -> RsLit RsLitUnit
     | [ RsPexp (RsPatWildcard, exp) ] -> exp.e_exp
     | _ -> RsMatch (exp, branches))
  | RsApp
      ( { e_annot = _; e_exp = RsPathSeparator (_rs_type_int, RsTypId "pow") }
      , []
      , [ { e_annot = _; e_exp = RsLit (RsLitNum n) }
        ; { e_annot = _; e_exp = RsAs ({ e_annot = _; e_exp = RsLit (RsLitNum m) }, _) }
        ] )
  | RsStaticApp
      ( _rs_type_int
      , "pow"
      , [ { e_annot = _; e_exp = RsLit (RsLitNum n) }
        ; { e_annot = _; e_exp = RsAs ({ e_annot = _; e_exp = RsLit (RsLitNum m) }, _) }
        ] ) -> (mk_big_num (Big_int.pow_int n (Big_int.to_int m))).e_exp
  | RsApp ({ e_annot = _; e_exp = RsId id }, _, _) when SMap.mem id ctx.defs.inline_fun ->
    (SMap.find id ctx.defs.inline_fun).e_exp
  | RsBlock exps ->
    let is_not_unit exp =
      match exp.e_exp with
      | RsLit RsLitUnit -> false
      | _ -> true
    in
    RsBlock (List.filter is_not_unit exps)
  | RsId id when SMap.mem id ctx.defs.num_constants ->
    let n = SMap.find id ctx.defs.num_constants in
    (mk_big_num n).e_exp
  (* If a let binding is defined right before returning a boolean
           literal, then we remove the binding as it is not used until the
           binding expires.
           Note that this assumes that the binding expression has no side
           effects. *)
  | RsLet (_, _, { e_annot = _; e_exp = RsLit RsLitFalse }) -> RsLit RsLitFalse
  | RsLet (_, _, { e_annot = _; e_exp = RsLit RsLitTrue }) -> RsLit RsLitTrue
  | _ -> rs_exp.e_exp
;;

let simplify_rs_exp (ctx : context) (rs_exp : rs_exp) : rs_exp =
  { rs_exp with e_exp = simplify_rs_exp_aux ctx rs_exp }
;;

let expression_optimizer : expr_type_transform =
  { id_expr_type_transform with exp_aux = simplify_rs_exp_aux }
;;

(* —————————————————————————— Constant Propagation —————————————————————————— *)

type bindings = rs_exp SMap.t

let rec invalidate_bindings (ctx : bindings) (ids : string list) : bindings =
  match ids with
  | id :: tail ->
    let ctx' = SMap.remove id ctx in
    invalidate_bindings ctx' tail
  | [] -> ctx
;;

let rec propagate_in_exp_aux (ctx : bindings) (exp : rs_exp) : rs_exp_aux =
  (* Helpers for propagating one or more exps *)
  let propagate exp = propagate_in_exp ctx exp in
  let propagate_list exps = List.map propagate exps in
  (* The actual constant propagation *)
  match exp.e_exp with
  | RsId id ->
    (match SMap.find_opt id ctx with
     | Some exp' -> exp'.e_exp (* The constant propagation happens here *)
     | None -> RsId id)
  | RsLet (RsPatId id, ({ e_annot = _; e_exp = RsLit _ } as lit), next) ->
    (* This binding is a Rust literal, we can remove it and inline the literal in the next expression *)
    let ctx' = SMap.add id lit ctx in
    (* We remove nested blocks here *)
    let next =
      match next.e_exp with
      | RsBlock [ next ] -> next
      | _ -> next
    in
    propagate_in_exp_aux ctx' next
  | RsLet (pat, pat_exp, next) ->
    (* We need to invalidate all bindings that are being re-defined *)
    let new_ids = ids_of_pat pat in
    let ctx' = invalidate_bindings ctx (SSet.to_list new_ids) in
    (* We keep the old bindings in the let expression, but we use the new bindings in the body of the let expression *)
    RsLet (pat, propagate pat_exp, propagate_in_exp ctx' next)
  | RsLetMut (pat, pat_exp, next) ->
    (* We need to invalidate all bindings that are being re-defined *)
    let new_ids = ids_of_pat pat in
    let ctx' = invalidate_bindings ctx (SSet.to_list new_ids) in
    (* We keep the old bindings in the let expression, but we use the new bindings in the body of the let expression *)
    RsLetMut (pat, propagate pat_exp, propagate_in_exp ctx' next)
  | RsApp (fn, generics, args) -> RsApp (propagate fn, generics, propagate_list args)
  | RsMethodApp app ->
    RsMethodApp { app with exp = propagate app.exp; args = propagate_list app.args }
  | RsStaticApp (typ, name, args) -> RsStaticApp (typ, name, propagate_list args)
  | RsLit lit -> RsLit lit
  | RsField (exp, field) -> RsField (propagate exp, field)
  | RsBlock exps -> RsBlock (propagate_list exps)
  | RsConstBlock exps -> RsConstBlock (propagate_list exps)
  | RsInstrList exps -> RsInstrList (propagate_list exps)
  | RsIf (cond, if_branch, else_branch) ->
    RsIf (propagate cond, propagate if_branch, propagate else_branch)
  | RsMatch (exp, pexps) -> RsMatch (propagate exp, List.map (propagate_in_pexp ctx) pexps)
  | RsTuple exps -> RsTuple (propagate_list exps)
  | RsArray exps -> RsArray (propagate_list exps)
  | RsArraySize (exp, size) -> RsArraySize (propagate exp, propagate size)
  | RsVec exps -> RsArray (propagate_list exps)
  | RsVecSize (exp, size) -> RsArraySize (propagate exp, propagate size)
  | RsAssign (lexp, exp) -> RsAssign (propagate_in_lexp ctx lexp, propagate exp)
  | RsIndex (exp1, exp2) -> RsIndex (propagate exp1, propagate exp2)
  | RsBinop (exp1, op, exp2) -> RsBinop (propagate exp1, op, propagate exp2)
  | RsUnop (op, exp) -> RsUnop (op, propagate exp)
  | RsAs (exp, typ) -> RsAs (propagate exp, typ)
  | RsSome exp -> RsSome (propagate exp)
  | RsNone -> RsNone
  | RsPathSeparator (typ, typ') -> RsPathSeparator (typ, typ')
  | RsFor (typ, lit, lit', exp) -> RsFor (typ, lit, lit', propagate exp)
  | RsForRev (typ, lit, lit', exp) -> RsForRev (typ, lit, lit', propagate exp)
  | RsStruct (typ, fields) ->
    RsStruct (typ, List.map (fun (s, exp) -> s, propagate exp) fields)
  | RsStructAssign (st, field, value) ->
    RsStructAssign (propagate st, field, propagate value)
  | RsReturn exp -> RsReturn (propagate exp)
  | RsTodo s -> RsTodo s
  | RsBorrow exp -> RsBorrow (propagate exp)

and propagate_in_exp (ctx : bindings) (exp : rs_exp) : rs_exp =
  { e_annot = exp.e_annot; e_exp = propagate_in_exp_aux ctx exp }

and propagate_in_pexp (ctx : bindings) (pexp : rs_pexp) : rs_pexp =
  match (pexp : rs_pexp) with
  (* We need to invalidate bindings that are being re-defined *)
  (* Note: we could further optimize patters of the form `x if x == y`, but for now we don't. *)
  | RsPexp (pat, exp) ->
    let new_ids = ids_of_pat pat in
    let ctx' = invalidate_bindings ctx (SSet.to_list new_ids) in
    RsPexp (pat, propagate_in_exp ctx' exp)
  | RsPexpWhen (pat, cond, exp) ->
    let new_ids = ids_of_pat pat in
    let ctx' = invalidate_bindings ctx (SSet.to_list new_ids) in
    RsPexpWhen (pat, propagate_in_exp ctx' cond, propagate_in_exp ctx' exp)

and propagate_in_lexp (ctx : bindings) (lexp : rs_lexp) : rs_lexp =
  let propagate lexp = propagate_in_lexp ctx lexp in
  match (lexp : rs_lexp) with
  | RsLexpId id -> RsLexpId id
  | RsLexpTyp (id, typ) -> RsLexpTyp (id, typ)
  | RsLexpField (fexp, field) -> RsLexpField (propagate_in_exp ctx fexp, field)
  | RsLexpIndex (lexp, exp) -> RsLexpIndex (propagate lexp, propagate_in_exp ctx exp)
  | RsLexpIndexRange (lexp, start, end') ->
    RsLexpIndexRange
      (propagate lexp, propagate_in_exp ctx start, propagate_in_exp ctx end')
  | RsLexpBitVectorAccess (lexp, exp) ->
    RsLexpBitVectorAccess (propagate_in_lexp ctx lexp, propagate_in_exp ctx exp)
  | RsLexpTodo -> RsLexpTodo
;;

(** Perform constant propagation, inlining all variables bound to literal values. **)
let constant_propagation (program : rs_program) : rs_program =
  let propagate_in_fn fn = { fn with body = propagate_in_exp SMap.empty fn.body } in
  let propagate obj =
    match obj with
    | RsFn fn -> RsFn (propagate_in_fn fn)
    | _ -> obj
  in
  let (RsProg objs) = program in
  RsProg (List.map propagate objs)
;;

(* ——————————————————————————— Nested Blocks remover ———————————————————————————— *)

(** Sail often generates blocks in constructs such as if statements, for which
    we already have blocks. This transformation removes the nested blocks. **)
let nested_block_remover_exp (_ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsIf (c, { e_annot = t1; e_exp = RsBlock e1 }, { e_annot = t2; e_exp = RsBlock e2 })
    ->
    RsIf
      ( c
      , { e_annot = t1; e_exp = RsInstrList e1 }
      , { e_annot = t2; e_exp = RsInstrList e2 } )
  | RsIf (c, { e_annot = t1; e_exp = RsBlock e1 }, e2) ->
    RsIf (c, { e_annot = t1; e_exp = RsInstrList e1 }, e2)
  | RsIf (c, e1, { e_annot = t2; e_exp = RsBlock e2 }) ->
    RsIf (c, e1, { e_annot = t2; e_exp = RsInstrList e2 })
  | RsFor (var, start, until, { e_annot = t; e_exp = RsBlock b }) ->
    RsFor (var, start, until, { e_annot = t; e_exp = RsInstrList b })
  | e -> e
;;

let nested_block_remover : expr_type_transform =
  { id_expr_type_transform with exp_aux = nested_block_remover_exp }
;;

(* ——————————————————————————— Native functions transformation ———————————————————————————— *)

let unsupported_fun : SSet.t =
  SSet.of_list
    [ (* Used only for side effects, not necessary in the Rust back-end *)
      "csr_name_write_callback"
    ; "csr_id_write_callback"
    ; "csr_full_write_callback"
    ; "long_csr_write_callback"
    ; "hex_bits_forwards"
    ]
;;

(* TODO: This list is probably incomplete and we might want to add extra fields in the future *)
let native_func_transform_exp (_ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsApp ({ e_annot = _; e_exp = RsId "add_atom" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopAdd, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "sub_atom" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopSub, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "negate_atom" }, _gens, [ e1 ]) ->
    RsUnop (RsUnopNeg, e1)
  | RsApp ({ e_annot = _; e_exp = RsId "ediv_int" }, _gens, _) ->
    RsId "BUILTIN_atom_ediv_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "emod_int" }, _gens, [ e1; e2 ]) ->
    RsBinop (mk_as e1 rs_type_usize, RsBinopMod, mk_as e2 rs_type_usize)
  | RsApp ({ e_annot = _; e_exp = RsId "abs_int_atom" }, _gens, [ e ]) ->
    RsStaticApp (rs_type_int, "abs", [ e ])
  | RsApp ({ e_annot = _; e_exp = RsId "not_bool" }, _gens, [ e ]) -> RsUnop (RsUnopNot, e)
  | RsApp ({ e_annot = _; e_exp = RsId "not_vec" }, _gens, [ v ]) -> RsUnop (RsUnopNot, v)
  | RsApp ({ e_annot = _; e_exp = RsId "eq_bit" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopEq, e2) (* TODO Is it correct to compare like that? *)
  | RsApp ({ e_annot = _; e_exp = RsId "eq_bool" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopEq, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "eq_string" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopEq, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "eq_int" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopEq, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "not" }, _gens, [ b ]) -> RsUnop (RsUnopNot, b)
  | RsApp ({ e_annot = _; e_exp = RsId "lt" }, _gens, _) -> RsId "BUILTIN_lt_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "lteq" }, _gens, _) -> RsId "BUILTIN_lteq_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "lteq_int" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopLe, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "gt" }, _gens, _) -> RsId "BUILTIN_gt_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "gteq" }, _gens, _) -> RsId "BUILTIN_gteq_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_length" }, gens, [ vec ]) ->
    RsMethodApp { exp = vec; name = "len"; generics = gens; args = [] }
  | RsApp ({ e_annot = _; e_exp = RsId "add_int" }, _gens, _) ->
    RsId "BUILTIN_add_int_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "sub_int" }, _gens, _) ->
    RsId "BUILTIN_sub_int_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "mult_int" }, _gens, _) ->
    RsId "BUILTIN_mult_int_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "neg_int" }, _gens, _) ->
    RsId "BUILTIN_neg_int_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "abs_int" }, _gens, _) ->
    RsId "BUILTIN_abs_int_TODO"
  (* | RsApp (RsId "max_int", _gens, _) -> RsId "BUILTIN_max_int_TODO" *)
  (*| RsApp (RsId "min_int", _gens, _) -> RsId "BUILTIN_min_int_TODO" *)
  | RsApp ({ e_annot = _; e_exp = RsId "tdiv_int" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopDiv, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "tmod_int" }, _gens, _) ->
    RsId "BUILTIN_tmod_int_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "pow2" }, [], [ n ]) ->
    RsApp
      ( { e_annot = None; e_exp = RsPathSeparator (rs_type_int, RsTypId "pow") }
      , []
      , [ mk_num 2; mk_as n (RsTypId "u32") ] )
  | RsApp ({ e_annot = _; e_exp = RsId "quot_positive_round_zero" }, [], [ a; b ]) ->
    RsBinop (a, RsBinopDiv, b)
  (* | RsApp (RsId "zeros", _gens, _) -> RsId "BUILTIN_zeros_TODO" *)
  (*| RsApp (RsId "ones", _gens, e) -> RsApp (RsId "ones", e) Handled by the integrated library *)
  (* Implemented in lib.sail *)
  (*| RsApp (RsId "zero_extend", _gens, e) -> RsApp (RsId "zero_extend", e)
    | RsApp (RsId "sign_extend", _gens, e) -> RsApp (RsId "sign_extend", e)
    | RsApp (RsId "sail_ones", _gens, e) -> RsApp (RsId "sail_ones", e) *)
  | RsApp ({ e_annot = _; e_exp = RsId "sail_signed" }, _gens, _) ->
    RsId "BUILTIN_sail_signed_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "sail_unsigned" }, _gens, _) ->
    RsId "BUILTIN_sail_unsigned_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "slice" }, _gens, args) ->
    RsApp (mk_exp_id "slice", [], args)
  | RsApp ({ e_annot = _; e_exp = RsId "slice_inc" }, _gens, _) ->
    RsId "BUILTIN_slice_inc_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "add_bits" }, _gens, _) ->
    RsId "BUILTIN_add_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "add_bits_int" }, _gens, [ b1; b2 ]) ->
    RsBinop (b1, RsBinopAdd, b2)
  | RsApp ({ e_annot = _; e_exp = RsId "sub_bits" }, _gens, _) ->
    RsId "BUILTIN_sub_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "sub_bits_int" }, _gens, _) ->
    RsId "BUILTIN_sub_bits_int_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "append" }, _gens, _) -> RsId "BUILTIN_append_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "eq_bits" }, _gens, _) ->
    RsId "BUILTIN_eq_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "neq_bits" }, _gens, _) ->
    RsId "BUILTIN_neq_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "not_bits" }, _gens, _) ->
    RsId "BUILTIN_not_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "sail_truncate" }, _gens, _) ->
    RsId "BUILTIN_sail_truncate_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "sail_truncateLSB" }, _gens, _) ->
    RsId "BUILTIN_sail_truncateLSB_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "shiftl" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopShiftLeft, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "shiftr" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopShiftRight, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "arith_shiftr" }, _gens, _) ->
    RsId "BUILTIN_arith_shiftr_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "and_bits" }, _gens, _) ->
    RsId "BUILTIN_and_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "or_bits" }, _gens, _) ->
    RsId "BUILTIN_or_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "xor_bits" }, _gens, _) ->
    RsId "BUILTIN_xor_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_init" }, _gens, _) ->
    RsId "BUILTIN_vector_init_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_access" }, _gens, _) ->
    RsId "BUILTIN_vector_access_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_access_inc" }, _gens, _) ->
    RsId "BUILTIN_vector_access_inc_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_subrange" }, _gens, _) ->
    RsId "BUILTIN_vector_subrange_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_subrange_inc" }, _gens, _) ->
    RsId "BUILTIN_vector_subrange_inc_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_update" }, _gens, _) ->
    RsId "BUILTIN_vector_update_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_update_inc" }, _gens, _) ->
    RsId "BUILTIN_vector_update_inc_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_update_subrange" }, _gens, _) ->
    RsId "BUILTIN_vector_update_subrange_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "vector_update_subrange_inc" }, _gens, _) ->
    RsId "BUILTIN_vector_update_subrange_inc_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "length" }, _gens, _) -> RsId "BUILTIN_length_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "replicate_bits" }, _gens, _) ->
    RsId "BUILTIN_replicate_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "count_leading_zeros" }, _gens, _) ->
    RsId "BUILTIN_count_leading_zeros_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "eq_real" }, _gens, _) ->
    RsId "BUILTIN_eq_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "neg_real" }, _gens, _) ->
    RsId "BUILTIN_neg_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "add_real" }, _gens, _) ->
    RsId "BUILTIN_add_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "sub_real" }, _gens, _) ->
    RsId "BUILTIN_sub_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "mult_real" }, _gens, _) ->
    RsId "BUILTIN_mult_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "div_real" }, _gens, _) ->
    RsId "BUILTIN_div_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "lt_real" }, _gens, _) ->
    RsId "BUILTIN_lt_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "gt_real" }, _gens, _) ->
    RsId "BUILTIN_gt_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "lteq_real" }, _gens, _) ->
    RsId "BUILTIN_lteq_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "gteq_real" }, _gens, _) ->
    RsId "BUILTIN_gteq_real_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "concat_str" }, gens, [ s1; s2 ]) ->
    RsApp
      ( { e_annot = None; e_exp = RsId "format!" }
      , gens
      , [ { e_annot = None; e_exp = RsId "\"{}{}\"" }; s1; s2 ] )
    (* There is a bug with hoisting here *)
  | RsApp ({ e_annot = _; e_exp = RsId "print_bits" }, gens, e) ->
    RsApp ({ e_annot = None; e_exp = RsId "print_output" }, gens, e)
  | RsApp ({ e_annot = _; e_exp = RsId "string_of_bits" }, _gens, _) ->
    RsId "BUILTIN_string_of_bits_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "dec_str" }, gens, e) ->
    RsApp ({ e_annot = None; e_exp = RsId "dec_str" }, gens, e)
    (* Handled by an external lib *)
  | RsApp ({ e_annot = _; e_exp = RsId "hex_str" }, gens, e) ->
    RsApp ({ e_annot = None; e_exp = RsId "hex_str" }, gens, e)
    (* Handled by an external lib *)
  | RsApp ({ e_annot = _; e_exp = RsId "hex_str_upper" }, _gens, _) ->
    RsId "BUILTIN_hex_str_upper_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "sail_assert" }, _gens, _) ->
    RsId "BUILTIN_sail_assert_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "reg_deref" }, _gens, _) ->
    RsId "BUILTIN_reg_deref_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "sail_cons" }, _gens, _) ->
    RsId "BUILTIN_sail_cons_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "eq_anything" }, _gens, _) ->
    RsId "BUILTIN_eq_anything_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "id" }, _gens, _) -> RsId "BUILTIN_id_TODO"
  | RsApp ({ e_annot = _; e_exp = RsId "gteq_int" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopGe, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "lt_int" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopLt, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "gt_int" }, _gens, [ e1; e2 ]) ->
    RsBinop (e1, RsBinopGt, e2)
  | RsApp ({ e_annot = _; e_exp = RsId "internal_error" }, _gens, [ file; line; message ])
    ->
    RsApp
      ( { e_annot = None; e_exp = RsId "panic!" }
      , []
      , [ { e_annot = None; e_exp = RsLit (RsLitStr "{}, l {}: {}") }
        ; file
        ; line
        ; message
        ] )
  | RsApp ({ e_annot = _; e_exp = RsId id }, _gens, _) when SSet.mem id unsupported_fun ->
    RsLit RsLitUnit
  | e -> e
;;

let native_func_transform : expr_type_transform =
  { id_expr_type_transform with exp_aux = native_func_transform_exp }
;;

(* ———————————————————————— Hoisting rewriting  ————————————————————————— *)

let create_variable_generator () =
  let counter = ref 0 in
  fun () ->
    counter := !counter + 1;
    Printf.sprintf "var_%d" !counter
;;

let variable_generator = ref (create_variable_generator ())
let reset_variable_generator () = variable_generator := create_variable_generator ()

let rec rename_in_exp_aux (rn : string * string) (exp : rs_exp) : rs_exp_aux =
  (* Helpers for renaming one or more exps *)
  let rename_in_exp exp = rename_in_exp rn exp in
  let rename_in_exps exps = List.map rename_in_exp exps in
  (* The actual renaming *)
  let id, new_id = rn in
  match exp.e_exp with
  | RsId id' -> if id' = id then RsId new_id (* Rename! *) else RsId id' (* No renaming *)
  | RsLet (pat, exp, next) ->
    let new_ids = ids_of_pat pat in
    if SSet.mem id new_ids
    then
      (* The ID is being shadowed, stop renaming at that point *)
      RsLet (pat, rename_in_exp exp, next)
    else
      (* The ID is not shadowed, so we need to continue the renaming *)
      RsLet (pat, rename_in_exp exp, rename_in_exp next)
  | RsLetMut (pat, exp, next) ->
    let new_ids = ids_of_pat pat in
    if SSet.mem id new_ids
    then
      (* The ID is being shadowed, stop renaming at that point *)
      RsLetMut (pat, rename_in_exp exp, next)
    else
      (* The ID is not shadowed, so we need to continue the renaming *)
      RsLetMut (pat, rename_in_exp exp, rename_in_exp next)
  | RsApp (fn, generics, args) -> RsApp (rename_in_exp fn, generics, rename_in_exps args)
  | RsMethodApp app ->
    RsMethodApp { app with exp = rename_in_exp app.exp; args = rename_in_exps app.args }
  | RsStaticApp (typ, name, args) -> RsStaticApp (typ, name, rename_in_exps args)
  | RsLit lit -> RsLit lit
  | RsField (exp, field) -> RsField (rename_in_exp exp, field)
  | RsBlock exps -> RsBlock (rename_in_exps exps)
  | RsConstBlock exps -> RsConstBlock (rename_in_exps exps)
  | RsInstrList exps -> RsInstrList (rename_in_exps exps)
  | RsIf (cond, if_branch, else_branch) ->
    RsIf (rename_in_exp cond, rename_in_exp if_branch, rename_in_exp else_branch)
  | RsMatch (exp, pexps) -> RsMatch (rename_in_exp exp, List.map (rename_in_pexp rn) pexps)
  | RsTuple exps -> RsTuple (rename_in_exps exps)
  | RsArray exps -> RsArray (rename_in_exps exps)
  | RsArraySize (exp, size) -> RsArraySize (rename_in_exp exp, rename_in_exp size)
  | RsVec exps -> RsVec (rename_in_exps exps)
  | RsVecSize (exp, size) -> RsVecSize (rename_in_exp exp, rename_in_exp size)
  | RsAssign (lexp, exp) -> RsAssign (rename_in_lexp rn lexp, rename_in_exp exp)
  | RsIndex (exp1, exp2) -> RsIndex (rename_in_exp exp1, rename_in_exp exp2)
  | RsBinop (exp1, op, exp2) -> RsBinop (rename_in_exp exp1, op, rename_in_exp exp2)
  | RsUnop (op, exp) -> RsUnop (op, rename_in_exp exp)
  | RsAs (exp, typ) -> RsAs (rename_in_exp exp, typ)
  | RsSome exp -> RsSome (rename_in_exp exp)
  | RsNone -> RsNone
  | RsPathSeparator (typ, typ') -> RsPathSeparator (typ, typ')
  | RsFor (typ, lit, lit', exp) -> RsFor (typ, lit, lit', rename_in_exp exp)
  | RsForRev (typ, lit, lit', exp) -> RsForRev (typ, lit, lit', rename_in_exp exp)
  | RsStruct (typ, fields) ->
    RsStruct (typ, List.map (fun (s, exp) -> s, rename_in_exp exp) fields)
  | RsStructAssign (st, field, value) ->
    RsStructAssign (rename_in_exp st, field, rename_in_exp value)
  | RsReturn exp -> RsReturn (rename_in_exp exp)
  | RsTodo s -> RsTodo s
  | RsBorrow exp -> RsBorrow (rename_in_exp exp)

and rename_in_exp (rn : string * string) (exp : rs_exp) : rs_exp =
  { exp with e_exp = rename_in_exp_aux rn exp }

and rename_in_pexp (rn : string * string) (pexp : rs_pexp) : rs_pexp =
  let id, _new_id = rn in
  match (pexp : rs_pexp) with
  (* First case: the ID is being shadowed, stop renaming at that point *)
  | (RsPexp (pat, _) | RsPexpWhen (pat, _, _)) when SSet.mem id (ids_of_pat pat) -> pexp
  (* Second case: the IS is not shadowed, continue renaming*)
  | RsPexp (pat, exp) -> RsPexp (pat, rename_in_exp rn exp)
  | RsPexpWhen (pat, cond, exp) ->
    RsPexpWhen (pat, rename_in_exp rn cond, rename_in_exp rn exp)

and rename_in_lexp (rn : string * string) (lexp : rs_lexp) : rs_lexp =
  let rename_in_lexp lexp = rename_in_lexp rn lexp in
  let id, new_id = rn in
  match (lexp : rs_lexp) with
  | RsLexpId id' ->
    if id' == id then RsLexpId new_id (* Rename! *) else RsLexpId id' (* No renaming *)
  | RsLexpTyp (id', typ) ->
    if id' == id
    then RsLexpTyp (new_id, typ) (* Rename! *)
    else RsLexpTyp (id', typ) (* No renaming *)
  | RsLexpField (fexp, field) -> RsLexpField (rename_in_exp rn fexp, field)
  | RsLexpIndex (lexp, exp) -> RsLexpIndex (rename_in_lexp lexp, rename_in_exp rn exp)
  | RsLexpIndexRange (lexp, start, end') ->
    RsLexpIndexRange (rename_in_lexp lexp, rename_in_exp rn start, rename_in_exp rn end')
  | RsLexpBitVectorAccess (lexp, exp) ->
    RsLexpBitVectorAccess (rename_in_lexp lexp, rename_in_exp rn exp)
  | RsLexpTodo -> RsLexpTodo
;;

(* Wheter an expression should be hoisted. *)
let rec should_hoist_exp (is_nested : bool) (exp : rs_exp) : bool =
  let core_ctx = RsId core_ctx in
  let contain_core_ctx args =
    List.exists (fun { e_annot = _; e_exp = e } -> e = core_ctx) args
  in
  match exp.e_exp with
  | RsApp (_exp, _generics, args) -> contain_core_ctx args || should_hoist_args args
  | RsMethodApp app ->
    should_hoist_exp true app.exp
    || contain_core_ctx app.args
    || should_hoist_args app.args
  | RsIf (_, _, _) -> true
  | RsField (e, _e2) -> should_hoist_exp true e
  | RsBinop (e1, _op, e2) -> should_hoist_exp true e1 || should_hoist_exp true e2
  | RsUnop (_op, e) -> should_hoist_exp true e
  | RsMatch _ -> true
  | e when e = core_ctx && is_nested -> true
  | _ -> false

(* Whether at least one argument should be hoisted *)
and should_hoist_args (args : rs_exp list) : bool =
  List.fold_left (fun acc arg -> acc || should_hoist_exp false arg) false args
;;

let rec hoist (exp : rs_exp list) : rs_exp list * rs_exp list =
  match exp with
  | e :: arr when should_hoist_exp false e ->
    let ident = !variable_generator () in
    let l1, l2 = hoist arr in
    ( { e_annot = None; e_exp = RsLet (RsPatId ident, e, mk_todo "hoist") } :: l1
    , mk_exp_id ident :: l2 )
  | e :: arr ->
    let l1, l2 = hoist arr in
    l1, e :: l2
  | [] -> [], []
;;

let rec generate_hoisted_block (exp : rs_exp list) app : rs_exp =
  match exp with
  | { e_annot = t; e_exp = RsLet (pat, exp2, _) } :: arr ->
    { e_annot = t; e_exp = RsLet (pat, exp2, generate_hoisted_block arr app) }
  | [] -> app
  | _ -> failwith "Unreachable code"
;;

let rec hoist_let_exp_aux (exp : rs_exp) : rs_exp_aux * (rs_pat * rs_exp) list =
  let hoit_let_exp_list (exps : rs_exp list) : rs_exp list * (rs_pat * rs_exp) list =
    let accumulate acc exp =
      let exp, defs = hoist_let_exp exp in
      acc @ [ exp, defs ]
    in
    let list = List.fold_left accumulate [] exps in
    let exps, defs = List.split list in
    exps, List.flatten defs
  in
  match exp.e_exp with
  | RsLet (pat, exp, next) ->
    (* We generate a new ID to to avoid shadowing existing variables when hoisting the let statement. *)
    let build_new_id id = id ^ "_" ^ !variable_generator () in
    let rename, pat =
      match pat with
      | RsPatId id ->
        let new_id = build_new_id id in
        Some (id, new_id), RsPatId new_id
      | RsPatType (typ, RsPatId id) ->
        let new_id = build_new_id id in
        Some (id, new_id), RsPatType (typ, RsPatId new_id)
      | _ -> None, pat
    in
    (match rename with
     (* We decided to hoist that definition *)
     | Some rename ->
       let def = pat, exp in
       let next = rename_in_exp rename next in
       let next, defs = hoist_let_exp_aux next in
       next, def :: defs
     (* We will not hoist that definition *)
     | None ->
       let next, defs = hoist_let_exp next in
       RsLet (pat, exp, next), defs)
  | RsApp (fn, generics, args) ->
    let fn, defs = hoist_let_exp fn in
    let args, defs_args = hoit_let_exp_list args in
    RsApp (fn, generics, args), defs @ defs_args
  | RsMethodApp app ->
    let exp, defs = hoist_let_exp app.exp in
    let args, defs_args = hoit_let_exp_list app.args in
    RsMethodApp { app with exp; args }, defs @ defs_args
  | RsStaticApp (typ, name, exps) ->
    let exps, defs = hoit_let_exp_list exps in
    RsStaticApp (typ, name, exps), defs
  | RsField (exp, field) ->
    let exp, defs = hoist_let_exp exp in
    RsField (exp, field), defs
  | RsBlock exps ->
    let exps, defs = hoit_let_exp_list exps in
    RsBlock exps, defs
  | RsInstrList exps ->
    let exps, defs = hoit_let_exp_list exps in
    RsInstrList exps, defs
  | RsTuple exps ->
    let exps, defs = hoit_let_exp_list exps in
    RsTuple exps, defs
  | RsIndex (exp1, exp2) ->
    let exp1, defs1 = hoist_let_exp exp1 in
    let exp2, defs2 = hoist_let_exp exp2 in
    RsIndex (exp1, exp2), defs1 @ defs2
  | RsBinop (exp1, op, exp2) ->
    let exp1, defs1 = hoist_let_exp exp1 in
    let exp2, defs2 = hoist_let_exp exp2 in
    RsBinop (exp1, op, exp2), defs1 @ defs2
  | RsUnop (unop, exp) ->
    let exp, defs = hoist_let_exp exp in
    RsUnop (unop, exp), defs
  | RsAs (exp, typ) ->
    let exp, defs = hoist_let_exp exp in
    RsAs (exp, typ), defs
  | _ -> exp.e_exp, []

and hoist_let_exp (exp : rs_exp) : rs_exp * (rs_pat * rs_exp) list =
  let e_exp, defs = hoist_let_exp_aux exp in
  { exp with e_exp }, defs
;;

let pexp_hoister (_ctx : context) (pexp : rs_pexp) : rs_pexp =
  match pexp with
  | RsPexpWhen (pat, cond, exp) ->
    let cond, defs = hoist_let_exp cond in
    let rec build_cond exp defs =
      match defs with
      | (pat, binding) :: tail ->
        { e_annot = exp.e_annot; e_exp = RsLet (pat, binding, build_cond exp tail) }
      | [] -> exp
    in
    let cond = build_cond cond defs in
    RsPexpWhen (pat, cond, exp)
  | _ -> pexp
;;

let expr_hoister (ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  (* We dont need to hoist external functions & some macro might not work with hoisting (for example: format!)*)
  | RsApp (({ e_annot = _; e_exp = RsId name } as id), generics, args)
    when should_hoist_args args && not (SMap.mem name ctx.arch.external_func) ->
    let ret = hoist args in
    RsBlock
      [ generate_hoisted_block
          (fst ret)
          { e_annot = exp.e_annot; e_exp = RsApp (id, generics, snd ret) }
      ]
  | RsMethodApp { exp; name; generics; args } when should_hoist_args args ->
    let ret = hoist args in
    RsBlock
      [ generate_hoisted_block
          (fst ret)
          { e_annot = exp.e_annot
          ; e_exp = RsMethodApp { exp; name; generics; args = snd ret }
          }
      ]
  | RsIf (cond, if_branch, else_branch) ->
    let cond, defs = hoist_let_exp cond in
    let rec build_cond (exp : rs_exp) defs : rs_exp =
      match defs with
      | (pat, binding) :: tail ->
        { e_annot = None; e_exp = RsLet (pat, binding, build_cond exp tail) }
      | [] -> exp
    in
    let cond = build_cond cond defs in
    RsIf (cond, if_branch, else_branch)
  | e -> e
;;

let obj_hoister (_ctx : context) (obj : rs_obj) : rs_obj =
  (* For each rust object we reset the counter
       This makes the IDs of variables more stable, as a change in a function
       doesn't rename variables in another *)
  reset_variable_generator ();
  obj
;;

let expr_type_hoister : expr_type_transform =
  { id_expr_type_transform with
    exp_aux = expr_hoister
  ; pexp = pexp_hoister
  ; obj = obj_hoister
  }
;;

(* ————————————————————————— VirtContext Call Graph ————————————————————————— *)
(* Infers which function needs to have the context as argument.               *)
(* The initial Sail-to-Rust translation tracks usage of registers and         *)
(* configuration, in this pass we aditionnaly check if a function calls       *)
(* another function that needs the context.                                   *)
(* —————————————————————————————————————————————————————————————————————————— *)

let exp_virt_ctx_usage (ctx : context) (exp : rs_exp) : rs_exp_aux =
  (match exp.e_exp with
   | RsApp ({ e_annot = _; e_exp = RsId fn }, _, _) ->
     (match ctx_fun fn ctx with
      | Some fn when fn.use_sail_ctx -> ctx.uses_sail_ctx <- true
      | _ -> ())
   | _ -> ());
  exp.e_exp
;;

let exp_virt_context_call_graph : expr_type_transform =
  { id_expr_type_transform with exp_aux = exp_virt_ctx_usage }
;;

let is_sail_context_needed (ctx : context) (func : rs_fn) : rs_fn =
  match func.use_sail_ctx with
  | true ->
    (* Nothing to do, we already know it uses the context *)
    func
  | false ->
    (* We need to search for any function call that needs the context *)
    ctx.uses_sail_ctx <- false;
    (* remove flag *)
    ignore (transform_fn exp_virt_context_call_graph ctx func);
    let ctx_func = Option.get (ctx_fun func.name ctx) in
    (match ctx.uses_sail_ctx with
     | true ->
       (* We need to keep the context in sync *)
       ctx_func.use_sail_ctx <- true;
       { func with use_sail_ctx = true }
     | false -> func)
;;

let virt_context_call_graph (ctx : context) (rust_program : rs_program) : rs_program =
  let ctx = update_context_fn_type ctx rust_program in
  rust_transform_func { func = is_sail_context_needed } ctx rust_program
;;

(* ———————————————————————— VirtContext transformer ————————————————————————— *)
(* Adds a virtual context as first argument to all functions.                 *)
(* —————————————————————————————————————————————————————————————————————————— *)

let sail_context_inserter (_ctx : context) (func : rs_fn) : rs_fn =
  if func.use_sail_ctx then Format.eprintf "Inserting context argument for %s\n" func.name;
  if func.use_sail_ctx
  then
    { func with
      args = RsPatId core_ctx :: func.args
    ; signature =
        { func.signature with args = RsTypId "&mut Core" :: func.signature.args }
    }
  else func
;;

let virt_context_transform (ctx : context) (rust_program : rs_program) : rs_program =
  let ctx = update_context_fn_type ctx rust_program in
  rust_transform_func { func = sail_context_inserter } ctx rust_program
;;

(* —————————————————————————— Enum Args Namespace ——————————————————————————— *)
(* Sail does not need to namespace its enum, but Rust does. This pass adds    *)
(* the approriate namespaces to all enum arguments.                           *)
(* —————————————————————————————————————————————————————————————————————————— *)

let add_namespace_to_arg_pats (ctx : context) (func : rs_fn) : rs_fn =
  let rec get_namespace enum enum_list =
    match enum_list with
    | (k, v) :: _ when k = enum -> Some v
    | _ :: tail -> get_namespace enum tail
    | [] -> None
  in
  (* Add the proper enum namespace to all enum pattern argument, leave other unchanged *)
  let add_namespace pat =
    match pat with
    | RsPatApp (RsPatId enum, args) ->
      (match get_namespace enum ctx.enum_entries with
       (* There is no concept of path in patterns yet, so we do a hacky string concatenation. *)
       (* TODO: fix that by adding a path to patterns *)
       | Some path -> RsPatApp (RsPatId (path ^ "::" ^ enum), args)
       | None -> pat (* Could not find enum *))
    | _ -> pat
  in
  { func with args = List.map add_namespace func.args }
;;

let enum_arg_namespace : func_transform = { func = add_namespace_to_arg_pats }

(* ———————————————————————— Fix Scattered Functions ————————————————————————— *)
(* Scattered functions are re-assembled as a single function composed of one  *)
(* big match statement by Sail.                                               *)
(* This can be a problem when matching over more than one argument, because   *)
(* Sail treats all arguments as a single tuple, which our back-end flatten to *)
(* fit the Rust model better. Therefore, the match will only match on the     *)
(* first argument, instead of the whole tuple as it should.                   *)
(* This transformation detects scattered functions matching on more than one  *)
(* argument and modify the match to encompass all the arguments of the        *)
(* scattered function.                                                        *)
(* —————————————————————————————————————————————————————————————————————————— *)

let fix_scattered_func (_ctx : context) (func : rs_fn) : rs_fn =
  let get_if_missing_arg arg =
    match arg with
    | RsPatId x when String.starts_with ~prefix:"missing_arg_" x -> Some x
    | _ -> None
  in
  let missing_args =
    func.args
    |> List.filter_map get_if_missing_arg
    |> List.map (fun x -> { e_annot = None; e_exp = RsId x })
  in
  match func.body.e_exp with
  | RsMatch (m_exp, branches) when List.length missing_args > 0 ->
    let new_m_exp = { e_annot = None; e_exp = RsTuple ([ m_exp ] @ missing_args) } in
    { func with body = { e_annot = None; e_exp = RsMatch (new_m_exp, branches) } }
  | _ -> func
;;

let fix_scattered_func : func_transform = { func = fix_scattered_func }

(* ———————————————————————————— Fix Generic Type ———————————————————————————— *)

(* TODO(Gurvan):
  This is probably the problem we need to fix:
  We don't know how functions will be used, so we cannot say that the calling
  context will know the size of array and that they won't use dependent type
  NOTE: Commenting it does not seem to change anything, except the fact that
  generics which used to be usize are now i128
*)

(* let fix_generic_type_func (_ctx : context) (func : rs_fn) : rs_fn = *)
(*   let rec get_array_type_vars (typs : rs_type list) = *)
(*     match typs with *)
(*     | RsTypArray (_, RsTypParamTyp (RsTypId n)) :: tail -> n :: get_array_type_vars tail *)
(*     | _ :: tail -> get_array_type_vars tail *)
(*     | [] -> [] *)
(*   in *)
(*   let set_array_generic_types (should_set : string list) (generic : rs_generic) = *)
(*     match generic with *)
(*     | RsGenConst (s, _typ) when List.mem s should_set -> RsGenConst (s, "usize") *)
(*     | _ -> generic *)
(*   in *)
(*   let array_type_vars = *)
(*     get_array_type_vars func.signature.args @ get_array_type_vars [ func.signature.ret ] *)
(*   in *)
(*   let new_generics = *)
(*     List.map (set_array_generic_types array_type_vars) func.signature.generics *)
(*   in *)
(*   let signature = { func.signature with generics = new_generics } in *)
(*   { func with signature } *)
(* ;; *)
(**)
(* let fix_generic_type : func_transform = { func = fix_generic_type_func } *)

(* ———————————————————————— Removed Unused Generics ————————————————————————— *)

(** Some functions might have generics are are used to express invariants in
    Sail, but do not have any impact on the Rust code generation. This function
    remove such generics.

    For instance, Sail might restrict the possible values of an argument, such as:
    > forall 'width, 'width in {32, 64}.
    Yet if 'width is not part of an argument or return type (such as a vector
    width), then it has no impact on the Rust code gen, and the Sail
    front-end already enforced the invariant.**)
let remove_unused_generics_func (_ctx : context) (func : rs_fn) : rs_fn =
  let fn_type = func.signature in
  (* Collect all the used generics *)
  let args_generics =
    List.fold_left
      (fun acc typ -> SSet.union acc (generics_of_typ typ))
      SSet.empty
      fn_type.args
  in
  let ret_generics = generics_of_typ fn_type.ret in
  let body_generics = generics_of_exp func.body in
  let used_generics =
    args_generics |> SSet.union ret_generics |> SSet.union body_generics
  in
  (* Update the function signature to remove the generics *)
  let fn_type =
    { fn_type with
      generics =
        List.filter
          (fun gen -> SSet.mem (name_of_generic gen) used_generics)
          fn_type.generics
    }
  in
  { func with signature = fn_type }
;;

let remove_unused_generics : func_transform = { func = remove_unused_generics_func }

(* ———————————————————————— Enumeration binder ————————————————————————— *)

let rec enum_prefix_inserter (key : string) (lst : (string * string) list) : string =
  match lst with
  | [] -> key
  | (k, v) :: rest -> if k = key then v ^ "::" ^ k else enum_prefix_inserter key rest
;;

let enum_binder_exp (ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsId id -> RsId (enum_prefix_inserter id ctx.enum_entries)
  | RsApp ({ e_annot = t; e_exp = RsId id }, generics, args) ->
    RsApp
      ( { e_annot = t; e_exp = RsId (enum_prefix_inserter id ctx.enum_entries) }
      , generics
      , args )
  | RsMethodApp { exp = { e_annot = t; e_exp = RsId id }; name; generics; args } ->
    RsMethodApp
      { exp = { e_annot = t; e_exp = RsId (enum_prefix_inserter id ctx.enum_entries) }
      ; name
      ; generics
      ; args
      }
  | _ -> exp.e_exp
;;

let enum_binder_lexp (ctx : context) (lexp : rs_lexp) : rs_lexp =
  match lexp with
  | RsLexpId id -> RsLexpId (enum_prefix_inserter id ctx.enum_entries)
  | _ -> lexp
;;

(*TODO: Maybe we should match RsPatId directly?*)
let enum_binder_pat (ctx : context) (pat : rs_pat) : rs_pat =
  match pat with
  | RsPatId id -> RsPatId (enum_prefix_inserter id ctx.enum_entries)
  | _ -> pat
;;

let enum_binder : expr_type_transform =
  { id_expr_type_transform with
    exp_aux = enum_binder_exp
  ; lexp = enum_binder_lexp
  ; pat = enum_binder_pat
  }
;;

(* ———————————————————————————— Const Functions ————————————————————————————— *)

(* Constant prelude functions *)
let const_prelude_func : SSet.t = SSet.of_list [ "sail_ones"; "sail_zeros" ]

(* For now we use very simple heuristics *)
let should_be_const (body : rs_exp) : bool =
  match body.e_exp with
  | RsLit _ -> true
  | RsApp ({ e_annot = _; e_exp = RsId id }, _, _) when SSet.mem id const_prelude_func ->
    true
  | _ -> false
;;

let const_functions (_ctx : context) (func : rs_fn) : rs_fn =
  { func with const = should_be_const func.body }
;;

let const_fn_rewriter = { func = const_functions }

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
;;

let operator_rewriter_func (_ctx : context) (func : rs_fn) : rs_fn =
  { func with name = remove_illegal_operator_char func.name }
;;

let operator_rewriter = { func = operator_rewriter_func }

(* ———————————————————————— Operator rewriter caller side  ————————————————————————— *)

let expr_operator_rewriter (_ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsApp ({ e_annot = t; e_exp = RsId id }, generics, args) ->
    RsApp ({ e_annot = t; e_exp = RsId (remove_illegal_operator_char id) }, generics, args)
  | e -> e
;;

let expr_type_operator_rewriter : expr_type_transform =
  { id_expr_type_transform with exp_aux = expr_operator_rewriter }
;;

(* —————————————————————————— Remove 'atom' types ——————————————————————————— *)

let remove_atom (_ctx : context) (typ : rs_type) : rs_type =
  match typ with
  | RsTypGenericParam ("atom_bool", _) -> rs_type_bool
  | RsTypGenericParam ("atom", _) -> rs_type_int
  | _ -> typ
;;

let atom_rewriter : expr_type_transform =
  { id_expr_type_transform with typ = remove_atom }
;;

(* ———————————————————————— type bits = bitvector filter  ————————————————————————— *)

let filter_bits_bitvector_alias (obj : rs_obj) : rs_program =
  match obj with
  | RsAlias { new_typ = "bits"; _ } -> RsProg []
  | _ -> RsProg [ obj ]
;;

let rust_remove_type_bits (RsProg objs) : rs_program =
  merge_rs_prog_list (List.map filter_bits_bitvector_alias objs)
;;

(* ———————————————————————— prelude_func_filter  ————————————————————————— *)

let prelude_func : SSet.t =
  SSet.of_list
    [ "not"
    ; "plain_vector_access"
    ; "neq_int"
    ; "neq_bits"
    ; "eq_int"
    ; "eq_bool"
    ; "eq_bits"
    ; "eq_anything"
    ; "neq_anything"
    ; "or_vec"
    ; "and_vec"
    ; "xor_vec"
    ; "add_bits"
    ; "and_bool"
    ; "or_bool"
    ; "zero_extend"
    ; "sail_zero_extend"
    ; "sign_extend"
    ; "sail_ones"
    ; "internal_error"
    ; "hex_bits_forwards"
    ; "hex_bits_12_forwards"
    ; "hex_bits_12_backwards"
    ; "parse_hex_bits"
    ]
;;

let rust_prelude_func_filter_alias (obj : rs_obj) : rs_program =
  match obj with
  | RsFn { name; _ } when SSet.mem name prelude_func -> RsProg []
  | _ -> RsProg [ obj ]
;;

let rust_prelude_func_filter (RsProg objs) : rs_program =
  merge_rs_prog_list (List.map rust_prelude_func_filter_alias objs)
;;

(* ———————————————————————— Annotations and imports inserter  ————————————————————————— *)

(* todo: Is a static function good enough here? *)
let insert_annotation_imports_aux () : rs_program =
  RsProg
    [ RsAttribute "allow(warnings)"
    ; RsImport "softcore_prelude::*"
    ; RsImport "crate::arch_prelude::*"
    ]
;;

let insert_annotation_imports (RsProg objs) : rs_program =
  merge_rs_prog_list [ insert_annotation_imports_aux (); RsProg objs ]
;;

(* ———————————————————————— BasicTypes rewriter  ————————————————————————— *)

let transform_basic_types_exp (ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  (* Reserved keywords in rust *)
  | RsId "priv" -> RsId "_priv_"
  | RsId "super" -> RsId "_super_"
  (* Conversion for `nat` type *)
  | RsApp (({ e_annot = _; e_exp = RsId id } as e_id), generics, args) ->
    let patch_arg (exp, typ) =
      match typ with
      (* Conversion between integer types is not automatic, therefore we need to insert some casts *)
      | RsTypId "nat" -> mk_as exp rs_type_nat
      | _ -> exp
    in
    let args =
      match ctx_fun_type id ctx with
      | Some fun_def ->
        (try List.map patch_arg (List.combine args fun_def.args) with
         (* TODO: This is due to a commented regbits special translation, what
             is it like? *)
         | Invalid_argument _ ->
           Format.eprintf "%s(" id;
           List.iter
             (fun (a : rs_exp) -> Format.eprintf "%s, " (string_of_rs_exp 0 a))
             args;
           Format.eprintf ") vs (";
           List.iter (fun t -> Format.eprintf "%s, " (string_of_rs_type t)) fun_def.args;
           Format.eprintf ")\n";
           args)
      | None -> args
    in
    RsApp (e_id, generics, args)
  | e -> e
;;

(* TODO: Should we apply the same logic in lexp here? *)
let transform_basic_types_lexp (_ctx : context) (lexp : rs_lexp) : rs_lexp =
  match lexp with
  | RsLexpId "priv" -> RsLexpId "_priv_"
  | RsLexpId "super" -> RsLexpId "_super_"
  | _ -> lexp
;;

let transform_basic_types_type (_ctx : context) (typ : rs_type) : rs_type =
  match typ with
  | RsTypId "string" ->
    RsTypBorrow (RsTypId "'static str")
    (* TODO(Gurvan): Lifetime should be part of type RsTypBorrow *)
  | RsTypId "int" -> rs_type_int
  | RsTypId "bit" -> rs_type_bool
  (* TODO: Is this transformation legal? Should we add an assertion at some place in the code? *)
  | RsTypGenericParam ("range", _) -> rs_type_int
  | RsTypGenericParam ("implicit", _) -> rs_type_int
  | _ -> typ
;;

let transform_basic_types_pat (_ctx : context) (pat : rs_pat) : rs_pat =
  match pat with
  | RsPatId "priv" -> RsPatId "_priv_"
  | RsPatId "super" -> RsPatId "_super_"
  | _ -> pat
;;

let transform_basic_types : expr_type_transform =
  { id_expr_type_transform with
    exp_aux = transform_basic_types_exp
  ; lexp = transform_basic_types_lexp
  ; typ = transform_basic_types_type
  ; pat = transform_basic_types_pat
  }
;;

(* ———————————————————————— Wildcard inserter  ————————————————————————— *)

let add_wildcard_match_expr (_ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsMatch (exp, pexps) ->
    RsMatch
      ( exp
      , pexps
        @ [ RsPexp
              ( RsPatWildcard
              , { e_annot = None
                ; e_exp = RsApp (mk_exp_id "panic!", [], [ mk_lit_str "Unreachable code" ])
                } )
          ] )
  | e -> e
;;

let add_wildcard_match : expr_type_transform =
  { id_expr_type_transform with exp_aux = add_wildcard_match_expr }
;;

(* ———————————————————————— VirtContext argument inserter  ————————————————————————— *)

let sail_context_arg_inserter_exp (ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsApp (({ e_annot = _; e_exp = RsId app_id } as e_id), generics, args) ->
    (match ctx_fun app_id ctx with
     | Some fn ->
       if fn.use_sail_ctx
       then (
         let args = mk_exp_id core_ctx :: args in
         RsApp (e_id, generics, args))
       else exp.e_exp
     | _ ->
       Reporting.simple_warn
         (Printf.sprintf "Could not find function '%s' in context" app_id);
       exp.e_exp)
  | e -> e
;;

let sail_context_arg_inserter (ctx : context) (rs_program : rs_program) : rs_program =
  let ctx = update_context_fn_type ctx rs_program in
  rust_transform_expr
    { id_expr_type_transform with exp_aux = sail_context_arg_inserter_exp }
    ctx
    rs_program
;;

(* TODO: This is a very (almost useless) basic dead code remover only for our use case. Extend it in the future *)
(* ———————————————————————— Dead code remover  ————————————————————————— *)

let filter_different_litterals (lit : Big_int.num) (pexp : rs_pexp) : bool =
  match pexp with
  | RsPexp (RsPatTuple [ _e; RsPatLit (RsLitNum n) ], _e2) when n <> lit -> false
  | RsPexpWhen (RsPatTuple [ _e; RsPatLit (RsLitNum n) ], _e2, _e3) when n <> lit -> false
  | _ -> true
;;

let dead_code_remover_exp (_ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsMatch
      ( ({ e_annot = _
         ; e_exp = RsTuple [ _; { e_annot = _; e_exp = RsLit (RsLitNum n) } ]
         } as e1)
      , pexps ) -> RsMatch (e1, List.filter (filter_different_litterals n) pexps)
  | RsIf
      ( ({ e_annot = t
         ; e_exp =
             RsBinop
               ( ({ e_annot = _; e_exp = RsLit (RsLitNum n1) } as lit1)
               , RsBinopEq
               , { e_annot = _; e_exp = RsLit (RsLitNum n2) } )
         } as lit2)
      , _then_exp
      , else_exp )
    when n1 <> n2 ->
    RsIf
      ( { e_annot = t; e_exp = RsBinop (lit1, RsBinopEq, lit2) }
      , { e_annot = None
        ; e_exp = RsApp (mk_exp_id "panic!", [], [ mk_lit_str "unreachable code" ])
        }
      , else_exp )
  | e -> e
;;

let dead_code_remover : expr_type_transform =
  { id_expr_type_transform with exp_aux = dead_code_remover_exp }
;;

(* ——————————————————————————— Remove Unsupported ——————————————————————————— *)
(* We do not yet support all sail features, so for now we allow ourselves     *)
(* to selectively drop parts of the Sail model. We hope to support all of     *)
(* those in the future.                                                       *)
(* —————————————————————————————————————————————————————————————————————————— *)

let is_supported_obj (ctx : context) (obj : rs_obj) : bool =
  let unsupported_obj = ctx.arch.unsupported_obj in
  match obj with
  | RsStruct s when SSet.mem s.name unsupported_obj -> false
  | RsAlias alias when SSet.mem alias.new_typ unsupported_obj -> false
  | RsFn fn when SSet.mem fn.name unsupported_obj -> false
  | RsConst const when SSet.mem const.name unsupported_obj -> false
  | _ -> true
;;

(* ——————————————————— Remove Unsupported Function Calls ———————————————————— *)

let remove_unsupported_func_calls (ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsApp ({ e_annot = _; e_exp = RsId app_id }, _generics, _args)
    when SSet.mem app_id ctx.arch.unsupported_func ->
    let err_message = Printf.sprintf "Unsupported function: '%s'" app_id in
    RsApp
      ( { e_annot = None; e_exp = RsId "panic!" }
      , []
      , [ { e_annot = None; e_exp = RsLit (RsLitStr err_message) } ] )
  | e -> e
;;

let remove_unsupported_calls : expr_type_transform =
  { id_expr_type_transform with exp_aux = remove_unsupported_func_calls }
;;

(* ——————————————————————————— Unsupported Match ———————————————————————————— *)

let remove_unsupported_match_arms (ctx : context) (pexp : rs_pexp) : rs_pexp =
  match (pexp : rs_pexp) with
  | RsPexp (RsPatId id, _) when SSet.mem id ctx.arch.unsupported_match ->
    RsPexp (RsPatId id, mk_todo ("Unsupported: '" ^ id ^ "'"))
  | RsPexp (RsPatApp (RsPatId id, args), _) when SSet.mem id ctx.arch.unsupported_match ->
    RsPexp (RsPatApp (RsPatId id, args), mk_todo ("Unsupported: '" ^ id ^ "'"))
  | RsPexpWhen (RsPatId id, cond, _) when SSet.mem id ctx.arch.unsupported_match ->
    RsPexpWhen (RsPatId id, cond, mk_todo ("Unsupported: '" ^ id ^ "'"))
  | RsPexpWhen (RsPatApp (RsPatId id, args), cond, _)
    when SSet.mem id ctx.arch.unsupported_match ->
    RsPexpWhen (RsPatApp (RsPatId id, args), cond, mk_todo ("Unsupported: '" ^ id ^ "'"))
  | _ -> pexp
;;

let remove_unsupported_match : expr_type_transform =
  { id_expr_type_transform with pexp = remove_unsupported_match_arms }
;;

(* ———————————————————————————— Dynamic Vectors ————————————————————————————— *)

let use_dynamic_vector_typ (ctx : context) (typ : rs_type) : rs_type =
  match typ with
  | RsTypArray (typ', size) ->
    if is_const_rs_typ_param ctx size
    then typ
    else RsTypGenericParam ("BoundedVec", [ typ'; RsTypParamNum (mk_num 32) ])
  | _ -> typ
;;

let use_dynamic_vector_exp (ctx : context) (e : rs_exp) : rs_exp_aux =
  (* TODO(Gurvan):
     We are here trying to guess the type from the form of the
     expression, but we should instead try to look at e.e_annot.
     Unfortunately it is sometime not set.
  *)
  match e.e_exp with
  | RsArray _es ->
    Reporting.simple_warn
      (Printf.sprintf "Using RsArray might not work with dynamic vector");
    e.e_exp
  | RsArraySize (e', size) ->
    if is_const_rs_exp ctx size then e.e_exp else RsVecSize (e', mk_as size rs_type_usize)
  | RsMethodApp
      { exp = { e_annot = annot; e_exp = _ } as e
      ; name = "len"
      ; generics = []
      ; args = []
      } -> RsApp (mk_exp_id "vector_length", [], [ mk_borrow e ])
  | RsApp ({ e_annot = _; e_exp = RsId "undefined_vector" }, _generics, [ size; value ])
    ->
    if is_const_rs_exp ctx size
    then RsArraySize (value, mk_as size rs_type_usize)
    else e.e_exp
  | _ -> e.e_exp
;;

let use_dynamic_vectors (ctx : context) (rust_program : rs_program) : rs_program =
  let ctx = update_context_constants ctx rust_program in
  rust_transform_expr
    { id_expr_type_transform with
      exp_aux = use_dynamic_vector_exp
    ; typ = use_dynamic_vector_typ
    }
    ctx
    rust_program
;;

(* —————————————————————— Dynamic Vectors Arguments ————————————————————————— *)
(* TODO(Gurvan): This is an ugly fix to a common problem: If an argument was
   changed from an array to a vec, and we used to call it with an array
   argument, then we need to cast it. *)

let cast_if_array_to_vec (typ : rs_type) (e : rs_exp) : rs_exp =
  match typ, e.e_exp with
  | RsTypGenericParam ("BoundedVec", _), (RsArray _ | RsArraySize _) ->
    { e_annot = None
    ; e_exp = RsMethodApp { exp = e; name = "into"; generics = []; args = [] }
    }
  | _ -> e
;;

let use_dynamic_vector_exp (ctx : context) (exp : rs_exp) : rs_exp_aux =
  match exp.e_exp with
  | RsApp (({ e_annot = _; e_exp = RsId id } as id_exp), generics, args) ->
    (match ctx_fun id ctx with
     | Some fn ->
       RsApp (id_exp, generics, List.map2 cast_if_array_to_vec fn.signature.args args)
     | None -> exp.e_exp)
  | e -> e
;;

let use_dynamic_vectors_args (ctx : context) (rust_program : rs_program) : rs_program =
  let ctx = update_context_fn_type ctx rust_program in
  rust_transform_expr
    { id_expr_type_transform with exp_aux = use_dynamic_vector_exp }
    ctx
    rust_program
;;

(* ————————————————————————————— Rust Transform ————————————————————————————— *)

(* TODO(Gurvan): could be made polymorphic, limit should be called fuel *)

(** Computes the fix point of a function. **)
let rec fix_point fn ctx limit rs_program =
  let new_args = fn ctx rs_program in
  (* if new_args = rs_program || limit = 0 then *)
  (*     new_args *)
  (* else *)
  (*     fix_point fn new_args (limit - 1) ctx *)
  if limit = 0 then new_args else fix_point fn ctx (limit - 1) new_args
;;

(* TODO(Gurvan): It seems like this optimizer is trying to outsmart the rust
   compiler for no valid reason. Constant propagation should only be done by
   propagating `const` qualifiers where possible.
   This means that num_constants and inline_fun could also be removed from
   context?
*)
let optimizer (ctx : context) (rust_program : rs_program) : rs_program =
  let get_num_constants (RsProg obj : rs_program) : (string * Big_int.num) list =
    let rec constants obj =
      match obj with
      | RsConst { value = { e_annot = _; e_exp = RsLit (RsLitNum n) }; name; typ = _ }
        :: tail -> (name, n) :: constants tail
      (* TODO: Should this also take const functions into account? Should const
         function be a different thing ? *)
      | _ :: tail -> constants tail
      | [] -> []
    in
    constants obj
  in
  let get_inline_funs (RsProg obj : rs_program) : (string * rs_exp) list =
    let rec funs obj =
      match obj with
      | RsFn fn :: tail ->
        (match fn.body.e_exp with
         | RsLit lit | RsBlock [ { e_annot = _; e_exp = RsLit lit } ] ->
           (fn.name, { e_annot = fn.body.e_annot; e_exp = RsLit lit }) :: funs tail
         | _ -> funs tail)
      | _ :: tail -> funs tail
      | [] -> []
    in
    funs obj
  in
  (* TODO(Gurvan): This could be optimized by creating a SMap directly instead
     of first creating a list and then doing SMap.of_list constants. We are also
     traversing the rs_program twice which is not efficient *)
  let constants = get_num_constants rust_program in
  let inline_fun = get_inline_funs rust_program in
  let defs =
    { ctx.defs with
      num_constants = SMap.of_list constants
    ; inline_fun = SMap.of_list inline_fun
    }
  in
  let ctx = { ctx with defs } in
  rust_program |> rust_transform_expr expression_optimizer ctx |> constant_propagation
;;

let transform (rust_program : rs_program) (ctx : context) : rs_program =
  (* Bitvector transformations

     We must first replace the Sail native function and perform a basic pass of optimization
     to detect some bitvec patterns properly *)
  let rust_program =
    rust_program
    |> rust_transform_expr remove_unsupported_calls ctx
    |> rust_transform_expr remove_unsupported_match ctx
    |> fix_point virt_context_call_graph ctx 3
    |> virt_context_transform ctx
    |> rust_transform_expr nested_block_remover ctx
    |> rust_transform_expr native_func_transform ctx
    |> fix_point optimizer ctx 10
    |> rust_transform_func enum_arg_namespace ctx
    |> rust_transform_func fix_scattered_func ctx
    (* |> rust_transform_func fix_generic_type ctx *)
    |> rust_transform_expr enum_binder ctx
    |> rust_remove_type_bits
    |> rust_prelude_func_filter
    |> insert_annotation_imports
    |> rust_transform_expr transform_basic_types ctx
    |> rust_transform_expr add_wildcard_match ctx
    |> sail_context_arg_inserter ctx
    |> rust_transform_expr expr_type_hoister ctx
    |> rust_transform_expr expr_type_operator_rewriter ctx
    |> rust_transform_expr atom_rewriter ctx
    |> rust_transform_func const_fn_rewriter ctx
    |> rust_transform_func operator_rewriter ctx
    |> fix_point optimizer ctx 5
    (* Optimizer: Dead code elimination *)
    |> rust_transform_expr dead_code_remover ctx
    |> use_dynamic_bitvec ctx
    (* |> use_dynamic_bitvec_args ctx *)
    |> use_dynamic_vectors ctx
    |> use_dynamic_vectors_args ctx
    |> rust_transform_func remove_unused_generics ctx
  in
  (* Filter unsupported items *)
  let rust_program =
    match rust_program with
    | RsProg objs -> RsProg (List.filter (is_supported_obj ctx) objs)
  in
  rust_program
;;
