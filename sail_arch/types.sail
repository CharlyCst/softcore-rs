default Order dec
$include <prelude.sail>

// ——————————————————————————————— Union Type ——————————————————————————————— //

union ExceptionType = {
 E_Fetch_Addr_Align   : unit,
 E_Fetch_Access_Fault : unit,
 E_Illegal_Instr      : unit,
 E_Breakpoint         : unit,

 /* extensions */
 E_Extension          : int
}

// —————————————————————————————— Simple Types —————————————————————————————— //

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
    E_Illegal_Instr()
}

function handle_empty() -> unit = {
    // Let's just ignore traps in this toy architecture
    ()
}

scattered union ast
val encdec : ast <-> bits(32)
scattered mapping encdec

// ————————————————————————————— Advanced Types ————————————————————————————— //

val zeros : forall 'n, 'n >= 0 . implicit('n) -> bits('n)
function zeros (n) = sail_zeros(n)

val "parse_hex_bits" : forall 'n, 'n > 0. (int('n), string) -> bits('n)
function parse_hex_bits(n, s) = {
    zeros(n)
}

val hex_bits_backwards : forall 'm, 'm > 0. (int('m), string) -> bits('m)
function hex_bits_backwards(m, str) = parse_hex_bits(m, str)

union Access_kind('arch_ak : Type) = {
  AK_ttw : unit, // Translation table walk
  AK_arch : 'arch_ak // Architecture specific type of access.
}

struct Mem_read_request('n : Int, 'vasize : Int, 'pa : Type, 'arch_ak: Type), 'n > 0 = {
  access_kind : Access_kind('arch_ak),
  va : option(bits('vasize)),
  pa : 'pa,
  size : int('n),
  tag : bool
}

// ————————————————————————————— Test Function —————————————————————————————— //

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

struct My_struct_generic('n) = {
  foo : bits('n),
}

val exceptionType_to_bits : ExceptionType -> bitvector(8)
function exceptionType_to_bits(e) =
  match (e) {
    E_Fetch_Addr_Align()   => 0x00,
    E_Fetch_Access_Fault() => 0x01,
    E_Illegal_Instr()      => 0x02,
    E_Breakpoint()         => 0x03,
  }

function clause execute TEST() = {
    handle_empty();
    handle_bool(true);
    let a = handle_int(1234);
    let b = handle_int_int(1234, 12345);
    let c = handle_int_int_bool_int(1234, 12345, false, 2);
    let d = handle_retired();
    let e = handle_union();
    let f = hex_bits_backwards(8, "00");

    if f != 0b00000000 then {
        assert(false, "failed to parse hex)");
    };

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

    let s2 : My_struct_generic(4) = struct {
        foo = 0b1010,
    };

    let value = exceptionType_to_bits(E_Fetch_Addr_Align());
}
