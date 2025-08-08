# Project 0002: RISC-V 32-bit IM Instruction Encoding 🚧

### Overview
Implementation of RISC-V 32-bit instruction encoder to convert Instruction enum variants into 32-bit instruction words. This complements the decoder by enabling bidirectional conversion between instruction representations and machine code.

## Tasks

### Instruction Infrastructure ✅
- ✅ Add encode() method to Instruction enum (returns Result<u32, EncodeError>)
- ✅ Create EncodeError type for error handling
- ✅ Add InvalidRegister variant to EncodeError for register bounds checking
- ✅ Add InvalidImmediate variant to EncodeError for immediate bounds checking
- ✅ Implement std::error::Error and std::fmt::Display for EncodeError
- ✅ Create helper function encode_r_type() for R-type instructions (with register bounds checking)
- ✅ Create helper function encode_i_type() for I-type instructions (with register and immediate bounds checking)
- ✅ Create helper function encode_s_type() for S-type instructions (with register and immediate bounds checking)
- ✅ Create helper function encode_b_type() for B-type instructions (with register and immediate bounds checking)
- ✅ Create helper function encode_j_type() for J-type instructions (with register and immediate bounds checking)
- ✅ Create helper function encode_u_type() for U-type instructions (with register and immediate bounds checking)
- ✅ Add comprehensive test suite structure (roundtrip tests in src/tests/instruction/roundtrip/)

### R-Type Instruction Encoding ✅
- ✅ ADD instruction
- ✅ SUB instruction  
- ✅ SLL instruction
- ✅ SLT instruction
- ✅ SLTU instruction
- ✅ XOR instruction
- ✅ SRL instruction
- ✅ SRA instruction
- ✅ OR instruction
- ✅ AND instruction

### I-Type Instruction Encoding ✅
- ✅ ADDI instruction
- ✅ SLTI instruction
- ✅ SLTIU instruction
- ✅ XORI instruction
- ✅ ORI instruction
- ✅ ANDI instruction
- ✅ SLLI instruction
- ✅ SRLI instruction
- ✅ SRAI instruction

### Load Instruction Encoding ✅
- ✅ LB instruction
- ✅ LH instruction
- ✅ LW instruction
- ✅ LBU instruction
- ✅ LHU instruction

### Store Instruction Encoding ✅
- ✅ SB instruction
- ✅ SH instruction
- ✅ SW instruction

### Branch Instruction Encoding ✅
- ✅ BEQ instruction
- ✅ BNE instruction
- ✅ BLT instruction
- ✅ BGE instruction
- ✅ BLTU instruction
- ✅ BGEU instruction

### Jump Instruction Encoding ✅
- ✅ JAL instruction
- ✅ JALR instruction

### U-Type Instruction Encoding ✅
- ✅ LUI instruction
- ✅ AUIPC instruction

### System Instruction Encoding ✅
- ✅ ECALL instruction
- ✅ EBREAK instruction

### M Extension Encoding ✅
- ✅ MUL instruction
- ✅ MULH instruction
- ✅ MULHSU instruction
- ✅ MULHU instruction
- ✅ DIV instruction
- ✅ DIVU instruction
- ✅ REM instruction
- ✅ REMU instruction

### Testing & Validation ✅
- ✅ Create helper function assert_encode_decode() for bidirectional testing
- ✅ Reorganize tests into roundtrip directory for combined encode/decode testing
- ✅ Remove duplicate decode tests that are covered by roundtrip tests
- ✅ Round-trip tests (encode then decode should match original)
- ✅ Register bounds checking tests for R-type instructions (InvalidRegister error)
- ✅ Immediate bounds checking tests for I-type instructions (InvalidImmediate error)
- ✅ Edge case testing for immediate value ranges
- ✅ 100% code coverage maintained
- ✅ All instruction types: Decode tests migrated to roundtrip tests where applicable
- ✅ M Extension: All decode tests migrated to roundtrip tests with bounds checking

### Documentation 📋
- 📋 Add encoding examples to documentation
- 📋 Update module-level documentation in src/instruction.rs to reflect encoding capability
- 📋 Update CLAUDE.md to document encoding infrastructure and conventions

## Implementation Notes

