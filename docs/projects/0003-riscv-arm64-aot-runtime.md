# Project 0003: RISC-V to ARM64 AOT Runtime 🚧

## Overview
Implementation of an Ahead-of-Time (AOT) compiler runtime that translates RISC-V machine code to native ARM64 instructions and executes them. This enables running RISC-V programs directly on ARM64 hardware with near-native performance using a single-pass compilation strategy. Programs are compiled when loaded, not during execution.

## Architecture

### Design Principles
- **Single-pass compilation**: Direct RISC-V to ARM64 translation, maximizes compilation speed
- **Module/Instance separation**: Compiled code (Module) is separate from runtime state (Instance), enabling code reuse
- **Dynamic module attachment**: Instances can attach/detach from different modules at runtime for flexibility
- **Fixed allocations**: All memory allocated in `new()`, no runtime allocations for predictable performance
- **Direct register mapping**: RISC-V registers live in ARM64 hardware registers for maximum performance
- **Direct execution**: Compiled code runs natively without interpretation overhead
- **x30 special case**: Preserves ARM64 link register functionality via memory storage
- **Separate spill stack**: Keeps instance memory space clean and predictable
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
- **Page Size**: 16KB (2^14 bytes)
- **Address Split**: Bits 31-24 = L1 index (8 bits), Bits 23-14 = L2 index (10 bits), Bits 13-0 = page offset
- **Page Table**: Two-level hierarchy with L1 table and L2 tables
- **Page Table Entry**: 16-bit index into global page pool (supports up to 65,536 pages = 1GB total)
- **Page Permissions**: Read/write only (no execute flag needed for instance memory)
- **Memory Object**: Stored as `Box<Memory>` so native ARM64 code can access via direct pointer
- **Global Page Pool**: Shared across all instances for efficient memory management

#### Other Memory Components
- **Code Buffer**: Fixed size for AOT-compiled ARM64 code, made executable, tracks emission position
- **Spill Stack**: Separate from instance memory, instance tracks stack depth with max size, native code increments/checks bounds
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
  2. Set x0 = instance pointer, x1 = syscall number (from a7)
  3. Call syscall handler following ARM64 ABI
  4. Restore all registers from spill stack
- **EBREAK**: Treated as NOP (or optionally halt)

### Module (`src/module.rs`)
- Contains compiled ARM64 code in fixed-size buffer
- PC to code offset mapping table for indirect jumps
- Stores `Box<u64>` pointer to active Instance's memory (set when Instance calls)
- Since runtime is single-threaded, only one Instance runs at a time
- Compiled code can directly access memory via this fixed pointer
- Immutable after compilation (except for memory pointer update)
- Code buffer made executable after compilation
- Tracks count of attached instances (prevents dropping while instances attached)

### Instance (`src/instance.rs`)
- Runtime state for executing a compiled Module
- Can be dynamically attached/detached from different modules
- x30 as `Box<u32>` for direct memory access
- Memory system as `Box<Memory>` with stable pointer for native code access
- Sets Module's memory pointer to its Memory before execution
- Spill stack for register save/restore during syscalls
- Pointer to Module for code execution
- Generic syscall handler type: `S: Fn(&mut Instance<S>, u32) -> Result<(), RuntimeError>`
- No RISC-V register storage (except x30) - registers live in ARM64 hardware

### Memory Management (`src/memory.rs`)
- **Global PageStore**: Pre-allocated page pool shared across all instances
- **Memory Struct**: Stored in `Box<Memory>` for stable pointer access from native code
  - Contains page table and references to allocated pages from global pool
- **Page Table**: Two-level hierarchy - L1 table points to L2 tables, L2 entries index into global page pool
- **Page Structure**: 16KB data buffer for actual memory contents
- **Memory Operations**:
  - Native ARM64 code directly accesses memory via pointer
  - Native ARM64 code calls Memory's allocate_page method when page table lookup finds unallocated page
- **Reset Functionality**: Return pages to global pool and clear page table
- **Sparse Mapping**: Only allocate pages that are actually accessed (lazy allocation)

