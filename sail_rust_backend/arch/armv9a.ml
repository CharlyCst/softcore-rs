open Rs_ast
open Rs_context

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

let overwritten_func : rs_fn_type option SMap.t = SMap.of_list []

let external_func : rs_fn_type option SMap.t =
  let externals =
    SMap.of_list
      [ "UInt0", None
      ; "SInt0", None
      ; "ZeroExtend0", None
      ; "SignExtend0", None
      ; "emod_nat", None
      ; "get_slice_int", None
      ; "sail_zeros", None
      ; "sail_shiftleft", None
      ; "sail_shiftright", None
      ; "update_subrange_bits", None
      ; "undefined_bitvector", None
      ; "undefined_bool", None
      ; "undefined_int", None
      ; "subrange_bits", None
      ; "max_int", None
      ; "min_int", None
      ; "format!", None
      ; "assert!", None
      ; "panic!", None
      ]
  in
  SMap.union (fun _ _ _ -> None) externals overwritten_func
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

let armv9a : arch_t =
  { call_set
  ; external_func
  ; overwritten_func
  ; unsupported_obj
  ; unsupported_func
  ; unsupported_match
  }
;;
