use crate::instruction::Instruction;

#[test]
fn invalid_immediate() {
    // System instruction with invalid immediate (not 0x000 or 0x001)
    // Opcode 0x73, funct3 = 0, rd = 0, rs1 = 0, but imm = 0x002
    let word = 0x00200073; // imm = 0x002 (not ECALL or EBREAK)
    match Instruction::decode(word) {
        Instruction::Unsupported(w) => assert_eq!(w, word),
        other => panic!("Expected Unsupported(0x{:08x}), got {:?}", word, other),
    }
}
