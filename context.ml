open Rust_gen

type context = {
    defs: Fun_defs.defs;
    call_set: Call_set.SSet.t;
}

let ctx_fun_is_used (fun_id: string) (ctx: context) : bool =
    Call_set.SSet.mem fun_id ctx.call_set

let ctx_fun_type (fun_id: string) (ctx: context) : rs_fn_type option =
    Fun_defs.SMap.find_opt fun_id ctx.defs.funs
