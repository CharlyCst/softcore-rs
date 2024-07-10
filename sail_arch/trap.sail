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

val bit_to_bool : bit -> bool
function bit_to_bool b = match b {
  bitone  => true,
  bitzero => false
}

val bool_to_bit : bool -> bit
function bool_to_bit x = if x then bitone else bitzero

val bool_to_bits : bool -> bits(1)
function bool_to_bits x = [bool_to_bit(x)]

overload ~ = {not_bool, not_vec}
overload operator & = {and_vec}

infix 7 <<
val "shift_bits_left"  : forall 'n 'm. (bits('n), bits('m)) -> bits('n)
val "shiftl" : forall 'm 'n, 'n >= 0. (bits('m), int('n)) -> bits('m)
overload operator << = {shift_bits_left, shiftl}

infix 4 >=_u
infix 4 <_u

val operator >=_u : forall 'n. (bits('n), bits('n)) -> bool
function operator >=_u (x, y) = unsigned(x) >= unsigned(y)

val operator <_u  : forall 'n. (bits('n), bits('n)) -> bool
function operator <_u  (x, y) = unsigned(x) < unsigned(y)

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

function haveSupMode() -> bool = true
function haveUsrMode() -> bool = true
function haveNExt()    -> bool = false

// ————————————————————————————————— Errors ————————————————————————————————— //

/* model-internal exceptions */
union exception = {
  Error_not_implemented : string,
  Error_internal_error  : unit
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
register mtval    : xlenbits
register mscratch : xlenbits
register sscratch : xlenbits

bitfield Medeleg : xlenbits = {
  SAMO_Page_Fault   : 15,
  Load_Page_Fault   : 13,
  Fetch_Page_Fault  : 12,
  MEnvCall          : 11,
  SEnvCall          : 9,
  UEnvCall          : 8,
  SAMO_Access_Fault : 7,
  SAMO_Addr_Align   : 6,
  Load_Access_Fault : 5,
  Load_Addr_Align   : 4,
  Breakpoint        : 3,
  Illegal_Instr     : 2,
  Fetch_Access_Fault: 1,
  Fetch_Addr_Align  : 0
}
register medeleg : Medeleg

bitfield Mcause : xlenbits = {
  IsInterrupt : xlen - 1,
  Cause       : xlen - 2 .. 0
}
register mcause : Mcause
register scause : Mcause
register ucause : Mcause

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

bitfield Mtvec : xlenbits = {
  Base : xlen - 1 .. 2,
  Mode : 1 .. 0
}
register mtvec : Mtvec
register stvec : Mtvec
register utvec : Mtvec

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
}

type exc_code = bits(8)

val num_of_ExceptionType : ExceptionType -> {'n, (0 <= 'n < xlen). int('n)}
function num_of_ExceptionType(e) =
  match (e) {
    E_Fetch_Addr_Align()   => 0,
    E_Fetch_Access_Fault() => 1,
    E_Illegal_Instr()      => 2,
    E_Breakpoint()         => 3,
    E_Load_Addr_Align()    => 4,
    E_Load_Access_Fault()  => 5,
    E_SAMO_Addr_Align()    => 6,
    E_SAMO_Access_Fault()  => 7,
    E_U_EnvCall()          => 8,
    E_S_EnvCall()          => 9,
    E_Reserved_10()        => 10,
    E_M_EnvCall()          => 11,
    E_Fetch_Page_Fault()   => 12,
    E_Load_Page_Fault()    => 13,
    E_Reserved_14()        => 14,
    E_SAMO_Page_Fault()    => 15,
  }

val exceptionType_to_bits : ExceptionType -> exc_code
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

struct sync_exception = {
  trap    : ExceptionType,
  excinfo : option(xlenbits),
}

/* types of privilege transitions */
union ctl_result = {
  CTL_TRAP : sync_exception,
}

type tv_mode = bits(2)
enum TrapVectorMode = {TV_Direct, TV_Vector, TV_Reserved}

val set_next_pc : xlenbits -> unit
function set_next_pc(pc) = {
  nextPC = pc
}

function tval(excinfo : option(xlenbits)) -> xlenbits = {
  match (excinfo) {
    Some(e) => e,
    None()  => EXTZ(0b0)
  }
}

val trapVectorMode_of_bits : tv_mode -> TrapVectorMode
function trapVectorMode_of_bits (m) =
  match (m) {
    0b00 => TV_Direct,
    0b01 => TV_Vector,
    _    => TV_Reserved
  }

/* Interpreting the trap-vector address */
function tvec_addr(m : Mtvec, c : Mcause) -> option(xlenbits) = {
  let base : xlenbits = m[Base] @ 0b00;
  match (trapVectorMode_of_bits(m[Mode])) {
    TV_Direct => Some(base),
    TV_Vector => if   c[IsInterrupt] == 0b1
                 then Some(base + (EXTZ(c[Cause]) << 2))
                 else Some(base),
    TV_Reserved => None()
  }
}

/* used for traps and ECALL */
function prepare_trap_vector(p : Privilege, cause : Mcause) -> xlenbits = {
  let tvec : Mtvec = match p {
                       Machine    => mtvec,
                       Supervisor => stvec,
                       User       => utvec
                     };
  match tvec_addr(tvec, cause) {
    Some(epc) => epc,
    None()    => throw Error_internal_error(),
  }
}

function trap_handler(del_priv : Privilege, intr : bool, c : exc_code, pc : xlenbits, info : option(xlenbits))
                     -> xlenbits = {
  match (del_priv) {
    Machine => {
       mcause[IsInterrupt] = bool_to_bits(intr);
       mcause[Cause]       = EXTZ(c);

       mstatus[MPIE] = mstatus[MIE];
       mstatus[MIE]  = 0b0;
       mstatus[MPP]  = privLevel_to_bits(cur_privilege);
       mtval           = tval(info);
       mepc            = pc;

       cur_privilege   = del_priv;

       prepare_trap_vector(del_priv, mcause)
    },
    Supervisor => {
       assert (haveSupMode(), "no supervisor mode present for delegation");

       scause[IsInterrupt] = bool_to_bits(intr);
       scause[Cause]       = EXTZ(c);

       mstatus[SPIE] = mstatus[SIE];
       mstatus[SIE]  = 0b0;
       mstatus[SPP]  = match cur_privilege {
                           User => 0b0,
                           Supervisor => 0b1,
                           Machine => throw Error_internal_error(),
                         };
       stval           = tval(info);
       sepc            = pc;

       cur_privilege   = del_priv;

       prepare_trap_vector(del_priv, scause)
    },
    User => {
       ucause[IsInterrupt] = bool_to_bits(intr);
       ucause[Cause]       = EXTZ(c);

       mstatus[UPIE] = mstatus[UIE];
       mstatus[UIE]  = 0b0;
       utval           = tval(info);
       uepc            = pc;

       cur_privilege   = del_priv;

       prepare_trap_vector(del_priv, ucause)
    }
  };
}

/* Exception delegation: given an exception and the privilege at which
 * it occured, returns the privilege at which it should be handled.
 */
function exception_delegatee(e : ExceptionType, p : Privilege) -> Privilege = {
  let idx   = num_of_ExceptionType(e);
  let super = bit_to_bool(medeleg.bits[idx]);
  let user  = false;
  let deleg = if      haveUsrMode() & user  then User
              else if haveSupMode() & super then Supervisor
              else                               Machine;
  /* We cannot transition to a less-privileged mode. */
  if   privLevel_to_bits(deleg) <_u privLevel_to_bits(p)
  then p else deleg
}

function exception_handler(cur_priv : Privilege, ctl : ctl_result,
                           pc: xlenbits) -> xlenbits = {
  match (cur_priv, ctl) {
    (_, CTL_TRAP(e)) => {
      let del_priv = exception_delegatee(e.trap, cur_priv);
      trap_handler(del_priv, false, exceptionType_to_bits(e.trap), pc, e.excinfo)
    },
  }
}

function handle_illegal() -> unit = {
  let t : sync_exception = struct { trap    = E_Illegal_Instr(),
                                    excinfo = None() };
  set_next_pc(exception_handler(cur_privilege, CTL_TRAP(t), PC))
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
