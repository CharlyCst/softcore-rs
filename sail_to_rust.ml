(** Converts a Sail AST to a Rust AST **)

open Libsail
open Ast
open Ast_util
open Ast_defs
open Rust_gen
open Context
open Fun_defs

module SSet = Call_set.SSet

let print_id id =
    match id with
        | Id_aux (Id (x), _) -> print_string "Id "; print_endline x;
        | Id_aux (Operator (x), _) -> print_string "Op "; print_endline x

let process_register reg : rs_program =
    print_string "Register ";
    let (typ, id, exp) = match reg with | DEC_reg (typ, id, exp) -> (typ, id, exp) in
    print_id id;
    RsProg []

let process_scattered scattered : rs_program =
    print_string "Scattered ";
    (match scattered with
        | SD_function (rec_aux, tannot, id) -> print_string "function"; print_id id
        | SD_funcl (funcl) -> print_string "funcl"
        | SD_variant (id, typquant) -> print_string "variant"; print_id id
        | SD_unioncl (id, union_type) -> print_string "union"; print_id id
        | SD_mapping (id, tannot_opt) -> print_string "mapping"; print_id id
        | _ -> ());
    RsProg []


let process_vector_pat (items: 'a pat list) : rs_lit =
    let is_only_bits acc pat = match pat with
        | P_aux ((P_lit (L_aux (lit, _))), _) -> (match lit with
            | L_zero -> acc
            | L_one -> acc
            | _ -> false)
        | _ -> false
    in
    let string_of_bit (P_aux (pat, _)) = match pat with
        | P_lit (L_aux (lit, _)) -> (match lit with
            | L_zero -> "0"
            | L_one -> "1"
            | _ -> "x")
        | _ -> "X"
    in
    if (List.fold_left is_only_bits true items) then
        RsLitBin (Printf.sprintf "0b%s" (String.concat "" (List.map string_of_bit items)))
    else RsLitTodo

let process_lit (L_aux (lit, _)) : rs_lit =
    match lit with
        | L_unit -> RsLitUnit
        | L_zero -> RsLitNum Int64.zero
        | L_one -> RsLitNum Int64.one
        | L_true -> RsLitTrue
        | L_false -> RsLitFalse
        | L_num n -> RsLitNum (Big_int.to_int64 n)
        | L_hex s -> RsLitHex s
        | L_bin s -> RsLitBin s
        | L_string s -> RsLitStr s
        | L_undef -> RsLitTodo
        | L_real s -> RsLitTodo

let rec process_pat (P_aux (pat, annot)) : rs_pat =
    match pat with
        | P_lit lit -> RsPatLit (process_lit lit)
        | P_id id -> RsPatId (string_of_id id)
        | P_typ (typ, pat) -> RsPatType ((extract_type typ), (process_pat pat))
        | P_wild -> RsPatWildcard
        | P_tuple pats -> RsPatTuple (List.map process_pat pats)
        | P_vector pats -> RsPatLit (process_vector_pat pats)
        | _ -> RsPatTodo

let process_vector (items: 'a exp list) : rs_lit =
    let is_only_bits acc exp = match exp with
        | E_aux (E_lit(L_aux(lit, _)), _) -> (match lit with
            | L_zero -> acc
            | L_one -> acc
            | _ -> false)
        | _ -> false
    in
    let string_of_bit (E_aux (exp, _)) = match exp with
        | E_lit (L_aux (lit, _)) -> (match lit with
            | L_zero -> "0"
            | L_one -> "1"
            | _ -> "x")
        | _ -> "X"
    in
    if (List.fold_left is_only_bits true items) then
        RsLitBin (Printf.sprintf "0b%s" (String.concat "" (List.map string_of_bit items)))
    else RsLitTodo

let rec process_exp (E_aux (exp, aux)) : rs_exp = 
    (* print_string "Exp "; *)
    (* print_endline (string_of_exp exp); *)
    match exp with
        | E_block exp_list -> RsBlock (List.map process_exp exp_list)
        | E_id id -> RsId (string_of_id id)
        | E_lit lit -> RsLit (process_lit lit)
        | E_typ (typ, exp) -> RsTodo
        | E_app (id, exp_list) -> RsApp (RsId (string_of_id id), (List.map process_exp exp_list))
        | E_app_infix (exp1, id, exp2) -> RsTodo
        | E_tuple (exp_list) -> RsTuple (List.map process_exp exp_list)
        | E_if (exp1, exp2, exp3) -> RsIf ((process_exp exp1), (process_exp exp2), (process_exp exp3)) 
        | E_loop (loop, measure, exp1, exp2) -> RsTodo
        | E_for (id, exp1, exp2, exp3, order, exp4) -> RsTodo
        | E_vector (exp_list) -> RsLit (process_vector exp_list)
        | E_vector_access (exp1, exp2) -> RsTodo
        | E_vector_subrange (exp1, exp2, exp3) -> RsTodo
        | E_vector_update (exp1, exp2, exp3) -> RsTodo
        | E_vector_update_subrange (exp1 ,exp2, exp3, exp4) -> RsTodo
        | E_vector_append (exp1 ,exp2) -> RsTodo
        | E_list (exp_list) -> RsTodo
        | E_cons (exp1, exp2) -> RsTodo
        | E_struct (fexp_list) -> RsTodo
        | E_struct_update (exp, fexp_list) -> RsTodo
        | E_field (exp, id) -> RsField ((process_exp exp), (string_of_id id))
        | E_match (exp, pexp_list)
            -> (RsMatch (
                (process_exp exp),
                (List.map process_pexp pexp_list)
            ))
        | E_let (LB_aux (LB_val (let_var, let_exp), _), exp)
            -> (RsLet (
                (process_pat let_var),
                (process_exp let_exp),
                (process_exp exp)
            ))
        | E_assign (lexp, exp) -> 
            (RsAssign (
                (process_lexp lexp),
                (process_exp exp)
            ))
        | E_sizeof nexp -> RsTodo
        | E_return exp -> RsTodo
        | E_exit exp -> RsTodo
        | E_ref id -> RsTodo
        | E_throw exp -> RsTodo
        | E_try (exp, pexp_list) -> RsTodo
        | E_assert (exp1, exp2) -> RsTodo
        | E_var (lexp, exp1, exp2) -> RsTodo
        | E_internal_plet (pat, exp1, exp2) -> RsTodo
        | E_internal_return exp -> RsTodo
        | E_internal_value value -> RsTodo
        | E_internal_assume (n_constraint, exp) -> RsTodo
        | E_constraint n_constraint -> RsTodo
and process_lexp (LE_aux (lexp, annot)) : rs_lexp =
    match lexp with
        | LE_id id -> RsLexpId (string_of_id id)
        | LE_vector (lexp, idx) ->
            (RsLexpIndex (
                (process_lexp lexp),
                (process_exp idx)))
        | LE_vector_range (lexp, range_end, range_start) ->
            (* NOTE: Sail effectively invert the range bounds compared to rust*)
            (RsLexpIndexRange (
                (process_lexp lexp),
                (process_exp range_start),
                (process_exp range_end)))
        | LE_field (lexp, id) ->
            (RsLexpField (
                (process_lexp lexp),
                (string_of_id id)))
        | LE_app _ -> RsLexpId "TodoLexpApp"
        | LE_deref _ -> RsLexpId "TodoLexpDeref"
        | LE_vector_concat _ -> RsLexpId "TodoLexpVectorConcat"
        | LE_tuple _ -> RsLexpId "TodoLexpTuple"
        | LE_typ _ -> RsLexpId "TodoLexpTyp"
and process_pexp (Pat_aux (pexp, annot)) : rs_pexp =
    match pexp with
        | Pat_exp (pat, exp) ->
            (RsPexp (
                (process_pat pat),
                (process_exp exp)
            ))
        | Pat_when (pat, exp1, exp2) ->
            (RsPexpWhen (
                (process_pat pat),
                (process_exp exp1),
                (process_exp exp2)
            ))



(* Return the ID of an application pattern as a string, or "" otherwise. *)
let pat_app_name (P_aux (pat_aux, _)) =
    match pat_aux with
        | P_app (id, _) -> string_of_id id
        | _ -> ""

let rec process_id_pat_list id_pat_list =
    match id_pat_list with
        | (id, pat) :: t -> print_string "id/pat:"; print_id id; process_id_pat_list t
        | _ -> ()

let process_args_pat_list (P_aux (pat_aux, annot)) : string = 
    match pat_aux with
        | P_id id -> string_of_id id
        | P_typ (_, P_aux (P_id id  , _)) -> string_of_id id
        | P_tuple _ -> "TodoTupleArg"
        | _ -> "TodoArg"

let rec process_args_pat (P_aux (pat_aux, annot)) : string list = 
    match pat_aux with
        | P_app (id, [P_aux ((P_tuple pats), _)]) -> List.map process_args_pat_list pats
        | P_app _ -> ["TodoArgsApp"]
        | P_struct (id_pat_list, field_pat_wildcard) -> ["TodoArgsStruct"]
        | P_list pats -> ["TodoArgsList"]
        | P_var (var, typ) -> ["TodoArgsVar"]
        | P_cons (h, t) -> ["TodoArgsCons"]
        | P_tuple pats -> List.map process_args_pat_list pats
        | P_id id -> [string_of_id id]
        | P_typ (_, recur) -> process_args_pat recur
        | P_lit lit -> ["TodoLiteral"]
        | _ -> ["TodoArgs"]  

type function_kind =
    | FunKindFunc
    | FunKindUnion of string

let build_function (kind: function_kind) (name: string) (pat: 'a pat) (exp: 'a exp) (ctx: context): rs_fn =
    (* This function balances the lenghts of the argument and argument type list by adding more arguments if neccesary *)
    let rec add_missing_args args args_type new_args : string list =
        match (args, args_type) with
            | (ha::ta, ht::tt) -> add_missing_args ta tt (new_args @ [ha])
            | ([], ht::tt) -> add_missing_args [] tt (new_args @ ["TodoMissingArg"])
            | (_, []) -> new_args
    in

    let arg_names = process_args_pat pat in
    let signature = match kind with
        | FunKindFunc -> (match ctx_fun_type name ctx with
            | Some signature -> signature
            | None -> ([RsTypId "TodoNoSignature"], RsTypUnit))
        | FunKindUnion union -> match ctx_union_type union ctx with
            | Some typ -> (match typ with
                | RsTypTuple types -> (types, RsTypUnit) (* TODO: handle non-unit return *)
                | RsTypId id -> ([RsTypId id], RsTypUnit) (* TODO: same here *)
                | _ -> ([RsTypId "TodoUnsupportedUnionSignature"], RsTypUnit))
            | None -> ([RsTypId "TodoNoUnionSignature"], RsTypUnit)
    in
    let (arg_types, _) = signature in
    let arg_names = add_missing_args arg_names arg_types [] in
    let rs_exp = process_exp exp in
    {
        name = name;
        args = arg_names;
        signature = signature;
        body = rs_exp;
    }

let process_func (FCL_aux (func, annot)) (ctx: context) : rs_program =
    let (id, pexp) = match func with | FCL_funcl (id, pexp) -> (id, pexp) in
    let (pexp, annot) = match pexp with | Pat_aux (pexp, annot) -> (pexp, annot) in
    let name = (string_of_id id) in
    if ctx_fun_is_used name ctx then match pexp with
        | Pat_exp (pat, exp) -> RsProg [RsFn(build_function FunKindFunc name pat exp ctx)]
        | Pat_when (pat1, exp, pat2) -> RsProg []
    else match pexp with
        | Pat_exp (pat, exp) ->
                let pat_name = pat_app_name pat in
                let fun_name = Printf.sprintf "%s_%s" name pat_name in
                if ctx_fun_is_used pat_name ctx then
                    RsProg [RsFn(build_function (FunKindUnion pat_name) fun_name pat exp ctx)]
                else RsProg []
        | _ -> RsProg []

let rec process_funcl (funcl: 'a funcl list) (s: context) : rs_program =
    match funcl with
        | h :: t -> merge_rs_prog (process_func h s) (process_funcl t s)
        | [] -> RsProg []

let process_fundef (FD_function (rec_opt, tannot_opt, funcl)) (s: context) : rs_program =
    process_funcl funcl s

let process_enum (id: Ast.id) (members: Ast.id list) : rs_enum =
        let enum_name = string_of_id id in
        let enum_fields = List.map string_of_id members in 
        {
            name = enum_name;
            fields = enum_fields; 
        }
    
let process_def (TD_aux (typ, _)) : rs_program = 
        match typ with
        | TD_enum (id, member, _) -> RsProg [RsEnum(process_enum id member)] 
        | _ -> RsProg []
    
let process_node (DEF_aux (def, annot)) (s: context) : rs_program =
    match def with
        | DEF_register (DEC_aux (dec_spec, annot)) -> process_register dec_spec
        | DEF_scattered (SD_aux (scattered, annot)) -> process_scattered scattered
        | DEF_fundef (FD_aux (fundef, annot)) -> process_fundef fundef s
        | DEF_impl funcl -> process_func funcl s
        | DEF_type typ -> process_def typ
        | _ -> RsProg []

let rec process_defs defs (ctx: context): rs_program =
    match defs with
        | h :: t -> merge_rs_prog (process_node h ctx) (process_defs t ctx)
        | [] -> RsProg []


(* ———————————————————————— Sail Virtual Context Generator ————————————————————————— *)

let process_reg_name_type reg : (string * rs_type) =
    let (typ, id, exp) = match reg with 
        | DEC_reg (typ, id, exp) -> (typ, id, exp) in (string_of_id id, extract_type typ)

let process_if_register  (DEF_aux (def, annot)) : string * rs_type * bool = 
    match def with 
        | DEF_register (DEC_aux (dec_spec, annot)) -> let (reg_name, reg_type) = process_reg_name_type dec_spec in
            (reg_name, reg_type, true)
        | _ -> ("",RsTypId "", false)

let rec gather_registers defs : ((string * rs_type) list) = 
    match defs with
        | h :: t -> let (value,typ, is_register) = process_if_register h in 
            if is_register then  (value, typ) :: gather_registers t
            else gather_registers t
        | [] -> []
let generate_sail_virt_ctx defs (ctx: context): rs_program = RsProg[
    RsStruct({
        name = "SailVirtCtx";
        fields =  gather_registers defs;
    })
]

let gather_registers_list (ast: 'a ast) : string list =
    List.map (fun (x, _) -> x) (gather_registers ast.defs)


(* ———————————————————————— Sail Types Generator ————————————————————————— *)


let extract_type_nexp (Nexp_aux (nexp, _)): string =
     match nexp with
        | Nexp_constant n -> string_of_int (Nat_big_num.to_int n)
        | _ -> "TodoNumExpr"
                
let process_sub_type (id: string) (A_aux (typ, _)) : rs_obj * bool =
    match typ with
        | A_typ typ -> (RsAlias {new_typ = id; old_type = extract_type typ }, true)
        | A_bool b -> (RsConst {name = "todo"; value = "todo"}, true)
        | A_nexp exp -> (RsConst {name = id; value = extract_type_nexp exp}, true)
        
let extract_first_item_type (items: (Libsail.Ast.typ * Libsail.Ast.id) list) : rs_type =   
    assert(List.length items = 1);
    match items with 
        | x :: _ -> extract_type (fst x)
        | _ -> RsTypTodo
    
let process_type_name_type (TD_aux (typ, _)) : (rs_obj * bool) =
    match typ with
        | TD_abbrev (id, typquant, typ_arg) -> process_sub_type (string_of_id id) typ_arg
        | TD_record (id, typquant, items, _) -> (RsAlias {new_typ = (string_of_id id); old_type = extract_first_item_type items}, true)
        | _ -> (RsConst {name = "dummy"; value = "dummy"}, false)
 
let process_if_abbrev  (DEF_aux (def, annot)) : (rs_obj * bool) =
    match def with
        | DEF_type typ -> process_type_name_type typ
        | _ -> (RsConst {name = "dummy"; value = "dummy"}, false)

let rec gather_abbrev defs : (rs_obj list) =
    match defs with
        | h :: t -> let (obj, is_register) = process_if_abbrev h in
            if is_register then  obj :: gather_abbrev t
            else gather_abbrev t
        | [] -> []

let generate_sail_abbrev_list defs : rs_program list =
    List.map (fun obj -> RsProg [obj]) (gather_abbrev defs)
        
let generate_sail_abbrev defs : rs_program =
    merge_rs_prog_list (generate_sail_abbrev_list defs)


(* ———————————————————————— Generate the enumeration context  ————————————————————————— *)
  
let gen_enum_list (id: Ast.id) (members: Ast.id list) : (string * string) list =
    let enum_name = string_of_id id in
    let enum_fields = List.map string_of_id members in 
    List.map (fun e -> (e, enum_name)) enum_fields

let process_enum_entries_aux (DEF_aux (def, annot)): (string * string) list =
match def with
    | DEF_type (TD_aux (TD_enum (id, member, _), _)) -> gen_enum_list id member
    | _ -> []
    
let rec process_enum_entries (defs: 'a Libsail.Ast.def list): (string * string) list =
    match defs with
        | h :: t -> (process_enum_entries_aux h) @ (process_enum_entries t)
        | [] -> []

(* ———————————————————————— Translation function  ————————————————————————— *)

let sail_to_rust (ast: 'a ast) (ctx: context) : rs_program =
    merge_rs_prog_list [generate_sail_abbrev ast.defs; generate_sail_virt_ctx ast.defs ctx; process_defs ast.defs ctx]
