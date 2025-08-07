# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Jigs - A RISC-V instruction decoder implementation in Rust. Currently implements decoding and display formatting for 32-bit RISC-V instructions.

## Architecture
The project implements a RISC-V instruction decoder with:
- **src/instruction.rs**: Core instruction representation and decoding logic
  - `Instruction` enum for different instruction types (currently Add and Unsupported)
  - Decode method that extracts fields from 32-bit instruction words using bitmasking
  - Display trait implementation for assembly-style output
- **src/tests/**: Comprehensive test suite organized by functionality
  - `decode/`: Tests for instruction decoding
  - `display/`: Tests for instruction display formatting

## Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Test specific functionality: `cargo test instruction::decode`
- Test with output: `cargo test -- --nocapture`
- Run single test: `cargo test test_add_x1_x2_x3`
- Code coverage: `cargo tarpaulin`

## RISC-V Implementation Details
- Currently supports R-type instructions (register-to-register operations)
- Instruction word format follows standard RISC-V 32-bit encoding
- Field extraction uses bit masks and shifts defined as constants
- Register addresses use x0-x31 notation in display output

## Code Coverage Requirements
- When implementing new features, ensure code coverage improves in modified files
- New code should have test coverage as close to 100% as possible
- Run `cargo tarpaulin` to verify coverage before completing implementation

## Testing Conventions
- Unit tests should live in the `src/tests/` directory
- For small test suites: use a single file (e.g., `src/tests/feature.rs`)
- For large test suites: create a folder with separate files organized by feature (e.g., `src/tests/instruction/decode/add.rs`, `src/tests/instruction/decode/sub.rs`)

## Code Style Conventions
- File ordering: module docs → `mod` declarations → `use` statements → constants → types → implementations
- Code must compile with no warnings (`cargo build` and `cargo test` should produce zero warnings)