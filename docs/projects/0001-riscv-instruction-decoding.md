# Project 0001: RISC-V 32-bit IM Instruction Decoding 🚧

### Overview
Implementation of RISC-V 32-bit instruction decoder with support for all RV32IM instructions (Integer base + Multiplication extension).

## Tasks

### Instruction Infrastructure ✅
- ✅ Implement instruction decoding framework
- ✅ Create Instruction enum with Display trait
- ✅ Setup bit masking constants for field extraction
- ✅ Implement decode() method for 32-bit instruction words
- ✅ Create comprehensive test suite structure (decode/display)

### RISC-V R-Type Instructions (Register-to-Register) ✅
- ✅ ADD instruction (funct3=0x0, funct7=0x00)
- ✅ SUB instruction (funct3=0x0, funct7=0x20)
- ✅ SLL instruction (shift left logical, funct3=0x1, funct7=0x00)
- ✅ SLT instruction (set less than, funct3=0x2, funct7=0x00)
- ✅ SLTU instruction (set less than unsigned, funct3=0x3, funct7=0x00)
- ✅ XOR instruction (funct3=0x4, funct7=0x00)
- ✅ SRL instruction (shift right logical, funct3=0x5, funct7=0x00)
- ✅ SRA instruction (shift right arithmetic, funct3=0x5, funct7=0x20)
- ✅ OR instruction (funct3=0x6, funct7=0x00)
- ✅ AND instruction (funct3=0x7, funct7=0x00)
- ✅ All R-type instructions complete with 92 passing tests

### I-Type Instructions (Immediate) ✅
- ✅ ADDI (add immediate)
- ✅ SLTI (set less than immediate)
- ✅ SLTIU (set less than immediate unsigned)
- ✅ XORI (XOR immediate)
- ✅ ORI (OR immediate)
- ✅ ANDI (AND immediate)
- ✅ SLLI (shift left logical immediate)
- ✅ SRLI (shift right logical immediate)
- ✅ SRAI (shift right arithmetic immediate)

### Load Instructions ✅
- ✅ LB (load byte)
- ✅ LH (load halfword)
- ✅ LW (load word)
- ✅ LBU (load byte unsigned)
- ✅ LHU (load halfword unsigned)

### Store Instructions ✅
- ✅ SB (store byte)
- ✅ SH (store halfword)
- ✅ SW (store word)

### Branch Instructions ✅
- ✅ BEQ (branch equal)
- ✅ BNE (branch not equal)
- ✅ BLT (branch less than)
- ✅ BGE (branch greater equal)
- ✅ BLTU (branch less than unsigned)
- ✅ BGEU (branch greater equal unsigned)

### Jump Instructions ✅
- ✅ JAL (jump and link)
- ✅ JALR (jump and link register)

### U-Type Instructions ✅
- ✅ LUI (load upper immediate)
- ✅ AUIPC (add upper immediate to PC)

### System Instructions ✅
- ✅ ECALL
- ✅ EBREAK

### M Extension (Multiply/Divide) 📋
- 📋 MUL (multiply)
- 📋 MULH (multiply high signed)
- 📋 MULHSU (multiply high signed-unsigned)
- 📋 MULHU (multiply high unsigned)
- 📋 DIV (divide signed)
- 📋 DIVU (divide unsigned)
- 📋 REM (remainder signed)
- 📋 REMU (remainder unsigned)

### Testing & Quality 🚧
- ✅ 100% code coverage for instruction.rs maintained throughout development
- ✅ Comprehensive test suite for each instruction (basic, zero_registers, max_registers, different_registers, wrong_funct7)
- 📋 Add module-level documentation to instruction.rs
- 📋 Compare implementation against RISC-V spec for correctness
- 📋 Update all project documentation (CLAUDE.md architecture section, ROADMAP.md status)