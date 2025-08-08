use crate::Instruction;
use crate::tests::instruction::assert_encode_decode;

#[test]
fn basic() {
    assert_encode_decode(&Instruction::Ecall, 0x00000073);
}

#[test]
fn verify_exact_encoding() {
    assert_encode_decode(&Instruction::Ecall, 0x00000073);
}
