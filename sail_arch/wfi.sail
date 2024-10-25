default Order dec
$include <prelude.sail>

// ———————————————————————————————— Predule ————————————————————————————————— //

infix 4 <_u

val operator <_u : forall 'n. (bits('n), bits('n)) -> bool
function operator <_u (x, y) = unsigned(x) < unsigned(y)

val not_implemented : forall ('a : Type). string -> 'a

// —————————————————————————————— Architecture —————————————————————————————— //

// Arch size
type xlen : Int = 64
type xlenbits = bits(xlen)

// Privilege levels
type priv_level = bits(2)
enum Privilege  = {User, Supervisor, Machine}


// ——————————————————————————————— Registers ———————————————————————————————— //

// register identifiers
type regidx  = bits(5)
type cregidx = bits(3)    /* identifiers in RVC instructions */
type csreg   = bits(12)   /* CSR addressing */

// CSRs
register mepc : xlenbits
register sepc : xlenbits
register uepc : xlenbits
register mtimecmp : bits(64)  
register mtime : bits(64)
register mcycle : bits(64)

bitfield Mstatus : xlenbits = {
  SD   : xlen - 1,

  TSR  : 22,
  TW   : 21,
  TVM  : 20,
  MXR  : 19,
  SUM  : 18,
  MPRV : 17,

  XS   : 16 .. 15,
  FS   : 14 .. 13,

  MPP  : 12 .. 11,
  VS   : 10 .. 9,
  SPP  : 8,

  MPIE : 7,
  SPIE : 5,
  UPIE : 4,

  MIE  : 3,
  SIE  : 1,
  UIE  : 0
}
register mstatus : Mstatus

// Privilege level
register cur_privilege : Privilege

// ————————————————————————————————— Traps —————————————————————————————————— //

function handle_illegal() -> unit = {
    // Let's just ignore traps in this toy architecture
    ()
}

function platform_wfi() -> unit = {
  /* speed execution by getting the timer to fire at the next instruction,
   * since we currently don't have any other devices raising interrupts.
   */
  if mtime <_u mtimecmp then {
    mtime  = mtimecmp;
    mcycle = mtimecmp;
  }
}
// —————————————————————————————— Instructions —————————————————————————————— //

enum Retired = {RETIRE_SUCCESS, RETIRE_FAIL}

scattered union ast
val encdec : ast <-> bits(32)
scattered mapping encdec

val execute : ast -> Retired

// —————————————————————————————————— WFI —————————————————————————————————— //

union clause ast = WFI : unit

mapping clause encdec = WFI()
  <-> 0b000100000101 @ 0b00000 @ 0b000 @ 0b00000 @ 0b1110011

function clause execute WFI() =
  match cur_privilege {
    Machine    => { platform_wfi(); RETIRE_SUCCESS },
    Supervisor => if   mstatus[TW] == 0b1
                  then { handle_illegal(); RETIRE_FAIL }
                  else { platform_wfi(); RETIRE_SUCCESS },
    User       => { handle_illegal(); RETIRE_FAIL }
  }
