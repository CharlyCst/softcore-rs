# Softcore-rs

Softcore-rs is a collection of Rust cores from different architectures, translated from the ISA specification.
Currently, only a subset of RISC-V 64 is available.

> **Warning:** The project is currently under development. The generated code does not provide any guarantees regarding completeness or correctness.

The soft cores are translated from the ISA specifications written in [Sail](https://github.com/rems-project/sail) through a custom Sail-to-Rust backend hosted in this repository.
Because the Rust implementation is directly translated from the specification, the resulting cores are be used as a reference for testing low-level Rust software.

## Cores

| Architecture | Crate                                |
|--------------|--------------------------------------|
| RISC-V 64    | [![Crates.io][rv64-badge]][rv64-url] |

[rv64-badge]: https://img.shields.io/crates/v/softcore-rv64
[rv64-url]: https://crates.io/crates/softcore-rv64

## Example

Soft cores can have a variety of use cases, notably for testing purposes.
For instance, a soft core can be used to query hardware functions, such as checking if a load will succeed or result in a fault for a given hardware state.

```rs
use softcore_rv64::*;

let mut core = new_core(config::U74);
let addr = 0x8000_0000;
let access = raw::AccessType::Read(());

// Check the default access rights
assert!(
    core.pmp_check(addr, access).is_none(),
    "M-mode can access all memory by default"
);

core.set_mode(Privilege::User);
assert_eq!(
    core.pmp_check(addr, access),
    Some(ExceptionType::E_Load_Access_Fault(())),
    "U-mode has no access by default"
);
```
