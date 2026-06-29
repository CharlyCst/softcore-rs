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
      ; ( "bitvector_access"
        , Some
            { generics = []
            ; args = [ rs_type_bitdynamic; rs_type_i128 ]
            ; ret = rs_type_bool
            ; linked_gen_args = []
            } )
      ; ( "bitvector_concat"
        , Some
            { generics = []
            ; args = [ rs_type_bitdynamic; rs_type_bitdynamic ]
            ; ret = rs_type_bitdynamic
            ; linked_gen_args = []
            } )
      ; ( "bitvector_update"
        , Some
            { generics = []
            ; args = [ rs_type_bitdynamic; rs_type_i128; rs_type_bool ]
            ; ret = rs_type_bitdynamic
            ; linked_gen_args = []
            } )
      ; "update_subrange_bits", None
      ; ( "subrange_bits"
        , Some
            { generics = []
            ; args = [ rs_type_bitdynamic; rs_type_i128; rs_type_i128 ]
            ; ret = rs_type_bitdynamic
            ; linked_gen_args = []
            } )
      ; ( "undefined_bitvector"
        , Some
            { generics = []
            ; args = [ rs_type_i128 ]
            ; ret = rs_type_bitdynamic
            ; linked_gen_args = []
            } )
      ; ( "truncate"
        , Some
            { generics = []
            ; args = [ rs_type_bitdynamic; rs_type_i128 ]
            ; ret = rs_type_bitdynamic
            ; linked_gen_args = []
            } )
      ; ( "sign_extend"
        , Some
            { generics = []
            ; args = [ rs_type_nat; rs_type_bitdynamic ]
            ; ret = rs_type_bitdynamic
            ; linked_gen_args = []
            } )
      ; "sail_sign_extend", None
      ; ( "sail_zeros"
        , Some
            { generics = []
            ; args = [ rs_type_i128 ]
            ; ret = rs_type_bitdynamic
            ; linked_gen_args = []
            } )
      ; ( "sail_ones"
        , Some
            { generics = []
            ; args = [ rs_type_i128 ]
            ; ret = rs_type_bitdynamic
            ; linked_gen_args = []
            } )
      ; "sub_vec", None
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
