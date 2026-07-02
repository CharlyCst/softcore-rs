open Rs_ast

let core_ctx = "core_ctx"
let default_copy_derive = [ "Eq"; "PartialEq"; "Clone"; "Copy"; "Debug" ]
let default_move_derive = [ "Eq"; "PartialEq"; "Clone"; "Debug" ]

let rs_type_is_builtin (t : rs_type) : bool =
  List.exists (fun t2 -> t = t2) rs_type_builtins
;;

let merge_rs_prog (prog1 : rs_program) (prog2 : rs_program) : rs_program =
  let (RsProg fn1) = prog1 in
  let (RsProg fn2) = prog2 in
  RsProg (fn1 @ fn2)
;;

let rec merge_rs_prog_list (programs : rs_program list) : rs_program =
  match programs with
  | h :: t -> merge_rs_prog h (merge_rs_prog_list t)
  | _ -> RsProg []
;;

let mk_fn_typ (args : rs_type list) (ret : rs_type) : rs_fn_type =
  { generics = []; args; ret; linked_gen_args = [] }
;;

let mk_fn_typ_gen (args : rs_type list) (ret : rs_type) (generics : rs_generic list)
  : rs_fn_type
  =
  { generics; args; ret; linked_gen_args = [] }
;;

let mk_as (exp : rs_exp) (typ : rs_type) : rs_exp =
  { e_annot = Some typ; e_exp = RsAs (exp, typ) }
;;

let mk_borrow (exp : rs_exp) : rs_exp =
  { e_annot = Option.bind exp.e_annot (fun t -> Some (RsTypBorrow t))
  ; e_exp = RsBorrow exp
  }
;;

let mk_todo ?(e_annot = None) (id : string) : rs_exp = { e_annot; e_exp = RsTodo id }
let mk_exp_id ?(e_annot = None) (id : string) : rs_exp = { e_annot; e_exp = RsId id }

let mk_lit_str ?(e_annot = None) (str : string) : rs_exp =
  { e_annot; e_exp = RsLit (RsLitStr str) }
;;

let mk_method_app (exp : rs_exp) (name : string) (args : rs_exp list) : rs_exp_aux =
  RsMethodApp { exp; name; generics = []; args }
;;

let mk_struct (name : string) (fields : (string * rs_type) list) : rs_obj =
  RsStruct { name; generics = []; fields; derive = default_move_derive; doc = [] }
;;

let mk_num (n : int) : rs_exp =
  { e_annot = None; e_exp = RsLit (RsLitNum (Big_int.of_int n)) }
;;

let mk_big_num (n : Big_int.num) : rs_exp = { e_annot = None; e_exp = RsLit (RsLitNum n) }

(** Removes the generic parameters from a type

    For instance, transforms `Foo<N>` into `Foo`.
**)
let rec strip_generic_parameters (typ : rs_type) : rs_type =
  let strip_typ_params params =
    match params with
    | RsTypParamTyp typ -> RsTypParamTyp (strip_generic_parameters typ)
    | RsTypParamNum n -> RsTypParamNum n
  in
  match typ with
  | RsTypTuple typs -> RsTypTuple (List.map strip_generic_parameters typs)
  | RsTypGenericParam (name, _) -> RsTypId name
  | RsTypArray (typ, size) ->
    RsTypArray (strip_generic_parameters typ, strip_typ_params size)
  | RsTypOption typ -> RsTypOption (strip_typ_params typ)
  | _ -> typ
;;

(** Returns the set of generics used in the type **)
let rec generics_of_typ (typ : rs_type) : SSet.t =
  match typ with
  | RsTypId id -> SSet.singleton id
  | RsTypTuple typs ->
    List.fold_left (fun acc typ -> SSet.union acc (generics_of_typ typ)) SSet.empty typs
  | RsTypUnit -> SSet.empty
  | RsTypGeneric t -> SSet.singleton t
  | RsTypGenericParam ("atom", _) -> SSet.empty (* atoms are replaced by their type *)
  | RsTypGenericParam (_, params) ->
    List.fold_left
      (fun acc param -> SSet.union acc (generics_param param))
      SSet.empty
      params
  | RsTypArray (param1, param2) ->
    SSet.union (generics_of_typ param1) (generics_param param2)
  | RsTypOption typ_param -> generics_param typ_param
  | RsTypTodo _ -> SSet.empty
  | RsTypBorrow t -> generics_of_typ t

