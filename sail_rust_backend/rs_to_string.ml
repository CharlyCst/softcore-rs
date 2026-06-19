open Rs_ast

let rec string_of_doc (doc : string list) : string =
  match doc with
  | head :: tail ->
    "///" ^ (if head = "" then "" else " " ^ head) ^ "\n" ^ string_of_doc tail
  | [] -> ""
;;

let string_of_const (const : bool) : string = if const then "const " else ""

let string_of_derive (derive : string list) : string =
  match derive with
  | _ :: _ -> "#[derive(" ^ String.concat ", " derive ^ ")]\n"
  | [] -> ""
;;

let string_of_generics (generics : string list) : string =
  match generics with
  | [] -> ""
  | _ -> Printf.sprintf "<%s>" (String.concat ", " generics)
;;

let string_of_generics_turbofish (generics : string list) : string =
  match generics with
  | [] -> ""
  | _ -> Printf.sprintf "::<%s>" (String.concat ", " generics)
;;

let string_of_generics_parameters (generics : rs_generic list) : string =
  let string_of_generic generic =
    match generic with
    | RsGenTyp s -> s
    | RsGenConst (s, typ) -> Printf.sprintf "const %s: %s" s typ
  in
  let generics = List.map string_of_generic generics in
  match generics with
  | [] -> ""
  | _ -> Printf.sprintf "<%s>" (String.concat ", " generics)
;;

let rec string_of_rs_type (typ : rs_type) : string =
  match typ with
  | RsTypId s -> s
  | RsTypTuple types ->
    Printf.sprintf "(%s)" (String.concat ", " (List.map string_of_rs_type types))
  | RsTypUnit -> "()"
  | RsTypGeneric t -> t
  | RsTypGenericParam (id, params) ->
    Printf.sprintf
      "%s::<%s>"
      id
      (String.concat ", " (List.map string_of_rs_type_param params))
  | RsTypArray (typ, size) ->
    Printf.sprintf "[%s; %s]" (string_of_rs_type_param typ) (string_of_rs_type_param size)
  | RsTypOption param -> Printf.sprintf "Option<%s>" (string_of_rs_type_param param)
  | RsTypTodo e -> e
  | RsTypBorrow t -> Printf.sprintf "&%s" (string_of_rs_type t)

and string_of_rs_type_param (typ : rs_type_param) : string =
  match typ with
  | RsTypParamTyp typ -> string_of_rs_type typ
  | RsTypParamNum n -> string_of_rs_exp 0 n

and string_of_rs_lit (lit : rs_lit) : string =
  match lit with
  | RsLitUnit -> "()"
  | RsLitTrue -> "true"
  | RsLitFalse -> "false"
  | RsLitNum n -> Printf.sprintf "%s" (Big_int.to_string n)
  | RsLitBin n -> n
  | RsLitHex n -> n
  | RsLitStr s -> Printf.sprintf "\"%s\"" s
  | RsLitTodo -> "LIT_TODO"

and string_of_rs_pat (pat : rs_pat) : string =
  match pat with
  | RsPatLit lit -> string_of_rs_lit lit
  | RsPatId id -> id
  | RsPatType (_typ, RsPatWildcard) -> "_"
  | RsPatType (typ, pat) ->
    Printf.sprintf "%s: %s" (string_of_rs_pat pat) (string_of_rs_type typ)
  | RsPatWildcard -> "_"
  | RsPatTuple pats ->
    Printf.sprintf "(%s)" (String.concat ", " (List.map string_of_rs_pat pats))
  | RsPatApp (name, args) ->
    Printf.sprintf
      "%s(%s)"
      (string_of_rs_pat name)
      (String.concat ", " (List.map string_of_rs_pat args))
  | RsPatSome pat -> Printf.sprintf "Some(%s)" (string_of_rs_pat pat)
  | RsPatNone -> "None"
  | RsPatTodo text -> Printf.sprintf "%s" text

and string_of_rs_binop (binop : rs_binop) : string =
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
  | RsBinopDiv -> "/"
  | RsBinopShiftLeft -> "<<"
  | RsBinopShiftRight -> ">>"
  | RsBinopMod -> "%"

and string_of_rs_unop (unop : rs_unop) : string =
  match unop with
  | RsUnopNeg -> "-"
  | RsUnopNot -> "!"

and indent (n : int) : string = String.make (n * 4) ' '

