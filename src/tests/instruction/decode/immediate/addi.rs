use crate::instruction::Instruction;

#[test]
fn basic() {
    // addi x1, x2, 10
    // rd=1, rs1=2, imm=10, funct3=0x0, opcode=0x13
    let instruction_word = 0x00A10093; // 000000001010 00010 000 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Addi { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 10);
        }
        _ => panic!("Expected Addi instruction"),
    }
}

#[test]
fn zero_registers() {
    // addi x0, x0, 0
    // rd=0, rs1=0, imm=0, funct3=0x0, opcode=0x13
    let instruction_word = 0x00000013; // 000000000000 00000 000 00000 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Addi { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Addi instruction"),
    }
}

#[test]
fn max_registers() {
    // addi x31, x31, 100
    // rd=31, rs1=31, imm=100, funct3=0x0, opcode=0x13
    let instruction_word = 0x064F8F93; // 000001100100 11111 000 11111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Addi { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Addi instruction"),
    }
}

#[test]
fn negative_immediate() {
    // addi x5, x10, -1
    // rd=5, rs1=10, imm=-1, funct3=0x0, opcode=0x13
    let instruction_word = 0xFFF50293; // 111111111111 01010 000 00101 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Addi { rd, rs1, imm } => {
            assert_eq!(rd, 5);
            assert_eq!(rs1, 10);
            assert_eq!(imm, -1);
        }
        _ => panic!("Expected Addi instruction"),
    }
}

#[test]
fn max_positive_immediate() {
    // addi x7, x8, 2047
    // rd=7, rs1=8, imm=2047 (0x7FF), funct3=0x0, opcode=0x13
    let instruction_word = 0x7FF40393; // 011111111111 01000 000 00111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Addi { rd, rs1, imm } => {
            assert_eq!(rd, 7);
            assert_eq!(rs1, 8);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Addi instruction"),
    }
}

#[test]
fn max_negative_immediate() {
    // addi x11, x12, -2048
    // rd=11, rs1=12, imm=-2048 (0x800), funct3=0x0, opcode=0x13
    let instruction_word = 0x80060593; // 100000000000 01100 000 01011 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Addi { rd, rs1, imm } => {
            assert_eq!(rd, 11);
            assert_eq!(rs1, 12);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Addi instruction"),
    }
}

#[test]
fn invalid_funct3() {
    // I-type instruction with invalid funct3 (0x5 instead of 0x0 for ADDI)
    // rd=1, rs1=2, imm=10, funct3=0x5 (invalid), opcode=0x13
    let instruction_word = 0x00A15093; // 000000001010 00010 101 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(word) => {
            assert_eq!(word, 0x00A15093);
        }
        _ => panic!("Expected Unsupported instruction for invalid funct3"),
    }
}
