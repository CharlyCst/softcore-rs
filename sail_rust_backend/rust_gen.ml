(** Rust generation module **)

module SSet = Call_set.SSet

type rs_type =
    | RsTypId of string
    | RsTypTuple of rs_type list
    | RsTypUnit
    | RsTypGeneric of string
    | RsTypGenericParam of string * rs_type_param list
    | RsTypArray of rs_type_param * rs_type_param
    | RsTypOption of rs_type_param
    | RsTypTodo of string

and  rs_type_param =
    | RsTypParamTyp of rs_type
    | RsTypParamNum of rs_exp

and rs_generic =
    | RsGenTyp of string
    | RsGenNum of string
    | RsGenBool of string

and rs_lit =
    | RsLitUnit
    | RsLitTrue
    | RsLitFalse
    | RsLitNum of int64
    | RsLitBin of string
    | RsLitHex of string
    | RsLitStr of string
    | RsLitTodo

and rs_pat =
    | RsPatLit of rs_lit
    | RsPatId of string
    | RsPatType of rs_type * rs_pat
    | RsPatWildcard
    | RsPatTuple of rs_pat list
    | RsPatApp of rs_pat * rs_pat list
    | RsPatTodo of string
    | RsPatSome of rs_pat
    | RsPatNone

and rs_binop =
    | RsBinopEq
    | RsBinopNeq
    | RsBinopGt
    | RsBinopGe
    | RsBinopLt
    | RsBinopLe
    | RsBinopAnd
    | RsBinopOr
    | RsBinopXor
    | RsBinopLAnd
    | RsBinopLOr
    | RsBinopAdd 
    | RsBinopSub 
    | RsBinopMult 
    | RsBinopShiftLeft
    | RsBinopShiftRight
    | RsBinopMod

and rs_unop = 
    | RsUnopNeg

and rs_method_app = {
    exp: rs_exp;
    name: string;
    generics: string list;
    args: rs_exp list;
}

and rs_exp =
    | RsLet of rs_pat * rs_exp * rs_exp
    | RsApp of rs_exp * string list * rs_exp list (* the strings are the generics *)
    | RsMethodApp of rs_method_app
    | RsStaticApp of rs_type * string * rs_exp list
    | RsId of string
    | RsLit of rs_lit
    | RsField of rs_exp * string
    | RsBlock of rs_exp list
    | RsConstBlock of rs_exp list
    | RsInstrList of rs_exp list
    | RsIf of rs_exp * rs_exp * rs_exp
    | RsMatch of rs_exp * rs_pexp list
    | RsTuple of rs_exp list
    | RsAssign of rs_lexp * rs_exp
    | RsIndex of rs_exp * rs_exp
    | RsBinop of rs_exp * rs_binop * rs_exp
    | RsUnop of rs_unop * rs_exp
    | RsAs of rs_exp * rs_type
    | RsSome of rs_exp
    | RsNone
    | RsPathSeparator of rs_type * rs_type
    | RsFor of rs_type * rs_lit * rs_lit * rs_exp
    | RsStruct of rs_type * (string * rs_exp) list
    | RsStructAssign of rs_exp * string * rs_exp
    | RsReturn of rs_exp
    | RsTodo of string
and rs_lexp =
    | RsLexpId of string
    | RsLexpField of rs_exp * string
    | RsLexpIndex of rs_lexp * rs_exp
    | RsLexpIndexRange of rs_lexp * rs_exp * rs_exp
    | RsLexpTodo
and rs_pexp =
    | RsPexp of rs_pat * rs_exp
    | RsPexpWhen of rs_pat * rs_exp * rs_exp

type rs_block = rs_exp list

type rs_fn_type = {
    generics: rs_generic list;
    args: rs_type list;
    ret: rs_type
}

type rs_fn = {
    name: string;
    signature: rs_fn_type;
    args: rs_pat list;
    body: rs_exp;
    mutable use_sail_ctx: bool;
}

type rs_enum = {
    name: string;
    generics: rs_generic list;
    fields: (string * rs_type option) list;
}

type rs_struct = {
    name: string;
    generics: rs_generic list;
    fields: (string * rs_type) list;
}

type rs_alias = {
    new_typ: string;
    generics: rs_generic list;
    old_type: rs_type;
}

type rs_const = {
    name: string;
    value: rs_exp;
}

type rs_obj = 
    | RsFn of rs_fn
    | RsEnum of rs_enum
    | RsStruct of rs_struct
    | RsAlias of rs_alias
    | RsConst of rs_const
    | RsImport of string 
    | RsAttribute of string
    | RsObjTodo of string

