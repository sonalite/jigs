use crate::instruction::Instruction;

#[test]
fn basic() {
    // sltiu x1, x2, 10
    // rd=1, rs1=2, imm=10, funct3=0x3, opcode=0x13
    let instruction_word = 0x00A13093; // 000000001010 00010 011 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 10);
        }
        _ => panic!("Expected Sltiu instruction"),
    }
}

#[test]
fn zero_registers() {
    // sltiu x0, x0, 0
    // rd=0, rs1=0, imm=0, funct3=0x3, opcode=0x13
    let instruction_word = 0x00003013; // 000000000000 00000 011 00000 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Sltiu instruction"),
    }
}

#[test]
fn max_registers() {
    // sltiu x31, x31, 100
    // rd=31, rs1=31, imm=100, funct3=0x3, opcode=0x13
    let instruction_word = 0x064FBF93; // 000001100100 11111 011 11111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Sltiu instruction"),
    }
}

#[test]
fn different_registers() {
    // sltiu x5, x10, 255
    // rd=5, rs1=10, imm=255, funct3=0x3, opcode=0x13
    let instruction_word = 0x0FF53293; // 000011111111 01010 011 00101 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 5);
            assert_eq!(rs1, 10);
            assert_eq!(imm, 255);
        }
        _ => panic!("Expected Sltiu instruction"),
    }
}

#[test]
fn medium_immediate() {
    // sltiu x7, x8, 2047
    // rd=7, rs1=8, imm=2047 (0x7FF), funct3=0x3, opcode=0x13
    let instruction_word = 0x7FF43393; // 011111111111 01000 011 00111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 7);
            assert_eq!(rs1, 8);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Sltiu instruction"),
    }
}

#[test]
fn max_immediate() {
    // sltiu x11, x12, 4095
    // rd=11, rs1=12, imm=4095 (0xFFF), funct3=0x3, opcode=0x13
    let instruction_word = 0xFFF63593; // 111111111111 01100 011 01011 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 11);
            assert_eq!(rs1, 12);
            assert_eq!(imm, 4095);
        }
        _ => panic!("Expected Sltiu instruction"),
    }
}

#[test]
fn invalid_funct3() {
    // I-type instruction with invalid funct3 for SLTIU context
    // Using funct3=0x7 which is not SLTIU
    // rd=1, rs1=2, imm=10, funct3=0x7 (invalid for SLTIU context), opcode=0x13
    let instruction_word = 0x00A17093; // 000000001010 00010 111 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x00A17093);
        }
        _ => panic!("Expected Unsupported instruction for invalid funct3"),
    }
}
