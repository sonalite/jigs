use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Ecall;
    assert_eq!(format!("{}", instruction), "ecall");
}
