use crate::Instruction;

#[test]
fn invalid_funct3() {
    // Load instruction with invalid funct3=0x3 (valid are 0x0, 0x1, 0x2, 0x4, 0x5)
    // opcode=0x03 (load), funct3=0x3 (invalid), rd=1, rs1=2, imm=0
    let word = 0x00013083; // imm[11:0]=0, rs1=2, funct3=3, rd=1, opcode=0x03
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}

#[test]
fn invalid_funct3_6() {
    // Load instruction with invalid funct3=0x6
    // opcode=0x03 (load), funct3=0x6 (invalid), rd=1, rs1=2, imm=0
    let word = 0x00016083; // imm[11:0]=0, rs1=2, funct3=6, rd=1, opcode=0x03
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}

#[test]
fn invalid_funct3_7() {
    // Load instruction with invalid funct3=0x7
    // opcode=0x03 (load), funct3=0x7 (invalid), rd=1, rs1=2, imm=0
    let word = 0x00017083; // imm[11:0]=0, rs1=2, funct3=7, rd=1, opcode=0x03
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}