### ARM64 Encoder (`src/encoder.rs`)
- Instruction encoding helpers for ARM64 machine code generation
- Register and immediate value encoding
- Branch offset calculation and encoding
- ARM64 instruction format constants and utilities

### Compiler (`src/compiler.rs`)
- **Compilation orchestration**: Manages single-pass RISC-V to ARM64 translation
- **Code emission**: Writes ARM64 instructions directly to Module's code buffer
- **PC tracking**: Maintains current RISC-V PC and PC to ARM64 offset mapping
- **Branch patching**: Forward branch fixup list and resolution
- **Buffer management**: Write position tracking and bounds checking
- **Module generation**: Creates immutable Module with compiled code and metadata
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

**Note**: The APIs shown below represent the planned interface. The actual implementation may differ as the project evolves.

#### Module API
```rust
impl Module {
    pub fn new(max_code_size: usize) -> Result<Module>  // Create a new Module
    pub fn set_code(&mut self, code: &[u8]) -> Result<()>  // Set and compile RISC-V code
}
```

#### Instance API
```rust
impl Instance {
    pub fn new() -> Self  // Create unattached instance
    pub fn attach(&mut self, module: &mut Module)  // Attach to a module (auto-detaches from previous)
    pub fn detach(&mut self)  // Detach from current module
    pub fn attached(&self) -> bool  // Check if attached to a module
    pub fn memory(&self) -> &Memory  // Get reference to instance's memory
    pub fn memory_mut(&mut self) -> &mut Memory  // Get mutable reference to instance's memory
    pub fn call_function(&mut self, address: u32, args: &[u32]) -> Result<u32>  // Executes compiled code
    pub fn read_register(&self, reg: u8) -> u32
    pub fn write_register(&mut self, reg: u8, value: u32)
    pub fn run(&mut self) -> Result<RunResult>
    pub fn reset(&mut self)  // Reset instance state while keeping the module
}
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
├── memory/            # Memory system tests ✅
│   ├── pagestore.rs   # PageStore tests (creation, limits, drop)
│   ├── memory.rs      # Memory struct tests (creation, debug, limits)
│   ├── allocation.rs  # Page allocation tests (single, multiple, L2 tables)
│   ├── reset.rs       # Memory reset tests (cleanup, reallocation)
│   ├── boundaries.rs  # Page boundary tests (address limits, L1/L2 indices)
│   ├── stress.rs      # Stress tests (many pages, cycles, multiple instances)
│   └── edge_cases.rs  # Edge case tests (zero capacity, exact limits)
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
├── module/            # Module compilation tests
│   ├── creation.rs    # Module instantiation and compilation
│   ├── api.rs         # Module public API tests
│   ├── compilation.rs # RISC-V to ARM64 compilation tests
│   ├── metadata.rs    # PC mapping and compilation metadata
│   └── reuse.rs       # Module sharing across instances
│
├── instance/          # Instance runtime tests
│   ├── creation.rs    # Instance instantiation with module
│   ├── api.rs         # Instance public API surface tests
│   ├── registers.rs   # Register read/write operations
│   ├── memory.rs      # Memory read/write operations
│   ├── syscalls.rs    # Syscall handler integration
│   ├── execution.rs   # call_function mechanism
│   ├── reset.rs       # Instance reset functionality
│   └── multi.rs       # Multiple instances sharing a module
│
└── integration/       # Combined module+instance tests
    ├── programs/      # Complete program execution tests
    │   ├── simple.rs      # Basic arithmetic programs
    │   ├── loops.rs       # Loop constructs
    │   ├── functions.rs   # Function calls and returns
    │   ├── recursive.rs   # Recursive functions
    │   ├── syscalls.rs    # Programs using syscalls
    │   └── stress.rs      # Performance stress tests
    └── instructions/  # Per-instruction instance integration tests
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

## Tasks

### Phase 1: Foundation Infrastructure 📋

#### Memory System ✅
- ✅ Global PageStore - Create static PageStore with pre-allocated page pool
- ✅ Memory struct and page table - Create Memory struct with page table array referencing global pool
- ✅ Page allocation and management - Implement lazy page allocation from global pool with tests
- ✅ Memory reset functionality - Return pages to global pool and clear page table with tests
- ✅ Memory boundary tests - Test page boundaries, sparse allocation, stress tests
- ✅ Memory test coverage - Achieved 100% test coverage for memory.rs
- ✅ Buffer read - Read arbitrary buffer from address, fill with zeros for unallocated pages
- ✅ Buffer write - Write arbitrary buffer to address with page allocation as needed

#### Module Core 🚧
- ✅ Module struct with instance tracking - Create Module struct with code buffer and instance count tracking to detect if dropped with instances
- ✅ Module drop protection - Implement drop checks that prevent module from being dropped while instances are attached
- ✅ Memory pointer for attached instance - Add Box<*mut Memory> to store pointer to attached instance's memory (pointer to pointer for swappability)
- ✅ Define ARM64_CODE_SIZE_MULTIPLIER constant - Define constant for maximum ARM64 code size as multiple of RISC-V code size
- ✅ Calculate code buffer size - Accept max_code_size parameter in Module::new, multiply by ARM64_CODE_SIZE_MULTIPLIER constant
- ✅ Create executable memory - Initially allocate code buffer with PROT_READ | PROT_WRITE permissions and MAP_JIT flag for macOS
- ✅ Module reusability - Add Module::new() and Module::set_code() to allow reusing modules with different code
- 📋 Revisit ARM64_CODE_SIZE_MULTIPLIER - After implementing all compiler instructions, revisit the multiplier value for correctness based on actual expansion ratios
- 📋 Basic Module::set_code stub - Enhance Module::set_code to do actual compilation
- 📋 Mark memory as executable - After compilation, change permissions to PROT_READ | PROT_EXEC using mprotect
- 📋 PC mapping table - Add PC to code offset mapping table to Module

#### Instance Core 🚧
- ✅ Instance struct creation - Create Instance struct with ability to attach/detach from modules
- ✅ Module attachment - Implement attach/detach methods with proper reference counting on module
- ✅ Instance memory integration - Add Memory struct to Instance with Box<Memory> for stable pointer
- 📋 x30 storage setup - Add Box<u32> for x30 register storage in Instance
- 📋 Spill stack allocation - Add spill stack for register save/restore during syscalls
- 📋 Register read/write API - Implement read_register/write_register methods with x30 special handling and tests
- 📋 Instance public API tests - Test all public methods and error cases
- 📋 Instance reset functionality - Reset instance state while keeping module with tests

#### ARM64 Encoder Foundation 📋
- 📋 Create encoder.rs module - Create empty encoder.rs file and add to lib.rs
- 📋 ARM64 instruction format constants - Add instruction format constants and masks
- 📋 Basic register encoding - Implement encoding for X0-X31 registers
- 📋 Special register encoding - Add support for SP and XZR registers
- 📋 Immediate encoding - Add immediate value encoding and validation with tests
- 📋 Branch offset encoding - Implement branch offset calculations with tests

#### Barebones Compiler and Execution 📋
- 📋 Minimal compiler setup - Create basic Compiler struct that can emit RET instruction
- 📋 RET instruction encoding - Implement ARM64 RET instruction encoding in encoder
- 📋 Make code buffer executable - Set up mmap with PROT_EXEC for ARM64 code execution
- 📋 Basic Module::set_code - Implement minimal set_code() that compiles single RET instruction
- 📋 Memory pointer setup - Add logic to set Module's memory pointer before execution
- 📋 Basic call_function - Implement minimal call_function that saves registers, jumps to code, restores registers
- 📋 First execution test - Test call_function with simple RET that returns immediately

#### Translator Foundation 📋
- 📋 Translator module - Create translator.rs with translate_instruction dispatch and tests
- 📋 Stub all instructions - Add stub methods returning NotImplemented for all RISC-V instructions with tests
- 📋 Translation result type - Define structure for returning ARM64 instruction sequences

#### Compiler Foundation 📋
- 📋 Basic Compiler struct - Create minimal Compiler struct with just code buffer and write position
- 📋 Code buffer management - Add buffer bounds checking and write position tracking
- 📋 Basic code emission - Implement emit_u32 method to write ARM64 instructions to buffer
- 📋 PC tracking - Add RISC-V PC tracking and current PC management
- 📋 PC mapping table - Implement RISC-V PC to ARM64 offset mapping table
- 📋 Branch fixup list - Add forward branch fixup list structure
- 📋 Branch patching - Implement branch resolution and patching logic
- 📋 Compiler error handling - Buffer overflow, invalid instructions with tests
- 📋 Spill stack management - Track stack depth, bounds checking with tests
- 📋 x30 special handling - Compiler support for x30 spill/reload sequences with tests
- 📋 Translator integration - Call translator and emit returned ARM64 instructions
- 📋 Memory access emission - Helper to emit calls to ARM64 memory access routines

### Phase 2: Minimal Execution Path 📋

#### Essential ARM64 Instructions 📋
- 📋 MOV instruction - Implement ARM64 MOV register-to-register encoding with tests
- 📋 MOVZ instruction - Implement ARM64 MOVZ for loading immediates with tests
- 📋 BR instruction - Implement ARM64 BR (branch register) encoding with tests
- 📋 RET instruction - Implement ARM64 RET encoding with tests (if not done in barebones)
- 📋 LDR instruction - Implement ARM64 LDR for loading from memory with tests
- 📋 STR instruction - Implement ARM64 STR for storing to memory with tests
- 📋 ADD immediate - Implement ARM64 ADD with immediate encoding with tests

#### Critical Translations 📋
- 📋 JALR translation - Indirect jump with PC table lookup, essential for RET with tests
- 📋 ADDI translation - ARM64 ADD with immediate (often used with JALR for returns) with tests

#### Execution Support 📋
- 📋 Compiler integration in Module - Wire up Compiler to Module::set_code method
- 📋 Single instruction compilation - Compile a single RISC-V instruction to ARM64
- 📋 Multi-instruction compilation - Extend to compile multiple instructions
- 📋 Register save logic - Implement ARM64 register save (x19-x28, x29, x30, sp)
- 📋 Register restore logic - Implement ARM64 register restore after execution
- 📋 Jump to compiled code - Implement the actual jump to Module's code buffer
- 📋 Return value handling - Extract a0 (x10) as return value from call_function
- 📋 Basic execution test - Test call_function with JALR return
- 📋 Module sharing test - Test multiple instances executing same module

### Phase 3: Memory Access Instructions 📋

#### Load/Store ARM64 Support 📋
- 📋 Memory bounds checking - ARM64 code for address validation with tests
- 📋 Page fault handling - ARM64 code for lazy page allocation with tests
- 📋 Byte/halfword/word access - ARM64 routines for different data sizes with tests

#### Load Translations 📋
- 📋 LW translation - ARM64 LDR using memory access routine with tests
- 📋 LB translation - ARM64 LDRSB using memory access routine with tests
- 📋 LH translation - ARM64 LDRSH using memory access routine with tests
- 📋 LBU translation - ARM64 LDRB using memory access routine with tests
- 📋 LHU translation - ARM64 LDRH using memory access routine with tests

#### Store Translations 📋
- 📋 SW translation - ARM64 STR using memory access routine with tests
- 📋 SB translation - ARM64 STRB using memory access routine with tests
- 📋 SH translation - ARM64 STRH using memory access routine with tests

### Phase 4: Core ARM64 Encoder Instructions 📋

#### Arithmetic and Logical 📋
- 📋 Arithmetic instructions - ADD, SUB, NEG ARM64 encoding with tests
- 📋 Logical instructions - AND, ORR, EOR, MVN ARM64 encoding with tests
- 📋 Shift instructions - LSL, LSR, ASR, ROR ARM64 encoding with tests
- 📋 Compare instructions - CMP, CMN, TST, CSET ARM64 encoding with tests

#### Data Movement 📋
- 📋 Extended move instructions - MOVK, MOVN ARM64 encoding with tests

#### Control Flow 📋
- 📋 Direct branch instructions - B, BL, BLR ARM64 encoding with tests
- 📋 Conditional branches - B.EQ, B.NE, B.LT, B.GE, B.LO, B.HS ARM64 encoding with tests

#### Multiplication and Division 📋
- 📋 Multiply instructions - MUL, SMULL, UMULL ARM64 encoding with tests
- 📋 Division instructions - SDIV, UDIV ARM64 encoding with tests
- 📋 MSUB instruction - MSUB for remainder calculation with tests

### Phase 5: RISC-V Instruction Translation 📋

#### R-Type Instructions 📋
- 📋 ADD translation - Direct ARM64 ADD with register mapping and tests
- 📋 SUB translation - ARM64 SUB instruction with tests
- 📋 AND translation - ARM64 AND instruction with tests
- 📋 OR translation - ARM64 ORR instruction with tests
- 📋 XOR translation - ARM64 EOR instruction with tests
- 📋 SLL translation - ARM64 LSL with register shift and tests
- 📋 SRL translation - ARM64 LSR with register shift and tests
- 📋 SRA translation - ARM64 ASR with register shift and tests
- 📋 SLT translation - CMP and CSET sequence with tests
- 📋 SLTU translation - CMP and CSET for unsigned with tests

#### I-Type Instructions 📋
- 📋 ANDI translation - ARM64 AND with immediate and tests
- 📋 ORI translation - ARM64 ORR with immediate and tests
- 📋 XORI translation - ARM64 EOR with immediate and tests
- 📋 SLLI translation - ARM64 LSL with immediate shift and tests
- 📋 SRLI translation - ARM64 LSR with immediate shift and tests
- 📋 SRAI translation - ARM64 ASR with immediate shift and tests
- 📋 SLTI translation - CMP and CSET with immediate and tests
- 📋 SLTIU translation - Unsigned CMP and CSET with immediate and tests

#### Branch Instructions 📋
- 📋 BEQ translation - ARM64 conditional branch B.EQ with tests
- 📋 BNE translation - ARM64 conditional branch B.NE with tests
- 📋 BLT translation - ARM64 signed comparison B.LT with tests
- 📋 BGE translation - ARM64 signed comparison B.GE with tests
- 📋 BLTU translation - ARM64 unsigned comparison B.LO with tests
- 📋 BGEU translation - ARM64 unsigned comparison B.HS with tests

#### Jump Instructions 📋
- 📋 JAL translation - Direct jump with link register save and tests

#### U-Type Instructions 📋
- 📋 LUI translation - Load upper immediate with MOVZ/MOVK and tests
- 📋 AUIPC translation - PC-relative address calculation with tests

#### M Extension 📋
- 📋 MUL translation - ARM64 MUL instruction with tests
- 📋 MULH translation - ARM64 SMULL high bits with tests
- 📋 MULHSU translation - Mixed sign multiplication with tests
- 📋 MULHU translation - ARM64 UMULL high bits with tests
- 📋 DIV translation - ARM64 SDIV instruction with tests
- 📋 DIVU translation - ARM64 UDIV instruction with tests
- 📋 REM translation - SDIV and MSUB for remainder with tests
- 📋 REMU translation - UDIV and MSUB for remainder with tests

#### System Instructions 📋
- 📋 ECALL translation - Save registers, call syscall handler, restore with tests
- 📋 EBREAK translation - NOP or halt implementation with tests

### Phase 6: Integration and Testing 📋

#### Program Execution Tests 📋
- 📋 Simple arithmetic programs - Test basic arithmetic operations
- 📋 Memory access programs - Test loads/stores with page allocation
- 📋 Loop constructs - Test branch and jump loops
- 📋 Function calls - Test JAL/JALR function call patterns
- 📋 Recursive functions - Stack-based recursion tests
- 📋 Syscall programs - Test ECALL integration
- 📋 Memory boundary operations - Test loads/stores across page boundaries
- 📋 M extension programs - Test multiply/divide operations
- 📋 Performance stress tests - Large program compilation and execution