and generics_param (typ_param : rs_type_param) : SSet.t =
  match typ_param with
  | RsTypParamTyp typ -> generics_of_typ typ
  | RsTypParamNum _ -> SSet.empty

and generics_of_exp (exp : rs_exp) : SSet.t =
  let generics_of_exps (exps : rs_exp list) =
    List.fold_left (fun acc exp -> SSet.union acc (generics_of_exp exp)) SSet.empty exps
  in
  match exp.e_exp with
  | RsLet (_, pat, exp, next) ->
    generics_of_pat pat
    |> SSet.union (generics_of_exp exp)
    |> SSet.union (generics_of_exp next)
  | RsApp (app, gens, args) ->
    generics_of_exp app
    |> SSet.union (SSet.of_list gens)
    |> SSet.union (generics_of_exps args)
  | RsMethodApp { exp; generics = gens; args; name = _ } ->
    generics_of_exp exp
    |> SSet.union (SSet.of_list gens)
    |> SSet.union (generics_of_exps args)
  | RsStaticApp (typ, _, args) ->
    generics_of_typ typ |> SSet.union (generics_of_exps args)
  | RsId id -> if id_is_generic id then SSet.singleton id else SSet.empty
  | RsLit _ -> SSet.empty
  | RsField (exp, _) -> generics_of_exp exp
  | RsBlock exps -> generics_of_exps exps
  | RsConstBlock exps -> generics_of_exps exps
  | RsInstrList exps -> generics_of_exps exps
  | RsIf (cond, if_branch, else_branch) ->
    generics_of_exp cond
    |> SSet.union (generics_of_exp if_branch)
    |> SSet.union (generics_of_exp else_branch)
  | RsMatch (exp, arms) ->
    let arms_gens =
      List.fold_left
        (fun acc pexp -> SSet.union acc (generics_of_pexp pexp))
        SSet.empty
        arms
    in
    SSet.union (generics_of_exp exp) arms_gens
  | RsTuple exps -> generics_of_exps exps
  | RsArray exps -> generics_of_exps exps
  | RsArraySize (exp1, exp2) -> SSet.union (generics_of_exp exp1) (generics_of_exp exp2)
  | RsVec exps -> generics_of_exps exps
  | RsVecSize (exp1, exp2) -> SSet.union (generics_of_exp exp1) (generics_of_exp exp2)
  | RsAssign (lexp, exp) -> SSet.union (generics_of_lexp lexp) (generics_of_exp exp)
  | RsIndex (exp1, exp2) -> SSet.union (generics_of_exp exp1) (generics_of_exp exp2)
  | RsBinop (exp1, _, exp2) -> SSet.union (generics_of_exp exp1) (generics_of_exp exp2)
  | RsUnop (_, exp) -> generics_of_exp exp
  | RsAs (exp, typ) -> SSet.union (generics_of_exp exp) (generics_of_typ typ)
  | RsSome exp -> generics_of_exp exp
  | RsNone -> SSet.empty
  | RsPathSeparator (typ1, typ2) ->
    SSet.union (generics_of_typ typ1) (generics_of_typ typ2)
  | RsFor (typ, _, _, body) -> SSet.union (generics_of_typ typ) (generics_of_exp body)
  | RsForRev (typ, _, _, body) -> SSet.union (generics_of_typ typ) (generics_of_exp body)
  | RsStruct (typ, fields) ->
    SSet.union
      (generics_of_typ typ)
      (generics_of_exps (List.map (fun (_, exp) -> exp) fields))
  | RsStructAssign (struc, _, exp) ->
    SSet.union (generics_of_exp struc) (generics_of_exp exp)
  | RsReturn exp -> generics_of_exp exp
  | RsTodo _ -> SSet.empty
  | RsBorrow e -> generics_of_exp e

and generics_of_lexp (lexp : rs_lexp) : SSet.t =
  match lexp with
  | RsLexpTyp (_, typ) -> generics_of_typ typ
  | _ -> SSet.empty

