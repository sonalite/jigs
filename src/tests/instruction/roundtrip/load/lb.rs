use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Lb {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_encode_decode(&instr, 0x06410083);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Lb {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x00000003);
}

#[test]
fn max_registers() {
    let instr = Instruction::Lb {
        rd: 31,
        rs1: 31,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x000F8F83);
}

#[test]
fn negative_offset() {
    let instr = Instruction::Lb {
        rd: 1,
        rs1: 2,
        imm: -100,
    };
    assert_encode_decode(&instr, 0xF9C10083);
}

#[test]
fn max_positive_offset() {
    let instr = Instruction::Lb {
        rd: 1,
        rs1: 2,
        imm: 2047,
    };
    assert_encode_decode(&instr, 0x7FF10083);
}

#[test]
fn max_negative_offset() {
    let instr = Instruction::Lb {
        rd: 1,
        rs1: 2,
        imm: -2048,
    };
    assert_encode_decode(&instr, 0x80010083);
}
