open Libsail

open Ast
open Ast_util
open Ast_defs
open Rust_gen
open Call_set

let opt_virt_preserve = ref ([]:string list)

let virt_options = [
    ( "-virt_preserve",
      Arg.String (fun id -> opt_virt_preserve := id :: !opt_virt_preserve),
      "<id> do not remove the provided id when generating IR");
  ]

(* Sail comes with some built-in passes, here we select the ones we want to apply*)
let virt_rewrites =
  let open Rewrites in
  [
    ("instantiate_outcomes", [String_arg "virt"]);
    ("realize_mappings", []);
    ("toplevel_string_append", []);
    ("pat_string_append", []);
    ("mapping_patterns", []);
    ("truncate_hex_literals", []);
    ("mono_rewrites", [If_flag opt_mono_rewrites]);
    ("recheck_defs", [If_flag opt_mono_rewrites]);
    ("toplevel_nexps", [If_mono_arg]);
    ("monomorphise", [String_arg "c"; If_mono_arg]);
    ("atoms_to_singletons", [String_arg "c"; If_mono_arg]);
    ("recheck_defs", [If_mono_arg]);
    ("undefined", [Bool_arg false]);
    ("vector_string_pats_to_bit_list", []);
    ("remove_not_pats", []);
    ("remove_vector_concat", []);
    ("remove_bitvector_pats", []);
    ("pattern_literals", [Literal_arg "all"]);
    ("tuple_assignments", []);
    ("vector_concat_assignments", []);
    ("simple_struct_assignments", []);
    ("exp_lift_assign", []);
    (* ("merge_function_clauses", []); *)
    ("recheck_defs", []);
    ("constant_fold", [String_arg "c"])
  ]

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

let process_type (Typ_aux (typ, annot)) : rs_type =
    match typ with
        | Typ_id id -> RsTypId (string_of_id id)
        | _ -> RsTypTodo

let rec process_pat (P_aux (pat, annot)) : rs_pat =
    match pat with
        | P_lit lit -> RsPatLit
        | P_id id -> RsPatId (string_of_id id)
        | P_typ (typ, pat) -> RsPatType ((process_type typ), (process_pat pat))
        | P_wild -> RsPatWildcard
        | _ -> RsPatTodo

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

let rec process_exp exp : rs_exp = 
    (* print_string "Exp "; *)
    (* print_endline (string_of_exp exp); *)
    let exp = match exp with | E_aux (exp, aux) -> exp in
    match exp with
        | E_block exp_list -> RsBlock (List.map process_exp exp_list)
        | E_id id -> RsId (string_of_id id)
        | E_lit lit -> RsLit (process_lit lit)
        | E_typ (typ, exp) -> RsTodo (*print_string (string_of_typ typ);*)
        | E_app (id, exp_list) -> RsApp ((string_of_id id), (List.map process_exp exp_list))
        | E_app_infix (exp1, id, exp2) -> RsTodo
        | E_tuple (exp_list) -> RsTodo
        | E_if (exp1, exp2, exp3) -> RsIf ((process_exp exp1), (process_exp exp2), (process_exp exp3)) 
        | E_loop (loop, measure, exp1, exp2) -> RsTodo
        | E_for (id, exp1, exp2, exp3, order, exp4) -> RsTodo
        | E_vector (exp_list) -> RsTodo
        | E_vector_access (exp1, exp2) -> RsTodo
        | E_vector_subrange (exp1, exp2, exp3) -> RsTodo
        | E_vector_update (exp1, exp2, exp3) -> RsTodo
        | E_vector_update_subrange (exp1 ,exp2, exp3, exp4) -> RsTodo
        | E_vector_append (exp1 ,exp2) -> RsTodo
        | E_list (exp_list) -> RsTodo
        | E_cons (exp1, exp2) -> RsTodo
        | E_struct (fexp_list) -> RsTodo
        | E_struct_update (exp, fexp_list) -> RsTodo
        | E_field (exp, id) -> RsTodo
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
        | E_assign (lexp, exp) -> RsTodo
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
and process_pexp (Pat_aux (pexp, annot)) : rs_pexp =
    match pexp with
        | Pat_exp (pat, exp)
            -> (RsPexp (
                (process_pat pat),
                (process_exp exp)
            ))
        | Pat_when (pat, exp1, exp2) -> RsPexpTodo



