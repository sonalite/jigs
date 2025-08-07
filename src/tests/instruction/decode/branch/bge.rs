use crate::instruction::Instruction;

#[test]
fn basic() {
    // bge x1, x2, 8
    // rs1=1, rs2=2, imm=8, funct3=0x5, opcode=0x63
    let instruction_word = 0x0020D463; // 0 000000 0 00010 00001 101 0100 0 1100011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Bge { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, 8);
        }
        _ => panic!("Expected Bge instruction"),
    }
}

#[test]
fn zero_registers() {
    // bge x0, x0, 0
    // rs1=0, rs2=0, imm=0, funct3=0x5, opcode=0x63
    let instruction_word = 0x00005063; // 0 000000 0 00000 00000 101 0000 0 1100011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Bge { rs1, rs2, imm } => {
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Bge instruction"),
    }
}

#[test]
fn max_registers() {
    // bge x31, x31, 16
    // rs1=31, rs2=31, imm=16, funct3=0x5, opcode=0x63
    let instruction_word = 0x01FFD863; // 0 000000 0 11111 11111 101 1000 0 1100011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Bge { rs1, rs2, imm } => {
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
            assert_eq!(imm, 16);
        }
        _ => panic!("Expected Bge instruction"),
    }
}

#[test]
fn negative_offset() {
    // bge x5, x6, -8
    // rs1=5, rs2=6, imm=-8, funct3=0x5, opcode=0x63
    let instruction_word = 0xFE62DCE3; // 1 111111 0 00110 00101 101 1100 1 1100011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Bge { rs1, rs2, imm } => {
            assert_eq!(rs1, 5);
            assert_eq!(rs2, 6);
            assert_eq!(imm, -8);
        }
        _ => panic!("Expected Bge instruction"),
    }
}

#[test]
fn large_positive_offset() {
    // bge x10, x11, 4094
    // rs1=10, rs2=11, imm=4094, funct3=0x5, opcode=0x63
    let instruction_word = 0x7EB55FE3; // 0 111111 0 01011 01010 101 1111 1 1100011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Bge { rs1, rs2, imm } => {
            assert_eq!(rs1, 10);
            assert_eq!(rs2, 11);
            assert_eq!(imm, 4094);
        }
        _ => panic!("Expected Bge instruction"),
    }
}

#[test]
fn large_negative_offset() {
    // bge x15, x16, -4096
    // rs1=15, rs2=16, imm=-4096, funct3=0x5, opcode=0x63
    let instruction_word = 0x8107D063; // 1 000000 1 10000 01111 101 0000 0 1100011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Bge { rs1, rs2, imm } => {
            assert_eq!(rs1, 15);
            assert_eq!(rs2, 16);
            assert_eq!(imm, -4096);
        }
        _ => panic!("Expected Bge instruction"),
    }
}
