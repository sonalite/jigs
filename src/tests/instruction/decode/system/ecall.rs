use crate::instruction::Instruction;

#[test]
fn basic() {
    // ecall
    // Full encoding: 0x00000073
    let instruction_word = 0x00000073;
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Ecall);
}

#[test]
fn verify_exact_encoding() {
    // ECALL must be exactly 0x00000073
    let instruction_word = 0x00000073;
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Ecall);
}

#[test]
fn invalid_with_nonzero_rd() {
    // ecall with rd != 0 should be unsupported
    // Setting rd = 1 (bits 11:7)
    let instruction_word = 0x000000F3; // rd = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn invalid_with_nonzero_rs1() {
    // ecall with rs1 != 0 should be unsupported
    // Setting rs1 = 1 (bits 19:15)
    let instruction_word = 0x00008073; // rs1 = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn invalid_with_nonzero_funct3() {
    // ecall with funct3 != 0 should be unsupported
    // Setting funct3 = 1 (bits 14:12)
    let instruction_word = 0x00001073; // funct3 = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