type rs_program =
    | RsProg of rs_obj list

(* ————————————————————————————————— Utils —————————————————————————————————— *)

let merge_rs_prog (prog1: rs_program) (prog2: rs_program) : rs_program =
    let RsProg (fn1) = prog1 in
    let RsProg (fn2) = prog2 in
    RsProg (fn1 @ fn2)

let rec merge_rs_prog_list (programs: rs_program list): rs_program =
    match programs with
        | h :: t -> merge_rs_prog h (merge_rs_prog_list t)
        | _ -> RsProg []

let mk_fn_typ (args: rs_type list) (ret: rs_type) : rs_fn_type =
    {
        generics = [];
        args = args;
        ret = ret;
    }

let mk_method_app (exp: rs_exp) (name: string) (args: rs_exp list) : rs_exp =
    RsMethodApp {
        exp = exp;
        name = name;
        generics = [];
        args = args;
    }

let mk_struct (name: string) (fields: (string * rs_type) list) : rs_obj =
    RsStruct {
        name = name;
        generics = [];
        fields = fields
    }

(** Removes the generic parameters from a type

    For instance, transforms `Foo<N>` into `Foo`.
**)
let rec strip_generic_parameters (typ: rs_type) : rs_type =
    let strip_typ_params params = match params with
        | RsTypParamTyp typ -> RsTypParamTyp (strip_generic_parameters typ)
        | RsTypParamNum n ->  RsTypParamNum n
    in
    match typ with
        | RsTypTuple typs -> RsTypTuple (List.map strip_generic_parameters typs)
        | RsTypGenericParam (name, gen_params) -> RsTypId name
        | RsTypArray (typ, size) -> RsTypArray (strip_typ_params typ, strip_typ_params size)
        | RsTypOption typ -> RsTypOption (strip_typ_params typ)
        | _ -> typ

