use crate::instruction::Instruction;

#[test]
fn basic() {
    // lhu x1, 100(x2)
    // rd=1, rs1=2, imm=100
    let word = 0x06415083;
    match Instruction::decode(word) {
        Instruction::Lhu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Lhu instruction"),
    }
}

#[test]
fn zero_registers() {
    // lhu x0, 0(x0)
    let word = 0x00005003;
    match Instruction::decode(word) {
        Instruction::Lhu { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Lhu instruction"),
    }
}

#[test]
fn max_registers() {
    // lhu x31, 0(x31)
    let word = 0x000FDF83;
    match Instruction::decode(word) {
        Instruction::Lhu { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Lhu instruction"),
    }
}

#[test]
fn negative_offset() {
    // lhu x1, -100(x2)
    let word = 0xF9C15083;
    match Instruction::decode(word) {
        Instruction::Lhu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -100);
        }
        _ => panic!("Expected Lhu instruction"),
    }
}

#[test]
fn max_positive_offset() {
    // lhu x1, 2047(x2)
    let word = 0x7FF15083;
    match Instruction::decode(word) {
        Instruction::Lhu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Lhu instruction"),
    }
}

#[test]
fn max_negative_offset() {
    // lhu x1, -2048(x2)
    let word = 0x80015083;
    match Instruction::decode(word) {
        Instruction::Lhu { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Lhu instruction"),
    }
}