and string_of_rs_exp (n : int) (exp : rs_exp) : string =
  match exp.e_exp with
  (* The block indentation if not needed after a let, remove it to pretify *)
  | RsLet (pat, exp, { e_annot = _; e_exp = RsBlock exps }) ->
    (* TODO: If we have a type annotation for the let print it *)
    Printf.sprintf
      "let %s = %s;\n%s%s"
      (string_of_rs_pat_annot pat exp.e_annot)
      (string_of_rs_exp n exp)
      (indent n)
      (String.concat
         (Printf.sprintf ";\n%s" (indent n))
         (List.map (string_of_rs_exp n) exps))
  | RsLet (pat, exp, next) ->
    Printf.sprintf
      "let %s = %s;\n%s%s"
      (string_of_rs_pat_annot pat exp.e_annot)
      (string_of_rs_exp n exp)
      (indent n)
      (string_of_rs_exp n next)
  | RsLetMut (pat, exp, next) ->
    Printf.sprintf
      "let mut %s = %s;\n%s%s"
      (string_of_rs_pat_annot pat exp.e_annot)
      (string_of_rs_exp n exp)
      (indent n)
      (string_of_rs_exp n next)
  | RsApp (fn, generics, args) ->
    Printf.sprintf
      "%s%s(%s)"
      (string_of_rs_exp n fn)
      (string_of_generics_turbofish generics)
      (String.concat ", " (List.map (string_of_rs_exp n) args))
  | RsStaticApp (typ, func, args) ->
    Printf.sprintf
      "%s::%s(%s)"
      (string_of_rs_type typ)
      func
      (String.concat ", " (List.map (string_of_rs_exp n) args))
  | RsMethodApp { exp; name; generics; args } ->
    Printf.sprintf
      "%s.%s%s(%s)"
      (string_of_rs_exp n exp)
      name
      (string_of_generics_turbofish generics)
      (String.concat ", " (List.map (string_of_rs_exp n) args))
  | RsId id -> id
  | RsLit lit -> string_of_rs_lit lit
  | RsField (exp, field) -> Printf.sprintf "%s.%s" (string_of_rs_exp n exp) field
  | RsBlock exps ->
    Printf.sprintf
      "{\n%s%s\n%s}"
      (indent (n + 1))
      (String.concat
         (Printf.sprintf ";\n%s" (indent (n + 1)))
         (List.map (string_of_rs_exp (n + 1)) exps))
      (indent n)
  | RsConstBlock exps ->
    Printf.sprintf
      "const {\n%s%s\n%s}"
      (indent (n + 1))
      (String.concat
         (Printf.sprintf ";\n%s" (indent (n + 1)))
         (List.map (string_of_rs_exp (n + 1)) exps))
      (indent n)
  | RsInstrList exps ->
    Printf.sprintf
      "%s"
      (String.concat
         (Printf.sprintf ";\n%s" (indent n))
         (List.map (string_of_rs_exp n) exps))
  | RsIf (cond, then_exp, else_exp) ->
    Printf.sprintf
      "if {%s} {\n%s%s\n%s} else %s"
      (string_of_rs_exp n cond)
      (indent (n + 1))
      (string_of_rs_exp (n + 1) then_exp)
      (indent n)
      (match else_exp.e_exp with
       | RsIf (_, _, _) -> string_of_rs_exp n else_exp
       | _ ->
         (Printf.sprintf
            "{\n%s%s\n%s}"
            (indent (n + 1))
            (string_of_rs_exp (n + 1) else_exp))
           (indent n))
  | RsMatch (exp, pexps) ->
    Printf.sprintf
      "match %s {\n%s%s%s}"
      (string_of_rs_exp n exp)
      (indent (n + 1))
      (String.concat (indent (n + 1)) (List.map (string_of_rs_pexp (n + 1)) pexps))
      (indent n)
  | RsTuple exps ->
    Printf.sprintf "(%s)" (String.concat ", " (List.map (string_of_rs_exp n) exps))
  | RsArray exps ->
    Printf.sprintf "[%s]" (String.concat ", " (List.map (string_of_rs_exp n) exps))
  | RsArraySize (exp, size) ->
    Printf.sprintf "[%s; %s]" (string_of_rs_exp n exp) (string_of_rs_exp n size)
  | RsVec exps ->
    (* TODO(Gurvan): Ugly fix for now to bake into here *)
    Printf.sprintf
      "vec![%s].into()"
      (String.concat ", " (List.map (string_of_rs_exp n) exps))
  | RsVecSize (exp, size) ->
    (* TODO(Gurvan): Ugly fix for now to bake into here *)
    Printf.sprintf
      "vec![%s; %s].into()"
      (string_of_rs_exp n exp)
      (string_of_rs_exp n size)
  | RsAssign (exp1, exp2) ->
    Printf.sprintf "%s = %s" (string_of_rs_lexp n exp1) (string_of_rs_exp n exp2)
  | RsIndex (exp1, exp2) ->
    Printf.sprintf "%s[%s]" (string_of_rs_exp n exp1) (string_of_rs_exp n exp2)
  | RsBinop (exp1, binop, exp2) ->
    Printf.sprintf
      "(%s %s %s)"
      (string_of_rs_exp n exp1)
      (string_of_rs_binop binop)
      (string_of_rs_exp n exp2)
  | RsUnop (unop, exp) ->
    Printf.sprintf "%s(%s)" (string_of_rs_unop unop) (string_of_rs_exp n exp)
  | RsAs (exp, typ) ->
    Printf.sprintf "(%s as %s)" (string_of_rs_exp (n + 1) exp) (string_of_rs_type typ)
  | RsSome exp -> Printf.sprintf "Some(%s)" (string_of_rs_exp n exp)
  | RsNone -> "None"
  | RsPathSeparator (t1, t2) ->
    Printf.sprintf "%s::%s" (string_of_rs_type t1) (string_of_rs_type t2)
  | RsFor (var, start, until, body) ->
    Printf.sprintf
      "for %s in %s..=%s {\n%s%s\n%s}"
      (string_of_rs_type var)
      (string_of_rs_exp 0 start)
      (string_of_rs_exp 0 until)
      (indent (n + 1))
      (string_of_rs_exp (n + 1) body)
      (indent n)
  | RsForRev (var, start, until, body) ->
    Printf.sprintf
      "for %s in (%s..=%s).rev() {\n%s%s\n%s}"
      (string_of_rs_type var)
      (string_of_rs_exp 0 start)
      (string_of_rs_exp 0 until)
      (indent (n + 1))
      (string_of_rs_exp (n + 1) body)
      (indent n)
  | RsStruct (name, entries) ->
    Printf.sprintf
      "%s {\n%s%s\n%s}"
      (string_of_rs_type name)
      (indent (n + 1))
      (String.concat
         (Printf.sprintf ",\n%s" (indent (n + 1)))
         (List.map
            (fun (name, typ) ->
               Printf.sprintf "%s: %s" name (string_of_rs_exp (n + 1) typ))
            entries))
      (indent n)
  | RsStructAssign (exp, field, value) ->
    Printf.sprintf
      "%s.%s = %s; %s"
      (string_of_rs_exp n exp)
      field
      (string_of_rs_exp n value)
      (string_of_rs_exp n exp)
  | RsReturn exp -> Printf.sprintf "return %s;" (string_of_rs_exp n exp)
  | RsTodo text -> Printf.sprintf "todo!(\"%s\")" text
  | RsBorrow e -> Printf.sprintf "&%s" (string_of_rs_exp n e)