(** In Sail type variables start with an apostrophe ('), which appears as
    lifetime in Rust.
    This function removes the apostrophe from the generic identifier, and
    leaves all other types as is.**)
let sanitize_generic_id (id: string) : string = 
    if String.get id 0 = '\'' then
        let n = (String.length id) - 1 in
        String.uppercase_ascii (String.sub id 1 n)
    else
        id

(** Turn a Sail ID into a valid Rust id.

   Sail is a tiny bit more permissive with IDs than Rust. In particular, Sail
   allows quotes (`'`) at the end of an ID, which Rust does not. This function
       sanitizes the Sail IDs to make them valid in Rust. **)
let sanitize_id (id: string) : string = 
    let quote_regexp = Str.regexp "'" in
    Str.global_replace quote_regexp "__quote" id

(** Returns the set of IDs redefined by this pattern **)
let rec ids_of_pat (pat: rs_pat) : SSet.t = 
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

let rec lexp_to_exp (lexp: rs_lexp) : rs_exp =
    match lexp with
        | RsLexpId id -> RsId id
        | RsLexpField (exp, field) -> RsField (exp, field)
        | _ -> RsId "LexpToExpTodo" 

(* ————————————————————————————— Rust to String ————————————————————————————— *)

let string_of_generics (generics: string list) : string =
    match generics with
    | [] -> ""
    | _ -> Printf.sprintf "<%s>" (String.concat ", " generics)

let string_of_generics_turbofish (generics: string list) : string =
    match generics with
    | [] -> ""
    | _ -> Printf.sprintf "::<%s>" (String.concat ", " generics)

let string_of_generics_parameters(generics: rs_generic list) : string =
    let string_of_generic generic = match generic with
        | RsGenTyp s -> s
        | RsGenNum s -> Printf.sprintf "const %s: usize" s
        | RsGenBool s -> Printf.sprintf "const %s: bool" s
    in
    let generics = List.map string_of_generic generics in
    match generics with
    | [] -> ""
    | _ -> Printf.sprintf "<%s>" (String.concat ", " generics)

let rec string_of_rs_type (typ: rs_type) : string =
    match typ with
        | RsTypId s -> s
        | RsTypTuple types ->
            Printf.sprintf "(%s)"
                (String.concat ", " (List.map string_of_rs_type types))
        | RsTypUnit -> "()"
        | RsTypGeneric t -> t
        | RsTypGenericParam (id, params) ->
            Printf.sprintf "%s<%s>"
                id
                (String.concat ", " (List.map string_of_rs_type_param params))
        | RsTypArray (typ, size) -> Printf.sprintf "[%s;%s]" (string_of_rs_type_param typ) (string_of_rs_type_param size)
        | RsTypOption param -> Printf.sprintf "Option<%s>" (string_of_rs_type_param param)
        | RsTypTodo e -> e
and string_of_rs_type_param (typ: rs_type_param) : string =
    match typ with
        | RsTypParamTyp typ -> string_of_rs_type typ
        | RsTypParamNum n -> string_of_rs_exp 0 n

and string_of_rs_lit (lit: rs_lit) : string =
    match lit  with
        | RsLitUnit -> "()"
        | RsLitTrue -> "true"
        | RsLitFalse -> "false"
        | RsLitNum n -> Printf.sprintf "%Li" n
        | RsLitBin n -> n
        | RsLitHex n -> n
        | RsLitStr s -> Printf.sprintf "\"%s\"" s
        | RsLitTodo -> "LIT_TODO"

and string_of_rs_pat (pat: rs_pat) : string =
    match pat with
        | RsPatLit lit -> string_of_rs_lit lit
        | RsPatId id-> id
        | RsPatType (_typ, RsPatWildcard) -> "_"
        | RsPatType (typ, pat) -> Printf.sprintf "%s: %s" (string_of_rs_pat pat) (string_of_rs_type typ)
        | RsPatWildcard -> "_"
        | RsPatTuple pats ->
            Printf.sprintf "(%s)" (String.concat ", " (List.map string_of_rs_pat pats))
        | RsPatApp (name, args) -> 
            Printf.sprintf "%s(%s)" (string_of_rs_pat name)  
            (String.concat ", " (List.map string_of_rs_pat args))
        | RsPatSome pat -> Printf.sprintf "Some(%s)" (string_of_rs_pat pat)
        | RsPatNone -> "None"
        | RsPatTodo text -> Printf.sprintf "%s" text

and string_of_rs_binop (binop: rs_binop) : string =
    match binop with
        | RsBinopEq -> "=="
        | RsBinopNeq -> "!="
        | RsBinopGt -> ">"
        | RsBinopGe -> ">="
        | RsBinopLt -> "<"
        | RsBinopLe -> "<="
        | RsBinopAnd -> "&"
        | RsBinopOr -> "|"
        | RsBinopXor -> "^"
        | RsBinopLAnd -> "&&"
        | RsBinopLOr -> "||"
        | RsBinopAdd -> "+"
        | RsBinopSub -> "-"
        | RsBinopMult -> "*"
        | RsBinopShiftLeft -> "<<"
        | RsBinopShiftRight -> ">>"
        | RsBinopMod -> "%"

and string_of_rs_unop (unop: rs_unop) : string =
    match unop with
        | RsUnopNeg -> "!"

and indent (n: int) : string =
    String.make (n * 4) ' '

and string_of_rs_exp (n: int) (exp: rs_exp) : string =
    match exp with
        (* The block indentation if not nedded after a  let, remove it to pretify*)
        | RsLet (pat, exp, RsBlock exps) ->
            Printf.sprintf "let %s = %s;\n%s%s"
                (string_of_rs_pat pat)
                (string_of_rs_exp n exp)
                (indent n)
                (String.concat
                    (Printf.sprintf ";\n%s" (indent n))
                    (List.map (string_of_rs_exp n) exps))
        | RsLet (pat, exp, next) ->
            Printf.sprintf "let %s = %s;\n%s%s"
                (string_of_rs_pat pat)
                (string_of_rs_exp n exp)
                (indent n)
                (string_of_rs_exp n next)
        | RsApp (fn, generics, args)->
            Printf.sprintf "%s%s(%s)"
                (string_of_rs_exp n fn)
                (string_of_generics_turbofish generics)
                (String.concat ", " (List.map (string_of_rs_exp n) args))
        | RsStaticApp(typ, func, args) ->
            Printf.sprintf "%s::%s(%s)" 
                (string_of_rs_type typ)
                func
                (String.concat ", " (List.map (string_of_rs_exp n) args))
        | RsMethodApp {exp; name; generics; args}->
            Printf.sprintf "%s.%s%s(%s)"
                (string_of_rs_exp n exp)
                name 
                (string_of_generics_turbofish generics)
                (String.concat ", " (List.map (string_of_rs_exp n) args))
        | RsId id -> id
        | RsLit lit  -> string_of_rs_lit lit
        | RsField (exp, field) ->
            Printf.sprintf "%s.%s"
                (string_of_rs_exp n exp)
                field
        | RsBlock exps ->
            Printf.sprintf "{\n%s%s\n%s}"
                (indent (n + 1))
                (String.concat
                    (Printf.sprintf ";\n%s" (indent (n + 1)))
                    (List.map (string_of_rs_exp (n + 1)) exps))
                (indent n)
        | RsConstBlock exps ->
            Printf.sprintf "const {\n%s%s\n%s}"
                (indent (n + 1))
                (String.concat
                    (Printf.sprintf ";\n%s" (indent (n + 1)))
                    (List.map (string_of_rs_exp (n + 1)) exps))
                (indent n)
        | RsInstrList exps ->
            Printf.sprintf "%s"
                (String.concat
                    (Printf.sprintf ";\n%s" (indent (n)))
                    (List.map (string_of_rs_exp (n)) exps))
        | RsIf (cond, then_exp, else_exp) ->
            Printf.sprintf "if {%s} {\n%s%s\n%s} else %s"
                (string_of_rs_exp n cond)
                (indent (n + 1))
                (string_of_rs_exp (n + 1) then_exp)
                (indent n)
                (match else_exp with
                    | RsIf (_, _, _) -> (string_of_rs_exp n else_exp)
                    | _ -> (Printf.sprintf "{\n%s%s\n%s}"
                        (indent (n + 1))
                        (string_of_rs_exp (n+1) else_exp))
                        (indent n))
        | RsMatch (exp, pexps) ->
            Printf.sprintf "match %s {\n%s%s%s}"
                (string_of_rs_exp n exp)
                (indent (n + 1))
                (String.concat
                    (indent (n + 1))
                    (List.map (string_of_rs_pexp (n+1)) pexps))
                (indent n)
        | RsTuple exps ->
            Printf.sprintf "(%s)" (String.concat ", " (List.map (string_of_rs_exp n) exps))
        | RsAssign (exp1, exp2) ->
            Printf.sprintf "%s = %s"
                (string_of_rs_lexp n exp1)
                (string_of_rs_exp n exp2)
        | RsIndex (exp1, exp2) ->
            Printf.sprintf "%s[%s]"
                (string_of_rs_exp n exp1)
                (string_of_rs_exp n exp2)
        | RsBinop (exp1, binop, exp2) ->
            Printf.sprintf "(%s %s %s)"
                (string_of_rs_exp n exp1)
                (string_of_rs_binop binop)
                (string_of_rs_exp n exp2)
        | RsUnop (unop, exp) ->
            Printf.sprintf "%s(%s)"
                (string_of_rs_unop unop)
                (string_of_rs_exp n exp)
        | RsAs (exp, typ) ->
            Printf.sprintf "(%s as %s)"
                (string_of_rs_exp (n + 1) exp)
                (string_of_rs_type typ)
        | RsSome (exp) -> 
            Printf.sprintf "Some(%s)" (string_of_rs_exp n exp)
        | RsNone -> "None"
        | RsPathSeparator (t1, t2) -> Printf.sprintf "%s::%s" (string_of_rs_type t1) (string_of_rs_type t2)
        | RsFor (var, start, until, body) -> Printf.sprintf "for %s in %s..=%s {\n%s%s\n%s}"
            (string_of_rs_type var)
            (string_of_rs_lit start)
            (string_of_rs_lit until)
            (indent (n + 1))
            (string_of_rs_exp (n + 1) body)
            (indent n)
        | RsStruct (name, entries) -> 
            Printf.sprintf "%s {\n%s%s\n%s}"
            (string_of_rs_type name)
            (indent (n + 1))
            (String.concat
                (Printf.sprintf ",\n%s" (indent (n + 1)))
                (List.map (fun (name, typ) -> (Printf.sprintf "%s: %s" name (string_of_rs_exp (n+1) typ))) entries))
            (indent n)
        | RsStructAssign (exp, field, value) -> 
            Printf.sprintf "%s.%s = %s; %s" (string_of_rs_exp n exp) field (string_of_rs_exp n value) (string_of_rs_exp n exp)
        | RsReturn exp -> Printf.sprintf "return %s;" (string_of_rs_exp n exp)
        | RsTodo text -> Printf.sprintf "todo!(\"%s\")" text
and string_of_rs_lexp (n: int) (lexp: rs_lexp) : string =
    match lexp with
        | RsLexpId id -> id
        | RsLexpField (exp, id) ->
            Printf.sprintf "%s.%s"
                (string_of_rs_exp n exp)
                id
        | RsLexpIndex (lexp, idx) ->
            Printf.sprintf "%s[%s]"
                (string_of_rs_lexp n lexp)
                (string_of_rs_exp n idx)
        | RsLexpIndexRange (lexp, range_start, range_end) ->
            (* Implement support for this case if the assertion fails *)
            (* assert ((string_of_rs_exp n range_start) = "(64 - 1)"); *) 
            (* assert ((string_of_rs_exp n range_end) = "0"); *) 
            Printf.sprintf "%s[%s..%s]" (string_of_rs_lexp n lexp) (string_of_rs_exp 0 range_start) (string_of_rs_exp 0 range_end)
            (* string_of_rs_lexp n lexp *)
        | RsLexpTodo ->  "LEXP_TODO"
and string_of_rs_pexp (n: int) (pexp: rs_pexp) : string =
    match pexp with
        | RsPexp (pat, exp) ->
            Printf.sprintf "%s => {%s}\n"
                (string_of_rs_pat pat)
                (string_of_rs_exp n exp)
        | RsPexpWhen (pat, cond_exp, exp) ->
            Printf.sprintf "%s if {%s} => {%s}\n"
                (string_of_rs_pat pat)
                (string_of_rs_exp n cond_exp)
                (string_of_rs_exp n exp)

let string_of_rs_fn_args (fn: rs_fn) : string =
    let string_of_arg_and_type (arg: rs_pat) (typ: rs_type) : string =
        match typ with
            | RsTypUnit -> "unit_arg: ()"
            | _ -> Printf.sprintf "%s: %s"
                (string_of_rs_pat arg)
                (string_of_rs_type typ)
    in
    let arg_types = fn.signature.args in
    String.concat ", " (List.map2 string_of_arg_and_type fn.args arg_types)

let string_of_rs_fn (fn: rs_fn) : string =
    let args = string_of_rs_fn_args fn in
    let ret_type = match fn.signature.ret with
        | RsTypUnit -> ""
        | ret_type -> Printf.sprintf " -> %s" (string_of_rs_type ret_type)
    in
    (* For now we assume all generics are const usize *)
    let generics = string_of_generics_parameters fn.signature.generics in
    let signature = Printf.sprintf "pub fn %s%s(%s)%s {\n%s" fn.name generics args ret_type (indent 1) in
    let stmts = (match fn.body with
        | RsBlock exps
            -> String.concat
                (Printf.sprintf ";\n%s" (indent 1))
                (List.map (string_of_rs_exp 1) exps)
        | _ ->string_of_rs_exp 1 fn.body) in
    Printf.sprintf "%s%s\n}" signature stmts

let remove_last_char (s: string) : string =
    if String.length s = 0 then
        s
    else
        String.sub s 0 (String.length s - 1)

let enum_attribute () : string = "#[derive(Eq, PartialEq, Clone, Copy, Debug)]"

let parse_enum_fields (entries: (string * rs_type option) list) : string = 
    let field_typ (typ : rs_type option) : string =
        match typ with
        | None -> ""
        | Some typ -> "(" ^ string_of_rs_type typ ^ ")"
    in
    let prefixed_entries = List.map (fun (name, typ) -> "    " ^ name ^ field_typ typ) entries in
    String.concat ",\n" prefixed_entries

let string_of_rs_enum (enum: rs_enum) : string = 
    Printf.sprintf "%s\npub enum %s%s {\n%s\n}" (enum_attribute ()) enum.name (string_of_generics_parameters enum.generics) (parse_enum_fields enum.fields)

let parse_struct_fields (entries: (string * rs_type)  list) : string = 
    let prefixed_entries = List.map (fun s -> "    pub " ^ (fst s) ^ ": " ^ string_of_rs_type (snd s) ^ ",\n") entries in
    let merged_fields = String.concat "" prefixed_entries in
    remove_last_char merged_fields (* Removes last '\n'*)

let string_of_rs_struct (struc: rs_struct) : string = 
    let generics = string_of_generics_parameters struc.generics in
    Printf.sprintf "#[derive(Eq, PartialEq, Clone, Copy, Debug)]\npub struct %s%s {\n%s\n}" struc.name generics (parse_struct_fields struc.fields)

let string_of_rs_obj (obj: rs_obj) : string =
    match obj with
        | RsFn fn -> string_of_rs_fn fn
        | RsEnum enum -> string_of_rs_enum enum
        | RsStruct struc -> string_of_rs_struct struc
        | RsAlias alias -> Printf.sprintf "pub type %s%s = %s;" alias.new_typ (string_of_generics_parameters alias.generics) (string_of_rs_type alias.old_type)
        | RsConst const -> Printf.sprintf "pub const %s: usize = %s;" const.name (string_of_rs_exp 0 const.value)
        | RsAttribute value -> Printf.sprintf "#![%s]" value
        | RsImport value -> Printf.sprintf "use %s;" value
        | RsObjTodo s -> s

let string_of_rs_prog (prog: rs_program) : string =
    let RsProg (funs) = prog in
    String.concat "\n\n" (List.map string_of_rs_obj funs)
