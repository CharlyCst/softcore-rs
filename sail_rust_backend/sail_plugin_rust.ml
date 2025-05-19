open Libsail
open Ast
open Ast_util
open Ast_defs
open Interactive.State

open Rust_transform
open Rust_gen
open Call_set
open Context

let opt_debug = ref false
let opt_branch_coverage = ref None
let opt_no_mangle = ref false
let opt_preserve_types = ref IdSet.empty

let rust_options =
  [
    (Flag.create ~prefix:["rust"] "debug", Arg.Set opt_debug, "enable debug logging");
  ]

let rust_rewrites =
  let open Rewrites in
  [
    ("instantiate_outcomes", [String_arg "c"]);
    ("realize_mappings", []);
    ("remove_vector_subrange_pats", []);
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
    ("merge_function_clauses", []);
    ("recheck_defs", []);
    ("constant_fold", [String_arg "c"]);
  ]

let collect_rust_name_info ast =
  let open Ast in
  let open Ast_defs in
  let reserved = ref Util.StringSet.empty in
  let overrides = ref Name_generator.Overrides.empty in
  List.iter
    (function
      | DEF_aux (DEF_val (VS_aux (VS_val_spec (_, _, extern), _)), _) -> (
          match extern_assoc "c" extern with Some name -> reserved := Util.StringSet.add name !reserved | None -> ()
        )
      | DEF_aux (DEF_pragma ("c_reserved", Pragma_line (name, _)), _) -> reserved := Util.StringSet.add name !reserved
      | DEF_aux (DEF_pragma ("c_override", Pragma_structured data), def_annot) -> (
          match Name_generator.parse_override data with
          | Some (from, target) -> overrides := Name_generator.Overrides.add from target !overrides
          | None -> raise (Reporting.err_general def_annot.loc "Failed to interpret $c_override directive")
        )
      | DEF_aux (DEF_register (DEC_aux (DEC_reg (_typ, Id_aux (id, _), _exp), _)), _) ->
          let id = match id with
            | Id id -> id
            | Operator id -> id
          in
          Printf.printf "Register: %s\n" id;
          ()
      | _ -> ()
      )
    ast.defs;
  (!reserved, !overrides)


(* Sail comes with some built-in passes, here we select the ones we want to apply*)
(* see https://github.com/rems-project/sail/blob/284c4795a25723139443dedee1d178f68ddb304e/src/lib/rewrites.ml#L4422 *)
let virt_rewrites =
  let open Rewrites in
  [
    ("instantiate_outcomes", [String_arg "c"]);
    ("realize_mappings", []);
    ("remove_vector_subrange_pats", []);
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
    (* ("simple_assignments", []); *)
    ("remove_not_pats", []);
    ("remove_vector_concat", []);
    ("remove_bitvector_pats", []); 
    (*("pattern_literals", [Literal_arg "all"]); *)
    ("remove_numeral_pats", []);
    ("tuple_assignments", []);
    ("vector_concat_assignments", []);
    (* ("simple_struct_assignments", []); (1* TODO: investigate the impact of this one on vector assignments *1) *)
    ("exp_lift_assign", []);
    ("top_sort_defs", []);
    (* ("merge_function_clauses", []); *)
    ("recheck_defs", []);
    ("constant_fold", [String_arg "c"]);
    (* ("simple_types", []); (1* This one seems to remove the generic types which we also use, maybe it should be put earlier? *1) *)
  ]

let rust_target out_file { ast; effect_info; env; _ } =

  let module Codegen = Rust_backend.Codegen () in
  Reporting.opt_warnings := true;
  let basename = match out_file with Some f -> f | None -> "out.rs" in

  let impl = Codegen.compile_ast env effect_info ast in

  let impl_out = Util.open_output_with_check basename in
  output_string impl_out.channel impl;
  flush impl_out.channel;
  Util.close_output_with_check impl_out

let _ =
  Target.register ~name:"rust" ~options:rust_options ~rewrites:virt_rewrites ~supports_abstract_types:true
    ~supports_runtime_config:true rust_target
