(** Core Config

    When compiling the Sail model to Rust we try to keep as much of the
    configuration dynamic as possible. Unfortunately, some parts of the config
    need to be known at compile time (or at least passed as generic rather than
    read from the context), like the sizes of some types.

    For now we solve this problem by using pre-defined configurations for some
    of the Sail models. When a configuration is known statically, the value is
    directly inlined in the Rust code, allowing for further optimization and
    making some pattern compile where they would not otherwise.
 **)

module SMap = Call_set.SMap

type core_config = int SMap.t

(** The RV64 static configuration **)
let rv64_config : core_config =
  SMap.of_list [ "extensions.V.vlen_exp", 3 (* Must be in range(3, 16) *) ]
;;

(** Returns the config value, if known at compile time. **)
let config_find (cfg : core_config) (item : string list) : int option =
  let key = String.concat "." item in
  SMap.find_opt key cfg
;;
