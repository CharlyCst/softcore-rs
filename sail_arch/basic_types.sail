default Order dec
$include <prelude.sail>

// ———————————————————————————————— Union type to make sure we generate correctly ————————————————————————————————— //

union ExceptionType = {
 E_Fetch_Addr_Align   : unit,
 E_Fetch_Access_Fault : unit,
 E_Illegal_Instr      : unit,
 E_Breakpoint         : unit,
 E_Load_Addr_Align    : unit,
 E_Load_Access_Fault  : unit,
 E_SAMO_Addr_Align    : unit,
 E_SAMO_Access_Fault  : unit,
 E_U_EnvCall          : unit,
 E_S_EnvCall          : unit,
 E_Reserved_10        : unit,
 E_M_EnvCall          : unit,
 E_Fetch_Page_Fault   : unit,
 E_Load_Page_Fault    : unit,
 E_Reserved_14        : unit,
 E_SAMO_Page_Fault    : unit,

 /* extensions */
 E_Extension          : int
}

// ———————————————————————————————— Function having int and bool in the signature ————————————————————————————————— //

function handle_int(a1: int): int -> int = {
    a1 + 4
}

function handle_int_int(a1: int, a2: int): (int,int) -> bool = {
    false
}

function handle_int_int_bool_int(a1: int, a2: int, a3: bool, a4: int): (int,int,bool,int) -> int = {
    123 + 4 + 4
}


function handle_bool(factor_bool: bool): bool -> unit = {
    ()
}

function handle_union(): unit -> ExceptionType = {
    E_Fetch_Page_Fault()
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


enum Retired = {RETIRE_SUCCESS, RETIRE_FAIL}
function handle_retired() -> Retired = {
    RETIRE_SUCCESS
}


function clause execute TEST() = {
    handle_empty();
    handle_bool(true);
    let a = handle_int(1234);
    let b = handle_int_int(1234, 12345);
    let c = handle_int_int_bool_int(1234, 12345, false, 2);
    let d = handle_retired();
    let e = handle_union();
}
