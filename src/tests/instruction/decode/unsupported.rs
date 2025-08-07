use crate::instruction::Instruction;

#[test]
fn test_decode_invalid_opcode() {
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
fn test_decode_invalid_funct7() {
    // Valid opcode, valid funct3, but invalid funct7 for ADD
    // rd=1, rs1=2, rs2=3, funct3=0x0, funct7=0x01 (invalid), opcode=0x33
    let instruction_word = 0x023100B3; // 0000001 00011 00010 000 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x023100B3);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}

#[test]
fn test_decode_invalid_funct3() {
    // Valid opcode, but invalid funct3
    // rd=1, rs1=2, rs2=3, funct3=0x1 (invalid), funct7=0x00, opcode=0x33
    let instruction_word = 0x003110B3; // 0000000 00011 00010 001 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x003110B3);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}