(* Return the ID of an application pattern as a string, or "" otherwise. *)
let pat_app_name (P_aux (pat_aux, _)) =
    match pat_aux with
        | P_app (id, _) -> string_of_id id
        | _ -> ""

let process_pat (P_aux (pat_aux, annot)) = 
    match pat_aux with
        | P_app (id, pat_list) -> print_id id
        | _ -> ()

let process_pexp_funcl (Pat_aux (pexp, annot)) : rs_fn option =
    match pexp with
        | Pat_exp (pat, exp) ->
            (* print_string "PatExp "; *)
            (* process_pat pat; *)
            (* print_string (string_of_pat pat); *)
            (* print_endline (string_of_exp exp); *)
            if pat_app_name pat = "CSR" || pat_app_name pat = "ITYPE" then begin
                print_string (pat_app_name pat);
                print_string " ";
                let rs_exp = process_exp exp in
                print_endline (string_of_exp exp);
                Some ((pat_app_name pat), rs_exp)
            end else None
        | Pat_when (pat1, exp, pat2) -> 
            (* print_string "PatWhen "; *)
            (* print_endline (string_of_pat pat1); *)
            None

let process_func func: rs_program =
    let func = match func with | FCL_aux (func, annot) -> func in
    let (id, pexp) = match func with | FCL_funcl (id, pexp) -> (id, pexp) in
    let fn = process_pexp_funcl pexp in
    match fn with
        | Some fn -> RsProg [fn]
        | None -> RsProg []

    (* let pexp = unau *)
    (* print_string "func "; *)
    (* print_endline (string_of_pexp pexp) *)
    (* print_id id; *)
    (* if string_of_id id = "execute" then print_endline (string_of_pexp pexp) *)

(* Find a pattern expression (Ast.pexp) by ID *)
let find_pexp_by_id pexp name =
    ()

let rec process_funcl funcl : rs_program =
    match funcl with
        | h :: t -> merge_rs_prog (process_func h) (process_funcl t)
        | [] -> RsProg []

let process_fundef fundef : rs_program =
    let (rec_opt, tannot_opt, funcl) = match fundef with
        | FD_function (rec_opt, tannot_opt, funcl) -> (rec_opt, tannot_opt, funcl) in
    process_funcl funcl

let process_node node : rs_program =
    let (def, annot) = match node with | DEF_aux (def, annot) -> (def, annot) in 
    match def with
        | DEF_register (DEC_aux (dec_spec, annot)) -> process_register dec_spec
        | DEF_scattered (SD_aux (scattered, annot)) -> process_scattered scattered
        | DEF_fundef (FD_aux (fundef, annot)) -> process_fundef fundef
        | _ -> RsProg []

let rec process_defs defs : rs_program =
    match defs with
        | h :: t -> merge_rs_prog (process_node h) (process_defs t)
        | [] -> print_endline "Done"; RsProg []

let analyse ast : rs_program =
    let call_set = get_call_set ast in
    process_defs ast.defs

(* This is the entry point *)
let virt_target _ _ out_file ast effect_info env =
  let out_file = match out_file with Some out_file -> out_file ^ ".rs" | None -> "out.rs" in
  let props = Property.find_properties ast in
  Bindings.bindings props |> List.map fst |> IdSet.of_list |> Specialize.add_initial_calls;

  let rust_program = analyse ast in
  let out_chan = open_out out_file in
  output_string out_chan (string_of_rs_prog rust_program);
  flush out_chan;
  close_out out_chan;
  print_endline (string_of_rs_prog rust_program)

let virt_initialize () =
  Preprocess.add_symbol "SYMBOLIC";

  (* These options are either needed for ARM, or increase performance significantly (memo_z3) *)
  Nl_flow.opt_nl_flow := true;
  Type_check.opt_no_lexp_bounds_check := true;
  Reporting.opt_warnings := false;
  Initial_check.opt_undefined_gen := true;
  Initial_check.opt_magic_hash := true;

  Specialize.add_initial_calls (IdSet.singleton (mk_id "virt_footprint"));
  Specialize.add_initial_calls (IdSet.singleton (mk_id "virt_footprint_bare"));
  Specialize.add_initial_calls (IdSet.singleton (mk_id "virt_client"));
  List.iter (fun id ->
      Specialize.add_initial_calls (IdSet.singleton (mk_id id))
    ) !opt_virt_preserve

let _ =
  Target.register
    ~name:"virt"
    ~options:virt_options
    ~pre_parse_hook:virt_initialize
    ~rewrites:virt_rewrites
    virt_target
