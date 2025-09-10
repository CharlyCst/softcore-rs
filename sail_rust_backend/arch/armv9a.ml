module SSet = Types.SSet

let call_set =
  SSet.of_list
    [ (* Integer arithmetics *)
      "execute_aarch64_instrs_integer_arithmetic_add_sub_carry"
    ]
;;

let external_func : SSet.t =
  SSet.of_list
    [ "UInt0"
    ; "SInt0"
    ; "ZeroExtend0"
    ; "get_slice_int"
    ; "sail_zeros"
    ; "update_subrange_bits"
    ; "undefined_bitvector"
    ; "bitvector_length"
    ; "bitvector_access"
    ; "bitvector_concat"
    ; "subrange_bits"
    ; "format!"
    ; "assert!"
    ; "panic!"
    ]
;;

let unsupported_obj : SSet.t =
  SSet.of_list
    [ (* Depend on const generic exprs, would require monomorphisation. *)
      "Mem_write_request"
    ; "integer_subrange"
    ; (* Mistyped global constants *)
      "GIC_BASE"
    ; "__GICC_IIDR"
    ; "__GICD_TYPER"
    ; "UART_BASE"
    ]
;;

let armv9a : Types.arch_t = { call_set; external_func; unsupported_obj }
