default Order dec
$include <prelude.sail>

// ——————————————————————————————— Arch size ———————————————————————————————— //

type xlen : Int = 64
type xlen_bytes : Int = 8
type xlenbits = bits(xlen)

// Variable version, which can have the same name
let xlen = sizeof(xlen)

// ———————————————————————————— To be optimized ————————————————————————————— //

// In Rust the 32 branch would not compile as it returns the wrong bit width.
// The compiler has to optimize it out.
function known_match_branch() -> xlenbits =
  match xlen {
    32 => sail_zeros(32),
    64 => sail_zeros(64),
    _ => sail_ones(64),
  }

// ————————————————————————————— Test Function —————————————————————————————— //

scattered union ast
val execute : ast -> unit
union clause ast = TEST : unit

function clause execute TEST() = {
    let a = known_match_branch();
}
