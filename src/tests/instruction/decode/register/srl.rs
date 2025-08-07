use crate::instruction::Instruction;

#[test]
fn basic() {
    // srl x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x5, funct7=0x00, opcode=0x33
    let instruction_word = 0x003150B3; // 0000000 00011 00010 101 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srl { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected Srl instruction"),
    }
}

#[test]
fn zero_registers() {
    // srl x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x5, funct7=0x00, opcode=0x33
    let instruction_word = 0x00005033; // 0000000 00000 00000 101 00000 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srl { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected Srl instruction"),
    }
}

#[test]
fn max_registers() {
    // srl x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x5, funct7=0x00, opcode=0x33
    let instruction_word = 0x01FFDFB3; // 0000000 11111 11111 101 11111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srl { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected Srl instruction"),
    }
}

#[test]
fn different_registers() {
    // srl x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x5, funct7=0x00, opcode=0x33
    let instruction_word = 0x0147D533; // 0000000 10100 01111 101 01010 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Srl { rd, rs1, rs2 } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 15);
            assert_eq!(rs2, 20);
        }
        _ => panic!("Expected Srl instruction"),
    }
}

#[test]
fn wrong_funct7() {
    // srl with wrong funct7 (should be 0x00, using 0x20)
    // rd=15, rs1=14, rs2=15, funct3=0x5, funct7=0x20, opcode=0x33
    let instruction_word = 0x20F757B3; // 0100000 01111 01110 101 01111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(_) => {}
        _ => panic!("Expected Unsupported instruction"),
    }
}
