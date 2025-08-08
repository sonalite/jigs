# Project 0002: RISC-V 32-bit IM Instruction Encoding 🚧

### Overview
Implementation of RISC-V 32-bit instruction encoder to convert Instruction enum variants into 32-bit instruction words. This complements the decoder by enabling bidirectional conversion between instruction representations and machine code.

## Tasks

### Instruction Infrastructure 🚧
- ✅ Add encode() method to Instruction enum (returns Result<u32, EncodeError>)
- ✅ Create EncodeError type for error handling
- 📋 Implement std::error::Error and std::fmt::Display for EncodeError
- ✅ Create helper function encode_r_type() for R-type instructions
- 🚧 Create helper functions for I, S, B, U, and J formats as needed
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

### Testing & Validation 🚧
- ✅ Create helper function assert_encode_decode() for bidirectional testing
- ✅ Reorganize tests into roundtrip directory for combined encode/decode testing
- ✅ Remove duplicate decode tests that are covered by roundtrip tests
- ✅ Round-trip tests (encode then decode should match original)
- 📋 Verify encoding matches RISC-V specification test vectors
- 📋 Edge case testing for immediate value ranges
- ✅ 100% code coverage maintained
- ✅ R-type instructions: All decode tests migrated to roundtrip tests
- 🚧 I-type, S-type, B-type, U-type, J-type: Decode tests remain, will migrate as encoding is implemented
- 📋 Final test structure review: Evaluate and reorganize test directory structure for optimal discoverability and maintainability

### Documentation 📋
- 📋 Add encoding examples to documentation
- 📋 Update module-level documentation in src/instruction.rs to reflect encoding capability
- 📋 Update CLAUDE.md to document encoding infrastructure and conventions

## Implementation Notes

### Completed Infrastructure
- Created `EncodeError` enum with `NotImplemented` variant for gradual implementation
- Added `encode()` method that returns `Result<u32, EncodeError>`
- Created `encode_r_type()` helper function at bottom of file for R-type encoding
- Reorganized tests: `src/tests/instruction/roundtrip/` for combined encode/decode tests
- Test utility `assert_encode_decode()` in `src/tests/instruction/mod.rs`
- **R-Type Instructions Complete**: All 10 R-type instructions (ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND) now have full encoding support with comprehensive roundtrip tests

### Current Test Structure
After R-type completion, the test organization is:
- `src/tests/instruction/decode/`: Contains decode-only tests for instructions not yet encoding-enabled
  - `immediate/`: All I-type decode tests (will migrate when encoding implemented)
  - `load/`: All load instruction decode tests
  - `store/`: All store instruction decode tests  
  - `branch/`: All branch instruction decode tests
  - `jump/`: All jump instruction decode tests
  - `multiply/`: All M-extension decode tests
  - `utype/`: All U-type decode tests
  - `system/`: All system instruction decode tests
  - `register/`: Only contains SLL, SLT, SLTU tests (special decode cases not covered by roundtrip)
- `src/tests/instruction/roundtrip/`: Contains bidirectional encode+decode tests
  - `register/`: All R-type instructions (add, sub, sll, slt, sltu, xor, srl, sra, or, and)
- `src/tests/instruction/encode/`: Contains encode-specific tests
  - `not_implemented.rs`: Tests verifying NotImplemented errors for unimplemented instructions
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
  - Consider whether encode/not_implemented.rs tests are still needed

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