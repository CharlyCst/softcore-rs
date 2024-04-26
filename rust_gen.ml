(** Rust generation module **)

type rs_stmt =
    | RsExp
    | RsTodo

type rs_block = rs_stmt list

type rs_fn = string * rs_block

let string_of_rs_stmt (stmt: rs_stmt) : string =
    match stmt with
        | RsExp -> "exp;\n"
        | RsTodo -> "todo!(\"Not yet implemented\");\n"

let string_of_rs_fn (fn: rs_fn) : string =
    let (name, block) = fn in
    let signature = Printf.sprintf "fn %s() {\n" name in
    let stmts = String.concat "" (List.map string_of_rs_stmt block) in
    Printf.sprintf "%s%s}" signature stmts