and string_of_rs_annot (annot : rs_type option) : string =
  match annot with
  | Some t -> Printf.sprintf ": %s" (string_of_rs_type t)
  | None -> ""

and string_of_rs_pat_annot (pat : rs_pat) (annot : rs_type option) : string =
  match pat with
  | RsPatType (t, p) -> Printf.sprintf "%s: %s" (string_of_rs_pat p) (string_of_rs_type t)
  | _ -> Printf.sprintf "%s%s" (string_of_rs_pat pat) (string_of_rs_annot annot)

and string_of_rs_lexp (n : int) (lexp : rs_lexp) : string =
  match lexp with
  | RsLexpId id -> id
  | RsLexpTyp (id, typ) -> Printf.sprintf "%s: %s" id (string_of_rs_type typ)
  | RsLexpField (exp, id) -> Printf.sprintf "%s.%s" (string_of_rs_exp n exp) id
  | RsLexpIndex (lexp, idx) ->
    Printf.sprintf "%s[%s]" (string_of_rs_lexp n lexp) (string_of_rs_exp n idx)
  | RsLexpIndexRange (lexp, range_start, range_end) ->
    (* Implement support for this case if the assertion fails *)
    (* assert ((string_of_rs_exp n range_start) = "(64 - 1)"); *)
    (* assert ((string_of_rs_exp n range_end) = "0"); *)
    Printf.sprintf
      "%s[%s..%s]"
      (string_of_rs_lexp n lexp)
      (string_of_rs_exp 0 range_start)
      (string_of_rs_exp 0 range_end)
    (* string_of_rs_lexp n lexp *)
  | RsLexpBitVectorAccess _ ->
    (* TODO: This constructor should no longer be available so we should
           fail *)
    assert false (* TODO(Gurvan) *)
  | RsLexpTodo -> "LEXP_TODO"

