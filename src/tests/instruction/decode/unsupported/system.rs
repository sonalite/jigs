use crate::Instruction;

#[test]
fn invalid_immediate() {
    // System instruction with invalid immediate (not 0x000 or 0x001)
    // Opcode 0x73, funct3 = 0, rd = 0, rs1 = 0, but imm = 0x002
    let word = 0x00200073; // imm = 0x002 (not ECALL or EBREAK)
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}

#[test]
fn ecall_invalid_with_nonzero_rd() {
    // ecall with rd != 0 should be unsupported
    // Setting rd = 1 (bits 11:7)
    let instruction_word = 0x000000F3; // rd = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn ecall_invalid_with_nonzero_rs1() {
    // ecall with rs1 != 0 should be unsupported
    // Setting rs1 = 1 (bits 19:15)
    let instruction_word = 0x00008073; // rs1 = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn ecall_invalid_with_nonzero_funct3() {
    // ecall with funct3 != 0 should be unsupported
    // Setting funct3 = 1 (bits 14:12)
    let instruction_word = 0x00001073; // funct3 = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn ebreak_invalid_with_nonzero_rd() {
    // ebreak with rd != 0 should be unsupported
    // Setting rd = 1 (bits 11:7)
    let instruction_word = 0x001000F3; // rd = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn ebreak_invalid_with_nonzero_rs1() {
    // ebreak with rs1 != 0 should be unsupported
    // Setting rs1 = 1 (bits 19:15)
    let instruction_word = 0x00108073; // rs1 = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn ebreak_invalid_with_nonzero_funct3() {
    // ebreak with funct3 != 0 should be unsupported
    // Setting funct3 = 1 (bits 14:12)
    let instruction_word = 0x00101073; // funct3 = 1
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
