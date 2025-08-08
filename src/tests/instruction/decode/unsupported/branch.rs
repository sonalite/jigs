use crate::Instruction;

#[test]
fn invalid_funct3_2() {
    // Branch instruction with invalid funct3=0x2 (unused)
    // rs1=1, rs2=2, imm=8, funct3=0x2, opcode=0x63
    let word = 0x0020A463; // 0 000000 0 00010 00001 010 0100 0 1100011
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}

#[test]
fn invalid_funct3_3() {
    // Branch instruction with invalid funct3=0x3 (unused)
    // rs1=1, rs2=2, imm=8, funct3=0x3, opcode=0x63
    let word = 0x0020B463; // 0 000000 0 00010 00001 011 0100 0 1100011
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}
