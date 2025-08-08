use crate::Instruction;

#[test]
fn invalid_funct7() {
    // R-type instruction with invalid funct7 for ADD/SUB operations
    // rd=1, rs1=2, rs2=3, funct3=0x0, funct7=0x02 (invalid), opcode=0x33
    let instruction_word = 0x043100B3; // 0000010 00011 00010 000 00001 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn sll_wrong_funct7() {
    // sll with wrong funct7 (should be 0x00, using 0x20)
    // rd=15, rs1=14, rs2=15, funct3=0x1, funct7=0x20, opcode=0x33
    let instruction_word = 0x20F717B3; // 0100000 01111 01110 001 01111 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn slt_wrong_funct7() {
    // slt with wrong funct7 (should be 0x00, using 0x20)
    // rd=15, rs1=14, rs2=15, funct3=0x2, funct7=0x20, opcode=0x33
    let instruction_word = 0x20F727B3; // 0100000 01111 01110 010 01111 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}

#[test]
fn sltu_wrong_funct7() {
    // sltu with wrong funct7 (should be 0x00, using 0x20)
    // rd=15, rs1=14, rs2=15, funct3=0x3, funct7=0x20, opcode=0x33
    let instruction_word = 0x20F737B3; // 0100000 01111 01110 011 01111 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
