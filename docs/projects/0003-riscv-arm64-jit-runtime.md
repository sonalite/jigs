# Project 0003: RISC-V to ARM64 JIT Runtime 📋

## Overview
Implementation of a Just-In-Time (JIT) compiler runtime that translates RISC-V machine code to native ARM64 instructions and executes them. This enables running RISC-V programs directly on ARM64 hardware with near-native performance using a single-pass compilation strategy.

## Architecture

### Design Principles
- **Single-pass compilation**: Direct RISC-V to ARM64 translation, maximizes compilation speed
- **Fixed allocations**: All memory allocated in `new()`, no runtime allocations for predictable performance
- **Direct register mapping**: RISC-V registers live in ARM64 hardware registers for maximum performance
- **Direct execution**: Compiled code runs natively without interpretation overhead
- **x30 special case**: Preserves ARM64 link register functionality via memory storage
- **Separate spill stack**: Keeps VM memory space clean and predictable
- **Generic syscall handler**: Avoids dynamic dispatch overhead
- **PC mapping table**: Enables efficient indirect jump handling
- **Stubbed implementation**: Allows incremental development and testing

### Register Mapping (RISC-V → ARM64)
- **x0**: Always zero (uses ARM64 xzr when needed)
- **x1-x29**: Direct 1:1 mapping to ARM64 x1-x29
- **x30**: Stored as `Box<u32>` in memory, spilled on write, loaded on read (preserves ARM64 link register)
- **x31**: Maps to ARM64 x31 (normal register)
- **ARM64 xzr**: Only used when instructions explicitly need zero

### Memory Layout

#### Page-Based Memory System
- **Address Space**: 32-bit RISC-V address space
- **Page Size**: 4KB (2^12 bytes)
- **Address Split**: High 20 bits = page number, Low 12 bits = page offset
- **Page Table**: 2MB fixed array (2^20 entries × 2 bytes per entry)
- **Page Table Entry**: 16-bit index into page array (supports up to 65,536 pages = 256MB total)
- **Page Permissions**: Read/write only (no execute flag needed for VM memory)
- **Memory Object**: Stored as `Box<Memory>` so native ARM64 code can access via direct pointer

#### Other Memory Components
- **Code Buffer**: Fixed size for JIT-compiled ARM64 code, made executable, tracks emission position
- **Spill Stack**: Separate from VM memory, VM tracks stack depth with max size, native code increments/checks bounds
- **x30 Storage**: `Box<u32>` with direct memory address accessible from JIT code
- **PC Mapping Table**: RISC-V PC to ARM64 code offset mapping for indirect jumps

### Program Counter (PC) Management
- PC not stored as register during execution
- Compilation maintains RISC-V PC → ARM64 code offset mapping
- Direct branches use compile-time offset calculation
- Indirect jumps (JALR) use runtime PC lookup table
- PC values always 4-byte aligned

### System Instructions
- **ECALL**: Generates ARM64 sequence to:
  1. Save all RISC-V registers to spill stack
  2. Set x0 = VM pointer, x1 = syscall number (from a7)
  3. Call syscall handler following ARM64 ABI
  4. Restore all registers from spill stack
- **EBREAK**: Treated as NOP (or optionally halt)

### Virtual Machine (`src/vm.rs`)
- Generic syscall handler type: `S: Fn(&mut VirtualMachine<S>, u32) -> Result<(), RuntimeError>`
- x30 as `Box<u32>` for direct memory access
- Memory system as `Box<Memory>` with pointer passed to native code
- Fixed-size code buffer and memory allocations
- PC to code offset mapping table
- No RISC-V register storage (except x30)

### Memory Management (`src/memory.rs`)
- **Memory Struct**: Stored in `Box<Memory>` for stable pointer access from native code
- **Page Table**: 2MB array of 2^20 u16 entries, each indexing into page array
- **Page Structure**: 
  - 4KB data buffer for actual memory contents
  - Start address field for remapping/reset functionality
  - No additional flags (pages are always read/write)
- **Page Pool**: Pre-allocated array of pages to avoid runtime allocation
- **Active Tracking**: Track which pages are actually mapped/in-use
- **Memory Operations**:
  - Native ARM64 code directly accesses memory via pointer
  - All page lookups and manipulation done by JIT-compiled code
  - Bounds-checked read/write helper methods for VM initialization
- **Reset Functionality**: Clear mapped pages and reset page table between executions
- **Sparse Mapping**: Only allocate pages that are actually accessed (lazy allocation)

### ARM64 Code Generation (`src/arm64/`)
- **`mod.rs`**: Module organization, register constants, types
- **`encoder.rs`**: Instruction encoding helpers, register/immediate encoding, branch offsets
- **`emitter.rs`**: Code emission to fixed buffer, write position tracking, forward branch patching

### Compiler (`src/compiler.rs`)
- Tracks current RISC-V PC during compilation
- Maintains PC to ARM64 offset mapping
- Forward branch fixup list
- Single-pass translation flow
- Pointer to x30 storage and spill stack

### Translator (`src/translator.rs`)
- Per-instruction translation methods (initially stubbed returning `NotImplemented`)
- Special handling for x0 (zero), x30 (memory access), branches, JALR, ECALL/EBREAK

### Executor (`src/executor.rs`)
- JIT compilation on first function call
- Compilation cache using PC to code pointer mapping
- VM entry/exit sequences with ARM64 register save/restore
- Direct jump to compiled code

### Public API
```rust
pub fn new<S>(memory_size: usize, code_buffer_size: usize, syscall_handler: S) -> Self
    where S: Fn(&mut VirtualMachine<S>, u32) -> Result<(), RuntimeError>

pub fn load_program(&mut self, code: &[u8], address: u32)
pub fn call_function(&mut self, address: u32, args: &[u32]) -> Result<u32>
pub fn read_register(&self, reg: u8) -> u32
pub fn write_register(&mut self, reg: u8, value: u32)
pub fn read_memory(&self, address: u32, size: usize) -> Result<Vec<u8>>
pub fn write_memory(&mut self, address: u32, data: &[u8]) -> Result<()>
pub fn run(&mut self) -> Result<RunResult>
```

### Testing Structure
```
src/tests/vm/
├── vm.rs              # VM creation and API
├── registers.rs       # Register operations, x30 special handling
├── memory.rs          # Memory operations and boundaries
├── performance.rs     # Performance benchmarks
├── programs.rs        # Complete program execution
└── instructions/      # Per-instruction test coverage
    ├── register/      # R-type instructions
    ├── immediate/     # I-type instructions
    ├── load/          # Load instructions with alignment
    ├── store/         # Store instructions with alignment
    ├── branch/        # Branch instructions with PC mapping
    ├── upper/         # U-type (LUI, AUIPC)
    ├── jump/          # Jump instructions including JALR
    ├── multiply/      # M extension
    └── system/        # ECALL and EBREAK
```

Each instruction requires full test coverage including basic operation, edge cases, x0 immutability, x30 spill/reload, PC alignment, and 100% code coverage.

### Edge Cases
- PC alignment violations (4-byte aligned)
- Self-modifying code (cache invalidation)
- Memory page boundaries
- Invalid/undefined opcodes
- Register hazards and dependencies
- Stack overflow scenarios
- Unaligned memory access
- Maximum branch distances
- Code buffer exhaustion
- Nested syscalls

### Performance Considerations
- Minimize register spilling
- Optimize common instruction sequences
- Cache compiled code efficiently
- Use direct jumps where possible
- Minimize syscall overhead
- Consider instruction fusion (future optimization)

## TODO
*Tasks to be defined after plan iteration and refinement*