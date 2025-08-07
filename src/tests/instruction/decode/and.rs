use crate::instruction::Instruction;

#[test]
fn basic() {
    // and x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x7, funct7=0x00, opcode=0x33
    let instruction_word = 0x003170B3; // 0000000 00011 00010 111 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected And instruction"),
    }
}

#[test]
fn zero_registers() {
    // and x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x7, funct7=0x00, opcode=0x33
    let instruction_word = 0x00007033; // 0000000 00000 00000 111 00000 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected And instruction"),
    }
}

#[test]
fn max_registers() {
    // and x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x7, funct7=0x00, opcode=0x33
    let instruction_word = 0x01FFFFB3; // 0000000 11111 11111 111 11111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected And instruction"),
    }
}

#[test]
fn different_registers() {
    // and x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x7, funct7=0x00, opcode=0x33
    let instruction_word = 0x0147F533; // 0000000 10100 01111 111 01010 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::And { rd, rs1, rs2 } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 15);
            assert_eq!(rs2, 20);
        }
        _ => panic!("Expected And instruction"),
    }
}

#[test]
fn wrong_funct7() {
    // and with wrong funct7 (should be 0x00, using 0x20)
    // rd=15, rs1=14, rs2=15, funct3=0x7, funct7=0x20, opcode=0x33
    let instruction_word = 0x20F777B3; // 0100000 01111 01110 111 01111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(_) => {}
        _ => panic!("Expected Unsupported instruction"),
    }
}
