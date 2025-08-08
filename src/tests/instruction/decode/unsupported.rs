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

#[test]
fn load_invalid_funct3() {
    // Load instruction with invalid funct3=0x3 (valid are 0x0, 0x1, 0x2, 0x4, 0x5)
    // opcode=0x03 (load), funct3=0x3 (invalid), rd=1, rs1=2, imm=0
    let word = 0x00013083; // imm[11:0]=0, rs1=2, funct3=3, rd=1, opcode=0x03
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}

#[test]
fn load_invalid_funct3_6() {
    // Load instruction with invalid funct3=0x6
    // opcode=0x03 (load), funct3=0x6 (invalid), rd=1, rs1=2, imm=0
    let word = 0x00016083; // imm[11:0]=0, rs1=2, funct3=6, rd=1, opcode=0x03
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}

#[test]
fn load_invalid_funct3_7() {
    // Load instruction with invalid funct3=0x7
    // opcode=0x03 (load), funct3=0x7 (invalid), rd=1, rs1=2, imm=0
    let word = 0x00017083; // imm[11:0]=0, rs1=2, funct3=7, rd=1, opcode=0x03
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}
