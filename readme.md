# Softcore-rs

Softcore-rs is a collection of Rust cores from different architectures, translated from the ISA specification.
Currently, only a subset of RISC-V 64 is available.

> **Warning:** The project is currently under development. The generated code does not provide any guarantees regarding completeness or correctness.

The soft cores are translated from the ISA specifications written in [Sail](https://github.com/rems-project/sail) through a custom Sail-to-Rust backend hosted in this repository.
Because the Rust implementation is directly translated from the specification, the resulting cores are be used as a reference for testing low-level Rust software.

## Example

Soft cores can have a variety of use cases, notably for testing purposes.
For instance, a soft core can be used to query hardware functions, such as checking if a load will succeed or result in a fault for a given hardware state.

```rs
use softcore_rv64::*;

let mut ctx = new_ctx(config::U74);
let addr = 0x8000_0000;
let access = raw::AccessType::Read(());

assert!(
    ctx.pmp_check(addr, access).is_none(),
    "M-mode can access all memory by default"
);

ctx.set_mode(Privilege::User);
assert_eq!(
    ctx.pmp_check(addr, access),
    Some(raw::ExceptionType::E_Load_Access_Fault(())),
    "U-mode has no access by default"
);
```
