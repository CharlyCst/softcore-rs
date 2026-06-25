open Rs_ast
open Rs_context

let overwritten_func : rs_fn_type option SMap.t = SMap.empty

let external_func : rs_fn_type option SMap.t =
  let externals : rs_fn_type option SMap.t =
    SMap.of_list
      [ ( "bitvector_length"
        , Some
            { generics = []
            ; args = [ rs_type_bitdynamic ]
            ; ret = rs_type_i128
            ; linked_gen_args = []
            } )
      ; "bitvector_access", None
      ; "bitvector_concat", None
      ; "bitvector_update", None
      ; "update_subrange_bits", None
      ; "subrange_bits", None
      ; "undefined_bitvector", None
      ; "truncate", None
      ; "sign_extend", None
      ; "sail_sign_extend", None
      ]
  in
  SMap.union (fun _ _ _ -> None) externals overwritten_func
;;

let add_prelude (arch : arch_t) : arch_t =
  { arch with
    overwritten_func =
      SMap.union (fun _ f _ -> Some f) arch.overwritten_func overwritten_func
  ; external_func = SMap.union (fun _ f _ -> Some f) arch.external_func external_func
  }
;;