### Completed Infrastructure
- Created `EncodeError` enum with `NotImplemented` variant for gradual implementation
- Added `InvalidRegister` variant to `EncodeError` for register bounds checking (stores register name and invalid value)
- Added `InvalidImmediate` variant to `EncodeError` for immediate bounds checking (stores field name and invalid value)
- Added `encode()` method that returns `Result<u32, EncodeError>`
- Created `encode_r_type()` helper function at bottom of file for R-type encoding with register bounds checking
- Created `encode_i_type()` helper function for I-type encoding with register and immediate bounds checking
- Created `encode_s_type()` helper function for S-type encoding with register and immediate bounds checking
- Created `encode_b_type()` helper function for B-type encoding with register and immediate bounds checking (includes alignment validation)
- Created `encode_j_type()` helper function for J-type encoding with register and immediate bounds checking (includes alignment validation)
- Created `encode_u_type()` helper function for U-type encoding with register and immediate bounds checking
- Reorganized tests: `src/tests/instruction/roundtrip/` for combined encode/decode tests
- Test utility `assert_encode_decode()` in `src/tests/instruction/mod.rs`
- **R-Type Instructions Complete**: All 10 R-type instructions (ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND) now have full encoding support with comprehensive roundtrip tests
- **I-Type Instructions Complete**: All 9 I-type instructions (ADDI, SLTI, SLTIU, XORI, ORI, ANDI, SLLI, SRLI, SRAI) now have full encoding support with comprehensive roundtrip tests
- **Load Instructions Complete**: All 5 load instructions (LB, LH, LW, LBU, LHU) now have full encoding support using the same encode_i_type() helper since they share the I-type format with opcode 0x03
- **Store Instructions Complete**: All 3 store instructions (SB, SH, SW) now have full encoding support using the new encode_s_type() helper with opcode 0x23 and appropriate funct3 values
- **Branch Instructions Complete**: All 6 branch instructions (BEQ, BNE, BLT, BGE, BLTU, BGEU) now have full encoding support using the new encode_b_type() helper with opcode 0x63 and appropriate funct3 values. B-type immediates must be even (aligned to 2-byte boundaries) and within the range -4096 to 4094
- **Jump Instructions Complete**: Both jump instructions (JAL, JALR) now have full encoding support:
  - JAL uses the new encode_j_type() helper with opcode 0x6F. J-type immediates must be even (aligned to 2-byte boundaries) and within the range -1048576 to 1048574
  - JALR uses the existing encode_i_type() helper with opcode 0x67 and funct3 = 0x0. I-type immediates must be within the range -2048 to 2047
- **U-Type Instructions Complete**: Both U-type instructions (LUI, AUIPC) now have full encoding support using the new encode_u_type() helper:
  - LUI uses opcode 0x37 and loads a 20-bit immediate into the upper 20 bits of the destination register
  - AUIPC uses opcode 0x17 and adds a 20-bit immediate to the upper 20 bits of the PC
  - U-type immediates are 20-bit unsigned values (0 to 1048575)
- **System Instructions Complete**: Both system instructions (ECALL, EBREAK) now have full encoding support:
  - ECALL encodes as 0x00000073 (SYSTEM opcode with imm=0x000)
  - EBREAK encodes as 0x00100073 (SYSTEM opcode with imm=0x001)
  - System instructions have no fields and are encoded as fixed values
- **M Extension Instructions Complete**: All 8 M-extension instructions (MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU) now have full encoding support:
  - All M-extension instructions use R-type format with opcode 0x33 and funct7=0x01
  - funct3 values: MUL=0x0, MULH=0x1, MULHSU=0x2, MULHU=0x3, DIV=0x4, DIVU=0x5, REM=0x6, REMU=0x7
  - Comprehensive roundtrip tests verify both encoding and decoding
  - Bounds checking tests ensure proper register validation
- **Register Bounds Checking**: Added comprehensive tests in `src/tests/instruction/encode/bounds/register/` for all R-type instructions
- **Immediate Bounds Checking**: Added comprehensive tests in `src/tests/instruction/encode/bounds/immediate/` for all I-type instructions, verifying proper InvalidImmediate errors for values outside their valid ranges:
  - Regular I-type instructions (ADDI, SLTI, SLTIU, XORI, ORI, ANDI): -2048 to 2047 range
  - Shift instructions (SLLI, SRLI, SRAI): 0 to 31 range for shamt field

### Current Test Structure
After completing all instruction encoding, the test organization is:
- `src/tests/instruction/decode/`: Contains decode-only tests for special validation cases
  - `system/`: System instruction decode validation tests (invalid field tests)
  - `register/`: Only contains SLL, SLT, SLTU tests (special decode cases not covered by roundtrip)
  - `unsupported.rs`: Tests for unsupported instruction patterns
