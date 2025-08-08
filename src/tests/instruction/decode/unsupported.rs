use crate::Instruction;

#[test]
fn unknown_opcode() {
    // Test instruction with unknown opcode (0x00 instead of valid opcodes)
    let instruction_word = 0x00000000;
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn register_invalid_funct7() {
    // R-type instruction with invalid funct7 for ADD/SUB operations
    // rd=1, rs1=2, rs2=3, funct3=0x0, funct7=0x02 (invalid), opcode=0x33
    let instruction_word = 0x043100B3; // 0000010 00011 00010 000 00001 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn immediate_slli_invalid_upper_bits() {
    // SLLI with non-zero upper bits (should be 0x00)
    // This sets upper bits to 0x01 instead of 0x00
    let word = 0x02051093; // rd=1, rs1=10, funct3=1, but upper bits are invalid
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}

#[test]
fn immediate_srli_srai_invalid_upper_bits() {
    // SRLI/SRAI with invalid upper bits (should be 0x00 or 0x20)
    // This sets upper bits to 0x10 which is neither SRLI nor SRAI
    let word = 0x20555293; // rd=5, rs1=10, funct3=5, but upper bits are 0x10
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}
