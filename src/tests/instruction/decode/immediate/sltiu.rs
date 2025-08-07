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
        _ => panic!("Expected Sltiu instruction, got {:?}", instruction),
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
        _ => panic!("Expected Sltiu instruction, got {:?}", instruction),
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
        _ => panic!("Expected Sltiu instruction, got {:?}", instruction),
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
        _ => panic!("Expected Sltiu instruction, got {:?}", instruction),
    }
}

#[test]
fn max_positive_immediate() {
    // sltiu x7, x8, 2047
    // rd=7, rs1=8, imm=2047 (0x7FF - max positive 12-bit signed), funct3=0x3, opcode=0x13
    let instruction_word = 0x7FF43393; // 011111111111 01000 011 00111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 7);
            assert_eq!(rs1, 8);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Sltiu instruction, got {:?}", instruction),
    }
}

#[test]
fn negative_immediate() {
    // sltiu x11, x12, -1
    // rd=11, rs1=12, imm=-1 (0xFFF sign-extended to 0xFFFFFFFF), funct3=0x3, opcode=0x13
    let instruction_word = 0xFFF63593; // 111111111111 01100 011 01011 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 11);
            assert_eq!(rs1, 12);
            assert_eq!(imm, -1); // 0xFFF sign-extended to -1
        }
        _ => panic!("Expected Sltiu instruction, got {:?}", instruction),
    }
}

#[test]
fn min_negative_immediate() {
    // sltiu x15, x16, -2048
    // rd=15, rs1=16, imm=-2048 (0x800 - min negative 12-bit signed), funct3=0x3, opcode=0x13
    let instruction_word = 0x80083793; // 100000000000 10000 011 01111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 15);
            assert_eq!(rs1, 16);
            assert_eq!(imm, -2048); // 0x800 sign-extended to -2048
        }
        _ => panic!("Expected Sltiu instruction, got {:?}", instruction),
    }
}

#[test]
fn seqz_pseudo_instruction() {
    // sltiu x1, x2, 1 (SEQZ pseudo-instruction: sets x1 to 1 if x2 equals zero)
    // rd=1, rs1=2, imm=1, funct3=0x3, opcode=0x13
    let instruction_word = 0x00113093; // 000000000001 00010 011 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sltiu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 1);
        }
        _ => panic!("Expected Sltiu instruction, got {:?}", instruction),
    }
}