- `src/tests/instruction/roundtrip/`: Contains bidirectional encode+decode tests
  - `register/`: All R-type instructions (add, sub, sll, slt, sltu, xor, srl, sra, or, and)
  - `immediate/`: All I-type instructions (addi, slti, sltiu, xori, ori, andi, slli, srli, srai)
  - `load/`: All load instructions (lb, lh, lw, lbu, lhu)
  - `store/`: All store instructions (sb, sh, sw)
  - `branch/`: All branch instructions (beq, bne, blt, bge, bltu, bgeu)
  - `jump/`: All jump instructions (jal, jalr)
  - `utype/`: All U-type instructions (lui, auipc)
  - `system/`: All system instructions (ecall, ebreak)
  - `multiply/`: All M-extension instructions (mul, mulh, mulhsu, mulhu, div, divu, rem, remu)
- `src/tests/instruction/encode/`: Contains encode-specific tests
  - `unimplemented.rs`: Tests verifying NotImplemented errors for unimplemented instructions (only Unsupported)
  - `bounds/register/`: Tests verifying InvalidRegister errors for out-of-bounds register values in R-type instructions
  - `bounds/immediate/`: Tests verifying InvalidImmediate errors for out-of-bounds immediate values in all I-type instructions
  - `bounds/load/`: Tests verifying InvalidRegister and InvalidImmediate errors for load instructions
  - `bounds/store/`: Tests verifying InvalidRegister and InvalidImmediate errors for store instructions
  - `bounds/branch/`: Tests verifying InvalidRegister and InvalidImmediate errors for branch instructions (including odd offset validation)
  - `bounds/jump/`: Tests verifying InvalidRegister and InvalidImmediate errors for jump instructions (including odd offset validation for JAL)
  - `bounds/utype/`: Tests verifying InvalidRegister and InvalidImmediate errors for U-type instructions (lui, auipc)
  - `bounds/multiply/`: Tests verifying InvalidRegister errors for M-extension instructions
  - `error.rs`: Tests for EncodeError Display and Error trait implementations
- `src/tests/instruction/display/`: Display formatting tests (unchanged)

### Key Learnings
- Helper functions like `encode_r_type()` should be placed at the bottom of the file
- Roundtrip tests eliminate duplication between encode and decode test suites
- Test organization should mirror instruction types (register/, immediate/, etc.)
- All instruction variants must derive `Debug`, `Clone`, and `PartialEq` for testing
- EncodeError should implement std::error::Error and std::fmt::Display traits for proper error handling
- **IMPORTANT**: When implementing encoding for an instruction, remove only the decode test functions that are covered by roundtrip tests (e.g., in `src/tests/instruction/decode/register/sub.rs`, remove functions like `basic()`, `zero_registers()`, etc. when adding `src/tests/instruction/roundtrip/register/sub.rs`). Keep any decode failure tests that aren't covered by roundtrip. If no tests remain in the file after removal, delete the file and update the mod.rs
- All R-type instructions follow the same encoding pattern using `encode_r_type()` with appropriate funct3 and funct7 values
- Test migration strategy: As each instruction type gets encoding support, its decode tests are replaced with roundtrip tests that verify both encode and decode operations
- **Register Bounds Checking**: The `encode_r_type()` function validates that all register values (rd, rs1, rs2) are within the valid range (0-31) and returns `InvalidRegister` error with the register name and invalid value if out of bounds
- **Immediate Bounds Checking**: The `encode_i_type()` function validates that immediate values are within the valid range for I-type instructions (-2048 to 2047) and returns `InvalidImmediate` error with the field name and invalid value if out of bounds
- **Comprehensive Error Testing**: Each instruction needs bounds checking tests for all applicable fields (registers and immediates) to ensure 100% code coverage

### Final Test Structure Review Considerations
When all encoding is complete, review the test structure for:
- **Consistency**: Ensure all instruction types follow the same organizational pattern
- **Discoverability**: Tests should be easy to find based on instruction name or type
- **Maintainability**: Consider consolidating directories if decode/ becomes mostly empty
- **Naming**: Ensure test function names are consistent and descriptive
- **Coverage**: Verify edge cases and error conditions are adequately tested
- **Documentation**: Add README files to test directories explaining the organization
- Potential reorganization options:
  - Keep roundtrip/ as primary test location, move remaining decode tests there
  - Organize by instruction format (R, I, S, B, U, J) vs functionality
  - Consider whether encode/unimplemented.rs tests are still needed

## Design Considerations

### Encoding Strategy
- Each instruction variant implements its own encoding logic
- Immediate values are validated to ensure they fit within bit width constraints

### Error Handling
- Return Result<u32, EncodeError> to handle invalid immediate values
- Provide clear error messages for out-of-range immediates
- Always use Result for error handling, never panic

### Testing Approach
- Modify existing decode tests to test bidirectionally using a shared helper function
- The helper should: decode the instruction word, verify it matches expected, encode it back, verify it matches original word
- Every encoded instruction should decode back to the original
- Test boundary values for all immediate fields
- Verify against known instruction encodings from RISC-V spec