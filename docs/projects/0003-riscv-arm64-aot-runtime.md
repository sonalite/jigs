# Project 0003: RISC-V to ARM64 AOT Runtime 📋

## Overview
Implementation of an Ahead-of-Time (AOT) compiler runtime that translates RISC-V machine code to native ARM64 instructions and executes them. This enables running RISC-V programs directly on ARM64 hardware with near-native performance using a single-pass compilation strategy. Programs are compiled when loaded, not during execution.

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
- **Code Buffer**: Fixed size for AOT-compiled ARM64 code, made executable, tracks emission position
- **Spill Stack**: Separate from VM memory, VM tracks stack depth with max size, native code increments/checks bounds
- **x30 Storage**: `Box<u32>` with direct memory address accessible from compiled code
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
  - All page lookups and manipulation done by AOT-compiled code
  - Bounds-checked read/write helper methods for VM initialization
- **Reset Functionality**: Clear mapped pages and reset page table between executions
- **Sparse Mapping**: Only allocate pages that are actually accessed (lazy allocation)

### ARM64 Encoder (`src/encoder.rs`)
- Instruction encoding helpers for ARM64 machine code generation
- Register and immediate value encoding
- Branch offset calculation and encoding
- ARM64 instruction format constants and utilities

### Compiler (`src/compiler.rs`)
- **Compilation orchestration**: Manages single-pass RISC-V to ARM64 translation
- **Code emission**: Writes ARM64 instructions directly to fixed code buffer
- **PC tracking**: Maintains current RISC-V PC and PC to ARM64 offset mapping
- **Branch patching**: Forward branch fixup list and resolution
- **Buffer management**: Write position tracking and bounds checking
- **Special registers**: Pointer to x30 storage and spill stack management
- Calls translator for per-instruction translation logic

### Translator (`src/translator.rs`)
- Per-instruction translation methods (initially stubbed returning `NotImplemented`)
- Special handling for x0 (zero), x30 (memory access), branches, JALR, ECALL/EBREAK
- Returns ARM64 instruction sequences for compiler to emit

### Function Call Mechanism (`call_function`)
- **Entry**: Save ARM64 callee-saved registers (x19-x28, x29, x30) and stack pointer
- **Execute**: Jump to compiled code at target address
- **Exit**: Restore all saved ARM64 registers and stack pointer, return a0 value

### Public API
```rust
pub fn new<S>(memory_size: usize, code_buffer_size: usize, syscall_handler: S) -> Self
    where S: Fn(&mut VirtualMachine<S>, u32) -> Result<(), RuntimeError>

pub fn load_program(&mut self, code: &[u8], address: u32)  // Compiles RISC-V to ARM64
pub fn call_function(&mut self, address: u32, args: &[u32]) -> Result<u32>  // Executes compiled code
pub fn read_register(&self, reg: u8) -> u32
pub fn write_register(&mut self, reg: u8, value: u32)
pub fn read_memory(&self, address: u32, size: usize) -> Result<Vec<u8>>
pub fn write_memory(&mut self, address: u32, data: &[u8]) -> Result<()>
pub fn run(&mut self) -> Result<RunResult>
```

### Testing

Coverage must be maintained at 100% for all new files.

#### Structure

