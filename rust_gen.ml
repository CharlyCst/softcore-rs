(** Rust generation module **)

type rs_pat =
    | RsPatLit
    | RsPatId of string
    | RsPatTodo

type rs_exp =
    | RsLet of rs_pat * rs_exp list
    | RsTodo

type rs_block = rs_exp list

type rs_fn = string * rs_block

let string_of_rs_pat (pat: rs_pat) : string =
    match pat with
        | RsPatLit -> "pat_lit"
        | RsPatId id-> id
        | RsPatTodo -> "pat_todo"

let rec string_of_rs_block (exps: rs_exp list) : string =
    String.concat "" (List.map string_of_rs_exp exps)

and string_of_rs_exp (exp: rs_exp) : string =
    match exp with
        | RsLet (pat, exp) -> Printf.sprintf "let %s = %s" (string_of_rs_pat pat) (string_of_rs_block exp)
        | RsTodo -> "todo!()"

let string_of_rs_fn (fn: rs_fn) : string =
    let (name, block) = fn in
    let signature = Printf.sprintf "fn %s() {\n" name in
    let stmts = String.concat ";\n" (List.map string_of_rs_exp block) in
    Printf.sprintf "%s%s\n}" signature stmts
