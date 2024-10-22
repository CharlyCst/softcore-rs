default Order dec
$include <prelude.sail>

// ———————————————————————————————— Function having int and bool in the signature ————————————————————————————————— //


function handle_int(a1: int): int -> int = {
    123
}

function handle_int_int(a1: int, a2: int): (int,int) -> bool = {
    false
}

function handle_int_int_bool_int(a1: int, a2: int, a3: bool, a4: int): (int,int,bool,int) -> int = {
    123
}


function handle_bool(factor_bool: bool): bool -> unit = {
    ()
}


function handle_empty() -> unit = {
    // Let's just ignore traps in this toy architecture
    ()
}

scattered union ast
val encdec : ast <-> bits(32)
scattered mapping encdec

// —————————————————————————————————— Test function  —————————————————————————————————— //

val execute : ast -> unit
union clause ast = TEST : unit

function clause execute TEST() = {
    handle_empty();
    handle_bool(true);
    let a = handle_int(1234);
    let b = handle_int_int(1234, 12345);
    let c = handle_int_int_bool_int(1234, 12345, false, 2);
}
