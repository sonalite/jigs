mod decode;
mod display;
mod roundtrip;

use crate::Instruction;

/// Test utility that encodes an instruction and verifies it decodes back to the same instruction
pub fn assert_encode_decode(instr: &Instruction, expected_encoding: u32) {
    // Test encoding
    let encoded = instr.encode().unwrap();
    assert_eq!(encoded, expected_encoding);

    // Test round-trip decode
    let decoded = Instruction::decode(encoded);
    assert_eq!(&decoded, instr);
}
