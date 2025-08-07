use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Jal { rd: 1, imm: 8 };
    assert_eq!(format!("{}", instruction), "jal x1, 8");
}

#[test]
fn zero_register() {
    let instruction = Instruction::Jal { rd: 0, imm: 100 };
    assert_eq!(format!("{}", instruction), "jal x0, 100");
}

#[test]
fn max_register() {
    let instruction = Instruction::Jal { rd: 31, imm: 256 };
    assert_eq!(format!("{}", instruction), "jal x31, 256");
}

#[test]
fn negative_offset() {
    let instruction = Instruction::Jal { rd: 5, imm: -100 };
    assert_eq!(format!("{}", instruction), "jal x5, -100");
}

#[test]
fn zero_offset() {
    let instruction = Instruction::Jal { rd: 10, imm: 0 };
    assert_eq!(format!("{}", instruction), "jal x10, 0");
}
