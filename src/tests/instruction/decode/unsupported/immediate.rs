use crate::Instruction;

#[test]
fn slli_invalid_upper_bits() {
    // SLLI with non-zero upper bits (should be 0x00)
    // This sets upper bits to 0x01 instead of 0x00
    let word = 0x02051093; // rd=1, rs1=10, funct3=1, but upper bits are invalid
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}

#[test]
fn srli_srai_invalid_upper_bits() {
    // SRLI/SRAI with invalid upper bits (should be 0x00 or 0x20)
    // This sets upper bits to 0x10 which is neither SRLI nor SRAI
    let word = 0x20555293; // rd=5, rs1=10, funct3=5, but upper bits are 0x10
    let inst = Instruction::decode(word);
    assert_eq!(inst, Instruction::Unsupported(word));
}
