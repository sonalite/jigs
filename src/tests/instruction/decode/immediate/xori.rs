use crate::instruction::Instruction;

#[test]
fn basic() {
    // xori x1, x2, 10
    // rd=1, rs1=2, imm=10, funct3=0x4, opcode=0x13
    let instruction_word = 0x00A14093; // 000000001010 00010 100 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Xori { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 10);
        }
        _ => panic!("Expected Xori instruction"),
    }
}

#[test]
fn zero_registers() {
    // xori x0, x0, 0
    // rd=0, rs1=0, imm=0, funct3=0x4, opcode=0x13
    let instruction_word = 0x00004013; // 000000000000 00000 100 00000 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Xori { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Xori instruction"),
    }
}

#[test]
fn max_registers() {
    // xori x31, x31, 100
    // rd=31, rs1=31, imm=100, funct3=0x4, opcode=0x13
    let instruction_word = 0x064FCF93; // 000001100100 11111 100 11111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Xori { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Xori instruction"),
    }
}

#[test]
fn negative_immediate() {
    // xori x5, x10, -1
    // rd=5, rs1=10, imm=-1, funct3=0x4, opcode=0x13
    let instruction_word = 0xFFF54293; // 111111111111 01010 100 00101 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Xori { rd, rs1, imm } => {
            assert_eq!(rd, 5);
            assert_eq!(rs1, 10);
            assert_eq!(imm, -1);
        }
        _ => panic!("Expected Xori instruction"),
    }
}

#[test]
fn max_positive_immediate() {
    // xori x7, x8, 2047
    // rd=7, rs1=8, imm=2047 (0x7FF), funct3=0x4, opcode=0x13
    let instruction_word = 0x7FF44393; // 011111111111 01000 100 00111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Xori { rd, rs1, imm } => {
            assert_eq!(rd, 7);
            assert_eq!(rs1, 8);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Xori instruction"),
    }
}

#[test]
fn max_negative_immediate() {
    // xori x11, x12, -2048
    // rd=11, rs1=12, imm=-2048 (0x800), funct3=0x4, opcode=0x13
    let instruction_word = 0x80064593; // 100000000000 01100 100 01011 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Xori { rd, rs1, imm } => {
            assert_eq!(rd, 11);
            assert_eq!(rs1, 12);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Xori instruction"),
    }
}

#[test]
fn all_ones_immediate() {
    // xori x15, x16, -1 (all ones in 12 bits)
    // rd=15, rs1=16, imm=-1, funct3=0x4, opcode=0x13
    let instruction_word = 0xFFF84793; // 111111111111 10000 100 01111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Xori { rd, rs1, imm } => {
            assert_eq!(rd, 15);
            assert_eq!(rs1, 16);
            assert_eq!(imm, -1);
        }
        _ => panic!("Expected Xori instruction"),
    }
}

#[test]
fn different_registers() {
    // xori x20, x25, 255
    // rd=20, rs1=25, imm=255, funct3=0x4, opcode=0x13
    let instruction_word = 0x0FFCCA13; // 000011111111 11001 100 10100 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Xori { rd, rs1, imm } => {
            assert_eq!(rd, 20);
            assert_eq!(rs1, 25);
            assert_eq!(imm, 255);
        }
        _ => panic!("Expected Xori instruction"),
    }
}
