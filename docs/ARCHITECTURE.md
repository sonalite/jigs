# Architecture

The project is structured as a Rust library with an example binary.

## Current Modules

### `src/lib.rs`
Library entry point that exports public APIs

### `src/main.rs`
Example binary demonstrating instruction decoding

### `src/instruction.rs`
Core instruction representation, decoding, and encoding logic (implemented)
- `Instruction` enum with variants for each RISC-V instruction (Add, Sub, etc.)
- `decode()` method that extracts fields from 32-bit instruction words using bitmasking
- `encode()` method that converts Instruction variants back to 32-bit instruction words
- Display trait implementation for assembly-style output
- `EncodeError` enum for encoding error handling (InvalidRegister, InvalidImmediate, NotImplemented)
- Supports RV32IM: base integer instructions plus M extension (multiply/divide)

### `src/memory.rs`
Page-based memory system (implemented)
- 32-bit RISC-V address space with 16KB pages (2^14 bytes)
- Two-level page table hierarchy:
  - L1 table: 256 entries (bits 31-24 = 8 bits)
  - L2 tables: 1024 entries each (bits 23-14 = 10 bits)
  - Page offset: bits 13-0 (16KB pages)
- Page table entry: 16-bit index into global page pool (supports 65,536 pages = 1GB total)
- Global PageStore: Pre-allocated page pool shared across all instances
- Memory struct stored as `Box<Memory>` for stable pointer access from native code
- Sparse allocation with lazy page allocation
- Page structure: 16KB data buffer
- Memory operations: `read()` and `write()` for arbitrary buffer access
- Reset functionality: Return pages to global pool and clear page table
- Direct pointer access from native ARM64 code (planned)

### `src/module.rs`
Compiled ARM64 code module (partially implemented)
- Fixed-size code buffer for compiled ARM64 instructions (allocated with MAP_JIT on macOS)
- Instance count tracking to prevent dropping while instances attached
- Memory pointer storage (`Box<*mut Memory>`) for attached instance's memory
- Public API: `new()`, `set_code()` (compilation stub)
- Planned: PC to code offset mapping table, code compilation, memory protection

### `src/instance.rs`
Runtime instance for executing a compiled Module (partially implemented)
- Module attachment/detachment with reference counting
- Memory system as `Box<Memory>` with stable pointer for native code
- Public API: `new()`, `attach()`, `detach()`, `attached()`, `memory()`, `memory_mut()`
- Planned: x30 storage, spill stack, syscall handler, execution methods

## Current Modules (continued)

### `src/arm64.rs`
ARM64 instruction encoding for AOT compilation (partially implemented)
- ARM64 machine code generation helpers
- ARM64 instruction format constants (RET instruction implemented)
- Planned: Register and immediate encoding utilities
- Planned: Branch offset calculations

### `src/compiler.rs`
AOT compiler managing RISC-V to ARM64 translation (partially implemented)
- Compiles RISC-V instructions to ARM64 machine code
- Accepts external buffer for code emission
- Currently emits single RET instruction for all inputs (stub implementation)
- Planned: PC tracking and RISC-V PC to ARM64 offset mapping
- Planned: Branch patching with forward branch fixup list
- Planned: Full instruction translation via translator module

## Planned Modules

### `src/translator.rs`
Per-instruction RISC-V to ARM64 translation logic
- Translation methods for each RISC-V instruction (initially stubbed)
- Special handling for x0 (zero), x30 (memory access)
- Branch and JALR translation with PC lookup
- ECALL/EBREAK system instruction handling
- Returns ARM64 instruction sequences for compiler to emit




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

#### `memory/`
Memory system tests (implemented)
- PageStore creation, limits, and drop behavior
- Memory struct creation and management
- Page allocation (single, multiple, L2 tables)
- Memory reset and reallocation
- Page boundary handling
- Stress tests and edge cases

#### `module/`
Module tests (partially implemented)
- Module creation and memory allocation
- Instance tracking and drop protection
- Code size validation

#### `instance/`
Instance tests (partially implemented)
- Instance creation and module attachment
- Memory integration

#### `arm64/`
ARM64 encoder tests (planned)
- Instruction encoding tests
- Register encoding tests
- Immediate value encoding tests

#### `compiler/`
Compiler tests (partially implemented)
- Basic RET compilation test
- Buffer management tests
- Multiple instruction compilation tests

#### Planned Test Modules
- `translator/` - Translator tests
- `integration/` - Combined module+instance integration tests