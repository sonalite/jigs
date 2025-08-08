use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Sh {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_encode_decode(&instr, 0x06209223);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Sh {
        rs1: 0,
        rs2: 0,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x00001023);
}

#[test]
fn max_registers() {
    let instr = Instruction::Sh {
        rs1: 31,
        rs2: 31,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x01FF9023);
}

#[test]
fn negative_offset() {
    let instr = Instruction::Sh {
        rs1: 1,
        rs2: 2,
        imm: -100,
    };
    assert_encode_decode(&instr, 0xF8209E23);
}

#[test]
fn max_positive_offset() {
    let instr = Instruction::Sh {
        rs1: 1,
        rs2: 2,
        imm: 2047,
    };
    assert_encode_decode(&instr, 0x7E209FA3);
}

#[test]
fn max_negative_offset() {
    let instr = Instruction::Sh {
        rs1: 1,
        rs2: 2,
        imm: -2048,
    };
    assert_encode_decode(&instr, 0x80209023);
}
