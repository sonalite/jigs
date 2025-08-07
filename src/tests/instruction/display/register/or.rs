use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Or {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(format!("{}", instruction), "or x1, x2, x3");
}

#[test]
fn zero_registers() {
    let instruction = Instruction::Or {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_eq!(format!("{}", instruction), "or x0, x0, x0");
}

#[test]
fn max_registers() {
    let instruction = Instruction::Or {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_eq!(format!("{}", instruction), "or x31, x31, x31");
}

#[test]
fn different_registers() {
    let instruction = Instruction::Or {
        rd: 10,
        rs1: 15,
        rs2: 20,
    };
    assert_eq!(format!("{}", instruction), "or x10, x15, x20");
}
