use crate::instruction::Instruction;

#[test]
fn basic() {
    // slli x1, x2, 5
    // rd=1, rs1=2, shamt=5, funct3=0x1, opcode=0x13
    let instruction_word = 0x00511093; // 0000000 00101 00010 001 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slli { rd, rs1, shamt } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(shamt, 5);
        }
        _ => panic!("Expected Slli instruction"),
    }
}

#[test]
fn zero_registers() {
    // slli x0, x0, 0
    // rd=0, rs1=0, shamt=0, funct3=0x1, opcode=0x13
    let instruction_word = 0x00001013; // 0000000 00000 00000 001 00000 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slli { rd, rs1, shamt } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(shamt, 0);
        }
        _ => panic!("Expected Slli instruction"),
    }
}

#[test]
fn max_registers() {
    // slli x31, x31, 10
    // rd=31, rs1=31, shamt=10, funct3=0x1, opcode=0x13
    let instruction_word = 0x00AF9F93; // 0000000 01010 11111 001 11111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slli { rd, rs1, shamt } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(shamt, 10);
        }
        _ => panic!("Expected Slli instruction"),
    }
}

#[test]
fn max_shift_amount() {
    // slli x7, x8, 31
    // rd=7, rs1=8, shamt=31 (max for RV32), funct3=0x1, opcode=0x13
    let instruction_word = 0x01F41393; // 0000000 11111 01000 001 00111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slli { rd, rs1, shamt } => {
            assert_eq!(rd, 7);
            assert_eq!(rs1, 8);
            assert_eq!(shamt, 31);
        }
        _ => panic!("Expected Slli instruction"),
    }
}

#[test]
fn min_shift_amount() {
    // slli x10, x15, 1
    // rd=10, rs1=15, shamt=1, funct3=0x1, opcode=0x13
    let instruction_word = 0x00179513; // 0000000 00001 01111 001 01010 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Slli { rd, rs1, shamt } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 15);
            assert_eq!(shamt, 1);
        }
        _ => panic!("Expected Slli instruction"),
    }
}

#[test]
fn wrong_upper_bits() {
    // slli with wrong upper bits (should be 0x00, using 0x20)
    // rd=15, rs1=14, shamt=5, wrong upper bits, funct3=0x1, opcode=0x13
    let instruction_word = 0x40571793; // 0100000 00101 01110 001 01111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(_) => {}
        _ => panic!("Expected Unsupported instruction"),
    }
}
