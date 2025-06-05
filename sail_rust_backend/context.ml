open Rust_gen

type defmap = rs_fn_type Call_set.SMap.t
type unionmap = rs_type Call_set.SMap.t

type defs = {
    funs : defmap;
    unions: unionmap;
}

type context = {
    defs: defs;
    call_set: Call_set.SSet.t;
    config_map: Call_set.config_map;
    registers: SSet.t;
    enum_entries: (string * string) list;
    mutable ret_type: rs_type;
}

let ctx_fun_is_used (fun_id: string) (ctx: context) : bool =
    Call_set.SSet.mem fun_id ctx.call_set

let ctx_fun_type (fun_id: string) (ctx: context) : rs_fn_type option =
    Call_set.SMap.find_opt fun_id ctx.defs.funs

let ctx_union_type (union_id: string) (ctx: context) : rs_type option =
    Call_set.SMap.find_opt union_id ctx.defs.unions
