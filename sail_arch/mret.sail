default Order dec
$include <prelude.sail>

// ———————————————————————————————— Predule ————————————————————————————————— //

val EXTZ : forall 'n 'm, 'm >= 'n. (implicit('m), bits('n)) -> bits('m)
function EXTZ(m, v) = sail_zero_extend(v, m)

val EXTS : forall 'n 'm, 'm >= 'n. (implicit('m), bits('n)) -> bits('m)
function EXTS(m, v) = sail_sign_extend(v, m)

val not : forall ('p : Bool). bool('p) -> bool(not('p))
function not(p) = not_bool(p)

val eq_anything : forall ('a : Type). ('a, 'a) -> bool
overload operator == = {eq_anything}

val neq_anything : forall ('a : Type). ('a, 'a) -> bool
overload operator != = {neq_anything}

overload ~ = {not_bool, not_vec}
overload operator & = {and_vec}

infix 4 >=_u

val operator >=_u : forall 'n. (bits('n), bits('n)) -> bool
function operator >=_u (x, y) = unsigned(x) >= unsigned(y)

val not_implemented : forall ('a : Type). string -> 'a

// —————————————————————————————— Architecture —————————————————————————————— //

// Arch size
type xlen : Int = 64
type xlen_bytes : Int = 8
type xlenbits = bits(xlen)

// Privilege levels
type priv_level = bits(2)
enum Privilege  = {User, Supervisor, Machine}

function haveSupMode() -> bool = true
function haveUsrMode() -> bool = true

val privLevel_to_bits : Privilege -> priv_level
function privLevel_to_bits (p) =
  match (p) {
    User       => 0b00,
    Supervisor => 0b01,
    Machine    => 0b11
  }

val privLevel_of_bits : priv_level -> Privilege
function privLevel_of_bits (p) =
  match (p) {
    0b00 => User,
    0b01 => Supervisor,
    0b11 => Machine,
    0b10 => not_implemented("Invalid privilege level")
  }

function pc_alignment_mask() -> xlenbits =
  ~(EXTZ(0b10))

// ——————————————————————————————— Registers ———————————————————————————————— //

// register identifiers
type regidx  = bits(5)
type cregidx = bits(3)    /* identifiers in RVC instructions */
type csreg   = bits(12)   /* CSR addressing */

// Program counter
register PC : xlenbits
register nextPC : xlenbits

// CSRs
register mepc : xlenbits
register sepc : xlenbits
register uepc : xlenbits

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

// General purpose registers
// Note: register 0 is hardwired to 0
register Xs : vector(32, dec, xlenbits)

val rX : regidx -> xlenbits
function rX(r) =
    match r {
        0b00000 => EXTZ(0x0),
        _ => Xs[unsigned(r)]
    }

val wX : (regidx, xlenbits) -> unit
function wX(r, v) =
    if r != 0b00000 then {
        Xs[unsigned(r)] = v;
    }

// Makes it possible to write thigs such as:
//   X(r) = value
// and
//   value = X(r)
overload X = {rX, wX}

mapping bool_bits : bool <-> bits(1) = {
  true   <-> 0b1,
  false  <-> 0b0
}

// ————————————————————————————————— Traps —————————————————————————————————— //

val set_next_pc : xlenbits -> unit
function set_next_pc(pc) = {
  nextPC = pc
}

function handle_illegal() -> unit = {
    // Let's just ignore traps in this toy architecture
    ()
}

val get_xret_target : Privilege -> xlenbits
function get_xret_target(p) =
  match p {
    Machine    => mepc,
    Supervisor => sepc,
    User       => uepc
  }

val set_xret_target : (Privilege, xlenbits) -> xlenbits
function set_xret_target(p, value) = {
  let target = value;
  match p {
    Machine    => mepc = target,
    Supervisor => sepc = target,
    User       => uepc = target
  };
  target
}

val prepare_xret_target : (Privilege) -> xlenbits
function prepare_xret_target(p) =
  get_xret_target(p)

function exception_handler(cur_priv : Privilege, pc: xlenbits) -> xlenbits = {
  let prev_priv   = cur_privilege;
  mstatus[MIE]  = mstatus[MPIE];
  mstatus[MPIE] = 0b1;
  cur_privilege   = privLevel_of_bits(mstatus[MPP]);
  mstatus[MPP]  = privLevel_to_bits(if haveUsrMode() then User else Machine);
  if   cur_privilege != Machine
  then mstatus[MPRV] = 0b0;

  prepare_xret_target(Machine) & pc_alignment_mask()
}

// —————————————————————————————— Instructions —————————————————————————————— //

enum Retired = {RETIRE_SUCCESS, RETIRE_FAIL}

scattered union ast
val encdec : ast <-> bits(32)
scattered mapping encdec

val execute : ast -> Retired

// —————————————————————————————————— MRET —————————————————————————————————— //

union clause ast = MRET : unit

mapping clause encdec = MRET()
  <-> 0b0011000 @ 0b00010 @ 0b00000 @ 0b000 @ 0b00000 @ 0b1110011

/* Is XRET from given mode permitted by extension? */
function ext_check_xret_priv (p : Privilege) : Privilege -> bool = true
/* Called if above check fails */
function ext_fail_xret_priv () : unit -> unit = ()

function clause execute MRET() = {
  if   cur_privilege != Machine
  then { handle_illegal(); RETIRE_FAIL }
  else if not(ext_check_xret_priv (Machine))
  then { ext_fail_xret_priv(); RETIRE_FAIL }
  else {
    set_next_pc(exception_handler(cur_privilege, PC));
    RETIRE_SUCCESS
  }
}
