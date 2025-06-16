//! The RISC-V registers

use crate::raw::regidx;
use softcore_prelude::BitVector;

/// The type of general purpose registers
pub type GeneralRegister = regidx;

/// The X0 register.
///
/// This register is hardwired to zero.
pub const X0: regidx = regidx::Regidx(BitVector::new(0));

/// The ZERO (X0) register.
///
/// This register is hardwired to zero.
pub const ZERO: regidx = X0;

/// The X1 (RA) register.
///
/// Used for the return address in the RISC-V ABI.
pub const X1: regidx = regidx::Regidx(BitVector::new(1));

/// The RA (X1) register.
///
/// Used for the return address in the RISC-V ABI.
pub const RA: regidx = X1;

/// The X2 (SP) register.
///
/// Used for the stack pointer in the RISC-V ABI.
pub const X2: regidx = regidx::Regidx(BitVector::new(2));

/// The SP (X2) register.
///
/// Used for the stack pointer in the RISC-V ABI.
pub const SP: regidx = X2;

/// The X3 (GP) register.
///
/// Used for the global pointer in the RISC-V ABI.
pub const X3: regidx = regidx::Regidx(BitVector::new(3));

/// The GP (X3) register.
///
/// Used for the global pointer in the RISC-V ABI.
pub const GP: regidx = X3;

/// The X4 (TP) register.
///
/// Used for the thread pointer in the RISC-V ABI.
pub const X4: regidx = regidx::Regidx(BitVector::new(4));

/// The TP (X4) register.
///
/// Used for the thread pointer in the RISC-V ABI.
pub const TP: regidx = X4;

/// The X5 (T0) register.
///
/// Temporary register in the RISC-V ABI.
pub const X5: regidx = regidx::Regidx(BitVector::new(5));

/// The T0 (X5) register.
///
/// Temporary register in the RISC-V ABI.
pub const T0: regidx = X5;

/// The X6 (T1) register.
///
/// Temporary register in the RISC-V ABI.
pub const X6: regidx = regidx::Regidx(BitVector::new(6));

/// The T1 (X6) register.
///
/// Temporary register in the RISC-V ABI.
pub const T1: regidx = X6;

/// The X7 (T2) register.
///
/// Temporary register in the RISC-V ABI.
pub const X7: regidx = regidx::Regidx(BitVector::new(7));

/// The T2 (X7) register.
///
/// Temporary register in the RISC-V ABI.
pub const T2: regidx = X7;

/// The X8 (S0/FP) register.
///
/// Saved register 0 / Frame pointer in the RISC-V ABI.
pub const X8: regidx = regidx::Regidx(BitVector::new(8));

/// The S0 (X8) register.
///
/// Saved register 0 / Frame pointer in the RISC-V ABI.
pub const S0: regidx = X8;

/// The FP (X8) register.
///
/// Frame pointer in the RISC-V ABI.
pub const FP: regidx = X8;

/// The X9 (S1) register.
///
/// Saved register 1 in the RISC-V ABI.
pub const X9: regidx = regidx::Regidx(BitVector::new(9));

/// The S1 (X9) register.
///
/// Saved register 1 in the RISC-V ABI.
pub const S1: regidx = X9;

/// The X10 (A0) register.
///
/// Function argument / Return value in the RISC-V ABI.
pub const X10: regidx = regidx::Regidx(BitVector::new(10));

/// The A0 (X10) register.
///
/// Function argument / Return value in the RISC-V ABI.
pub const A0: regidx = X10;

/// The X11 (A1) register.
///
/// Function argument / Return value in the RISC-V ABI.
pub const X11: regidx = regidx::Regidx(BitVector::new(11));

/// The A1 (X11) register.
///
/// Function argument / Return value in the RISC-V ABI.
pub const A1: regidx = X11;

/// The X12 (A2) register.
///
/// Function argument in the RISC-V ABI.
pub const X12: regidx = regidx::Regidx(BitVector::new(12));

/// The A2 (X12) register.
///
/// Function argument in the RISC-V ABI.
pub const A2: regidx = X12;

/// The X13 (A3) register.
///
/// Function argument in the RISC-V ABI.
pub const X13: regidx = regidx::Regidx(BitVector::new(13));

/// The A3 (X13) register.
///
/// Function argument in the RISC-V ABI.
pub const A3: regidx = X13;

/// The X14 (A4) register.
///
/// Function argument in the RISC-V ABI.
pub const X14: regidx = regidx::Regidx(BitVector::new(14));

/// The A4 (X14) register.
///
/// Function argument in the RISC-V ABI.
pub const A4: regidx = X14;

/// The X15 (A5) register.
///
/// Function argument in the RISC-V ABI.
pub const X15: regidx = regidx::Regidx(BitVector::new(15));

