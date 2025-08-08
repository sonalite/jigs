use crate::Instruction;
use crate::tests::instruction::assert_encode_decode;

#[test]
fn basic() {
    assert_encode_decode(&Instruction::Auipc { rd: 1, imm: 74565 }, 0x12345097);
}

#[test]
fn zero_register() {
    assert_encode_decode(&Instruction::Auipc { rd: 0, imm: 703710 }, 0xABCDE017);
}

#[test]
fn max_register() {
    assert_encode_decode(
        &Instruction::Auipc {
            rd: 31,
            imm: 344865,
        },
        0x54321F97,
    );
}

#[test]
fn zero_immediate() {
    assert_encode_decode(&Instruction::Auipc { rd: 5, imm: 0 }, 0x00000297);
}

#[test]
fn max_immediate() {
    assert_encode_decode(
        &Instruction::Auipc {
            rd: 10,
            imm: 1048575,
        },
        0xFFFFF517,
    );
}

#[test]
fn small_immediate() {
    assert_encode_decode(&Instruction::Auipc { rd: 15, imm: 1 }, 0x00001797);
}

#[test]
fn different_registers() {
    assert_encode_decode(
        &Instruction::Auipc {
            rd: 20,
            imm: 524288,
        },
        0x80000A17,
    );
}
