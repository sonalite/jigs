use crate::instruction::Instruction;

#[test]
fn basic() {
    // lb x1, 100(x2)
    // rd=1, rs1=2, imm=100
    let word = 0x06410083;
    match Instruction::decode(word) {
        Instruction::Lb { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Lb instruction"),
    }
}

#[test]
fn zero_registers() {
    // lb x0, 0(x0)
    let word = 0x00000003;
    match Instruction::decode(word) {
        Instruction::Lb { rd, rs1, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Lb instruction"),
    }
}

#[test]
fn max_registers() {
    // lb x31, 0(x31)
    let word = 0x000F8F83;
    match Instruction::decode(word) {
        Instruction::Lb { rd, rs1, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Lb instruction"),
    }
}

#[test]
fn negative_offset() {
    // lb x1, -100(x2)
    let word = 0xF9C10083;
    match Instruction::decode(word) {
        Instruction::Lb { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -100);
        }
        _ => panic!("Expected Lb instruction"),
    }
}

#[test]
fn max_positive_offset() {
    // lb x1, 2047(x2)
    let word = 0x7FF10083;
    match Instruction::decode(word) {
        Instruction::Lb { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Lb instruction"),
    }
}

#[test]
fn max_negative_offset() {
    // lb x1, -2048(x2)
    let word = 0x80010083;
    match Instruction::decode(word) {
        Instruction::Lb { rd, rs1, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Lb instruction"),
    }
}
