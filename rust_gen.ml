(** Rust generation module **)

type rs_pat =
    | RsPatLit
    | RsPatId of string
    | RsPatTodo

type rs_lit =
    | RsLitUnit
    | RsLitTrue
    | RsLitFalse
    | RsLitNum of int64
    | RsLitBin of string
    | RsLitHex of string
    | RsLitStr of string
    | RsLitTodo

type rs_exp =
    | RsLet of rs_pat * rs_exp * rs_exp
    | RsApp of string * rs_exp list
    | RsId of string
    | RsLit of rs_lit
    | RsBlock of rs_exp list
    | RsTodo

type rs_block = rs_exp list

type rs_fn = string * rs_exp

let string_of_rs_pat (pat: rs_pat) : string =
    match pat with
        | RsPatLit -> "pat_lit"
        | RsPatId id-> id
        | RsPatTodo -> "PAT_TODO"

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

let rec string_of_rs_block (exps: rs_exp list) : string =
    String.concat "" (List.map string_of_rs_exp exps)

and string_of_rs_exp (exp: rs_exp) : string =
    match exp with
        (* The block indentation if not nedded after a  let, remove it to pretify*)
        | RsLet (pat, exp, RsBlock exps) ->
            Printf.sprintf "let %s = %s;\n%s"
                (string_of_rs_pat pat)
                (string_of_rs_exp exp)
                (String.concat ";\n" (List.map string_of_rs_exp exps))
        | RsLet (pat, exp, next) ->
            Printf.sprintf "let %s = %s;\n%s"
                (string_of_rs_pat pat)
                (string_of_rs_exp exp)
                (string_of_rs_exp next)
        | RsApp (id, args)-> Printf.sprintf "%s(%s)" id (String.concat ", " (List.map string_of_rs_exp args))
        | RsId id -> id
        | RsLit lit  -> string_of_rs_lit lit
        | RsBlock exps -> Printf.sprintf "{\n%s\n}" (String.concat ";\n" (List.map string_of_rs_exp exps))
        | RsTodo -> "todo!()"

let string_of_rs_fn (fn: rs_fn) : string =
    let (name, exp) = fn in
    let signature = Printf.sprintf "fn %s() {\n" name in
    let stmts = string_of_rs_exp exp in
    Printf.sprintf "%s%s\n}" signature stmts
