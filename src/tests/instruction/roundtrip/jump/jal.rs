use crate::Instruction;
use crate::tests::instruction::assert_encode_decode;

#[test]
fn basic() {
    assert_encode_decode(&Instruction::Jal { rd: 1, imm: 8 }, 0x008000EF);
}

#[test]
fn zero_register() {
    assert_encode_decode(&Instruction::Jal { rd: 0, imm: 8 }, 0x0080006F);
}

#[test]
fn max_register() {
    assert_encode_decode(&Instruction::Jal { rd: 31, imm: 8 }, 0x00800FEF);
}

#[test]
fn negative_offset() {
    assert_encode_decode(&Instruction::Jal { rd: 1, imm: -8 }, 0xFF9FF0EF);
}

#[test]
fn large_positive_offset() {
    assert_encode_decode(
        &Instruction::Jal {
            rd: 1,
            imm: 1048574,
        },
        0x7FFFF0EF,
    );
}

#[test]
fn large_negative_offset() {
    assert_encode_decode(
        &Instruction::Jal {
            rd: 1,
            imm: -1048576,
        },
        0x800000EF,
    );
}

#[test]
fn different_registers() {
    assert_encode_decode(&Instruction::Jal { rd: 5, imm: 8 }, 0x008002EF);
    assert_encode_decode(&Instruction::Jal { rd: 10, imm: 8 }, 0x0080056F);
    assert_encode_decode(&Instruction::Jal { rd: 15, imm: 8 }, 0x008007EF);
}

#[test]
fn different_offsets() {
    assert_encode_decode(&Instruction::Jal { rd: 1, imm: 24 }, 0x018000EF);
    assert_encode_decode(&Instruction::Jal { rd: 1, imm: 100 }, 0x064000EF);
    assert_encode_decode(&Instruction::Jal { rd: 1, imm: 1000 }, 0x3E8000EF);
}
