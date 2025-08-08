use crate::instruction::Instruction;

#[test]
fn invalid_immediate() {
    // System instruction with invalid immediate (not 0x000 or 0x001)
    // Opcode 0x73, funct3 = 0, rd = 0, rs1 = 0, but imm = 0x002
    let word = 0x00200073; // imm = 0x002 (not ECALL or EBREAK)
    assert_eq!(Instruction::decode(word), Instruction::Unsupported(word));
}
