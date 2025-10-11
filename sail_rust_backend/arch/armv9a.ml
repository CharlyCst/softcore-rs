module SSet = Types.SSet

let call_set =
  SSet.of_list
    [ (* Integer arithmetics *)
      "execute_aarch64_instrs_integer_arithmetic_add_sub_carry"
    ; "execute_aarch64_instrs_integer_arithmetic_add_sub_immediate"
      (* ; "execute_aarch64_instrs_integer_arithmetic_add_sub_extendedreg" *)
      (* ; "execute_aarch64_instrs_integer_arithmetic_add_sub_shiftedreg" *)
      (* System registers *)
      (* "execute_aarch64_instrs_system_register_system_128" *)
    ]
;;

let external_func : SSet.t =
  SSet.of_list
    [ "UInt0"
    ; "SInt0"
    ; "ZeroExtend0"
    ; "SignExtend0"
    ; "emod_nat"
    ; "get_slice_int"
    ; "sail_zeros"
    ; "sail_shiftleft"
    ; "sail_shiftright"
    ; "update_subrange_bits"
    ; "undefined_bitvector"
    ; "undefined_bool"
    ; "undefined_int"
    ; "bitvector_length"
    ; "bitvector_access"
    ; "bitvector_concat"
    ; "subrange_bits"
    ; "min_int"
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

let unsupported_func : SSet.t = SSet.of_list [ "NVMem_read__1" ]
let unsupported_match : SSet.t = SSet.of_list []

let armv9a : Types.arch_t =
  { call_set; external_func; unsupported_obj; unsupported_func; unsupported_match }
;;
