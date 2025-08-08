use crate::instruction::Instruction;

#[test]
fn wrong_funct7() {
    // slt with wrong funct7 (should be 0x00, using 0x20)
    // rd=15, rs1=14, rs2=15, funct3=0x2, funct7=0x20, opcode=0x33
    let instruction_word = 0x20F727B3; // 0100000 01111 01110 010 01111 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
