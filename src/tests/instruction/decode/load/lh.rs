use crate::instruction::Instruction;

#[test]
fn basic() {
    // lh x1, 100(x2)
    // rd=1, rs1=2, imm=100
    let word = 0x06411083;
    match Instruction::decode(word) {
        Instruction::Lh { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Lh instruction"),
    }
}

#[test]
fn zero_registers() {
    // lh x0, 0(x0)
    let word = 0x00001003;
    match Instruction::decode(word) {
        Instruction::Lh { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Lh instruction"),
    }
}

#[test]
fn max_registers() {
    // lh x31, 0(x31)
    let word = 0x000F9F83;
    match Instruction::decode(word) {
        Instruction::Lh { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Lh instruction"),
    }
}

#[test]
fn negative_offset() {
    // lh x1, -100(x2)
    let word = 0xF9C11083;
    match Instruction::decode(word) {
        Instruction::Lh { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -100);
        }
        _ => panic!("Expected Lh instruction"),
    }
}

#[test]
fn max_positive_offset() {
    // lh x1, 2047(x2)
    let word = 0x7FF11083;
    match Instruction::decode(word) {
        Instruction::Lh { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Lh instruction"),
    }
}

#[test]
fn max_negative_offset() {
    // lh x1, -2048(x2)
    let word = 0x80011083;
    match Instruction::decode(word) {
        Instruction::Lh { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Lh instruction"),
    }
}
