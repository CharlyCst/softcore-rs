open Libsail

open Ast
open Ast_util
open Jib
open Jib_compile
open Jib_util
open Type_check
open PPrint
open Value2
module Document = Pretty_print_sail.Document

open Anf

open Rust_gen
open Call_set

module Big_int = Nat_big_num

let c_error ?loc:(l = Parse_ast.Unknown) message = raise (Reporting.err_general l ("\nC backend: " ^ message))


(** Converts a Sail AST to a Rust **)
module Codegen () = struct
    open Libsail
    open Ast
    open Ast_util
    open Ast_defs
    open Rust_gen
    open Context
    
    module SSet = Call_set.SSet

    type function_kind =
        | FunKindFunc
        | FunKindUnion of string * string

    (* ——————————————————————————————— Type Utils ——————————————————————————————— *)
    
    let map_union (a: 'a SMap.t) (b: 'a SMap.t) : 'a SMap.t =
        let select key elt_a elt_b = Some(elt_a) in
        SMap.union select a b
    
    let defs_empty = {
        funs = SMap.empty;
        unions = SMap.empty;
    }
    
    let defs_merge (a: defs) (b: defs) : defs =
        {
            funs = map_union a.funs b.funs;
            unions = map_union a.unions b.unions;
        }
    
    let defs_add_union (defs: defs) (union: unionmap) : defs =
        let unions = map_union union defs.unions in
        {
            funs = defs.funs;
            unions = unions;
        }
    
    let defs_from_union (union: unionmap) : defs = 
        {
            funs = SMap.empty;
            unions = union;
        }
    
    let defs_from_funs (funs: defmap) : defs =
        {
            funs = funs;
            unions = SMap.empty;
        }

    (* —————————————————————————————— Other Utils ——————————————————————————————— *)

    let capitalize_after_removal s =
        if String.length s > 1 then
          String.uppercase_ascii (String.sub s 1 (String.length s - 1))
        else
          ""
    
    let print_id id =
        match id with
            | Id_aux (Id (x), _) -> print_string "Id "; print_endline x;
            | Id_aux (Operator (x), _) -> print_string "Op "; print_endline x

    let parse_order order: string = 
        match order with
            | Ord_aux (Ord_inc, _) -> "inc"
            | Ord_aux (Ord_dec, _) -> "dec"
    
    let process_register reg : rs_program =
        print_string "Register ";
        let (typ, id, exp) = match reg with | DEC_reg (typ, id, exp) -> (typ, id, exp) in
        print_id id;
        RsProg []
    
    let process_scattered scattered : rs_program =
        print_string "Scattered ";
        (match scattered with
            | SD_function (id, tannot) -> print_string "function"; print_id id
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
    
    let rec process_lit (L_aux (lit, _)) : rs_lit =
        match lit with
            | L_unit -> RsLitUnit
            | L_zero -> RsLitFalse
            | L_one -> RsLitTrue
            | L_true -> RsLitTrue
            | L_false -> RsLitFalse
            | L_num n -> RsLitNum (Big_int.to_int64 n)
            | L_hex s -> RsLitHex s
            | L_bin s -> RsLitBin s
            | L_string s -> RsLitStr s
            | L_undef -> RsLitTodo
            | L_real s -> RsLitTodo
    
    and process_pat (P_aux (pat, annot)) : rs_pat =
        match pat with
            | P_lit lit -> RsPatLit (process_lit lit)
            | P_id id -> RsPatId (string_of_id id)
            | P_typ (typ, pat) -> RsPatType ((extract_type typ), (process_pat pat))
            | P_wild -> RsPatWildcard
            | P_tuple pats -> RsPatTuple (List.map process_pat pats)
            | P_vector pats -> RsPatLit (process_vector_pat pats)
            | P_or (_, _) -> RsPatId "TODO_PAT_or"
            | P_not _ -> RsPatId "TODO_PAT_not"
            | P_as (_, _) -> RsPatId "TODO_PAT_as"
            | P_var (_, _) -> RsPatId "TODO_PAT_var"
            | P_app (id, exp_list) -> RsPatApp(RsPatId (string_of_id id), List.map process_pat exp_list)
            
            | P_vector_concat _ -> RsPatId "TODO_PAT_vector_concat"
            | P_vector_subrange (_, _, _) -> RsPatId "TODO_PAT_vector_subrange"
            | P_list _ -> RsPatId "TODO_PAT_list"
            | P_cons (_, _) -> RsPatId "TODO_PAT_cons"
            | P_string_append _ -> RsPatId "TODO_PAT_string_append"
            | P_struct (_, _) -> RsPatId "TODO_PAT_struct"
    
    and process_exp (ctx: context) (E_aux (exp, aux)) : rs_exp = 
        (* print_string "Exp "; *)
        (* print_endline (string_of_exp exp); *)
        match exp with
            | E_block exp_list -> RsBlock (List.map (process_exp ctx) exp_list)
            | E_id id -> RsId (string_of_id id)
            | E_lit lit -> RsLit (process_lit lit)
            | E_typ (typ, exp) -> RsAs(process_exp ctx exp, extract_type typ)
            | E_app (id, exp_list) -> RsApp (RsId (string_of_id id), (List.map (process_exp ctx) exp_list))
            | E_app_infix (exp1, id, exp2) -> RsTodo "E_app_infix"
            | E_tuple (exp_list) -> RsTuple (List.map (process_exp ctx) exp_list)
            | E_if (exp1, exp2, exp3) -> RsIf ((process_exp ctx exp1), (process_exp ctx exp2), (process_exp ctx exp3)) 
            | E_loop (loop, measure, exp1, exp2) -> RsTodo "E_loop"
            | E_for (id, E_aux (E_lit lit1, _), E_aux (E_lit lit2, _) , exp3, order, exp4) ->  assert(string_of_exp exp3 ="1"); assert(parse_order order = "inc");
               RsFor (RsTypId(string_of_id id), process_lit lit1, process_lit lit2, process_exp ctx exp4 )(* TODO: Implement a more general for loop*)
            | E_for (_,_,_,_,_,_) -> RsTodo "E_for"
            | E_vector (exp_list) -> process_vector ctx exp_list
            | E_vector_access (exp1, exp2) -> RsTodo "E_vector_access"
            | E_vector_subrange (exp1, exp2, exp3) -> RsTodo "E_vector_subrange"
            | E_vector_update (exp1, exp2, exp3) -> RsTodo "E_vector_update"
            | E_vector_update_subrange (exp1 ,exp2, exp3, exp4) -> RsTodo "E_update_subrange"
            | E_vector_append (exp1 ,exp2) -> RsTodo "E_vector_append"
            | E_list (exp_list) -> RsTodo "E_list"
            | E_cons (exp1, exp2) -> RsTodo "E_cons"
            | E_struct fexp_list -> RsStruct (ctx.ret_type, process_fexp_entries ctx fexp_list)
            (* todo: This is enough for the risc-v translation, but must be refactored for a more general transpiler *)
            | E_struct_update (exp, fexp_list) -> assert(List.length fexp_list = 1); RsStruct (RsTypId "BitField", (process_fexp_entries ctx fexp_list))
            | E_field (exp, id) -> RsField ((process_exp ctx exp), (string_of_id id))
            | E_match (exp, pexp_list)
                -> (RsMatch (
                    (process_exp ctx exp),
                    (List.map (process_pexp ctx) pexp_list)
                ))
            | E_let (LB_aux (LB_val (let_var, let_exp), _), exp)
                -> 
                let new_pat = process_pat let_var in
                let new_pat = match new_pat with 
                    | RsPatType (typ, exp) -> 
                        ctx.ret_type <- typ; 
                        RsPatType (typ, exp)
                    | _ -> new_pat
                in
                (RsLet (
                    (new_pat),
                    (process_exp ctx let_exp),
                    (process_exp ctx exp)
                ))
            | E_assign (lexp, exp) -> 
                (RsAssign (
                    (process_lexp ctx lexp),
                    (process_exp ctx exp)
                ))
            | E_sizeof nexp -> RsTodo "E_sizeof"
            | E_return exp -> RsReturn (process_exp ctx exp)
            | E_exit exp -> RsApp(RsId "__exit" , [])
            | E_ref id -> RsTodo "E_ref"
            | E_throw exp -> RsApp(RsId "panic!", [RsLit(RsLitStr "todo_process_panic_type")])
            | E_try (exp, pexp_list) -> RsTodo "E_try"
            (* TODO: In the future process the assertion message *)
            | E_assert (exp1, exp2) -> RsApp(RsId "assert!", [process_exp ctx exp1; RsId "\"Process message\""])
            | E_var (lexp, exp1, exp2) -> RsTodo "E_var"
            | E_internal_plet (pat, exp1, exp2) -> RsTodo "E_internal_plet"
            | E_internal_return exp -> RsTodo "E_internal_return"
            | E_internal_value value -> RsTodo "E_internal_value"
            | E_internal_assume (n_constraint, exp) -> RsTodo "E_internal_assume"
            | E_constraint n_constraint -> RsTodo "E_constraint"
            | E_config cfgs -> RsTodo "E_config"
    and process_lexp (ctx: context) (LE_aux (lexp, annot)) : rs_lexp =
        match lexp with
            | LE_id id -> RsLexpId (string_of_id id)
            | LE_vector (lexp, idx) ->
                (RsLexpIndex (
                    (process_lexp ctx lexp),
                    (process_exp ctx idx)))
            | LE_vector_range (lexp, range_start, range_end) ->
                (* RsLexpTodo *) (* TODO: properly access subranges *)
                (RsLexpIndexRange (
                    (process_lexp ctx lexp),
                    (process_exp ctx range_start),
                    (process_exp ctx range_end)))
            | LE_field (lexp, id) ->
                (RsLexpField (
                    (process_lexp ctx lexp),
                    (string_of_id id)))
            | LE_app _ -> RsLexpId "TodoLexpApp"
            | LE_deref _ -> RsLexpId "TodoLexpDeref"
            | LE_vector_concat _ -> RsLexpId "TodoLexpVectorConcat"
            | LE_tuple _ -> RsLexpId "TodoLexpTuple"
            | LE_typ _ -> RsLexpId "TodoLexpTyp"
    and process_pexp (ctx: context) (Pat_aux (pexp, annot)) : rs_pexp =
        match pexp with
            | Pat_exp (pat, exp) ->
                (RsPexp (
                    (process_pat pat),
                    (process_exp ctx exp)
                ))
            | Pat_when (pat, exp1, exp2) ->
                (RsPexpWhen (
                    (process_pat pat),
                    (process_exp ctx exp1),
                    (process_exp ctx exp2)
                ))
    (* todo: Currently we assume that all vectors are vector of bits, in the future we should make this function more general*)
    and process_vector (ctx: context) (items: 'a exp list) : rs_exp =
        let is_only_bits acc exp = match exp with
            | E_aux (E_lit(L_aux(lit, _)), _) -> (match lit with
                | L_zero -> acc
                | L_one -> acc
                | _ -> false)
            | _ -> false
        in
        let is_literral = (List.fold_left is_only_bits true items) in
        let string_of_bit (E_aux (exp, _)) = match exp with
            | E_lit (L_aux (lit, _)) -> (match lit with
                | L_zero -> "0"
                | L_one -> "1"
                | _ -> "x")
            | _ -> "X"
        in
        let vector_length = List.length items in
        let vector_length_exp = RsLit (RsLitNum (Int64.of_int vector_length)) in
        let rec parse_arguments (idx: int) (elements: 'a exp list) : rs_exp list = match elements with
            | e1 :: e -> RsMethodApp(RsId "__generated_vector", "set_vector_entry", [RsLit(RsLitNum (Int64.of_int idx)); process_exp ctx e1]) :: parse_arguments (idx+1) (e)
            | [] -> []
        in 
        if is_literral then
            (* Generate a bitvector *)
            RsStaticApp(RsTypGenericParam ("BitVector::", [RsTypParamNum vector_length_exp]), "new", [RsLit(RsLitBin (Printf.sprintf "0b%s" (String.concat "" (List.map string_of_bit items))))])
        else 
            let init_type = RsId (Printf.sprintf "BitVector::<%d>::new_empty()" vector_length) in
            let init_expr = RsInstrList ((parse_arguments 0 items) @ [ RsId "__generated_vector"]) in
            RsBlock[RsLet (RsPatType (RsTypGenericParam ("vector", [RsTypParamNum vector_length_exp]), RsPatId "mut __generated_vector"), init_type, init_expr)]
    and process_fexp_entries (ctx: context) (fexps: 'a Libsail.Ast.fexp list) : (string * rs_exp) list =
        match fexps with
        | (FE_aux (FE_fexp (id, exp), _)) :: r -> (string_of_id id, process_exp ctx exp) :: process_fexp_entries ctx r
        | [] -> []
    
    (* Return the ID of an application pattern as a string, or "" otherwise. *)
    and pat_app_name (P_aux (pat_aux, _)) =
        match pat_aux with
            | P_app (id, _) -> string_of_id id
            | _ -> ""
    
    and process_id_pat_list id_pat_list =
        match id_pat_list with
            | (id, pat) :: t -> print_string "id/pat:"; print_id id; process_id_pat_list t
            | _ -> ()
    
    and extract_pat_name (pat: rs_pat) : string =
        match pat with
            | RsPatLit lit -> string_of_rs_lit lit
            | RsPatId id-> id
            | RsPatType (typ, pat) -> (string_of_rs_pat pat) 
            | RsPatWildcard -> "_"
            | _ -> failwith "Implement this part"
    
    and process_args_pat_list value : string = (extract_pat_name (process_pat value))
            
    and process_args_pat (P_aux (pat_aux, annot)) : string list = 
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
            | P_wild -> ["_"]
            | P_or (_, _) -> ["TodoOr"]
            | P_not _ -> ["TodoNot"]
            | P_as (_, _) -> ["TodoAs"]
            | P_vector vec -> [string_of_rs_pat(RsPatLit (process_vector_pat vec))]
            | P_vector_concat _ -> ["TodoVectorConcat"]
            | P_vector_subrange (_, _, _) -> ["TodoVectorSubrange"]
            | P_string_append _  -> ["TodoStringAppend"]
    
   
    and build_function (kind: function_kind) (name: string) (pat: 'a pat) (exp: 'a exp) (ctx: context): rs_fn =
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
            | FunKindUnion (func, union) ->
                (* We look up the function definition to get the return type *)
                let ret_type = match SMap.find_opt func ctx.defs.funs with
                    | Some func_t -> snd func_t
                    | None -> RsTypUnit
                in
                
                match ctx_union_type union ctx with
                | Some typ -> (match typ with
                    | RsTypTuple types -> (types, ret_type)
                    | RsTypId id -> ([RsTypId id], ret_type)
                    | RsTypUnit -> ([], ret_type)
                    | _ -> ([RsTypId "todo_signature"], RsTypId "todo_signature"))
                | None -> ([RsTypId "TodoNoUnionSignature"], RsTypUnit)
        in
        let (arg_types, _) = signature in
        let arg_names = add_missing_args arg_names arg_types [] in
        let arg_names = List.map (fun e -> RsId e) arg_names in
        ctx.ret_type <- snd signature;
        assert (List.length arg_names = List.length (fst signature));
        let rs_exp = process_exp ctx exp in
        {
            name = name;
            args = arg_names;
            signature = signature;
            body = rs_exp;
        }
    
    and process_func (FCL_aux (func, annot)) (ctx: context) : rs_program =
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
                        RsProg [RsFn(build_function (FunKindUnion (name, pat_name)) fun_name pat exp ctx)]
                    else RsProg []
            | _ -> RsProg []
    
    and process_funcl (funcl: 'a funcl list) (s: context) : rs_program =
        match funcl with
            | h :: t -> merge_rs_prog (process_func h s) (process_funcl t s)
            | [] -> RsProg []
    
    and process_fundef (FD_function (rec_opt, tannot_opt, funcl)) (s: context) : rs_program =
        process_funcl funcl s
    
    and process_enum (id: Ast.id) (members: Ast.id list) : rs_enum =
            let enum_name = string_of_id id in
            let enum_fields = List.map string_of_id members in 
            {
                name = enum_name;
                fields = enum_fields; 
            }
    
    and process_unions (members: Ast.type_union list) : (string * rs_type) list = 
        match members with 
            | (Tu_aux (Tu_ty_id (typ, id), annot)) :: v -> (string_of_id id, extract_type typ) :: process_unions v
            | [] -> [] 
                 
    and process_def (TD_aux (typ, _)) : rs_program = 
            match typ with
            | TD_enum (id, member, _) -> RsProg [RsEnum(process_enum id member)] 
            | TD_variant (id, typquant, members, _) when string_of_id id = "option" -> RsProg [] (* Special semantics in rust *)
            | TD_variant (id, typquant, members, _) -> RsProg[RsTypedEnum({ 
                name = string_of_id id;
                fields = process_unions members;
            })]
            | _ -> RsProg []
        
    and process_node (DEF_aux (def, annot)) (s: context) : rs_program =
        match def with
            | DEF_register (DEC_aux (dec_spec, annot)) -> process_register dec_spec
            | DEF_scattered (SD_aux (scattered, annot)) -> process_scattered scattered
            | DEF_fundef (FD_aux (fundef, annot)) -> process_fundef fundef s
            | DEF_impl funcl -> process_func funcl s
            | DEF_type typ -> process_def typ
            | _ -> RsProg []
    
    and process_defs defs (ctx: context): rs_program =
        match defs with
            | h :: t -> merge_rs_prog (process_node h ctx) (process_defs t ctx)
            | [] -> RsProg []
    
    
    (* ———————————————————————— Sail Virtual Context Generator ————————————————————————— *)
    
    and process_reg_name_type reg : (string * rs_type) =
        let (typ, id, exp) = match reg with 
            | DEC_reg (typ, id, exp) -> (typ, id, exp) in (string_of_id id, extract_type typ)
    
    and process_if_register  (DEF_aux (def, annot)) : string * rs_type * bool = 
        match def with 
            | DEF_register (DEC_aux (dec_spec, annot)) -> let (reg_name, reg_type) = process_reg_name_type dec_spec in
                (reg_name, reg_type, true)
            | _ -> ("",RsTypId "", false)
    
    and gather_registers defs : ((string * rs_type) list) = 
        match defs with
            | h :: t -> let (value,typ, is_register) = process_if_register h in 
                if is_register then  (value, typ) :: gather_registers t
                else gather_registers t
            | [] -> []
    and generate_sail_virt_ctx defs (ctx: context): rs_program = RsProg[
        RsStruct({
            name = "SailVirtCtx";
            fields =  gather_registers defs;
        })
    ]
    
    and gather_registers_list (ast: ('a, 'b) ast) : string list =
        List.map (fun (x, _) -> x) (gather_registers ast.defs)
    
    
    (* ———————————————————————— Sail Types Generator ————————————————————————— *)
    
    and nconstraint_to_rs_exp (NC_aux (n, _)): rs_exp =
        match n with
            | NC_true -> RsLit (RsLitTrue)
            | NC_false -> RsLit (RsLitFalse)
            | _ -> RsTodo "TodoNConstraint"
    
    and nexp_to_rs_exp (Nexp_aux (nexp, _)): rs_exp =
         match nexp with
            | Nexp_constant n -> RsLit (RsLitNum (Nat_big_num.to_int64 n))
            | Nexp_times (n, m) -> RsBinop (nexp_to_rs_exp n, RsBinopMult, nexp_to_rs_exp m)
            | Nexp_sum   (n, m) -> RsBinop (nexp_to_rs_exp n, RsBinopAdd, nexp_to_rs_exp m)
            | Nexp_minus (n, m) -> RsBinop (nexp_to_rs_exp n, RsBinopSub, nexp_to_rs_exp m)
            | Nexp_exp n ->  RsStaticApp (RsTypId "usize", "pow", [nexp_to_rs_exp n; RsLit (RsLitNum (Int64.of_int 2))])  (* exponential, it seems it is always 2 ^ n *)
            | Nexp_neg n -> RsUnop (RsUnopNeg, nexp_to_rs_exp n)
            | Nexp_id id -> RsId (string_of_id id)
            | Nexp_var kid  -> RsTodo "TodoVarExpr" (* variable *)
            | Nexp_app (fn, args) -> RsTodo "TodoAppExpr" (* app *)
            | Nexp_if (cond, if_block, else_block) -> RsTodo "TodoIfExpr" (* if-then-else *)
     
    and get_first_two_elements lst =
        assert ((List.length lst) = 2);
        match lst with
        | first :: second :: _ -> (first, second)
        | _ -> failwith "List does not have enough elements"
      
    and extract_type (Typ_aux (typ, _)): rs_type =
        match typ with
            | Typ_id id when string_of_id id = "unit" -> RsTypUnit
            | Typ_id id -> RsTypId (string_of_id id)
            | Typ_var (Kid_aux (Var x, _)) -> RsTypGeneric x
            | Typ_tuple types -> RsTypTuple (List.map extract_type types)
            | Typ_fn _ -> RsTypId "TodoFnType"
            | Typ_app (id, params) -> if (string_of_id id) = "vector" then 
                    let (size, typ) = get_first_two_elements (List.map extract_type_arg params) in
                    RsTypArray (typ,size)
                else 
                    RsTypGenericParam ((string_of_id id), (List.map extract_type_arg params))
            | Typ_internal_unknown -> RsTypId "TodoUnknownType"
            | Typ_bidir (_, _) -> RsTypId "TodoBidirType"
            | Typ_exist (_, _, typ) -> extract_type typ
    and extract_type_arg (A_aux (typ, _)): rs_type_param =
        match typ with
            | A_nexp exp -> extract_type_nexp exp
            | A_typ typ -> RsTypParamTyp (extract_type typ)
            | A_bool b -> RsTypParamTyp (RsTypId "TodoBoolType")
    and extract_type_nexp (Nexp_aux (nexp, l)): rs_type_param =
        match nexp with
            | Nexp_constant n -> RsTypParamNum (RsLit (RsLitNum (Nat_big_num.to_int64 n)))
            | Nexp_app (Id_aux (_, _), _) -> RsTypParamTyp (RsTypId "TodoNexpTypeApp")
            | Nexp_id id -> RsTypParamTyp (RsTypId (string_of_id id))
            | Nexp_var var -> RsTypParamTyp (RsTypId (capitalize_after_removal (string_of_kid var))) 
            | Nexp_times (_, _)
            | Nexp_sum (_, _)
            | Nexp_minus (_, _)
            | Nexp_exp _
            | Nexp_neg _ -> RsTypParamNum ( RsBlock [nexp_to_rs_exp (Nexp_aux (nexp, l))])
            | _ -> RsTypParamTyp (RsTypId "TodoNexpOther")
    
    (* ———————————————————————————— Value Definition ———————————————————————————— *)
    
    let extract_types (TypSchm_aux (typeschm, _)) : rs_fn_type =
        (* We ignore the type quantifier for now, there is no `forall` on most types of interest *)
        let TypSchm_ts (type_quant, typ) = typeschm in
        let Typ_aux (typ, _) = typ in
        match typ with 
            | Typ_fn (args, ret) -> ((List.map extract_type args), extract_type ret)
            | _ -> ([RsTypTodo "todo_extract_types"], RsTypTodo "todo_extract_types")
    
    let val_fun_def (val_spec: val_spec_aux) : defmap =
        let map = SMap.empty in
        let VS_val_spec (typeschm, id, extern) = val_spec in
        (* print_string (string_of_id id); *)
        (* print_string " - "; *)
        (* print_endline (string_of_typschm typeschm); *)
        SMap.add (string_of_id id) (extract_types typeschm) map
    
    (* ————————————————————————————————— Union —————————————————————————————————— *)
    
    let type_union_def (Tu_aux (union, _)) : unionmap = 
        let Tu_ty_id (typ, id) = union in
        SMap.add (string_of_id id) (extract_type typ) SMap.empty 
    
    let rec type_union_defs (members: type_union list) : unionmap =
        match members with
            | head :: tail -> map_union (type_union_def head) (type_union_defs tail)
            | [] -> SMap.empty
    
    let type_def_fun_def (TD_aux (typ, _)) : unionmap =
        match typ with
            | TD_abbrev (id, typquant, typ_arg) -> print_string (string_of_id id); print_endline " Abbrev"; SMap.empty
            | TD_record (id, typquant, items, _) -> print_string (string_of_id id); print_endline " Record"; SMap.empty
            | TD_variant (id, typquant, members, _) ->
                print_string (string_of_id id); print_endline " Variant";
                type_union_defs members
            | TD_enum (id, member, _) -> print_string (string_of_id id); print_endline " Enum"; SMap.empty
            | TD_bitfield _ -> print_endline "Bitfield"; SMap.empty
            | _ -> print_endline "TypeFunDef: other"; SMap.empty
    
    (* ——————————————————————— Iterating over definitions ——————————————————————— *)
    
    let node_defs (DEF_aux (def, annot)) : defs =
        match def with
            | DEF_val (VS_aux (val_spec, annot)) -> defs_from_funs (val_fun_def val_spec)
            | DEF_register (DEC_aux (dec_spec, annot)) -> defs_empty
            | DEF_scattered (SD_aux (scattered, annot)) -> defs_empty
            | DEF_fundef (FD_aux (fundef, annot)) -> defs_empty
            | DEF_impl funcl -> defs_empty
            | DEF_type typ -> defs_from_union (type_def_fun_def typ)
            | _ -> defs_empty
    
    let rec collect_defs(defs: ('a, 'b) def list) : defs =
        match defs with
            | h :: t -> defs_merge (node_defs h) (collect_defs t)
            | [] -> defs_empty
    
    let get_defs (ast: ('a, 'b) ast) : defs =
        collect_defs ast.defs
    
    let extract_type_alias typ = match extract_type typ with
        | RsTypUnit -> RsTypId "()"
        | e -> e
                    
    let process_sub_type (id: string) (A_aux (typ, _)) : rs_obj * bool =
        match typ with
            | A_typ typ -> (RsAlias {new_typ = id; old_type = extract_type_alias typ }, true)
            | A_bool b -> (RsConst {name = id; value = nconstraint_to_rs_exp b}, true)
            | A_nexp exp -> (RsConst {name = id; value = nexp_to_rs_exp exp}, true)
            
    let extract_first_item_type (items: (Libsail.Ast.typ * Libsail.Ast.id) list) : rs_type =   
        assert(List.length items = 1); 
        match items with 
            | x :: _ -> extract_type (fst x)
            | _ -> RsTypTodo "type_extract_first_item_type"
    
    let parse_bitfield_single (items: (Libsail.Ast.typ * Libsail.Ast.id) list) = 
        let item = extract_first_item_type items in 
        match item with 
            | RsTypGenericParam (e, [RsTypParamNum n]) -> RsTypGenericParam ("BitField", [RsTypParamNum n]) ;
            | _ -> item
    
    let process_record_type (id) (items: (Libsail.Ast.typ * Libsail.Ast.id) list) : (rs_obj * bool) =
        if List.length items <> 1 then
            let arg_value = List.map (fun (typ, id) -> (string_of_id id, extract_type typ)) items in
           (RsStruct { name = string_of_id id; fields = arg_value }, true)
        else
            (RsAlias { new_typ = string_of_id id; old_type = parse_bitfield_single items }, true)
            
    let process_type_name_type (TD_aux (typ, _)) : (rs_obj * bool) =
        match typ with
            | TD_abbrev (id, typquant, typ_arg) -> process_sub_type (string_of_id id) typ_arg
            (* Fix this in the future *)
            | TD_record (id, typquant, items, _) 
                when string_of_id id = "Mem_write_request" || 
                    string_of_id id = "Mem_read_request" ->
                        (RsConst {name = "dummy"; value = RsTodo "Tododummy"}, false)
            | TD_record (id, typquant, items, _) -> process_record_type id items
            | _ -> (RsConst {name = "dummy"; value = RsTodo "TodoDummy"}, false)
     
    let process_if_abbrev  (DEF_aux (def, annot)) : (rs_obj * bool) =
        match def with
            | DEF_type typ -> process_type_name_type typ
            | _ -> (RsConst {name = "dummy"; value = RsTodo "Tododummy"}, false)
    
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
      
    let gen_enum_list (id: Ast.id) (enum_fields: string list) : (string * string) list =
        let enum_name = string_of_id id in
        List.map (fun e -> (e, enum_name)) enum_fields
        
    let ast_id_list_to_string_list (members: Ast.id list) : string list = (List.map string_of_id members)
    
    let rec ast_union_type_list_to_string_list (members: Ast.type_union list) : string list = 
        match members with 
            | (Tu_aux (Tu_ty_id (typ, id), annot)) :: v -> (string_of_id id) :: ast_union_type_list_to_string_list v
            | [] -> []
           
    let process_enum_entries_aux (DEF_aux (def, annot)): (string * string) list =
    match def with
        | DEF_type (TD_aux (TD_enum (id, members, _), _)) -> gen_enum_list id (ast_id_list_to_string_list members)
        | DEF_type (TD_aux (TD_variant (id, _, members, _), _)) -> gen_enum_list id (ast_union_type_list_to_string_list members)
        | _ -> []
        
    let rec process_enum_entries (defs: ('a, 'b) Libsail.Ast.def list): (string * string) list =
        match defs with
            | h :: t -> (process_enum_entries_aux h) @ (process_enum_entries t)
            | [] -> []


    (* ————————————————————————————————— Utils —————————————————————————————————— *)
    
    let print_all_defs (defs: defs) =
        let print_one_fun key (args, ret) =
            Printf.printf "  %s(%s) -> %s\n"
                key
                (String.concat ", " (List.map string_of_rs_type args))
                (string_of_rs_type ret)
        in
        let print_one_union key typ =
            Printf.printf "  %s %s\n"
                key
                (string_of_rs_type typ)
        in
        print_endline "Fun defs:";
        SMap.iter print_one_fun defs.funs;
        print_endline "Union defs:";
        SMap.iter print_one_union defs.unions;
        ()
    
    (* ———————————————————————— Translation function  ————————————————————————— *)
    
    let sail_to_rust (ast: ('a, 'b) ast) (ctx: context) : rs_program =
        merge_rs_prog_list [generate_sail_abbrev ast.defs; generate_sail_virt_ctx ast.defs ctx; process_defs ast.defs ctx]
    
  let compile_ast env effect_info ast =
    try
      (* Compute call set *)
      let call_set = get_call_set ast in
      (* SSet.iter (Printf.printf "%s ") call_set; *)
      (* print_endline ""; *)

      (* Collect definitions *)
      let defs = get_defs ast in
      (* print_all_defs defs; *)

      (* Build the context *)
      let ctx = {
        defs = defs;
        call_set = call_set;
        ret_type = RsTypUnit;
      } in

      (*List.map (fun e -> print_endline(Printf.sprintf "%s %s" (fst e) (snd e))) enum_entries;
      print_endline ""; *)


      (* Build list of registers *)
      let register_list = Util.StringSet.of_list (gather_registers_list ast) in
      
      (* Process enumerations *)
      let enum_entries = process_enum_entries ast.defs in

      (* First stage : sail to raw (invalid) rust *)
      let rust_program = sail_to_rust ast ctx in
      let rust_program = Rust_transform.transform rust_program register_list enum_entries in

      let rust_program_string = string_of_rs_prog rust_program in

      (* Post processing stage: replace illegals # and ' in rust *)
      (* TODO: Rewrite in the future, as the code is a bit hugly *)
      let replace_hashtags input =
        let regex = Str.regexp "#" in
        let regex2 = Str.regexp "_hashtag_\\[" in
        let regex3 = Str.regexp "_hashtag_\\!" in
        let input = Str.global_replace regex "_hashtag_" input in
        let input = Str.global_replace regex2 "#[" input in
        let input = Str.global_replace regex3 "#!" input in  
        input 
      in 
      let replace_atom input =
        let regex = Str.regexp "atom<N>" in
        Str.global_replace regex "usize" input
      in 

      let rust_program_string = replace_hashtags rust_program_string in
      let rust_program_string = replace_atom rust_program_string in

      rust_program_string
    with Type_error.Type_error (l, err) ->
      c_error ~loc:l ("Unexpected type error when compiling to C:\n" ^ fst (Type_error.string_of_type_error err))
end
