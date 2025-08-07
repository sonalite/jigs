use crate::instruction::Instruction;

#[test]
fn basic() {
    // srli x1, x2, 5
    // rd=1, rs1=2, shamt=5, funct3=0x5, upper=0x00, opcode=0x13
    let instruction_word = 0x00515093; // 0000000 00101 00010 101 00001 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srli { rd, rs1, shamt } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(shamt, 5);
        }
        _ => panic!("Expected Srli instruction"),
    }
}

#[test]
fn zero_registers() {
    // srli x0, x0, 0
    // rd=0, rs1=0, shamt=0, funct3=0x5, upper=0x00, opcode=0x13
    let instruction_word = 0x00005013; // 0000000 00000 00000 101 00000 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srli { rd, rs1, shamt } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(shamt, 0);
        }
        _ => panic!("Expected Srli instruction"),
    }
}

#[test]
fn max_registers() {
    // srli x31, x31, 10
    // rd=31, rs1=31, shamt=10, funct3=0x5, upper=0x00, opcode=0x13
    let instruction_word = 0x00AFDF93; // 0000000 01010 11111 101 11111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srli { rd, rs1, shamt } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(shamt, 10);
        }
        _ => panic!("Expected Srli instruction"),
    }
}

#[test]
fn max_shift_amount() {
    // srli x7, x8, 31
    // rd=7, rs1=8, shamt=31 (max for RV32), funct3=0x5, upper=0x00, opcode=0x13
    let instruction_word = 0x01F45393; // 0000000 11111 01000 101 00111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srli { rd, rs1, shamt } => {
            assert_eq!(rd, 7);
            assert_eq!(rs1, 8);
            assert_eq!(shamt, 31);
        }
        _ => panic!("Expected Srli instruction"),
    }
}

#[test]
fn min_shift_amount() {
    // srli x10, x15, 1
    // rd=10, rs1=15, shamt=1, funct3=0x5, upper=0x00, opcode=0x13
    let instruction_word = 0x0017D513; // 0000000 00001 01111 101 01010 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srli { rd, rs1, shamt } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 15);
            assert_eq!(shamt, 1);
        }
        _ => panic!("Expected Srli instruction"),
    }
}

#[test]
fn wrong_upper_bits() {
    // srli with wrong upper bits (should be 0x00, using 0x20 which would be SRAI)
    // rd=15, rs1=14, shamt=5, wrong upper bits, funct3=0x5, opcode=0x13
    let instruction_word = 0x40575793; // 0100000 00101 01110 101 01111 0010011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(_) => {}
        _ => panic!("Expected Unsupported instruction (this will be SRAI when implemented)"),
    }
}
