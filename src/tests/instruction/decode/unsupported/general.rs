use crate::Instruction;

#[test]
fn unknown_opcode() {
    // Test instruction with unknown opcode (0x00 instead of valid opcodes)
    let instruction_word = 0x00000000;
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
