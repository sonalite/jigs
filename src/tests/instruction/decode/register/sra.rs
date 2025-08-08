use crate::instruction::Instruction;

#[test]
fn basic() {
    // sra x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x5, funct7=0x20, opcode=0x33
    let instruction_word = 0x403150B3; // 0100000 00011 00010 101 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sra { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected Sra instruction"),
    }
}

#[test]
fn zero_registers() {
    // sra x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x5, funct7=0x20, opcode=0x33
    let instruction_word = 0x40005033; // 0100000 00000 00000 101 00000 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sra { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected Sra instruction"),
    }
}

#[test]
fn max_registers() {
    // sra x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x5, funct7=0x20, opcode=0x33
    let instruction_word = 0x41FFDFB3; // 0100000 11111 11111 101 11111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sra { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected Sra instruction"),
    }
}

#[test]
fn different_registers() {
    // sra x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x5, funct7=0x20, opcode=0x33
    let instruction_word = 0x4147D533; // 0100000 10100 01111 101 01010 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sra { rd, rs1, rs2 } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 15);
            assert_eq!(rs2, 20);
        }
        _ => panic!("Expected Sra instruction"),
    }
}

#[test]
fn wrong_funct7() {
    // sra with wrong funct7 (should be 0x20, using 0x10)
    // rd=15, rs1=14, rs2=15, funct3=0x5, funct7=0x10, opcode=0x33
    let instruction_word = 0x20F757B3; // 0010000 01111 01110 101 01111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(_) => {}
        _ => panic!("Expected Unsupported instruction"),
    }
}