and string_of_rs_pexp (n : int) (pexp : rs_pexp) : string =
  match pexp with
  | RsPexp (pat, exp) ->
    Printf.sprintf "%s => {%s}\n" (string_of_rs_pat pat) (string_of_rs_exp n exp)
  | RsPexpWhen (pat, cond_exp, exp) ->
    Printf.sprintf
      "%s if {%s} => {%s}\n"
      (string_of_rs_pat pat)
      (string_of_rs_exp n cond_exp)
      (string_of_rs_exp n exp)
;;

let string_of_rs_fn_args (fn : rs_fn) : string =
  let string_of_arg_and_type (arg : rs_pat) (typ : rs_type) : string =
    match typ with
    | RsTypUnit -> "unit_arg: ()"
    | _ -> Printf.sprintf "%s: %s" (string_of_rs_pat arg) (string_of_rs_type typ)
  in
  let arg_types = fn.signature.args in
  String.concat ", " (List.map2 string_of_arg_and_type fn.args arg_types)
;;

let string_of_rs_fn (fn : rs_fn) : string =
  let doc = string_of_doc fn.doc in
  let const = string_of_const fn.const in
  let args = string_of_rs_fn_args fn in
  let ret_type =
    match fn.signature.ret with
    | RsTypUnit -> ""
    | ret_type -> Printf.sprintf " -> %s" (string_of_rs_type ret_type)
  in
  let generics = string_of_generics_parameters fn.signature.generics in
  let signature =
    Printf.sprintf
      "%spub %sfn %s%s(%s)%s {\n%s"
      doc
      const
      fn.name
      generics
      args
      ret_type
      (indent 1)
  in
  let stmts =
    match fn.body.e_exp with
    | RsBlock exps ->
      String.concat
        (Printf.sprintf ";\n%s" (indent 1))
        (List.map (string_of_rs_exp 1) exps)
    | _ -> string_of_rs_exp 1 fn.body
  in
  Printf.sprintf "%s%s\n}" signature stmts
;;

let remove_last_char (s : string) : string =
  if String.length s = 0 then s else String.sub s 0 (String.length s - 1)
;;

let parse_enum_fields (entries : (string * rs_type option) list) : string =
  let field_typ (typ : rs_type option) : string =
    match typ with
    | None -> ""
    | Some typ -> "(" ^ string_of_rs_type typ ^ ")"
  in
  let prefixed_entries =
    List.map (fun (name, typ) -> "    " ^ name ^ field_typ typ) entries
  in
  String.concat ",\n" prefixed_entries
;;

let string_of_rs_enum (enum : rs_enum) : string =
  let doc = string_of_doc enum.doc in
  Printf.sprintf
    "%s%spub enum %s%s {\n%s\n}"
    doc
    (string_of_derive enum.derive)
    enum.name
    (string_of_generics_parameters enum.generics)
    (parse_enum_fields enum.fields)
;;

let parse_struct_fields (entries : (string * rs_type) list) : string =
  let prefixed_entries =
    List.map
      (fun s -> "    pub " ^ fst s ^ ": " ^ string_of_rs_type (snd s) ^ ",\n")
      entries
  in
  let merged_fields = String.concat "" prefixed_entries in
  remove_last_char merged_fields (* Removes last '\n'*)
;;

let string_of_rs_struct (struc : rs_struct) : string =
  let generics = string_of_generics_parameters struc.generics in
  let attributes = string_of_derive struc.derive in
  Printf.sprintf
    "%s%spub struct %s%s {\n%s\n}"
    (string_of_doc struc.doc)
    attributes
    struc.name
    generics
    (parse_struct_fields struc.fields)
;;

let string_of_rs_obj (obj : rs_obj) : string =
  match obj with
  | RsFn fn -> string_of_rs_fn fn
  | RsEnum enum -> string_of_rs_enum enum
  | RsStruct struc -> string_of_rs_struct struc
  | RsAlias alias ->
    Printf.sprintf
      "pub type %s%s = %s;"
      alias.new_typ
      (string_of_generics_parameters alias.generics)
      (string_of_rs_type alias.old_type)
  | RsConst const ->
    Printf.sprintf
      "%spub const %s: %s = %s;"
      (string_of_doc const.doc)
      const.name
      (string_of_rs_type const.typ)
      (string_of_rs_exp 0 const.value)
  | RsAttribute value -> Printf.sprintf "#![%s]" value
  | RsImport value -> Printf.sprintf "use %s;" value
  | RsObjTodo s -> s
;;

let string_of_rs_prog (prog : rs_program) : string =
  let (RsProg funs) = prog in
  String.concat "\n\n" (List.map string_of_rs_obj funs) ^ "\n"
;;
