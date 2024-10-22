# Virt-sail: A Sail to Rust Transpiler

> **Warning:** The project is currently under development. The generated code does not provide any guarantees regarding completeness or correctness.
 
**Virt-sail** transpiles the official Sail RISC-V specification into Rust. Sail is an ISA description language that contains formal specifications for RISC-V and other hardware architectures.

### Installation

To install Virt-sail, simply run:

```bash
make install
```

It is important to choose the version 0.17.1 [link](https://ocaml.org/p/libsail/0.17.1) of libsail, otherwise the code won't compile.

### Example Usage

You can generate a few examples by running:

```bash
make basic
```

This will generate a file out.rs.