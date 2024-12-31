default Order dec
$include <prelude.sail>


// —————————————————————————————— Instructions —————————————————————————————— //

enum iop = {RISCV_ADDI, RISCV_SLTI, RISCV_SLTIU, RISCV_XORI, RISCV_ORI, RISCV_ANDI}
enum csrop = {CSRRW, CSRRS, CSRRC}
enum Retired = {RETIRE_SUCCESS, RETIRE_FAIL}

scattered union ast
val encdec : ast <-> bits(32)
scattered mapping encdec

val execute : ast -> Retired

// —————————————————————————————————— MRET —————————————————————————————————— //

union clause ast = MRET : unit

mapping clause encdec = MRET()
  <-> 0b0011000 @ 0b00010 @ 0b00000 @ 0b000 @ 0b00000 @ 0b1110011

// —————————————————————————————————— WFI —————————————————————————————————— //

union clause ast = WFI : unit

mapping clause encdec = WFI()
  <-> 0b000100000101 @ 0b00000 @ 0b000 @ 0b00000 @ 0b1110011

// —————————————————————————————————— Ecall —————————————————————————————————— //

union clause ast = ECALL : unit

mapping clause encdec = ECALL()
  <-> 0b000000000000 @ 0b00000 @ 0b000 @ 0b00000 @ 0b1110011

// —————————————————————————————————— Ebreak —————————————————————————————————— //

union clause ast = EBREAK : unit

mapping clause encdec = EBREAK()
  <-> 0b000000000001 @ 0b00000 @ 0b000 @ 0b00000 @ 0b1110011

// —————————————————————————————————— Sret —————————————————————————————————— //

union clause ast = SRET : unit

mapping clause encdec = SRET()
  <-> 0b0001000 @ 0b00010 @ 0b00000 @ 0b000 @ 0b00000 @ 0b1110011

// —————————————————————————————————— Sfence —————————————————————————————————— //

union clause ast = SFENCE_VMA : (bits(5), bits(5))

mapping clause encdec = SFENCE_VMA(rs1, rs2)
  <-> 0b0001001 @ rs2 @ rs1 @ 0b000 @ 0b00000 @ 0b1110011
