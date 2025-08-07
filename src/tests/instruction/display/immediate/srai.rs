use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Srai {
        rd: 1,
        rs1: 2,
        shamt: 5,
    };
    assert_eq!(format!("{}", instruction), "srai x1, x2, 5");
}

#[test]
fn zero_registers() {
    let instruction = Instruction::Srai {
        rd: 0,
        rs1: 0,
        shamt: 0,
    };
    assert_eq!(format!("{}", instruction), "srai x0, x0, 0");
}

#[test]
fn max_registers() {
    let instruction = Instruction::Srai {
        rd: 31,
        rs1: 31,
        shamt: 10,
    };
    assert_eq!(format!("{}", instruction), "srai x31, x31, 10");
}

#[test]
fn max_shift_amount() {
    let instruction = Instruction::Srai {
        rd: 7,
        rs1: 8,
        shamt: 31,
    };
    assert_eq!(format!("{}", instruction), "srai x7, x8, 31");
}

#[test]
fn min_shift_amount() {
    let instruction = Instruction::Srai {
        rd: 10,
        rs1: 15,
        shamt: 1,
    };
    assert_eq!(format!("{}", instruction), "srai x10, x15, 1");
}