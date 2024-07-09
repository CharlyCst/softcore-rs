(** Rust generation module **)


type rs_type =
    | RsTypId of string
    | RsTypTuple of rs_type list
    | RsTypUnit
    | RsTypGeneric of string
    | RsTypGenericParam of string * rs_type_param list
    | RsTypTodo
and  rs_type_param =
    | RsTypParamTyp of rs_type
    | RsTypParamNum of int

type rs_fn_type = rs_type list * rs_type

type rs_lit =
    | RsLitUnit
    | RsLitTrue
    | RsLitFalse
    | RsLitNum of int64
    | RsLitBin of string
    | RsLitHex of string
    | RsLitStr of string
    | RsLitTodo

type rs_pat =
    | RsPatLit of rs_lit
    | RsPatId of string
    | RsPatType of rs_type * rs_pat
    | RsPatWildcard
    | RsPatTuple of rs_pat list
    | RsPatTodo

type rs_binop =
    | RsBinopEq
    | RsBinopNeq
    | RsBinopAnd
    | RsBinopOr
    | RsBinopXor
    | RsBinopLAnd
    | RsBinopLOr

type rs_exp =
    | RsLet of rs_pat * rs_exp * rs_exp
    | RsApp of string * rs_exp list
    | RsMethodApp of rs_exp * string * rs_exp list
    | RsId of string
    | RsLit of rs_lit
    | RsField of rs_exp * string
    | RsBlock of rs_exp list
    | RsIf of rs_exp * rs_exp * rs_exp
    | RsMatch of rs_exp * rs_pexp list
    | RsTuple of rs_exp list
    | RsAssign of rs_lexp * rs_exp
    | RsIndex of rs_exp * rs_exp
    | RsBinop of rs_exp * rs_binop * rs_exp
    | RsAs of rs_exp * rs_type
    | RsTodo
and rs_lexp =
    | RsLexpId of string
    | RsLexpField of rs_lexp * string
    | RsLexpIndex of rs_lexp * rs_exp
    | RsLexpIndexRange of rs_lexp * rs_exp * rs_exp
    | RsLexpTodo
and rs_pexp =
    | RsPexp of rs_pat * rs_exp
    | RsPexpWhen of rs_pat * rs_exp * rs_exp

type rs_block = rs_exp list

type rs_fn = {
    name: string;
    signature: rs_fn_type;
    args: string list;
    body: rs_exp;
}

type rs_program =
    | RsProg of rs_fn list

let merge_rs_prog (prog1: rs_program) (prog2: rs_program) : rs_program =
    let RsProg (fn1) = prog1 in
    let RsProg (fn2) = prog2 in
    RsProg (fn1 @ fn2)

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
        | RsTypTodo -> "TYPE_TODO"
and string_of_rs_type_param (typ: rs_type_param) : string =
    match typ with
        | RsTypParamTyp typ -> string_of_rs_type typ
        | RsTypParamNum n -> Printf.sprintf "%d" n

let string_of_rs_lit (lit: rs_lit) : string =
    match lit  with
        | RsLitUnit -> "()"
        | RsLitTrue -> "true"
        | RsLitFalse -> "false"
        | RsLitNum n -> Printf.sprintf "%Li" n
        | RsLitBin n -> n
        | RsLitHex n -> n
        | RsLitStr s -> Printf.sprintf "\"%s\"" s
        | RsLitTodo -> "LIT_TODO"

let rec string_of_rs_pat (pat: rs_pat) : string =
    match pat with
        | RsPatLit lit -> string_of_rs_lit lit
        | RsPatId id-> id
        | RsPatType (typ, pat) -> Printf.sprintf "%s: %s" (string_of_rs_pat pat) (string_of_rs_type typ)
        | RsPatWildcard -> "_"
        | RsPatTuple pats ->
            Printf.sprintf "(%s)" (String.concat ", " (List.map string_of_rs_pat pats))
        | RsPatTodo -> "PAT_TODO"

