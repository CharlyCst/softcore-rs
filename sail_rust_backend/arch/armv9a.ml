module SSet = Types.SSet

let call_set = SSet.of_list [ (* Empty for now *) ]

let external_func : SSet.t =
  SSet.of_list
    [ "UInt0"; "ZeroExtend0"; "get_slice_int"; "sail_zeros"; "update_subrange_bits" ]
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
