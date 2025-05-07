open Libsail

open Type_check

module Codegen () : sig
  val compile_ast : Env.t -> Effects.side_effect_info -> typed_ast -> string
end