and generics_of_pexp (pexp : rs_pexp) : SSet.t =
  match pexp with
  | RsPexp (pat, exp) -> SSet.union (generics_of_pat pat) (generics_of_exp exp)
  | RsPexpWhen (pat, guard, exp) ->
    generics_of_pat pat
    |> SSet.union (generics_of_exp guard)
    |> SSet.union (generics_of_exp exp)

and generics_of_pat (pat : rs_pat) : SSet.t =
  let generics_of_pats (pats : rs_pat list) =
    List.fold_left (fun acc pat -> SSet.union acc (generics_of_pat pat)) SSet.empty pats
  in
  match pat with
  | RsPatLit _ -> SSet.empty
  | RsPatId id -> if id_is_generic id then SSet.singleton id else SSet.empty
  | RsPatType (typ, pat) -> SSet.union (generics_of_typ typ) (generics_of_pat pat)
  | RsPatWildcard -> SSet.empty
  | RsPatTuple pats -> generics_of_pats pats
  | RsPatApp (pat, pats) -> SSet.union (generics_of_pat pat) (generics_of_pats pats)
  | RsPatSome pat -> generics_of_pat pat
  | RsPatNone -> SSet.empty
  | RsPatTodo _ -> SSet.empty

and id_is_generic (id : string) : bool = String.uppercase_ascii id = id

let name_of_generic (gen : rs_generic) : string =
  match gen with
  | RsGenTyp x -> x
  | RsGenConst (x, _) -> x
;;

(** In Sail type variables start with an apostrophe ('), which appears as
    lifetime in Rust.
    This function removes the apostrophe from the generic identifier, and
    leaves all other types as is.**)
let sanitize_generic_id (id : string) : string =
  if String.get id 0 = '\''
  then (
    let n = String.length id - 1 in
    String.uppercase_ascii (String.sub id 1 n))
  else id
;;

let quote_regexp = Str.regexp_string "'"
let qmark_regexp = Str.regexp_string "?"

(** Turn a Sail ID into a valid Rust id.

   Sail is a tiny bit more permissive with IDs than Rust. In particular, Sail
   allows quotes (`'`) at the end of an ID, which Rust does not. This function
       sanitizes the Sail IDs to make them valid in Rust. **)
let sanitize_id (id : string) : string =
  id
  |> Str.global_replace quote_regexp "__quote"
  |> Str.global_replace qmark_regexp "__qmark"
;;

(** Returns the set of IDs redefined by this pattern **)
let rec ids_of_pat (pat : rs_pat) : SSet.t =
  let empty = SSet.empty in
  let ids_of_list pats =
    List.fold_left (fun acc pat -> SSet.union acc (ids_of_pat pat)) empty pats
  in
  match pat with
  | RsPatLit _ -> empty
  | RsPatId id -> SSet.add id empty
  | RsPatType (_, pat) -> ids_of_pat pat
  | RsPatWildcard -> SSet.add "_" empty
  | RsPatTuple pats -> ids_of_list pats
  | RsPatApp (_, pats) -> ids_of_list pats
  | RsPatTodo _ -> empty
  | RsPatSome pat -> ids_of_pat pat
  | RsPatNone -> empty
;;

let rec lexp_to_exp (lexp : rs_lexp) : rs_exp =
  let e_exp =
    match lexp with
    | RsLexpId id -> RsId id
    | RsLexpField (exp, field) -> RsField (exp, field)
    | RsLexpIndex (lexp, exp) -> RsIndex (lexp_to_exp lexp, exp)
    | _ -> RsId "LexpToExpTodo"
  in
  { e_annot = None; e_exp }
;;

let empty_function_from_type (name : string) (signature : rs_fn_type) : rs_fn =
  { name
  ; signature
  ; const = false
  ; body = { e_annot = None; e_exp = RsTodo "Undefined function" }
  ; doc =
      []
      (* This can actually be true, in which case the function should later be updated in the context *)
  ; use_sail_ctx = false
  ; args = List.map (fun _ -> RsPatWildcard) signature.args
  }
;;
