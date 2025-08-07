use crate::instruction::Instruction;

#[test]
fn invalid_funct3() {
    // Invalid branch instruction with funct3=0x2 (unused)
    // rs1=1, rs2=2, imm=8, funct3=0x2, opcode=0x63
    let instruction_word = 0x0020A463; // 0 000000 0 00010 00001 010 0100 0 1100011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x0020A463);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}

#[test]
fn invalid_funct3_3() {
    // Invalid branch instruction with funct3=0x3 (unused)
    // rs1=1, rs2=2, imm=8, funct3=0x3, opcode=0x63
    let instruction_word = 0x0020B463; // 0 000000 0 00010 00001 011 0100 0 1100011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x0020B463);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}
