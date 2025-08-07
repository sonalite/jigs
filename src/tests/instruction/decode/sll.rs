use crate::instruction::Instruction;

#[test]
fn basic() {
    // sll x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x1, funct7=0x00, opcode=0x33
    let instruction_word = 0x003110B3; // 0000000 00011 00010 001 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sll { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected Sll instruction"),
    }
}

#[test]
fn zero_registers() {
    // sll x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x1, funct7=0x00, opcode=0x33
    let instruction_word = 0x00001033; // 0000000 00000 00000 001 00000 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sll { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected Sll instruction"),
    }
}

#[test]
fn max_registers() {
    // sll x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x1, funct7=0x00, opcode=0x33
    let instruction_word = 0x01FF9FB3; // 0000000 11111 11111 001 11111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sll { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected Sll instruction"),
    }
}

#[test]
fn different_registers() {
    // sll x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x1, funct7=0x00, opcode=0x33
    let instruction_word = 0x01479533; // 0000000 10100 01111 001 01010 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sll { rd, rs1, rs2 } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 15);
            assert_eq!(rs2, 20);
        }
        _ => panic!("Expected Sll instruction"),
    }
}

#[test]
fn wrong_funct7() {
    // sll with wrong funct7 (should be 0x00, using 0x20)
    // rd=15, rs1=14, rs2=15, funct3=0x1, funct7=0x20, opcode=0x33
    let instruction_word = 0x20F717B3; // 0100000 01111 01110 001 01111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Unsupported(_) => {}
        _ => panic!("Expected Unsupported instruction"),
    }
}
