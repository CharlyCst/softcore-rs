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

overload ~ = {not_bool, not_vec}

infix 4 >=_u

val operator >=_u : forall 'n. (bits('n), bits('n)) -> bool
function operator >=_u (x, y) = unsigned(x) >= unsigned(y)

// —————————————————————————————— Architecture —————————————————————————————— //

// Arch size
type xlen : Int = 64
type xlen_bytes : Int = 8
type xlenbits = bits(xlen)

// Privilege levels
type priv_level = bits(2)
enum Privilege  = {User, Supervisor, Machine}

val privLevel_to_bits : Privilege -> priv_level
function privLevel_to_bits (p) =
  match (p) {
    User       => 0b00,
    Supervisor => 0b01,
    Machine    => 0b11
  }

// ——————————————————————————————— Registers ———————————————————————————————— //

// register identifiers
type regidx  = bits(5)
type cregidx = bits(3)    /* identifiers in RVC instructions */
type csreg   = bits(12)   /* CSR addressing */

// Program counter
register PC : xlenbits
register nextPC : xlenbits

// CSRs
register mscratch : xlenbits
register sscratch : xlenbits

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

function handle_illegal() -> unit = {
    // Let's just ignore traps in this toy architecture
    ()
}

// —————————————————————————————— Instructions —————————————————————————————— //

enum iop = {RISCV_ADDI, RISCV_SLTI, RISCV_SLTIU, RISCV_XORI, RISCV_ORI, RISCV_ANDI}
enum csrop = {CSRRW, CSRRS, CSRRC}
enum Retired = {RETIRE_SUCCESS, RETIRE_FAIL}

scattered union ast
val encdec : ast <-> bits(32)
scattered mapping encdec

val execute : ast -> Retired


// ————————————————————————————————— IType —————————————————————————————————— //

union clause ast = ITYPE : (bits(12), regidx, regidx, iop)

mapping clause encdec = ITYPE(imm, rs1, rd, RISCV_ADDI)
    <-> imm : bits(12) @ rs1 : regidx @ 0b000 @ rd : regidx @ 0b0010011

function clause execute (ITYPE (imm, rs1, rd, RISCV_ADDI)) = {
    let rs1_val = X(rs1) in
    let imm_ext : xlenbits = EXTS(imm) in
    let result = rs1_val + imm_ext in
    X(rd) = result;
    RETIRE_SUCCESS
}

// —————————————————————————————————— CSR ——————————————————————————————————— //

union clause ast = CSR  : (bits(12), regidx, regidx, bool, csrop)

mapping encdec_csrop : csrop <-> bits(2) = {
  CSRRW <-> 0b01,
  CSRRS <-> 0b10,
  CSRRC <-> 0b11
}

type csrRW = bits(2)
function csrAccess(csr : csreg) -> csrRW = csr[11..10]
function csrPriv(csr : csreg) -> priv_level = csr[9..8]

mapping clause encdec = CSR(csr, rs1, rd, is_imm, op)
  <-> csr @ rs1 @ bool_bits(is_imm) @ encdec_csrop(op) @ rd @ 0b1110011

function is_CSR_defined (csr : csreg, p : Privilege) -> bool =
  match (csr) {
    0x340 => p == Machine, // mscratch
    0x140 => (p == Machine | p == Supervisor), // sscratch
    _     => false 
  }

val check_CSR_access : (csrRW, priv_level, Privilege, bool) -> bool
function check_CSR_access(csrrw, csrpr, p, isWrite) =
    not(isWrite == true & csrrw == 0b11)  /* read/write */
  & (privLevel_to_bits(p) >=_u csrpr)      /* privilege */

function check_CSR(csr : csreg, p : Privilege, isWrite : bool) -> bool =
    is_CSR_defined(csr, p) & check_CSR_access(csrAccess(csr), csrPriv(csr), p, isWrite)

function readCSR csr : csreg -> xlenbits = {
  let res : xlenbits =
  match (csr, sizeof(xlen)) {
    (0x340,  _) => mscratch,
    (0x140,  _) => sscratch,
    _ => EXTZ(0x0),
  };
  res
}

function writeCSR (csr : csreg, value : xlenbits) -> unit = {
  let res : option(xlenbits) =
  match (csr, sizeof(xlen)) {
    (0x340,  _) => { mscratch = value; Some(mscratch) },
    (0x140,  _) => { sscratch = value; Some(sscratch) },
    _ => None()
  };
}

function clause execute CSR(csr, rs1, rd, is_imm, op) = {
  let rs1_val : xlenbits = if is_imm then EXTZ(rs1) else X(rs1);
  let isWrite : bool = match op {
    CSRRW  => true,
    _      => if is_imm then unsigned(rs1_val) != 0 else unsigned(rs1) != 0
  };
  if not(check_CSR(csr, cur_privilege, isWrite))
  then { handle_illegal(); RETIRE_FAIL }
  else {
    let csr_val = readCSR(csr); /* could have side-effects, so technically shouldn't perform for CSRW[I] with rd == 0 */
    if isWrite then {
      let new_val : xlenbits = match op {
        CSRRW => rs1_val,
        CSRRS => csr_val | rs1_val,
        CSRRC => csr_val & ~(rs1_val)
      };
      writeCSR(csr, new_val)
    };
    X(rd) = csr_val;
    RETIRE_SUCCESS
  }
}
