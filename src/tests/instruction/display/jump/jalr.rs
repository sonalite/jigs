use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Jalr {
        rd: 1,
        rs1: 2,
        imm: 8,
    };
    assert_eq!(format!("{}", instruction), "jalr x1, 8(x2)");
}

#[test]
fn zero_registers() {
    let instruction = Instruction::Jalr {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", instruction), "jalr x0, 0(x0)");
}

#[test]
fn max_registers() {
    let instruction = Instruction::Jalr {
        rd: 31,
        rs1: 31,
        imm: 16,
    };
    assert_eq!(format!("{}", instruction), "jalr x31, 16(x31)");
}

#[test]
fn negative_offset() {
    let instruction = Instruction::Jalr {
        rd: 5,
        rs1: 6,
        imm: -8,
    };
    assert_eq!(format!("{}", instruction), "jalr x5, -8(x6)");
}

#[test]
fn different_registers() {
    let instruction = Instruction::Jalr {
        rd: 10,
        rs1: 20,
        imm: 100,
    };
    assert_eq!(format!("{}", instruction), "jalr x10, 100(x20)");
}
