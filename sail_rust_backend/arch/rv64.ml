module SSet = Types.SSet

let call_set =
  SSet.of_list
    [ (* "execute"; *)
      "CSR"
    ; "MRET"
    ; "SRET"
    ; "ITYPE"
    ; "TEST"
    ; "WFI"
    ; "EBREAK"
    ; "SFENCE_VMA"
    ; "HFENCE_VVMA"
    ; "HFENCE_GVMA"
    ; (* Decoder *)
      "encdec_backwards"
    ; (* Registers *)
      "rX"
    ; "wX"
    ; "is_CSR_defined"
    ; "creg2reg_idx"
    ; (* PMP checks *)
      "pmpCheck"
    ; "pmpWriteAddrReg"
    ; "pmpWriteCfgReg"
    ; (* Trap handling *)
      "trap_handler"
    ; "exception_delegatee"
    ; "exceptionType_to_bits"
    ; "dispatchInterrupt"
    ; "handle_interrupt"
    ; (* CSRs *)
      "read_CSR"
    ; "write_CSR"
    ; "doCSR"
    ; (* System reset *)
      "reset_sys"
    ]
;;

let external_func : SSet.t =
  SSet.of_list
    [ "subrange_bits"
    ; "not_implemented"
    ; "print_output"
    ; "format!"
    ; "assert!"
    ; "panic!"
    ; "dec_str"
    ; "hex_str"
    ; "update_subrange_bits"
    ; "zero_extend_16"
    ; "zero_extend_63"
    ; "zero_extend_64"
    ; "sign_extend"
    ; "sail_ones"
    ; "min_int"
    ; "__exit"
    ; "signed"
    ; "lteq_int"
    ; "sail_branch_announce"
    ; "bitvector_length"
    ; "bits_str"
    ; "print_reg"
    ; "bitvector_access"
    ; "get_16_random_bits"
    ; "bitvector_concat"
    ; "print_platform"
    ; "cancel_reservation"
    ; "plat_mtval_has_illegal_inst_bits"
    ; "truncate"
    ; "subrange_bits"
    ; "internal_error"
    ; "bitvector_update"
    ; "hex_bits_12_forwards"
    ; "hex_bits_12_backwards"
    ; "sail_zeros"
    ; "parse_hex_bits"
    ; "get_slice_int"
    ]
;;

let unsupported_obj : SSet.t =
  SSet.of_list
    [ (* Used only for side effects, not necessary in the Rust back-end *)
      "csr_name_write_callback"
    ; "csr_id_write_callback"
    ; "csr_full_write_callback"
    ; "long_csr_write_callback"
    ; (* Depend on const generic exprs, would require monomorphisation. *)
      "Mem_write_request"
    ; "PTW_Output"
    ; "PTW_Result"
    ; "pte_bits"
    ; "ppn_bits"
    ; "vpn_bits"
    ]
;;

let rv64 : Types.arch_t = { call_set; external_func; unsupported_obj }
