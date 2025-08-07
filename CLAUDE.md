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
- Run single test: `cargo test tests::instruction::decode::add::basic`
- Test specific functionality: `cargo test instruction::decode`
- Test with output: `cargo test -- --nocapture`
- Code coverage: `cargo tarpaulin`
- Format code: `cargo fmt`
- Check formatting: `cargo fmt -- --check`
- Lint code: `cargo clippy`

## RISC-V Implementation Details
- Currently supports R-type instructions (register-to-register operations)
- Instruction word format follows standard RISC-V 32-bit encoding
- Field extraction uses bit masks and shifts defined as constants
- Register addresses use x0-x31 notation in display output

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
- Before committing: ensure `cargo build`, `cargo test`, `cargo tarpaulin`, `cargo fmt -- --check`, and `cargo clippy` produce no warnings

## Git Commit Conventions
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