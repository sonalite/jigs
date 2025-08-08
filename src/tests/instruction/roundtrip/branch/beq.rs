use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instruction = Instruction::Beq {
        rs1: 1,
        rs2: 2,
        imm: 8,
    };
    assert_encode_decode(&instruction, 0x00208463);
}

#[test]
fn zero_registers() {
    let instruction = Instruction::Beq {
        rs1: 0,
        rs2: 0,
        imm: 0,
    };
    assert_encode_decode(&instruction, 0x00000063);
}

#[test]
fn max_registers() {
    let instruction = Instruction::Beq {
        rs1: 31,
        rs2: 31,
        imm: 16,
    };
    assert_encode_decode(&instruction, 0x01FF8863);
}

#[test]
fn negative_offset() {
    let instruction = Instruction::Beq {
        rs1: 5,
        rs2: 6,
        imm: -8,
    };
    assert_encode_decode(&instruction, 0xFE628CE3);
}

#[test]
fn large_positive_offset() {
    let instruction = Instruction::Beq {
        rs1: 10,
        rs2: 11,
        imm: 4094,
    };
    assert_encode_decode(&instruction, 0x7EB50FE3);
}

#[test]
fn large_negative_offset() {
    let instruction = Instruction::Beq {
        rs1: 15,
        rs2: 16,
        imm: -4096,
    };
    assert_encode_decode(&instruction, 0x81078063);
}
