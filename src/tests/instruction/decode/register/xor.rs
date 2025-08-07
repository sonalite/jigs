use crate::instruction::Instruction;

#[test]
fn basic() {
    // xor x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x4, funct7=0x00, opcode=0x33
    let word = 0x003140b3; // 0000000 00011 00010 100 00001 0110011
    let instruction = Instruction::decode(word);
    match instruction {
        Instruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected Xor instruction"),
    }
}

#[test]
fn zero_registers() {
    // xor x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x4, funct7=0x00, opcode=0x33
    let word = 0x00004033; // 0000000 00000 00000 100 00000 0110011
    let instruction = Instruction::decode(word);
    match instruction {
        Instruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected Xor instruction"),
    }
}

#[test]
fn max_registers() {
    let word = 0x01ffcfb3; // xor x31, x31, x31
    let instruction = Instruction::decode(word);
    match instruction {
        Instruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected Xor instruction"),
    }
}

#[test]
fn different_registers() {
    // xor x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x4, funct7=0x00, opcode=0x33
    let word = 0x0147c533; // 0000000 10100 01111 100 01010 0110011
    let instruction = Instruction::decode(word);
    match instruction {
        Instruction::Xor { rd, rs1, rs2 } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 15);
            assert_eq!(rs2, 20);
        }
        _ => panic!("Expected Xor instruction"),
    }
}

#[test]
fn wrong_funct7() {
    let word = 0x40f747b3; // Invalid XOR with wrong funct7
    let instruction = Instruction::decode(word);
    match instruction {
        Instruction::Unsupported(_) => {}
        _ => panic!("Expected Unsupported instruction"),
    }
}
