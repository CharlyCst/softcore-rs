default Order dec
$include <prelude.sail>

// ——————————————————————————————— Arch size ———————————————————————————————— //

type xlen : Int = 64
type xlen_bytes : Int = 8
type xlenbits = bits(xlen)

// ——————————————————————————————— Registers ———————————————————————————————— //

type regbits = bits(5)

// Program counter
register PC : xlenbits

// —————————————————————————————— Entry point ——————————————————————————————— //

scattered union ast

val execute : ast -> unit
union clause ast = TEST : unit

function clause execute TEST() = {
    // In rust we need to hoist the `let` binding here
    if 0b101 != (let foo = 0b101 in foo) then {
        assert(false, "Branch should not be taken");
    }
}
