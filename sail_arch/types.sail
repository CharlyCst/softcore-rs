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

union exception = {
  Error_not_implemented : string,
  Error_internal_error  : unit
}

struct My_struct = {
  field1 : bits(5),
  field2 : int,
  field3 : string,
}

val exceptionType_to_bits : ExceptionType -> bitvector(8)
function exceptionType_to_bits(e) =
  match (e) {
    E_Fetch_Addr_Align()   => 0x00,
    E_Fetch_Access_Fault() => 0x01,
    E_Illegal_Instr()      => 0x02,
    E_Breakpoint()         => 0x03,
    E_Load_Addr_Align()    => 0x04,
    E_Load_Access_Fault()  => 0x05,
    E_SAMO_Addr_Align()    => 0x06,
    E_SAMO_Access_Fault()  => 0x07,
    E_U_EnvCall()          => 0x08,
    E_S_EnvCall()          => 0x09,
    E_Reserved_10()        => 0x0a,
    E_M_EnvCall()          => 0x0b,
    E_Fetch_Page_Fault()   => 0x0c,
    E_Load_Page_Fault()    => 0x0d,
    E_Reserved_14()        => 0x0e,
    E_SAMO_Page_Fault()    => 0x0f,
  }

function clause execute TEST() = {
    handle_empty();
    handle_bool(true);
    let a = handle_int(1234);
    let b = handle_int_int(1234, 12345);
    let c = handle_int_int_bool_int(1234, 12345, false, 2);
    let d = handle_retired();
    let e = handle_union();

    assert(true, "works");
    throw Error_internal_error();

    foreach (i from 0 to 3) {
        let idx = i;
    };

    let s : My_struct = struct {
        field1 = 0b11111,
        field2 = 5,
        field3 = "test",
    };

    let value = exceptionType_to_bits(E_Fetch_Addr_Align());
}
