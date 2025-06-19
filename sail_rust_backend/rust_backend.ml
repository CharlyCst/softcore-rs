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
open Core_config

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

    type field_kind =
        | FieldKindStruct of field_kind SMap.t
        | FieldKindLeaf of rs_type

    (* ——————————————————————————————— Type Utils ——————————————————————————————— *)
    
    let map_union (a: 'a SMap.t) (b: 'a SMap.t) : 'a SMap.t =
        let select key elt_a elt_b = Some(elt_a) in
        SMap.union select a b
    
    let defs_empty = {
        fun_typs = SMap.empty;
        unions = SMap.empty;
        funmap = SMap.empty;
        constants = SSet.empty;
        num_constants = SMap.empty;
    }
    
    let defs_merge (a: defs) (b: defs) : defs =
        {
            fun_typs = map_union a.fun_typs b.fun_typs;
            unions = map_union a.unions b.unions;
            funmap = map_union a.funmap b.funmap;
            constants = SSet.union a.constants b.constants;
            num_constants = map_union a.num_constants b.num_constants;
        }
    
    let defs_add_union (defs: defs) (union: unionmap) : defs =
        let unions = map_union union defs.unions in
        { defs with
            unions = unions;
        }
    
    let defs_from_union (union: unionmap) : defs = 
        { defs_empty with
            unions = union;
        }
    
    let defs_from_funs (funs: defmap) : defs =
        { defs_empty with
            fun_typs = funs;
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


    (** Note: This function is already available as part of Sail, but there was
        a bug in the implementation.
        Once the bugfix is merged and a new version is release, we should
        remove the function from here and use the upstream version.

        See https://github.com/rems-project/sail/pull/1347
     **)
    let rec big_int_of_nexp (Nexp_aux (nexp, _)) =
        match nexp with
        | Nexp_constant c -> Some c
        | Nexp_times (n1, n2) -> Util.option_binop Big_int.mul (big_int_of_nexp n1) (big_int_of_nexp n2)
        | Nexp_sum (n1, n2) -> Util.option_binop Big_int.add (big_int_of_nexp n1) (big_int_of_nexp n2)
        | Nexp_minus (n1, n2) -> Util.option_binop Big_int.sub (big_int_of_nexp n1) (big_int_of_nexp n2)
        | Nexp_exp n -> Option.map (fun n -> Big_int.pow_int_positive 2 (Big_int.to_int n)) (big_int_of_nexp n)
        | _ -> None

    (** Format a location in a human readeable format. **)
    let rec pretty_loc (l: l) =
        (* Some files have the full path in thei file location, this removes the prefix *)
        let strip_prefix s =
            let rec drop n lst =
                if n <= 0 then
                    lst
                else
                    match lst with
                    | [] -> []
                    | _ :: tail -> drop (n - 1) tail
            in
            let segments = String.split_on_char '/' s in
            (* The path has the shape: /XXX/username/.opam/default/share/sail/xxx*)
            if Option.is_some (List.find_opt (fun s -> s = ".opam") segments) then
                match List.find_index (fun x -> x = "share") segments with
                    | Some n ->
                        String.concat "/" (drop (n + 1) segments)
                    | None -> s
            else
                s
        in
        match l with
            | Parse_ast.Unknown -> None
            | Parse_ast.Unique (n, l) -> pretty_loc l
            | Parse_ast.Generated l -> pretty_loc l
            | Parse_ast.Hint (_, l1, l2) -> pretty_loc l
            | Parse_ast.Range (lx1, lx2) ->
                let (l1, l2) = (lx1.pos_lnum, lx2.pos_lnum) in
                let lines = if l1 = l2 then
                    Int.to_string l1
                else
                    (Int.to_string l1) ^ "-" ^ (Int.to_string l2)
                in
                Some ( "`" ^ (strip_prefix lx1.pos_fname) ^ "` L" ^ lines )

    let loc_to_doc (l: l) =
        match pretty_loc l with
            | Some loc -> "Generated from the Sail sources at " ^ loc ^ "."
            | None -> "Generated from the Sail sources."

    (* ———————————————————————— Sail-to-Rust Conversion ————————————————————————— *)
    
    let process_register reg : rs_program =
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
            | L_num n -> RsLitNum n
            | L_hex s -> RsLitHex s
            | L_bin s -> RsLitBin s
            | L_string s -> RsLitStr s
            | L_undef -> RsLitTodo
            | L_real s -> RsLitTodo
    
    and process_pat (P_aux (pat, annot)) : rs_pat =
        match pat with
            | P_lit lit -> RsPatLit (process_lit lit)
            | P_id id -> RsPatId (sanitize_id (string_of_id id))
            | P_typ (typ, pat) -> RsPatType ((typ_to_rust typ), (process_pat pat))
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
    
    (** Translate a Sail binary operation into a Rust Binary operation.

        Sail has a more flexible type system, which prevents overflow in some
        cases. For instance, the type of `a * b` is `int(sizeof(a) * int(sizeof(b))`.
        In Rust we need to fit all integers in the existing integer types,
        which requires conversions to bigger types when the result can
        overflow.
        This function leverages the Sail types to infer potential overflow, and
        performs conversions if appropriate.**)
    and process_binop_exp (ctx: context) (exp1: tannot exp) (binop: rs_binop) (exp2: tannot exp) : rs_exp =
        let E_aux (_, t1) = exp1 in
        let E_aux (_, t2) = exp2 in
        let t1 = typ_of_annot t1 in
        let t2 = typ_of_annot t2 in
        let size1 = size_of_num_type t1 in
        let size2 = size_of_num_type t2 in
        let exp1 = process_exp ctx exp1 in
        let exp2 = process_exp ctx exp2 in
        match binop with
            | RsBinopMult ->
                begin match (size1, size2) with
                    (* 64 bits overflow, switch to 128 bits *)
                    | (Some n, Some m) when Int64.mul n m >= 64L ->
                        RsBinop(RsAs (exp1, nat_typ), RsBinopMult, RsAs (exp2, nat_typ))
                    | _ -> RsBinop(exp1, RsBinopMult, exp2)
                end
            | _ ->
                Reporting.simple_warn (Printf.sprintf "Binop not yet implemented: %s" (string_of_rs_binop binop));
                RsTodo (Printf.sprintf "Binop%s" (string_of_rs_binop binop))

    and process_exp (ctx: context) (E_aux (exp, aux)) : rs_exp = 
        let as_kid str = kid_of_id (mk_id str) in
        let typ = typ_of_annot aux in
        let env = env_of_annot aux in
        match exp with
            | E_block exp_list -> RsBlock (List.map (process_exp ctx) exp_list)
            | E_id id ->
                let id = (sanitize_id (string_of_id id)) in
                if SSet.mem id ctx.registers then
                    let _ = ctx.uses_sail_ctx <- true in (* set flag *)
                    RsField (RsId core_ctx, id)
                else
                    RsId id
            | E_lit lit -> RsLit (process_lit lit)
            | E_typ (typ, exp) -> RsAs(process_exp ctx exp, typ_to_rust typ)
            | E_app (id, [e1; e2]) when string_of_id id = "mult_atom" ->
                process_binop_exp ctx e1 RsBinopMult e2
            | E_app (id, exp_list) when string_of_id id = "bitvector_concat" ->
                (* We need to infer the vectors dimensions. To do so we look-up the type variable bindings in the typing context. *)
                let bindings = instantiation_of (E_aux (exp, aux)) in
                let n = KBindings.find (as_kid "n") bindings in
                let m = KBindings.find (as_kid "m") bindings in
                let as_int64 (A_aux (typ, l)) = match typ with
                    | A_nexp nexp ->
                        begin match  big_int_of_nexp nexp with
                            | Some n -> Big_int.to_int64 n
                            | None ->
                                Reporting.warn "Could not infer 'bitvector_concat' type" l (Printf.sprintf "found %s" (string_of_typ_arg (A_aux (typ, l))));
                                64L
                        end
                    | _ ->
                            Reporting.warn "Could not infer 'bitvector_concat' type" l (Printf.sprintf "found %s" (string_of_typ_arg (A_aux (typ, l))));
                        64L
                in
                let nm = Int64.to_string (Int64.add (as_int64 n) (as_int64 m)) in
                let n = Int64.to_string (as_int64 n) in
                let m = Int64.to_string (as_int64 m) in
                RsApp (RsId (sanitize_id (string_of_id id)), [n; m; nm], (List.map (process_exp ctx) exp_list))
            | E_app (id, exp_list) when let sid = string_of_id id in sid = "ones" || sid = "sail_ones" ->
                (* Those functions' const generic type might not always be
                   known at compile time as it might depend the configuration.
                   When it can not be known, we need to use a conservative
                   approximation.

                   When we do an approximation, we use the SMT solver to prove
                   that the approximation is correct. In this case correct
                   means that we use more bits rather than fewer.*)
                let id = sanitize_id (string_of_id id) in
                let exp_list = List.map (process_exp ctx) exp_list in
                let bindings = instantiation_of (E_aux (exp, aux)) in
                begin match KBindings.find_opt (as_kid "n") bindings with
                    (* The type variable depends on another variable.
                       For now we leave that case to the Rust type inference,
                       and simply omit the generic. *)
                    | None -> RsApp (RsId id, [], exp_list)
                    (* We found the type variable *)
                    | Some (A_aux (A_nexp n, _)) ->
                        begin match big_int_of_nexp n with
                            | Some n -> (* The constant can be determined at compile time *)
                                RsApp (RsId id, [Big_int.to_string n], exp_list)
                            | None -> (* If the constant is unknown, we need to make a conservative approximation *)
                                (* First, we find the type variables and contraints *)
                                let (kind_ids, constraints) = match typ with
                                    | Typ_aux (Typ_exist (kinded_ids, constriants, ret_typ), _) -> (kinded_ids, constriants)
                                    | Typ_aux (_, l) -> Reporting.unreachable l __POS__ "Unexpected type"
                                in
                                (* Then we add them to the current environment *)
                                let env = add_existential (fst aux) kind_ids constraints env in
                                (* And finally we use the SMT solver to prove that our approximation is conservative *)
                                if prove __POS__ env (nc_lteq n (nconstant (Big_int.of_int 64))) then
                                    RsApp (RsId id, ["64"], exp_list)
                                else
                                    Reporting.unreachable (fst aux) __POS__ "Could not prove that the bit width is less or equal to 64"
                        end
                    (* We found the type variable, but it is not a nexp! *)
                    | Some (A_aux (_, l)) -> Reporting.unreachable l __POS__ "Expected a nexp"
                end
            | E_app (id, exp_list) -> RsApp (RsId (sanitize_id (string_of_id id)), [], (List.map (process_exp ctx) exp_list))
            | E_app_infix (exp1, id, exp2) -> RsTodo "E_app_infix"
            | E_tuple (exp_list) -> RsTuple (List.map (process_exp ctx) exp_list)
            | E_if (exp1, exp2, exp3) -> RsIf ((process_exp ctx exp1), (process_exp ctx exp2), (process_exp ctx exp3)) 
            | E_loop (loop, measure, exp1, exp2) -> RsTodo "E_loop"
            | E_for (id, E_aux (E_lit lit1, _), E_aux (E_lit lit2, _) , exp3, order, exp4) ->  assert(string_of_exp exp3 ="1"); assert(parse_order order = "inc");
               RsFor (RsTypId(string_of_id id), process_lit lit1, process_lit lit2, process_exp ctx exp4 )(* TODO: Implement a more general for loop*)
            | E_for (_,_,_,_,_,_) -> RsTodo "E_for"
            | E_vector (exp_list) -> 
                process_vector ctx exp_list typ
            | E_vector_access (exp1, exp2) -> RsTodo "E_vector_access"
            | E_vector_subrange (exp1, exp2, exp3) -> RsTodo "E_vector_subrange"
            | E_vector_update (exp1, exp2, exp3) -> RsTodo "E_vector_update"
            | E_vector_update_subrange (exp1 ,exp2, exp3, exp4) -> RsTodo "E_update_subrange"
            | E_vector_append (exp1 ,exp2) -> RsTodo "E_vector_append"
            | E_list (exp_list) -> RsTodo "E_list"
            | E_cons (exp1, exp2) -> RsTodo "E_cons"
            | E_struct fexp_list ->
                let typ = typ_to_rust typ in
                RsStruct (strip_generic_parameters typ, process_fexp_entries ctx fexp_list)
            | E_struct_update (exp, fexp_list) -> begin match fexp_list with
                (* The struct update is expexted to return the new struct with the field updated *)
                | [FE_aux (FE_fexp (field, fexp), _)] ->
                    let struct_typ = match typ with
                        | Typ_aux (Typ_id id, l) -> RsTypId (string_of_id id)
                        | Typ_aux (_, l) ->
                            Reporting.warn "Could not infer struct type in field update" l "TODO: improve type inference";
                            RsTypId "TodoStructAssingType"
                    in
                    RsStruct (struct_typ, [(string_of_id field, process_exp ctx fexp)]);
                | _ -> Reporting.unreachable (fst aux) __POS__ "TODO: handle multiple field update"
                end
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
            | E_exit exp -> RsApp(RsId "panic!", [], [RsLit(RsLitStr "exit")]) (* How should we handle exits? *)
            | E_ref id -> RsTodo "E_ref"
            | E_throw exp -> RsApp(RsId "panic!", [], [RsLit(RsLitStr "todo_process_panic_type")])
            | E_try (exp, pexp_list) -> RsTodo "E_try"
            | E_assert (exp1, E_aux (E_lit (L_aux (L_string err_msg, _)), _)) -> RsApp(RsId "assert!", [], [process_exp ctx exp1; RsLit (RsLitStr err_msg)])
            | E_assert (exp1, exp2) -> RsApp(RsId "assert!", [], [process_exp ctx exp1; RsLit (RsLitStr "[Compiler TODO] process non-trivial error messages")])
            | E_var (lexp, exp1, exp2) -> RsTodo "E_var"
            | E_internal_plet (pat, exp1, exp2) -> RsTodo "E_internal_plet"
            | E_internal_return exp -> RsTodo "E_internal_return"
            | E_internal_value value -> RsTodo "E_internal_value"
            | E_internal_assume (n_constraint, exp) -> RsTodo "E_internal_assume"
            | E_constraint n_constraint -> RsTodo "E_constraint"
            | E_config cfgs -> begin match config_find rv64_config cfgs with
                (* known values are inlined directly *)
                | Some value -> mk_num value
                (* All other values are retrieved from the core context *)
                | None ->
                    let rec construct_fields (expr: rs_exp) (fields: string list) : rs_exp = match fields with
                        | head :: tail -> construct_fields (RsField (expr, head)) tail
                        | [] -> expr
                    in
                    ctx.uses_sail_ctx <- true; (* set flag *)
                    construct_fields (RsField (RsId core_ctx, "config")) cfgs
                end
    and process_lexp (ctx: context) (LE_aux (lexp, annot)) : rs_lexp =
        match lexp with
            | LE_id id ->
                let id = (sanitize_id (string_of_id id)) in
                if SSet.mem id ctx.registers then
                    let _ = ctx.uses_sail_ctx <- true in (* set flag *)
                    RsLexpField (RsId core_ctx, id)
                else
                    RsLexpId id
            | LE_vector (lexp, idx) ->
                (RsLexpIndex (
                    (process_lexp ctx lexp),
                    (RsAs (process_exp ctx idx, usize_typ))))
            | LE_vector_range (lexp, range_start, range_end) ->
                (* RsLexpTodo *) (* TODO: properly access subranges *)
                (RsLexpIndexRange (
                    (process_lexp ctx lexp),
                    (process_exp ctx range_start),
                    (process_exp ctx range_end)))
            | LE_field (lexp, id) ->
                (RsLexpField (
                    (process_exp ctx (Ast_util.lexp_to_exp lexp)),
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
    

    and process_vector (ctx: context) (items: 'a exp list) (typ: typ) : rs_exp =
        let is_only_bits acc exp = match exp with
            | E_aux (E_lit(L_aux(lit, _)), _) -> (match lit with
                | L_zero -> acc
                | L_one -> acc
                | _ -> false)
            | _ -> false
        in
        let is_literal = (List.fold_left is_only_bits true items) in
        let string_of_bit (E_aux (exp, _)) = match exp with
            | E_lit (L_aux (lit, _)) -> (match lit with
                | L_zero -> "0"
                | L_one -> "1"
                | _ -> "x")
            | _ -> "X"
        in
        let is_bitvector = match typ with
            | Typ_aux (Typ_app (id, args), _) when string_of_id id = "bitvector"-> true
            | _ -> false
        in
        if is_literal then
            (* Generate a bitvector literal *)
            let vector_length = List.length items in
            RsStaticApp(RsTypGenericParam ("BitVector::", [RsTypParamNum (mk_num vector_length)]), "new", [RsLit(RsLitBin (Printf.sprintf "0b%s" (String.concat "" (List.map string_of_bit items))))])
        else if is_bitvector then
            (* Generate a bitvector from individual bits *)
            let rec set_bits bits idx exp = match bits with
                | head :: tail -> 
                        let new_exp = mk_method_app exp "set_bit" [mk_num idx; process_exp ctx head] in
                        set_bits tail (idx + 1) new_exp
                | [] -> exp
            in
            let new_vec = RsStaticApp (RsTypId "BitVector", "new", [mk_num 0]) in
            set_bits items 0 new_vec
        else 
            (* Generate other kinds of vectors *)
            RsArray (List.map (process_exp ctx) items)
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
            | _ -> Reporting.simple_warn "`extaract_pat_name`: pattern not implemented"; "TodoPat"

    (** Return the size occupied by a numeral type in bits. **)
    and size_of_num_type (Typ_aux (typ, l): typ) : int64 option =
        let int_log2 (n : Int64.t) : int64 =
          if n <= 0L then
            invalid_arg "int_log2: Input must be positive."
          else
            let rec find_log current_val current_log_acc =
              if current_val = 1L then
                current_log_acc
              else
                find_log (Int64.shift_right_logical current_val 1) (Int64.add current_log_acc 1L)
            in
            find_log n 0L
        in
        match typ with
            (* We expect all num types to be represented as ranges *)
            | Typ_app (id, [start; A_aux (A_nexp (Nexp_aux (nexp, l)), _)]) when string_of_id id = "range" ->
                begin match nexp with
                    (* We expect a range of the form 2 ^ X - 1 *)
                    | Nexp_minus (Nexp_aux (Nexp_exp exponent, _), Nexp_aux (Nexp_constant n, _)) ->
                        Option.map Big_int.to_int64 (big_int_of_nexp exponent)
                    | _ ->
                        Reporting.warn
                            "Unexpected range for numeral type"
                            l
                            (Printf.sprintf "Expected a range of shape `2 ^ X - 1`, found: %s" (string_of_typ (Typ_aux (typ, l))));
                        None
                end;
            (* Note: 'atom' corresponds to 'int' type. *)
            | Typ_app (id, [A_aux ((A_nexp nexp), _)]) when string_of_id id = "atom" ->
                Option.map int_log2 (Option.map Big_int.to_int64 (big_int_of_nexp nexp))
            (* Unexpected numeral type, return a default value *)
            | _ ->
                Reporting.warn "Unknown numeral type" l (Printf.sprintf "Expected a range or int type, found: %s" (string_of_typ (Typ_aux (typ, l))));
                None

    and process_args_pat (P_aux (pat_aux, annot)) : rs_pat list = 
        match pat_aux with
            | P_app (id, [P_aux (P_tuple pats, _)]) -> List.flatten (List.map process_args_pat pats)
            | P_app (id, pats) ->
                [RsPatApp (RsPatId (sanitize_id (string_of_id id)), List.flatten (List.map process_args_pat pats))]
            | P_struct (id_pat_list, field_pat_wildcard) -> [RsPatId "TodoArgsStruct"]
            | P_list pats -> [RsPatId "TodoArgsList"]
            | P_var (var, typ) -> [RsPatId "TodoArgsVar"]
            | P_cons (h, t) -> [RsPatId "TodoArgsCons"]
            | P_tuple pats -> List.flatten (List.map process_args_pat pats)
            | P_id id -> [RsPatId (string_of_id id)]
            | P_typ (_, pat) -> process_args_pat pat
            | P_lit (L_aux (L_unit, _)) -> [RsPatLit RsLitUnit]
            | P_lit _ -> [RsPatTodo "TodoPatLit"]
            | P_wild -> [RsPatWildcard]
            | P_or (_, _) -> [RsPatId "TodoOr"]
            | P_not _ -> [RsPatId "TodoNot"]
            | P_as (_, _) -> [RsPatId "TodoAs"]
            | P_vector vec -> [RsPatLit (process_vector_pat vec)]
            | P_vector_concat _ -> [RsPatId "TodoVectorConcat"]
            | P_vector_subrange (_, _, _) -> [RsPatId "TodoVectorSubrange"]
            | P_string_append _  -> [RsPatId "TodoStringAppend"]
    
   
    and build_function (kind: function_kind) (name: string) (pat: 'a pat) (exp: 'a exp) (ctx: context) (l: l) (doc_comment: string option) : rs_fn =
        (* This function balances the lenghts of the argument and argument type list by adding more arguments if neccesary *)
        let counter = ref 0 in
        let fresh_arg () =
            let arg_id = Printf.sprintf "missing_arg_%d" !counter in
            counter := !counter + 1;
            RsPatId arg_id
        in
        let rec add_missing_args args args_type new_args : rs_pat list =
            match (args, args_type) with
                | (ha::ta, ht::tt) -> add_missing_args ta tt (new_args @ [ha])
                | ([], ht::tt) -> add_missing_args [] tt (new_args @ [fresh_arg ()])
                | (_, []) -> new_args
        in
    
        let arg_names = process_args_pat pat in
        let signature = match kind with
            | FunKindFunc -> (match ctx_fun_type name ctx with
                | Some signature -> signature
                | None -> mk_fn_typ [RsTypId "TodoNoSignature"] RsTypUnit)
            | FunKindUnion (func, union) ->
                (* We look up the function definition to get the return type *)
                let ret_type = match ctx_fun_type func ctx with
                    | Some func_t -> func_t.ret
                    | None -> RsTypUnit
                in
                
                match ctx_union_type union ctx with
                | Some typ -> (match typ with
                    | RsTypTuple types -> mk_fn_typ types ret_type
                    | RsTypId id -> mk_fn_typ [RsTypId id] ret_type
                    | RsTypUnit -> mk_fn_typ [] ret_type
                    | _ -> mk_fn_typ [RsTypId "todo_signature"] (RsTypId "todo_signature"))
                | None -> mk_fn_typ [RsTypId "TodoNoUnionSignature"] RsTypUnit
        in
        let arg_names = add_missing_args arg_names signature.args [] in
        ctx.uses_sail_ctx <- false;
        assert (List.length arg_names = List.length signature.args);
        let rs_exp = process_exp ctx exp in
        {
            name = name;
            args = arg_names;
            signature = signature;
            body = rs_exp;
            use_sail_ctx = ctx.uses_sail_ctx;
            const = false;
            doc = 
                [ name ] @
                begin match doc_comment with | Some c -> [""; c] | None -> [] end @
                [""; loc_to_doc l]
        }
    
    and process_func (FCL_aux (func, annot)) (ctx: context) : rs_program =
        let l = (fst annot).loc in
        let doc_comment = (fst annot).doc_comment in
        let (id, pexp) = match func with | FCL_funcl (id, pexp) -> (id, pexp) in
        let (pexp, annot) = match pexp with | Pat_aux (pexp, annot) -> (pexp, annot) in
        let name = (string_of_id id) in
        if ctx_fun_is_used name ctx then match pexp with
            | Pat_exp (pat, exp) -> RsProg [RsFn(build_function FunKindFunc name pat exp ctx l doc_comment)]
            | Pat_when (pat1, exp, pat2) -> RsProg []
        else match pexp with
            | Pat_exp (pat, exp) ->
                    let pat_name = pat_app_name pat in
                    let fun_name = Printf.sprintf "%s_%s" name pat_name in
                    if ctx_fun_is_used pat_name ctx then
                        RsProg [RsFn(build_function (FunKindUnion (name, pat_name)) fun_name pat exp ctx l doc_comment)]
                    else RsProg []
            | _ -> RsProg []
    
    and process_funcl (funcl: 'a funcl list) (s: context) : rs_program =
        match funcl with
            | h :: t -> merge_rs_prog (process_func h s) (process_funcl t s)
            | [] -> RsProg []
    
    and process_fundef (FD_function (rec_opt, tannot_opt, funcl)) (s: context) : rs_program =
        process_funcl funcl s
    
    and enum_to_rust (id: Ast.id) (members: Ast.id list) (l: l) : rs_enum =
            let enum_name = string_of_id id in
            let enum_fields = List.map string_of_id members in 
            {
                name = enum_name;
                generics = [];
                fields =  List.map (fun id -> (id, None)) enum_fields; 
                derive = default_copy_derive;
                doc = [
                    enum_name;
                    "";
                    loc_to_doc l;
                ]
            }

    and process_unions (members: Ast.type_union list) : (string * (rs_type option)) list = 
        match members with 
            | (Tu_aux (Tu_ty_id (typ, id), annot)) :: v -> (string_of_id id, Some (typ_to_rust typ)) :: process_unions v
            | [] -> [] 

    and typequant_to_generics (TypQ_aux (_, l) as typq: typquant) : rs_generic list =
        let kset = ref KidSet.empty in
        let tyvars_of_quant_item (QI_aux (qi, _)) : (kind * kid) option =
            match qi with
            | QI_id (KOpt_aux (KOpt_kind (kind, kid), _)) -> Some (kind, kid)
            | QI_constraint _ -> None
        in
        let into_generic (K_aux (kind, _)) id =
            let id = sanitize_generic_id id in
            match kind with 
                | K_type -> RsGenTyp (id)
                | K_int -> RsGenConst (id, "i128")
                | K_bool -> RsGenConst (id, "bool")
        in
        let rec add_generic rest = match rest with
            | head :: tail -> (match tyvars_of_quant_item head with
                | Some (kind, kid) ->
                    if not (KidSet.mem kid !kset) then
                        (kset := KidSet.add kid !kset;
                        [into_generic kind (string_of_kid kid)] @ add_generic tail)
                    else
                        add_generic tail
                | None -> add_generic tail)
            | [] -> []
        in
        add_generic (quant_items typq)

    and variant_to_rust (id: id) (typq: typquant) (members: type_union list) (l: l) : rs_enum =
        let id = string_of_id id in
        {
            name = id;
            generics = typequant_to_generics typq;
            fields = process_unions members;
            derive = default_copy_derive;
            doc = [
                id;
                "";
                loc_to_doc l;
            ]
        }

    and record_to_rust (id: id) (typeq: typquant) (fields: (typ * id) list) (l: l): rs_obj =
        let to_rs_fields ((typ, id) : (typ  * id)) =
            (string_of_id id, typ_to_rust typ)
        in
        let id = string_of_id id in
        RsStruct ({
            name = id;
            generics = typequant_to_generics typeq;
            fields = List.map to_rs_fields fields;
            derive = default_copy_derive;
            doc = [
                id;
                "";
                loc_to_doc l;
            ];
        })
                 
    and typdef_to_rust (s: context) (TD_aux (typ, (l, _))) : rs_program = 
            match typ with
            | TD_enum (id, members, _) -> RsProg [RsEnum(enum_to_rust id members l)] 
            | TD_variant (id, typq, members, _) when string_of_id id = "option" -> RsProg [] (* Special semantics in rust *)
            | TD_variant (id, typq, members, _) -> RsProg [RsEnum (variant_to_rust id typq members l)]
            | TD_abstract _ -> Reporting.unreachable l __POS__ "Abstract type not supported in Rust backend"
            | TD_record (id, typq, fields, _) -> RsProg [record_to_rust id typq fields l]
            | TD_abbrev (id, typq, A_aux (A_typ typ, _)) ->
                let alias = {
                    new_typ = string_of_id id;
                    generics = typequant_to_generics typq;
                    old_type = typ_to_rust typ;
                } in
                RsProg [RsAlias (alias)] (* TODO *)
            (* NOTE: we should create a constant for numeral types only if there is no constant with the same name already defined. *)
            | TD_abbrev (id, typq, A_aux (A_nexp nexp, _)) when not (SSet.mem (string_of_id id) s.defs.constants) ->
                let value = match big_int_of_nexp nexp with
                    | Some n -> mk_big_num n
                    | None -> nexp_to_rs_exp nexp
                in
                let const = {
                    name = string_of_id id;
                    value = value;
                    typ = int_typ;
                } in
                RsProg [RsConst const]
            | TD_abbrev _ -> RsProg [] (* Ignore all other abbreviations *)
            | _ -> RsProg []

    and toplevel_let_to_rust (LB_aux (LB_val (pat, exp), aux)) (ctx: context) : rs_program =
        let pat = process_pat pat in
        let rexp = process_exp ctx exp in
        let rexp = Rust_transform.simplify_rs_exp ctx rexp in
        match pat with
            | RsPatId id ->
                let const = { name = id; value = rexp; typ = int_typ } in
                RsProg [RsConst const]
            | RsPatType (typ, RsPatId id) ->
                let const = { name = id; value = rexp; typ = typ } in
                RsProg [RsConst const]
            | _ -> RsProg []
        
    and def_to_rust (DEF_aux (def, annot)) (s: context) : rs_program =
        match def with
            | DEF_register (DEC_aux (dec_spec, annot)) -> process_register dec_spec
            | DEF_scattered (SD_aux (scattered, annot)) -> process_scattered scattered
            | DEF_fundef (FD_aux (fundef, annot)) -> process_fundef fundef s
            | DEF_impl funcl -> process_func funcl s
            | DEF_type typ -> typdef_to_rust s typ
            | DEF_let binding -> toplevel_let_to_rust binding s
            | _ -> RsProg []
    
    and defs_to_rust defs (ctx: context): rs_program =
        match defs with
            | h :: t -> merge_rs_prog (def_to_rust h ctx) (defs_to_rust t ctx)
            | [] -> RsProg []
    
    
    (* ———————————————————————— Sail Virtual Context Generator ————————————————————————— *)
    
    and process_reg_name_type reg : (string * rs_type) =
        let (typ, id, exp) = match reg with 
            | DEC_reg (typ, id, exp) -> (typ, id, exp) in (string_of_id id, typ_to_rust typ)
    
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

    (** We decompose the configuration into multiple Rust struct to stay close
        to the JSon-like structure.

        This functionc reates all the necessary structs and keep the fields
        names compatible with the `E_config` Sail expression.
    **)
    and build_config_structs (config_map: config_map) : rs_obj list =
        let rec build_struct (fields: string list) (typ: rs_type) (s: field_kind SMap.t) : field_kind SMap.t =
            match fields with
                | [] -> s
                | head :: [] -> SMap.add head (FieldKindLeaf typ) s
                | head :: tail ->
                    let s' = match SMap.find_opt head s with
                        | Some (FieldKindStruct s') -> s'
                        | Some (FieldKindLeaf _) ->
                            Reporting.simple_warn
                                (Printf.sprintf "Invalid config: %s used as both a struct and a leaf" head);
                            SMap.empty (* Go on, but there is a bug somewhere! *)
                        | None -> SMap.empty
                    in
                    let child_struct = build_struct tail typ s' in
                    SMap.add head (FieldKindStruct child_struct) s
        in
        let cfg_struct =
            List.fold_left
                (fun s  (fields, typ) -> build_struct (String.split_on_char '.' fields) (typ_to_rust typ) s)
                SMap.empty
                (SMap.to_list config_map)
        in
        let rec cfg_struct_to_rust (s: field_kind SMap.t) (name_suffix: string) : rs_obj list =
            let fold_acc ((fields, structs): (string * rs_type) list * rs_obj list) ((field_name, field_kind): string * field_kind) : (string * rs_type) list * rs_obj list =
                match field_kind with
                    | FieldKindLeaf typ -> (fields @ [(field_name, typ)], structs)
                    | FieldKindStruct s ->
                        let struct_typ = RsTypId ("Config" ^ (String.capitalize_ascii field_name)) in
                        (fields @ [(field_name, struct_typ)], structs @ (cfg_struct_to_rust s field_name))
            in
            let (fields, structs) = List.fold_left fold_acc ([], []) (SMap.to_list s) in
            let struct_name = "Config" ^ (String.capitalize_ascii name_suffix) in
            [mk_struct struct_name fields] @ structs
        in
        cfg_struct_to_rust cfg_struct ""

    and generate_core_ctx defs (ctx: context): rs_program =
        let config_structs = build_config_structs ctx.config_map in
        let config_field = if List.length config_structs = 0 then
            []
        else
            ["config", RsTypId "Config"]
        in
        RsProg ([
            RsStruct({
                name = "Core";
                generics = [];
                fields = (gather_registers defs) @ config_field;
                derive = ["Eq"; "PartialEq"; "Clone"; "Debug"];
                doc = [
                    "The software core.";
                    "";
                    "This struct represents a software core, and holds all the registers as well as the core configuration.";
                    "The core is the main abstraction exposed by the softcore library and represents a single execution thread.";
                    "";
                    "The raw functions translated directly from the specification are available in the `raw` module, whereas higher-level wrappers are implemented as methods on the [Core] struct directly.";
                ]
            })
            ] @ config_structs
        )
    
    and gather_registers_list (ast: ('a, 'b) ast) : string list =
        List.map (fun (x, _) -> x) (gather_registers ast.defs)
    
    
    (* ———————————————————————— Sail Types Generator ————————————————————————— *)
    
    and nconstraint_to_rs_exp (NC_aux (n, _)): rs_exp =
        match n with
            | NC_true -> RsLit (RsLitTrue)
            | NC_false -> RsLit (RsLitFalse)
            | _ -> RsTodo "TodoNConstraint"
    
    and nexp_to_rs_exp (nexp: nexp): rs_exp =
        let Nexp_aux (nexp, l) = nexp_simp nexp in
        match nexp with
           | Nexp_constant n -> mk_big_num n
           | Nexp_times (n, m) -> RsBinop (nexp_to_rs_exp n, RsBinopMult, nexp_to_rs_exp m)
           | Nexp_sum   (n, m) -> RsBinop (nexp_to_rs_exp n, RsBinopAdd, nexp_to_rs_exp m)
           | Nexp_minus (n, m) -> RsBinop (nexp_to_rs_exp n, RsBinopSub, nexp_to_rs_exp m)
           | Nexp_exp n -> (* exponential, it seems it is always 2 ^ n *)
               let n_exp = match nexp_to_rs_exp n with
                   | RsLit n -> RsLit n (* Types is inferred automatically for literals *)
                   | n_exp -> RsAs (n_exp, RsTypId "u32") (* For all other types we do the conversion manually *)
               in
               RsStaticApp (int_typ, "pow", [mk_num 2; RsAs (n_exp, RsTypId "u32")])
           | Nexp_neg n -> RsUnop (RsUnopNeg, nexp_to_rs_exp n)
           | Nexp_id id -> RsId (string_of_id id)
           | Nexp_var kid  -> RsId (sanitize_generic_id (string_of_kid kid)) (* variable *)
           | Nexp_app (fn, args) -> RsTodo "TodoAppExpr" (* app *)
           | Nexp_if (cond, if_block, else_block) -> RsTodo "TodoIfExpr" (* if-then-else *)
     
    and get_first_two_elements lst =
        assert ((List.length lst) = 2);
        match lst with
        | first :: second :: _ -> (first, second)
        | _ -> failwith "List does not have enough elements"
      
    and typ_to_rust (Typ_aux (typ, _)): rs_type =
        match typ with
            | Typ_id id when string_of_id id = "unit" -> RsTypUnit
            | Typ_id id -> RsTypId (sanitize_generic_id (string_of_id id))
            | Typ_var (Kid_aux (Var x, _)) -> RsTypGeneric (sanitize_generic_id x)
            | Typ_tuple types -> RsTypTuple (List.map typ_to_rust types)
            | Typ_fn _ -> RsTypId "TodoFnType"
            | Typ_app (id, params) -> if (string_of_id id) = "vector" then 
                    let (size, typ) = get_first_two_elements (List.map extract_type_arg params) in
                    let size = match size with
                        | RsTypParamNum n -> RsTypParamNum (RsAs (n, usize_typ))
                        | _ -> size
                    in
                    RsTypArray (typ, size)
                else 
                    RsTypGenericParam ((string_of_id id), (List.map extract_type_arg params))
            | Typ_internal_unknown -> RsTypId "TodoUnknownType"
            | Typ_bidir (_, _) -> RsTypId "TodoBidirType"
            | Typ_exist (_, _, typ) -> typ_to_rust typ
    and extract_type_arg (A_aux (typ, _)): rs_type_param =
        match typ with
            | A_nexp exp -> extract_type_nexp exp
            | A_typ typ -> RsTypParamTyp (typ_to_rust typ)
            | A_bool b -> RsTypParamTyp (RsTypId "TodoBoolType")
    and extract_type_nexp (Nexp_aux (nexp, l)): rs_type_param =
        match nexp with
            | Nexp_constant n -> RsTypParamNum (mk_big_num n)
            | Nexp_app (Id_aux (_, _), _) -> RsTypParamTyp (RsTypId "TodoNexpTypeApp")
            | Nexp_id id -> RsTypParamNum (RsId (sanitize_generic_id (string_of_id id)))
            | Nexp_var var -> RsTypParamTyp (RsTypId (capitalize_after_removal (string_of_kid var))) 
            | Nexp_times (_, _)
            | Nexp_sum (_, _)
            | Nexp_minus (_, _)
            | Nexp_exp _
            | Nexp_neg _ -> RsTypParamNum ( RsBlock [nexp_to_rs_exp (Nexp_aux (nexp, l))])
            | _ -> RsTypParamTyp (RsTypId "TodoNexpOther")
    
    (* ———————————————————————————— Value Definition ———————————————————————————— *)
    
    let find_linked_gen_args (generics: rs_generic list) (args: rs_type list) (ret: rs_type) : (int * int) list =
        (* For now we focus on simple function with one generic and one
           argument that return a bitvector whose size is the argument. *)
        match (generics, args, ret) with
            | ([RsGenConst (x, _)],
               [RsTypGenericParam (arg_t, [RsTypParamTyp (RsTypId y)])],
               RsTypGenericParam ("bitvector", [RsTypParamTyp (RsTypId z)]))
               when x = y && y = z && (arg_t = "atom" || arg_t = "implicit") ->
                [(0, 0)]
            | _ -> []

    let extract_types (TypSchm_aux (typeschm, _)) : rs_fn_type =
        (* We ignore the type quantifier for now, there is no `forall` on most types of interest *)
        let TypSchm_ts (typq, typ) = typeschm in
        let generics = typequant_to_generics typq in
        let Typ_aux (typ, l) = typ in
        match typ with 
            (* When Sail infers type, it sometimes uses a single tuple as argument.
               In such cases, we flatten the tuple. *)
            | Typ_fn ([Typ_aux (Typ_tuple args, _)], ret) 
            | Typ_fn (args, ret) ->
                let args = List.map typ_to_rust args in
                let ret = typ_to_rust ret in
                let fn = mk_fn_typ_gen args ret generics in
                { fn with linked_gen_args = find_linked_gen_args generics args ret }
            | _ -> mk_fn_typ [RsTypTodo "todo_extract_types"] (RsTypTodo "todo_extract_types")
    
    let val_fun_def (val_spec: val_spec_aux) : defmap =
        let map = SMap.empty in
        let VS_val_spec (typeschm, id, extern) = val_spec in
        (* print_string (string_of_id id); *)
        (* print_string " - "; *)
        (* print_endline (string_of_typschm typeschm); *)
        (* print_string (string_of_id id); *)
        (* print_string " - "; *)
        (* print_endline (String.concat ", " (List.map string_of_rs_type (fst (extract_types typeschm)))); *)
        SMap.add (string_of_id id) (extract_types typeschm) map
    
    (* ————————————————————————————————— Union —————————————————————————————————— *)
    
    let type_union_def (Tu_aux (union, _)) : unionmap = 
        let Tu_ty_id (typ, id) = union in
        SMap.add (string_of_id id) (typ_to_rust typ) SMap.empty 
    
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
            | DEF_let (LB_aux (LB_val (pat, _), _)) ->
                let pat = process_pat pat in
                begin match pat with
                    | RsPatId id | RsPatType (_, RsPatId id) -> { defs_empty with constants = SSet.of_list [id] }
                    | _ -> defs_empty
                end
            | _ -> defs_empty
    
    let rec collect_defs(defs: ('a, 'b) def list) : defs =
        match defs with
            | h :: t -> defs_merge (node_defs h) (collect_defs t)
            | [] -> defs_empty
    
    let get_defs (ast: ('a, 'b) ast) : defs =
        collect_defs ast.defs

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

    (* ———————————————————————— Translation function  ————————————————————————— *)
    
    let sail_to_rust (ast: ('a, 'b) ast) (ctx: context) : rs_program =
        merge_rs_prog_list [generate_core_ctx ast.defs ctx; defs_to_rust ast.defs ctx]

    let get_funs (RsProg obj : rs_program) : (string * rs_fn) list =
        let rec funs obj =
            match obj with
                | RsFn fn :: tail -> (fn.name, fn) :: funs tail
                | head :: tail -> funs tail
                | [] -> []
        in
        funs obj
    
  let compile_ast env effect_info ast =
    try
      (* Compute call set *)
      let sail_ctx = get_call_set ast in
      SSet.iter (Printf.printf "%s ") sail_ctx.call_set;
      print_endline "";
      SMap.iter (fun cfg typ ->Printf.printf "%s : %s\n" cfg (string_of_typ typ)) sail_ctx.config_map;

      (* Collect definitions *)
      let defs = get_defs ast in

      (* Build the context *)
      let ctx = {
        defs = defs;
        call_set = sail_ctx.call_set;
        config_map = sail_ctx.config_map;
        registers = Util.StringSet.of_list (gather_registers_list ast);
        enum_entries = process_enum_entries ast.defs;
        uses_sail_ctx = false;
      } in

      
      (* First stage : sail to raw (invalid) rust *)
      let rust_program = sail_to_rust ast ctx in

      (* Update context with all function definitions *)
      let funs = get_funs rust_program in
      let defs = defs_merge ctx.defs { defs_empty with funmap = SMap.of_list funs} in
      let ctx = { ctx with defs = defs } in
      
      let rust_program = Rust_transform.transform rust_program ctx in

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
          let regex = Str.regexp "atom<[A-Za-z0-9]*>" in
        Str.global_replace regex "i128" input
      in 

      let rust_program_string = replace_hashtags rust_program_string in
      let rust_program_string = replace_atom rust_program_string in

      rust_program_string
    with Type_error.Type_error (l, err) ->
      c_error ~loc:l ("Unexpected type error when compiling to C:\n" ^ fst (Type_error.string_of_type_error err))
end
