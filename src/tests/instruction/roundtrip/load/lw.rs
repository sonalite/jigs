use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Lw {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_encode_decode(&instr, 0x06412083);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Lw {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x00002003);
}

#[test]
fn max_registers() {
    let instr = Instruction::Lw {
        rd: 31,
        rs1: 31,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x000FAF83);
}

#[test]
fn negative_offset() {
    let instr = Instruction::Lw {
        rd: 1,
        rs1: 2,
        imm: -100,
    };
    assert_encode_decode(&instr, 0xF9C12083);
}

#[test]
fn max_positive_offset() {
    let instr = Instruction::Lw {
        rd: 1,
        rs1: 2,
        imm: 2047,
    };
    assert_encode_decode(&instr, 0x7FF12083);
}

#[test]
fn max_negative_offset() {
    let instr = Instruction::Lw {
        rd: 1,
        rs1: 2,
        imm: -2048,
    };
    assert_encode_decode(&instr, 0x80012083);
}