/// The A5 (X15) register.
///
/// Function argument in the RISC-V ABI.
pub const A5: regidx = X15;

/// The X16 (A6) register.
///
/// Function argument in the RISC-V ABI.
pub const X16: regidx = regidx::Regidx(BitVector::new(16));

/// The A6 (X16) register.
///
/// Function argument in the RISC-V ABI.
pub const A6: regidx = X16;

/// The X17 (A7) register.
///
/// Function argument in the RISC-V ABI.
pub const X17: regidx = regidx::Regidx(BitVector::new(17));

/// The A7 (X17) register.
///
/// Function argument in the RISC-V ABI.
pub const A7: regidx = X17;

/// The X18 (S2) register.
///
/// Saved register 2 in the RISC-V ABI.
pub const X18: regidx = regidx::Regidx(BitVector::new(18));

/// The S2 (X18) register.
///
/// Saved register 2 in the RISC-V ABI.
pub const S2: regidx = X18;

/// The X19 (S3) register.
///
/// Saved register 3 in the RISC-V ABI.
pub const X19: regidx = regidx::Regidx(BitVector::new(19));

/// The S3 (X19) register.
///
/// Saved register 3 in the RISC-V ABI.
pub const S3: regidx = X19;

/// The X20 (S4) register.
///
/// Saved register 4 in the RISC-V ABI.
pub const X20: regidx = regidx::Regidx(BitVector::new(20));

/// The S4 (X20) register.
///
/// Saved register 4 in the RISC-V ABI.
pub const S4: regidx = X20;

/// The X21 (S5) register.
///
/// Saved register 5 in the RISC-V ABI.
pub const X21: regidx = regidx::Regidx(BitVector::new(21));

/// The S5 (X21) register.
///
/// Saved register 5 in the RISC-V ABI.
pub const S5: regidx = X21;

/// The X22 (S6) register.
///
/// Saved register 6 in the RISC-V ABI.
pub const X22: regidx = regidx::Regidx(BitVector::new(22));

/// The S6 (X22) register.
///
/// Saved register 6 in the RISC-V ABI.
pub const S6: regidx = X22;

/// The X23 (S7) register.
///
/// Saved register 7 in the RISC-V ABI.
pub const X23: regidx = regidx::Regidx(BitVector::new(23));

/// The S7 (X23) register.
///
/// Saved register 7 in the RISC-V ABI.
pub const S7: regidx = X23;

/// The X24 (S8) register.
///
/// Saved register 8 in the RISC-V ABI.
pub const X24: regidx = regidx::Regidx(BitVector::new(24));

/// The S8 (X24) register.
///
/// Saved register 8 in the RISC-V ABI.
pub const S8: regidx = X24;

/// The X25 (S9) register.
///
/// Saved register 9 in the RISC-V ABI.
pub const X25: regidx = regidx::Regidx(BitVector::new(25));

/// The S9 (X25) register.
///
/// Saved register 9 in the RISC-V ABI.
pub const S9: regidx = X25;

/// The X26 (S10) register.
///
/// Saved register 10 in the RISC-V ABI.
pub const X26: regidx = regidx::Regidx(BitVector::new(26));

/// The S10 (X26) register.
///
/// Saved register 10 in the RISC-V ABI.
pub const S10: regidx = X26;

/// The X27 (S11) register.
///
/// Saved register 11 in the RISC-V ABI.
pub const X27: regidx = regidx::Regidx(BitVector::new(27));

/// The S11 (X27) register.
///
/// Saved register 11 in the RISC-V ABI.
pub const S11: regidx = X27;

/// The X28 (T3) register.
///
/// Temporary register in the RISC-V ABI.
pub const X28: regidx = regidx::Regidx(BitVector::new(28));

/// The T3 (X28) register.
///
/// Temporary register in the RISC-V ABI.
pub const T3: regidx = X28;

/// The X29 (T4) register.
///
/// Temporary register in the RISC-V ABI.
pub const X29: regidx = regidx::Regidx(BitVector::new(29));

/// The T4 (X29) register.
///
/// Temporary register in the RISC-V ABI.
pub const T4: regidx = X29;

/// The X30 (T5) register.
///
/// Temporary register in the RISC-V ABI.
pub const X30: regidx = regidx::Regidx(BitVector::new(30));

/// The T5 (X30) register.
///
/// Temporary register in the RISC-V ABI.
pub const T5: regidx = X30;

/// The X31 (T6) register.
///
/// Temporary register in the RISC-V ABI.
pub const X31: regidx = regidx::Regidx(BitVector::new(31));

/// The T6 (X31) register.
///
/// Temporary register in the RISC-V ABI.
pub const T6: regidx = X31;
