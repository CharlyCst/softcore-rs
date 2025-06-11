# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The Softcore-rs provides Rust translations of ISA specifications written in Sail, such as for RISC-V.

A key objective is to use softcore for low-level software testing and verification. Therefore, **correctness is of uttermost importance**. We do not take shortcuts that would make the generated Rust code semantically different from the Sail source.

## Architecture Overview

### Core Components

1. **OCaml Sail Backend** (`sail_rust_backend/`)
   - Custom Sail-to-Rust compiler plugin
   - Handles AST transformation and code generation
   - Integrates with the official Sail compiler toolchain

2. **Rust Prelude** (`prelude/`)
   -  Rust implementation of Sail prelude functions and types

3. **RISC-V Rust translation** (`rv64/`)
   - Translated RISC-V 64-bit specification
   - Configuration system for different processor variants
   - High-level API wrapper around raw generated code

4. **Compiler test suite** (`tests/`)
   - Multiple test architectures for compiler validation
   - Each test contains: Sail source, generated Rust, and unit tests
   - Covers various Sail language features and edge cases

## Build System and Workflow

- Translate a single Sail test file: ` make tests/<test_name>/arch.rs`
  For instance: `make tests/trap/arch.rs`
- Run all tests: `just test`
- Run rust tests on generated code: `cargo test`
  This is useful when working on the Rust prelude.

### Compilation Pipeline

1. **Sail Source** (`.sail` files)
   - Architecture specifications written in Sail DSL
   - Include type definitions, registers, instructions, and execution logic

2. **Sail Compiler + Rust Backend**
   - `./sail_to_rust input.sail -o output.rs`
   - Applies Sail rewrites and transformations
   - Generates Rust code via custom backend plugin

3. **Generated Rust Code**
   - Raw generated code in `raw.rs` modules
   - High-level wrappers in library interfaces
   - Integration with prelude utilities

## Development Workflows

### Adding New Test Cases
1. Create new directory in `tests/`
2. Write `.sail` specification file
3. Add Cargo.toml with prelude dependency
4. Create `lib.rs` with module inclusion and tests
5. Add to workspace members in root `Cargo.toml`
6. Update `TEST_ARCHS` in `Makefile`

### Extending the Compiler
1. Modify OCaml files in `sail_rust_backend/`
2. Update transformation passes in `rust_transform.ml`
3. Extend code generation in `rust_gen.ml`
4. Test with existing and new test cases

### Adding Processor Configurations
1. Define new configuration in `rv64/src/config.rs`
2. Specify extension support and memory parameters
3. Use in initialization: `new_core(config::NEW_CONFIG)`

