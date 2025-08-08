use crate::Instruction;

#[test]
fn invalid_funct3() {
    // Store instruction with invalid funct3=0x3 (valid are 0x0, 0x1, 0x2)
    // opcode=0x23 (store), funct3=0x3 (invalid)
    let word = 0x00003023;
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}

#[test]
fn invalid_funct3_4() {
    // Store instruction with invalid funct3=0x4
    let word = 0x00004023;
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}

#[test]
fn invalid_funct3_5() {
    // Store instruction with invalid funct3=0x5
    let word = 0x00005023;
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}

#[test]
fn invalid_funct3_6() {
    // Store instruction with invalid funct3=0x6
    let word = 0x00006023;
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}

#[test]
fn invalid_funct3_7() {
    // Store instruction with invalid funct3=0x7
    let word = 0x00007023;
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}
