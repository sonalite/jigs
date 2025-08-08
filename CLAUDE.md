# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Jigs - A high-performance RISC-V runtime for ARM64 systems with gas-metered execution. The project aims to build a complete RISC-V execution environment that:
1. Decodes and encodes RISC-V 32-bit instructions
2. JIT-compiles RISC-V code to native ARM64 for near-native performance
3. Provides gas-metered execution for controlled resource usage in blockchain/sandboxed environments

Currently implements decoding and display formatting for 32-bit RISC-V instructions, with encoding, JIT compilation, and gas tracking planned.

## Architecture
The project is structured as a Rust library with an example binary:
- **src/lib.rs**: Library entry point that exports public APIs
- **src/main.rs**: Example binary demonstrating instruction decoding
- **src/instruction.rs**: Core instruction representation and decoding logic
  - `Instruction` enum with variants for each RISC-V instruction (Add, Sub, etc.)
  - Decode method that extracts fields from 32-bit instruction words using bitmasking
  - Display trait implementation for assembly-style output
  - Supports RV32IM: base integer instructions plus M extension (multiply/divide)
- **src/tests/**: Comprehensive test suite organized by functionality
  - `decode/`: Tests for instruction decoding (register, immediate, load, store, branch, multiply, jump, system, utype)
  - `display/`: Tests for instruction display formatting

## Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Run single test: `cargo test tests::instruction::decode::add::basic`
- Test specific functionality: `cargo test instruction::decode`
- Test with output: `cargo test -- --nocapture`
- Run documentation tests: `cargo test --doc`
- Code coverage: `cargo tarpaulin`
- Format code: `cargo fmt`
- Check formatting: `cargo fmt -- --check`
- Lint code: `cargo clippy`

## Code Coverage Requirements
- New code should have test coverage as close to 100% as possible
- Always run `cargo tarpaulin` before committing
- Coverage percentage must never decrease from the previous commit

## Testing Conventions
- Unit tests should live in the `src/tests/` directory
- For small test suites: use a single file (e.g., `src/tests/feature.rs`)
- For large test suites: create a folder with separate files organized by feature (e.g., `src/tests/instruction/decode/add.rs`, `src/tests/instruction/decode/sub.rs`)
- Test names should be concise and NOT include "test" or the subject being tested, as this is implied by the module structure (e.g., in `tests/instruction/decode/add.rs`, use `fn basic()` not `fn test_add_basic()`)
- When implementing similar functionality to existing features, review existing tests to ensure consistent test coverage (e.g., if ADD has tests for basic, zero_registers, max_registers, and different_registers, similar instructions should have the same test cases)

## Code Style Conventions
- File ordering: module docs → `mod` declarations → `use` statements → constants → types → implementations
- Before committing: ensure `cargo build`, `cargo test`, `cargo test --doc`, `cargo tarpaulin`, `cargo fmt -- --check`, and `cargo clippy` produce no warnings
- Documentation: Keep all module-level documentation up-to-date, including examples in doc comments
- Error handling: Always use Result for error handling, never panic

## Pre-Commit Checklist
Before committing any changes, ensure all of the following pass without warnings:
1. `cargo build` - Code compiles successfully
2. `cargo test` - All unit tests pass
3. `cargo test --doc` - All documentation tests pass
4. `cargo fmt -- --check` - Code is properly formatted
5. `cargo clippy` - No linting issues
6. `cargo tarpaulin` - Code coverage hasn't decreased
7. Review all module-level documentation to ensure it's up-to-date
8. Update CLAUDE.md Architecture section if structure changed

## Git Commit Conventions
- Always run the full pre-commit checklist before committing
- Focus commit messages on the primary functionality (e.g., "Implement XOR instruction" not "Update tests and add XOR")
- Ask for user confirmation before committing to ensure accuracy

## Development Tracking
- **@docs/ROADMAP.md**: Contains the roadmap of planned features and implementation status
- When implementing features or making changes to the codebase, update the relevant project document linked from ROADMAP.md to reflect the current status
- Mark individual tasks as "Completed" (✅) when fully implemented and tested
- Update project status icons in both the project file header and ROADMAP.md:
  - Use ✅ (Complete) when ALL tasks in a project are marked with ✅
  - Use 🚧 (In Progress) when any tasks are marked with 🚧 or a mix of ✅ and 📋
  - Use 📋 (Planned) when all tasks are marked with 📋
- Add notes about any deviations from the original plan or additional features discovered during implementation