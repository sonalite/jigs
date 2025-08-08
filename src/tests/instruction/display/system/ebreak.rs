use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Ebreak;
    assert_eq!(format!("{}", instruction), "ebreak");
}
