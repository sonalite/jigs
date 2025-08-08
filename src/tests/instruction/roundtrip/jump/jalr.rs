use crate::Instruction;
use crate::tests::instruction::assert_encode_decode;

#[test]
fn basic() {
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 0,
            rs1: 5,
            imm: 8,
        },
        0x00828067,
    );
}

#[test]
fn zero_registers() {
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 0,
            rs1: 0,
            imm: 8,
        },
        0x00800067,
    );
}

#[test]
fn max_registers() {
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 31,
            rs1: 31,
            imm: 8,
        },
        0x008F8FE7,
    );
}

#[test]
fn negative_immediate() {
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 0,
            rs1: 5,
            imm: -8,
        },
        0xFF828067,
    );
}

#[test]
fn max_immediate() {
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 0,
            rs1: 5,
            imm: 2047,
        },
        0x7FF28067,
    );
}

#[test]
fn min_immediate() {
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 0,
            rs1: 5,
            imm: -2048,
        },
        0x80028067,
    );
}

#[test]
fn different_registers() {
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 5,
            rs1: 5,
            imm: 8,
        },
        0x008282E7,
    );
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 10,
            rs1: 10,
            imm: 8,
        },
        0x00850567,
    );
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 15,
            rs1: 15,
            imm: 8,
        },
        0x008787E7,
    );
}

#[test]
fn different_immediates() {
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 0,
            rs1: 5,
            imm: 24,
        },
        0x01828067,
    );
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 0,
            rs1: 5,
            imm: 100,
        },
        0x06428067,
    );
    assert_encode_decode(
        &Instruction::Jalr {
            rd: 0,
            rs1: 5,
            imm: 1000,
        },
        0x3E828067,
    );
}
