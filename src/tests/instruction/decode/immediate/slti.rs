use crate::instruction::Instruction;

#[test]
fn basic() {
    // slti x1, x2, 10
    // rd=1, rs1=2, imm=10, funct3=0x2, opcode=0x13
    let instruction_word = 0x00A12093; // 000000001010 00010 010 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 10);
        }
        _ => panic!("Expected Slti instruction"),
    }
}

#[test]
fn zero_registers() {
    // slti x0, x0, 0
    // rd=0, rs1=0, imm=0, funct3=0x2, opcode=0x13
    let instruction_word = 0x00002013; // 000000000000 00000 010 00000 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Slti instruction"),
    }
}

#[test]
fn max_registers() {
    // slti x31, x31, 100
    // rd=31, rs1=31, imm=100, funct3=0x2, opcode=0x13
    let instruction_word = 0x064FAF93; // 000001100100 11111 010 11111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Slti instruction"),
    }
}

#[test]
fn negative_immediate() {
    // slti x5, x10, -1
    // rd=5, rs1=10, imm=-1, funct3=0x2, opcode=0x13
    let instruction_word = 0xFFF52293; // 111111111111 01010 010 00101 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 5);
            assert_eq!(rs1, 10);
            assert_eq!(imm, -1);
        }
        _ => panic!("Expected Slti instruction"),
    }
}

#[test]
fn max_positive_immediate() {
    // slti x7, x8, 2047
    // rd=7, rs1=8, imm=2047 (0x7FF), funct3=0x2, opcode=0x13
    let instruction_word = 0x7FF42393; // 011111111111 01000 010 00111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 7);
            assert_eq!(rs1, 8);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Slti instruction"),
    }
}

#[test]
fn max_negative_immediate() {
    // slti x11, x12, -2048
    // rd=11, rs1=12, imm=-2048 (0x800), funct3=0x2, opcode=0x13
    let instruction_word = 0x80062593; // 100000000000 01100 010 01011 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slti { rd, rs1, imm } => {
            assert_eq!(rd, 11);
            assert_eq!(rs1, 12);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Slti instruction"),
    }
}

#[test]
fn invalid_funct3() {
    // I-type instruction with invalid funct3 for SLTI context
    // Using funct3=0x5 which is not yet implemented
    // rd=1, rs1=2, imm=10, funct3=0x5 (SRLI/SRAI - not yet implemented), opcode=0x13
    let instruction_word = 0x00A15093; // 000000001010 00010 101 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x00A15093);
        }
        _ => panic!("Expected Unsupported instruction for invalid funct3"),
    }
}
