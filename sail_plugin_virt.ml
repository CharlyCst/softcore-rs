open Libsail

open Ast
open Ast_util
open Ast_defs
open Rust_gen

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

let process_register reg =
    print_string "Register ";
    let (typ, id, exp) = match reg with | DEC_reg (typ, id, exp) -> (typ, id, exp) in
    print_id id

let process_scattered scattered =
    print_string "Scattered ";
    match scattered with
        | SD_function (rec_aux, tannot, id) -> print_string "function"; print_id id
        | SD_funcl (funcl) -> print_string "funcl"
        | SD_variant (id, typquant) -> print_string "variant"; print_id id
        | SD_unioncl (id, union_type) -> print_string "union"; print_id id
        | SD_mapping (id, tannot_opt) -> print_string "mapping"; print_id id
        | _ -> ()

let process_exp exp = 
    print_string "Exp ";
    print_endline (string_of_exp exp);
    let exp = match exp with | E_aux (exp, aux) -> exp in
    match exp with
        | E_block exp_list -> ()
        | E_id id -> print_id id; () (*print_id id*)
        | E_lit lit -> () (*print_endline (string_of_lit lit)*)
        | E_typ (typ, exp) -> () (*print_string (string_of_typ typ);*)
        | E_app (id, exp_list) -> (*print_string "AppID "; print_id id;*) ()
        | E_app_infix (exp1, id, exp2) -> ()
        | E_tuple (exp_list) -> ()
        | E_if (exp1, exp2, exp3) -> ()
        | E_loop (loop, measure, exp1, exp2) -> ()
        | E_for (id, exp1, exp2, exp3, order, exp4) -> ()
        | E_vector (exp_list) -> ()
        | E_vector_access (exp1, exp2) -> ()
        | E_vector_subrange (exp1, exp2, exp3) -> ()
        | E_vector_update (exp1, exp2, exp3) -> ()
        | E_vector_update_subrange (exp1 ,exp2, exp3, exp4) -> ()
        | E_vector_append (exp1 ,exp2) -> ()
        | E_list (exp_list) -> ()
        | E_cons (exp1, exp2) -> ()
        | E_struct (fexp_list) -> ()
        | E_struct_update (exp, fexp_list) -> ()
        | E_field (exp, id) -> ()
        | E_match (exp, pexp_list) -> ()
        | E_let (letbind, exp) -> (*print_endline (string_of_letbind letbind);*) ()
        | E_assign (lexp, exp) -> ()
        | E_sizeof nexp -> ()
        | E_return exp -> ()
        | E_exit exp -> ()
        | E_ref id -> ()
        | E_throw exp -> ()
        | E_try (exp, pexp_list) -> ()
        | E_assert (exp1, exp2) -> ()
        | E_var (lexp, exp1, exp2) -> ()
        | E_internal_plet (pat, exp1, exp2) -> ()
        | E_internal_return exp -> ()
        | E_internal_value value -> ()
        | E_internal_assume (n_constraint, exp) -> ()
        | E_constraint n_constraint -> ()

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
                process_exp exp;
                Some ((pat_app_name pat), [])
            end else None
        | Pat_when (pat1, exp, pat2) -> 
            (* print_string "PatWhen "; *)
            (* print_endline (string_of_pat pat1); *)
            None

let process_func func =
    let func = match func with | FCL_aux (func, annot) -> func in
    let (id, pexp) = match func with | FCL_funcl (id, pexp) -> (id, pexp) in
    let fn = process_pexp_funcl pexp in
    match fn with
        | Some fn -> print_endline (string_of_rs_fn fn)
        | None -> ();

    (* let pexp = unau *)
    (* print_string "func "; *)
    (* print_endline (string_of_pexp pexp) *)
    (* print_id id; *)
    (* if string_of_id id = "execute" then print_endline (string_of_pexp pexp) *)
    ()

(* Find a pattern expression (Ast.pexp) by ID *)
let find_pexp_by_id pexp name =
    ()

let rec process_funcl funcl =
    match funcl with
        | h :: t -> process_func h; process_funcl t
        | [] -> ()

let process_fundef fundef =
    let (rec_opt, tannot_opt, funcl) = match fundef with
        | FD_function (rec_opt, tannot_opt, funcl) -> (rec_opt, tannot_opt, funcl) in
    process_funcl funcl

let process_node node =
    let (def, annot) = match node with | DEF_aux (def, annot) -> (def, annot) in 
    match def with
        | DEF_register (DEC_aux (dec_spec, annot)) -> process_register dec_spec
        | DEF_scattered (SD_aux (scattered, annot)) -> process_scattered scattered
        | DEF_fundef (FD_aux (fundef, annot)) -> process_fundef fundef
        | _ -> ()

let rec process_defs defs =
    match defs with
        | h :: t -> process_node h; process_defs t
        | [] -> print_endline "Done"

let analyse ast =
    process_defs ast.defs

(* This is the entry point *)
let virt_target _ _ out_file ast effect_info env =
  (* let out_file = match out_file with Some out_file -> out_file ^ ".ir" | None -> "out.ir" in *)
  let props = Property.find_properties ast in
  Bindings.bindings props |> List.map fst |> IdSet.of_list |> Specialize.add_initial_calls;

  analyse ast

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
