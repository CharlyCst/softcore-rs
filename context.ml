module SSet = Call_set.SSet
module SMap = Map.Make(String)

type arg = string * string option

type context = {
    fun_args: arg list SMap.t;
    call_set: SSet.t;
}
