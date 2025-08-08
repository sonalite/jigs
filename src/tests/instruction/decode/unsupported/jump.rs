use crate::Instruction;

#[test]
fn jalr_wrong_funct3() {
    // Invalid jalr with funct3 != 0
    // rd=1, rs1=2, imm=8, funct3=0x1 (wrong), opcode=0x67
    let instruction_word = 0x008110E7; // 000000001000 00010 001 00001 1100111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
