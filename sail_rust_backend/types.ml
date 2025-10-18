module SSet = Set.Make (String)
module SMap = Map.Make (String)

(** Architecture-specific configuration **)
type arch_t =
  { call_set : SSet.t
  ; external_func : SSet.t
  ; overwritten_func : SSet.t
  ; unsupported_obj : SSet.t
  ; unsupported_func : SSet.t
  ; unsupported_match : SSet.t
  }