let string_of_rs_binop (binop: rs_binop) : string =
    match binop with
        | RsBinopEq -> "=="
        | RsBinopNeq -> "!="
        | RsBinopAnd -> "&"
        | RsBinopOr -> "|"
        | RsBinopXor -> "^"
        | RsBinopLAnd -> "&&"
        | RsBinopLOr -> "||"

let indent (n: int) : string =
    String.make (n * 4) ' '

let rec string_of_rs_exp (n: int) (exp: rs_exp) : string =
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
        | RsApp (id, args)->
            Printf.sprintf "%s(%s)"
                id 
                (String.concat ", " (List.map (string_of_rs_exp n) args))
        | RsMethodApp (exp, id, args)->
            Printf.sprintf "%s.%s(%s)"
                (string_of_rs_exp n exp)
                id 
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
        | RsIf (cond, then_exp, else_exp) ->
            Printf.sprintf "if %s {\n%s%s\n%s} else %s"
                (string_of_rs_exp n cond)
                (indent (n + 1))
                (string_of_rs_exp (n + 1) then_exp)
                (indent n)
                (match else_exp with
                    | RsIf (_, _, _) -> (string_of_rs_exp n else_exp)
                    | _ -> (Printf.sprintf "{\n%s%s\n%s}"
                        (indent (n + 1))
                        (string_of_rs_exp n else_exp))
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
        | RsAs (exp, typ) ->
            Printf.sprintf "(%s as %s)"
                (string_of_rs_exp (n + 1) exp)
                (string_of_rs_type typ)
        | RsTodo -> "todo!()"
and string_of_rs_lexp (n: int) (lexp: rs_lexp) : string =
    match lexp with
        | RsLexpId id -> id
        | RsLexpField (lexp, id) ->
            Printf.sprintf "%s.%s"
                (string_of_rs_lexp n lexp)
                id
        | RsLexpIndex (lexp, idx) ->
            Printf.sprintf "%s[%s]"
                (string_of_rs_lexp n lexp)
                (string_of_rs_exp n idx)
        | RsLexpIndexRange (lexp, range_start, range_end) ->
            Printf.sprintf "%s[(%s)..=(%s)]"
                (string_of_rs_lexp n lexp)
                (string_of_rs_exp n range_start)
                (string_of_rs_exp n range_end)
        | RsLexpTodo ->  "LEXP_TODO"
and string_of_rs_pexp (n: int) (pexp: rs_pexp) : string =
    match pexp with
        | RsPexp (pat, exp) ->
            Printf.sprintf "%s => %s,\n"
                (string_of_rs_pat pat)
                (string_of_rs_exp n exp)
        | RsPexpWhen (pat, cond_exp, exp) ->
            Printf.sprintf "%s if %s => %s,\n"
                (string_of_rs_pat pat)
                (string_of_rs_exp n cond_exp)
                (string_of_rs_exp n exp)

let string_of_rs_fn_args (fn: rs_fn) : string =
    let string_of_arg_and_type (arg: string) (typ: rs_type) : string =
        Printf.sprintf "%s: %s"
            arg
            (string_of_rs_type typ)
    in
    let (arg_types, _) = fn.signature in
    String.concat ", " (List.map2 string_of_arg_and_type fn.args arg_types)

let string_of_rs_fn (fn: rs_fn) : string =
    let (args, ret) = fn.signature in 
    let args = string_of_rs_fn_args fn in
    let (_, ret_type) = fn.signature in
    let ret_type = match ret_type with
        | RsTypUnit -> ""
        | _ -> Printf.sprintf " -> %s" (string_of_rs_type ret_type)
    in
    let signature = Printf.sprintf "fn %s(%s)%s {\n%s" fn.name args ret_type (indent 1) in
    let stmts = (match fn.body with
        | RsBlock exps
            -> String.concat
                (Printf.sprintf ";\n%s" (indent 1))
                (List.map (string_of_rs_exp 1) exps)
        | _ ->string_of_rs_exp 1 fn.body) in
    Printf.sprintf "%s%s\n}" signature stmts

let string_of_rs_prog (prog: rs_program) : string =
    let RsProg (funs) = prog in
    String.concat "\n\n" (List.map string_of_rs_fn funs)
