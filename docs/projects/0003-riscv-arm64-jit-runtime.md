# Project 0003: RISC-V to ARM64 JIT Runtime 📋

## Overview
Implementation of a Just-In-Time (JIT) compiler runtime that translates RISC-V 32-bit IM (Integer + Multiply/Divide) machine code to native ARM64 instructions and executes them. This enables running RISC-V programs directly on ARM64 hardware with near-native performance using a single-pass compilation strategy. The runtime is designed for single-threaded execution only.

## Architecture

### Design Principles
- **Native execution**: Compiled code runs natively without interpretation overhead
- **Direct register mapping**: RISC-V registers live in ARM64 hardware registers for maximum performance
- **Fixed allocations**: All memory allocated in `new()`, no runtime allocations for predictable performance
- **Manual memory page management**: Precise control over page allocations and access
- **Single-pass compilation**: Direct RISC-V to ARM64 translation for predictable compile speed
- **ARM64-only target**: Both RISC architectures with 32 registers enabling direct mapping
- **Single-threaded execution**: Runtime designed for single-threaded operation only
- **PC mapping table**: Enables efficient indirect jump handling

### Public API
```rust
pub fn new<S>(config: Config, syscall_handler: S) -> Result<Self, VmError>
    where S: Fn(&mut VirtualMachine<S>, u32) -> Result<(), RuntimeError>

pub fn load_program(&mut self, code: &[u8], address: u32) -> Result<(), VmError>
pub fn call_function(&mut self, address: u32, args: &[u32]) -> Result<u32, RuntimeError>
pub fn read_register(&self, reg: u8) -> u32
pub fn write_register(&mut self, reg: u8, value: u32)
pub fn read_memory(&self, address: u32, size: usize) -> Result<Vec<u8>, MemoryError>
pub fn write_memory(&mut self, address: u32, data: &[u8]) -> Result<(), MemoryError>
pub fn clear_memory(&mut self)
```

### Register Mapping (RISC-V → ARM64)
- **x0**: Always zero (uses ARM64 xzr when needed)
- **x1-x29**: Direct 1:1 mapping to ARM64 x1-x29
- **x30**: Stored as `Box<u32>` in memory, spilled on write, loaded on read (preserves ARM64 link register)
- **x31**: Maps to ARM64 x31 (normal register)
- **ARM64 xzr**: Only used when instructions explicitly need zero

### Code Generation Security
- **W^X Protection**: Code buffer starts as writable for JIT compilation, then marked execute-only with `mprotect()`
- **MAP_JIT Flag**: On macOS, memory mapped with `MAP_JIT` flag to allow JIT compilation
- **Instruction Alignment**: All ARM64 instructions must be 32-bit (4-byte) aligned in the code buffer
- **Platform Check**: Compile-time `#[cfg(target_arch = "aarch64")]` ensures code only compiles on ARM64

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
- **Spill Stack**: Separate from VM memory, always contains at least one register set at top, VM tracks stack depth with max size, native code increments/checks bounds
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
- Register stack always maintains at least one set of saved registers at the top for `read_register` access

### Compiler (`src/compiler.rs`)
- Tracks current RISC-V PC during compilation
- Maintains PC to ARM64 offset mapping
- Forward branch fixup list
- Single-pass translation flow
- Pointers to x30 storage, spill stack, memory, and VM itself

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
  - Read/write operations compiled directly into native code (no Rust helpers)
- **Reset Functionality**: Clear mapped pages and reset page table between executions
- **Sparse Mapping**: Only acquire pages from pool when actually accessed (lazy mapping)

### ARM64 Code Generation (`src/arm64/`)
- **`mod.rs`**: Module organization, register constants, types
- **`encoder.rs`**: Instruction encoding helpers, register/immediate encoding, branch offsets
- **`emitter.rs`**: Code emission to fixed buffer, write position tracking, forward branch patching

### Translator (`src/translator.rs`)
- Per-instruction translation methods (initially stubbed returning `NotImplemented`)
- Special handling for x0 (zero), x30 (memory access), branches, JALR, ECALL/EBREAK
- **Divide by Zero Handling**: RISC-V DIV/DIVU/REM/REMU instructions require special handling:
  - DIV/DIVU: Returns all bits set (0xFFFFFFFF) when divisor is zero
  - REM/REMU: Returns dividend when divisor is zero
  - ARM64 integer division by zero returns zero (not an exception), so must check explicitly and return correct RISC-V semantics

### Executor (`src/executor.rs`)
- JIT compilation on first function call
- Compilation cache using PC to code pointer mapping
- VM entry/exit sequences with ARM64 register save/restore
- Direct jump to compiled code
- **Register preservation**: Registers saved to spill stack on entry and remain at top of stack after execution for `read_register` access

### VM Configuration
- **Configuration Object**: `Config` struct passed to `new()` containing:
  - `max_memory_pages: u16` - Maximum number of 4KB pages (default: 16384 = 64MB)
  - `max_code_size: usize` - Maximum JIT code buffer size (default: 1MB)
  - `max_stack_depth: usize` - Maximum spill stack depth (default: 64KB)
- **Validation**: Configuration validated at VM creation time

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