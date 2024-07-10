default Order dec
$include <prelude.sail>

// ——————————————————————— Zero and signed extensions ——————————————————————— //

val EXTZ : forall 'n 'm, 'm >= 'n. (implicit('m), bits('n)) -> bits('m)
function EXTZ(m, v) = sail_zero_extend(v, m)

val EXTS : forall 'n 'm, 'm >= 'n. (implicit('m), bits('n)) -> bits('m)
function EXTS(m, v) = sail_sign_extend(v, m)

// ——————————————————————————————— Arch size ———————————————————————————————— //

type xlen : Int = 64
type xlen_bytes : Int = 8
type xlenbits = bits(xlen)

// ——————————————————————————————— Registers ———————————————————————————————— //

type regbits = bits(5)

// Program counter
register PC : xlenbits
register nextPC : xlenbits

// General purpose registers
// Note: register 0 is hardwired to 0
register Xs : vector(32, dec, xlenbits)

val rX : regbits -> xlenbits
function rX(r) =
    match r {
        0b00000 => EXTZ(0x0),
        _ => Xs[unsigned(r)]
    }

val wX : (regbits, xlenbits) -> unit
function wX(r, v) =
    if r != 0b00000 then {
        Xs[unsigned(r)] = v;
    }

// Makes it possible to write thigs such as:
//   X(r) = value
// and
//   value = X(r)
overload X = {rX, wX}

// ————————————————————————————————— Memory ————————————————————————————————— //

// Built-in
val MEMr = monadic {lem: "MEMr", coq: "MEMr", _: "read_ram"}: forall 'n 'm, 'n >= 0.
    (int('m), int('n), bits('m), bits('m)) -> bits(8 * 'n)

val read_mem : forall 'n, 'n >= 0. (xlenbits, int('n)) -> bits(8 * 'n)
function read_mem(addr, width) =
    MEMr(sizeof(xlen), width, EXTZ(0x0), addr)

// —————————————————————————————— Instructions —————————————————————————————— //

enum iop = {RISCV_ADDI, RISCV_SLTI, RISCV_SLTIU, RISCV_XORI, RISCV_ORI, RISCV_ANDI}

scattered union ast

val decode : bits(32) -> option(ast)
val execute : ast -> unit

// ————————————————————————————————— IType —————————————————————————————————— //

union clause ast = ITYPE : (bits(12), regbits, regbits, iop)

function clause decode imm : bits(12) @ rs1 : regbits @ 0b000 @ rd : regbits @ 0b0010011
    = Some(ITYPE(imm, rs1, rd, RISCV_ADDI))

function clause execute (ITYPE (imm, rs1, rd, RISCV_ADDI)) =
    let rs1_val = X(rs1) in
    let imm_ext : xlenbits = EXTS(imm) in
    let result = rs1_val + imm_ext in
    X(rd) = result

// —————————————————————————————————— Load —————————————————————————————————— //

union clause ast = LOAD : (bits(12), regbits, regbits)

function clause decode imm : bits(12) @ rs1 : regbits @ 0b011 @ rd : regbits @ 0b0000011
    = Some(LOAD(imm, rs1, rd))

function clause execute(LOAD(imm, rs1, rd)) =
    let addr : xlenbits = X(rs1) + EXTS(imm) in
    let result : xlenbits = read_mem(addr, sizeof(xlen_bytes)) in
    X(rd) = result

