use crate::instruction::Instruction;

#[test]
fn wrong_funct7() {
    // sll with wrong funct7 (should be 0x00, using 0x20)
    // rd=15, rs1=14, rs2=15, funct3=0x1, funct7=0x20, opcode=0x33
    let instruction_word = 0x20F717B3; // 0100000 01111 01110 001 01111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(_) => {}
        _ => panic!("Expected Unsupported instruction"),
    }
}
