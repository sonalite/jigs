# Jigs - RISC-V Instruction Decoder

## Completed

### Project Setup ✅ COMPLETE
- [x] Setup basic project structure (cargo init)
- [x] Setup GitHub Actions CI workflow for ARM
- [x] Add codecov integration for test coverage reporting
- [x] Configure project with CLAUDE.md for development standards
- [x] Establish testing conventions and directory structure

### Instruction Infrastructure ✅ COMPLETE
- [x] Implement instruction decoding framework
- [x] Create Instruction enum with Display trait
- [x] Setup bit masking constants for field extraction
- [x] Implement decode() method for 32-bit instruction words
- [x] Create comprehensive test suite structure (decode/display)

### RISC-V R-Type Instructions (Register-to-Register) ✅ COMPLETE
- [x] ADD instruction (funct3=0x0, funct7=0x00)
- [x] SUB instruction (funct3=0x0, funct7=0x20)
- [x] SLL instruction (shift left logical, funct3=0x1, funct7=0x00)
- [x] SLT instruction (set less than, funct3=0x2, funct7=0x00)
- [x] SLTU instruction (set less than unsigned, funct3=0x3, funct7=0x00)
- [x] XOR instruction (funct3=0x4, funct7=0x00)
- [x] SRL instruction (shift right logical, funct3=0x5, funct7=0x00)
- [x] SRA instruction (shift right arithmetic, funct3=0x5, funct7=0x20)
- [x] OR instruction (funct3=0x6, funct7=0x00)
- [x] AND instruction (funct3=0x7, funct7=0x00)

## In Progress

### I-Type Instructions (Immediate)
- [ ] ADDI
- [ ] SLTI
- [ ] SLTIU
- [ ] XORI
- [ ] ORI
- [ ] ANDI
- [ ] SLLI
- [ ] SRLI
- [ ] SRAI

### Load Instructions
- [ ] LB (load byte)
- [ ] LH (load halfword)
- [ ] LW (load word)
- [ ] LBU (load byte unsigned)
- [ ] LHU (load halfword unsigned)

### Store Instructions
- [ ] SB (store byte)
- [ ] SH (store halfword)
- [ ] SW (store word)

### Branch Instructions
- [ ] BEQ (branch equal)
- [ ] BNE (branch not equal)
- [ ] BLT (branch less than)
- [ ] BGE (branch greater equal)
- [ ] BLTU (branch less than unsigned)
- [ ] BGEU (branch greater equal unsigned)

### Jump Instructions
- [ ] JAL (jump and link)
- [ ] JALR (jump and link register)

### U-Type Instructions
- [ ] LUI (load upper immediate)
- [ ] AUIPC (add upper immediate to PC)

### System Instructions
- [ ] ECALL
- [ ] EBREAK

### M Extension (Multiply/Divide)
- [ ] MUL (multiply)
- [ ] MULH (multiply high signed)
- [ ] MULHSU (multiply high signed-unsigned)
- [ ] MULHU (multiply high unsigned)
- [ ] DIV (divide signed)
- [ ] DIVU (divide unsigned)
- [ ] REM (remainder signed)
- [ ] REMU (remainder unsigned)

### Testing & Quality
- [x] 100% code coverage for instruction.rs maintained throughout development
- [x] Comprehensive test suite for each instruction (basic, zero_registers, max_registers, different_registers, wrong_funct7)
- [x] All R-type instructions complete with 92 passing tests
- [ ] Compare implementation against RISC-V spec for correctness