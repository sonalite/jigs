use crate::instruction::Instruction;

#[test]
fn basic() {
    // ebreak
    // Full encoding: 0x00100073
    let instruction_word = 0x00100073;
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Ebreak);
}

#[test]
fn verify_exact_encoding() {
    // EBREAK must be exactly 0x00100073
    let instruction_word = 0x00100073;
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Ebreak);
}

#[test]
fn invalid_with_nonzero_rd() {
    // ebreak with rd != 0 should be unsupported
    // Setting rd = 1 (bits 11:7)
    let instruction_word = 0x001000F3; // rd = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn invalid_with_nonzero_rs1() {
    // ebreak with rs1 != 0 should be unsupported
    // Setting rs1 = 1 (bits 19:15)
    let instruction_word = 0x00108073; // rs1 = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn invalid_with_nonzero_funct3() {
    // ebreak with funct3 != 0 should be unsupported
    // Setting funct3 = 1 (bits 14:12)
    let instruction_word = 0x00101073; // funct3 = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
