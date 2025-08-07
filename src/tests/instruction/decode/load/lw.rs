use crate::instruction::Instruction;

#[test]
fn basic() {
    // lw x1, 100(x2)
    // rd=1, rs1=2, imm=100
    let word = 0x06412083;
    match Instruction::decode(word) {
        Instruction::Lw { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Lw instruction"),
    }
}

#[test]
fn zero_registers() {
    // lw x0, 0(x0)
    let word = 0x00002003;
    match Instruction::decode(word) {
        Instruction::Lw { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Lw instruction"),
    }
}

#[test]
fn max_registers() {
    // lw x31, 0(x31)
    let word = 0x000FAF83;
    match Instruction::decode(word) {
        Instruction::Lw { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Lw instruction"),
    }
}

#[test]
fn negative_offset() {
    // lw x1, -100(x2)
    let word = 0xF9C12083;
    match Instruction::decode(word) {
        Instruction::Lw { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -100);
        }
        _ => panic!("Expected Lw instruction"),
    }
}

#[test]
fn max_positive_offset() {
    // lw x1, 2047(x2)
    let word = 0x7FF12083;
    match Instruction::decode(word) {
        Instruction::Lw { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Lw instruction"),
    }
}

#[test]
fn max_negative_offset() {
    // lw x1, -2048(x2)
    let word = 0x80012083;
    match Instruction::decode(word) {
        Instruction::Lw { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Lw instruction"),
    }
}
