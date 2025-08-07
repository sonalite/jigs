use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: 10,
    };
    assert_eq!(format!("{}", instruction), "addi x1, x2, 10");
}

#[test]
fn zero_registers() {
    let instruction = Instruction::Addi {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", instruction), "addi x0, x0, 0");
}

#[test]
fn max_registers() {
    let instruction = Instruction::Addi {
        rd: 31,
        rs1: 31,
        imm: 100,
    };
    assert_eq!(format!("{}", instruction), "addi x31, x31, 100");
}

#[test]
fn negative_immediate() {
    let instruction = Instruction::Addi {
        rd: 5,
        rs1: 10,
        imm: -1,
    };
    assert_eq!(format!("{}", instruction), "addi x5, x10, -1");
}

#[test]
fn max_positive_immediate() {
    let instruction = Instruction::Addi {
        rd: 7,
        rs1: 8,
        imm: 2047,
    };
    assert_eq!(format!("{}", instruction), "addi x7, x8, 2047");
}

#[test]
fn max_negative_immediate() {
    let instruction = Instruction::Addi {
        rd: 11,
        rs1: 12,
        imm: -2048,
    };
    assert_eq!(format!("{}", instruction), "addi x11, x12, -2048");
}
