# Architecture

The project is structured as a Rust library with an example binary.

## Core Modules

### `src/lib.rs`
Library entry point that exports public APIs

### `src/main.rs`
Example binary demonstrating instruction decoding

### `src/instruction.rs`
Core instruction representation, decoding, and encoding logic
- `Instruction` enum with variants for each RISC-V instruction (Add, Sub, etc.)
- `decode()` method that extracts fields from 32-bit instruction words using bitmasking
- `encode()` method that converts Instruction variants back to 32-bit instruction words
- Display trait implementation for assembly-style output
- `EncodeError` enum for encoding error handling (InvalidRegister, InvalidImmediate, NotImplemented)
- Supports RV32IM: base integer instructions plus M extension (multiply/divide)

## Planned Modules

### `src/encoder.rs`
ARM64 instruction encoding for AOT compilation
- ARM64 machine code generation helpers
- Register and immediate encoding utilities
- Branch offset calculations
- ARM64 instruction format constants and utilities

### `src/compiler.rs`
AOT compiler managing RISC-V to ARM64 translation
- Single-pass compilation orchestration
- Direct code emission to fixed-size buffer
- PC tracking and RISC-V PC to ARM64 offset mapping
- Branch patching with forward branch fixup list
- Buffer management with write position tracking
- Special register handling (x30 storage pointer, spill stack)
- Calls translator for per-instruction logic

### `src/translator.rs`
Per-instruction RISC-V to ARM64 translation logic
- Translation methods for each RISC-V instruction (initially stubbed)
- Special handling for x0 (zero), x30 (memory access)
- Branch and JALR translation with PC lookup
- ECALL/EBREAK system instruction handling
- Returns ARM64 instruction sequences for compiler to emit

### `src/vm.rs`
Virtual machine runtime
- Generic syscall handler: `S: Fn(&mut VirtualMachine<S>, u32) -> Result<(), RuntimeError>`
- x30 register stored as `Box<u32>` for memory access
- Memory system as `Box<Memory>` with stable pointer for native code
- Fixed-size code buffer and allocations (no runtime allocation)
- PC to code offset mapping table for indirect jumps
- No RISC-V register storage (except x30) - registers live in ARM64 hardware
- Public API: `new()`, `load_program()`, `call_function()`, `read/write_register()`, `read/write_memory()`, `run()`

### `src/memory.rs`
Page-based memory system
- 32-bit RISC-V address space with 4KB pages
- Page table: 2MB fixed array (2^20 entries × 2 bytes)
- Page table entry: 16-bit index into page array (supports 256MB total)
- Sparse allocation with lazy page allocation
- Page structure: 4KB data buffer + start address field
- Pre-allocated page pool to avoid runtime allocation
- Reset functionality between executions
- Direct pointer access from native ARM64 code

## Test Structure

### `src/tests/`
Unit tests kept separate from code to enable extensive test coverage

#### `instruction/`
RISC-V instruction tests (subfolders contain tests for each instruction type)
- `roundtrip/` - Bidirectional encode+decode tests for all instruction types
- `encode/` - Encoding-specific tests (bounds checking, error handling)
- `decode/` - Remaining decode-only tests for special validation cases
- `display/` - Tests for instruction display formatting
- `error.rs` - Error type tests

#### Planned Test Modules
- `encoder/` - ARM64 encoder tests
- `memory/` - Memory system tests
- `compiler/` - Compiler tests
- `translator/` - Translator tests
- `vm/` - Virtual machine and program execution tests