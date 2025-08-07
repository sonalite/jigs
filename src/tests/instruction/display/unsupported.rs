use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Unsupported(0xDEADBEEF);
    assert_eq!(format!("{}", instruction), "unsupported: 0xdeadbeef");
}

#[test]
fn zero() {
    let instruction = Instruction::Unsupported(0x00000000);
    assert_eq!(format!("{}", instruction), "unsupported: 0x00000000");
}
