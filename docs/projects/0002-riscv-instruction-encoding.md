# Project 0002: RISC-V 32-bit IM Instruction Encoding 📋

### Overview
Implementation of RISC-V 32-bit instruction encoder to convert Instruction enum variants into 32-bit instruction words. This complements the decoder by enabling bidirectional conversion between instruction representations and machine code.

## Tasks

### Instruction Infrastructure 📋
- 📋 Add encode() method to Instruction enum
- 📋 Create encode module with helper functions for different instruction formats
- 📋 Implement field packing functions for R, I, S, B, U, and J formats
- 📋 Add comprehensive test suite structure (encode tests)

### R-Type Instruction Encoding 📋
- 📋 ADD instruction
- 📋 SUB instruction  
- 📋 SLL instruction
- 📋 SLT instruction
- 📋 SLTU instruction
- 📋 XOR instruction
- 📋 SRL instruction
- 📋 SRA instruction
- 📋 OR instruction
- 📋 AND instruction

### I-Type Instruction Encoding 📋
- 📋 ADDI instruction
- 📋 SLTI instruction
- 📋 SLTIU instruction
- 📋 XORI instruction
- 📋 ORI instruction
- 📋 ANDI instruction
- 📋 SLLI instruction
- 📋 SRLI instruction
- 📋 SRAI instruction

### Load Instruction Encoding 📋
- 📋 LB instruction
- 📋 LH instruction
- 📋 LW instruction
- 📋 LBU instruction
- 📋 LHU instruction

### Store Instruction Encoding 📋
- 📋 SB instruction
- 📋 SH instruction
- 📋 SW instruction

### Branch Instruction Encoding 📋
- 📋 BEQ instruction
- 📋 BNE instruction
- 📋 BLT instruction
- 📋 BGE instruction
- 📋 BLTU instruction
- 📋 BGEU instruction

### Jump Instruction Encoding 📋
- 📋 JAL instruction
- 📋 JALR instruction

### U-Type Instruction Encoding 📋
- 📋 LUI instruction
- 📋 AUIPC instruction

### System Instruction Encoding 📋
- 📋 ECALL instruction
- 📋 EBREAK instruction

### M Extension Encoding 📋
- 📋 MUL instruction
- 📋 MULH instruction
- 📋 MULHSU instruction
- 📋 MULHU instruction
- 📋 DIV instruction
- 📋 DIVU instruction
- 📋 REM instruction
- 📋 REMU instruction

### Testing & Validation 📋
- 📋 Round-trip tests (encode then decode should match original)
- 📋 Verify encoding matches RISC-V specification test vectors
- 📋 Edge case testing for immediate value ranges
- 📋 100% code coverage maintained

### Integration 📋
- 📋 Add encoding examples to documentation
- 📋 Create helper methods for common encoding patterns
- 📋 Performance optimization for encoding operations

## Design Considerations

### Encoding Strategy
- Each instruction variant implements its own encoding logic
- Immediate values are validated to ensure they fit within bit width constraints
- Sign extension is handled appropriately for each instruction format

### Error Handling
- Return Result<u32, EncodeError> to handle invalid immediate values
- Provide clear error messages for out-of-range immediates
- Consider panic vs Result for internal invariant violations

### Testing Approach
- Every encoded instruction should decode back to the original
- Test boundary values for all immediate fields
- Verify against known instruction encodings from RISC-V spec