```
src/tests/
├── encoder/           # ARM64 encoder module tests
│   ├── instructions/  # ARM64 instruction encoding
│   │   ├── arithmetic.rs  # ADD, SUB, MUL, etc.
│   │   ├── logical.rs     # AND, ORR, EOR, MVN
│   │   ├── shifts.rs      # LSL, LSR, ASR, ROR
│   │   ├── branches.rs    # B, BL, BR, BLR, RET
│   │   ├── loads.rs       # LDR, LDRB, LDRH, LDRSW
│   │   ├── stores.rs      # STR, STRB, STRH
│   │   └── moves.rs       # MOV, MOVZ, MOVK, MOVN
│   ├── registers.rs   # Register encoding (X0-X31, SP, XZR)
│   ├── immediates.rs  # Immediate value encoding and validation
│   └── offsets.rs     # Branch offset calculations
│
├── memory/            # Memory system tests
│   ├── pages.rs       # Page allocation and management
│   ├── table.rs       # Page table operations
│   ├── sparse.rs      # Sparse allocation tests
│   ├── boundaries.rs  # Page boundary handling
│   ├── reset.rs       # Memory reset functionality
│   └── stress.rs      # Memory stress tests
│
├── compiler/          # Compiler module tests
│   ├── emission.rs    # Code emission and buffer management
│   ├── pc_mapping.rs  # PC to ARM64 offset mapping
│   ├── branches.rs    # Branch patching and forward references
│   ├── buffer.rs      # Code buffer bounds and overflow
│   ├── x30_handling.rs # Special x30 register compilation
│   └── errors.rs      # Compilation error handling
│
├── translator/        # Translator module tests
│   ├── stubs.rs       # Initial stubbed implementation (temporary, removed when fully implemented)
│   ├── register_mapping.rs  # RISC-V to ARM64 register mapping
│   ├── zero_register.rs     # x0 special handling
│   ├── x30_spill.rs        # x30 memory spill/reload
│   └── instruction_sequences.rs  # Instruction translation patterns
│
└── vm/                # Virtual machine tests
    ├── creation.rs    # VM instantiation and initialization
    ├── api.rs         # Public API surface tests
    ├── registers.rs   # Register read/write operations
    ├── memory.rs      # Memory read/write operations
    ├── syscalls.rs    # Syscall handler integration
    ├── execution.rs   # call_function mechanism
    ├── programs/      # Complete program execution tests
    │   ├── simple.rs      # Basic arithmetic programs
    │   ├── loops.rs       # Loop constructs
    │   ├── functions.rs   # Function calls and returns
    │   ├── recursive.rs   # Recursive functions
    │   ├── syscalls.rs    # Programs using syscalls
    │   └── stress.rs      # Performance stress tests
    └── instructions/  # Per-instruction VM integration tests
        ├── register/      # R-type instructions
        │   ├── add.rs     # ADD with all register combinations
        │   ├── sub.rs     # SUB with overflow cases
        │   ├── and.rs     # AND logical operations
        │   ├── or.rs      # OR logical operations
        │   ├── xor.rs     # XOR logical operations
        │   ├── sll.rs     # Shift left logical
        │   ├── srl.rs     # Shift right logical
        │   ├── sra.rs     # Shift right arithmetic
        │   ├── slt.rs     # Set less than
        │   └── sltu.rs    # Set less than unsigned
        ├── immediate/     # I-type instructions
        │   ├── addi.rs    # ADDI with immediate bounds
        │   ├── andi.rs    # ANDI with bit patterns
        │   ├── ori.rs     # ORI with bit patterns
        │   ├── xori.rs    # XORI with bit patterns
        │   ├── slli.rs    # SLLI shift amounts
        │   ├── srli.rs    # SRLI shift amounts
        │   ├── srai.rs    # SRAI with sign extension
        │   ├── slti.rs    # SLTI comparisons
        │   └── sltiu.rs   # SLTIU unsigned comparisons
        ├── load/          # Load instructions
        │   ├── lb.rs      # LB with sign extension
        │   ├── lh.rs      # LH with alignment
        │   ├── lw.rs      # LW with page boundaries
        │   ├── lbu.rs     # LBU zero extension
        │   └── lhu.rs     # LHU zero extension
        ├── store/         # Store instructions
        │   ├── sb.rs      # SB byte stores
        │   ├── sh.rs      # SH halfword alignment
        │   └── sw.rs      # SW word alignment
        ├── branch/        # Branch instructions
        │   ├── beq.rs     # BEQ with PC updates
        │   ├── bne.rs     # BNE branch conditions
        │   ├── blt.rs     # BLT signed comparisons
        │   ├── bge.rs     # BGE signed comparisons
        │   ├── bltu.rs    # BLTU unsigned comparisons
        │   └── bgeu.rs    # BGEU unsigned comparisons
        ├── upper/         # U-type instructions
        │   ├── lui.rs     # LUI upper immediate
        │   └── auipc.rs   # AUIPC PC-relative
        ├── jump/          # Jump instructions
        │   ├── jal.rs     # JAL direct jumps
        │   └── jalr.rs    # JALR indirect jumps with PC lookup
        ├── multiply/      # M extension
        │   ├── mul.rs     # MUL multiplication
        │   ├── mulh.rs    # MULH high bits signed
        │   ├── mulhsu.rs  # MULHSU mixed sign
        │   ├── mulhu.rs   # MULHU high bits unsigned
        │   ├── div.rs     # DIV signed division
        │   ├── divu.rs    # DIVU unsigned division
        │   ├── rem.rs     # REM signed remainder
        │   └── remu.rs    # REMU unsigned remainder
        └── system/        # System instructions
            ├── ecall.rs   # ECALL syscall mechanism
            └── ebreak.rs  # EBREAK handling
```

## TODO
*Tasks to be defined after plan iteration and refinement*