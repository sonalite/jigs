use crate::instruction::Instruction;

#[test]
fn invalid_opcode() {
    // Invalid opcode (not 0x33)
    let instruction_word = 0x00000000;
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x00000000);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}

#[test]
fn invalid_funct7() {
    // Valid opcode, valid funct3, but invalid funct7 for ADD/SUB/MUL
    // rd=1, rs1=2, rs2=3, funct3=0x0, funct7=0x02 (invalid), opcode=0x33
    let instruction_word = 0x043100B3; // 0000010 00011 00010 000 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x043100B3);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}
