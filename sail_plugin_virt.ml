open Libsail
open Ast
open Ast_util
open Ast_defs

open Context_pass
open Sail_to_rust
open Rust_transform
open Rust_gen
open Call_set
open Fun_defs
open Context

let opt_virt_preserve = ref ([]:string list)

let virt_options = [
    ( "-virt_preserve",
      Arg.String (fun id -> opt_virt_preserve := id :: !opt_virt_preserve),
      "<id> do not remove the provided id when generating IR");
  ]

(* Sail comes with some built-in passes, here we select the ones we want to apply*)
(* see https://github.com/rems-project/sail/blob/284c4795a25723139443dedee1d178f68ddb304e/src/lib/rewrites.ml#L4422 *)
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
    (* ("remove_bitvector_pats", []); *)
    (* ("pattern_literals", [Literal_arg "all"]); *)
    ("tuple_assignments", []);
    ("vector_concat_assignments", []);
    (* ("simple_struct_assignments", []); (1* TODO: investigate the impact of this one on vector assignments *1) *)
    ("exp_lift_assign", []);
    (* ("merge_function_clauses", []); *)
    ("recheck_defs", []);
    ("constant_fold", [String_arg "c"])
  ]

(* This is the entry point *)
let virt_target _ _ out_file ast effect_info env =
  let out_file = match out_file with Some out_file -> out_file | None -> "out.rs" in
  let props = Property.find_properties ast in
  Bindings.bindings props |> List.map fst |> IdSet.of_list |> Specialize.add_initial_calls;

  (* Compute call set *)
  let set = Call_set.SSet.empty in
  let set = SSet.add "CSR" set in
  let set = SSet.add "MRET" set in
  let set = SSet.add "ITYPE" set in
  let set = SSet.add "TEST" set in
  let call_set = get_call_set ast set in
  (* SSet.iter (Printf.printf "%s ") call_set; *)
  (* print_endline ""; *)

  (* Collect definitions *)
  let defs = get_defs ast in
  (* print_all_defs defs; *)

  (* Build the context *)
  let ctx = {
    defs = defs;
    call_set = call_set;
  } in

  let rust_program = sail_to_rust ast ctx in
  let rust_program = rust_transform_expr bitvec_transform rust_program in
  let rust_program = rust_transform_func virt_context_transform rust_program in
  let rust_program = rust_transform_func unit_remove_transform rust_program in
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
