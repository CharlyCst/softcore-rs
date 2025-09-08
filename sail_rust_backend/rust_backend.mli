open Libsail
open Type_check

module type CODEGEN_CONFIG = sig
  val arch : Types.arch_t
end

module Codegen (CodegenConfig : CODEGEN_CONFIG) : sig
  val compile_ast : Env.t -> Effects.side_effect_info -> typed_ast -> string
end
