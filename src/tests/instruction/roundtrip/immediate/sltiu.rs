use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Sltiu {
        rd: 1,
        rs1: 2,
        imm: 10,
    };
    assert_encode_decode(&instr, 0x00A13093);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Sltiu {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x00003013);
}

#[test]
fn max_registers() {
    let instr = Instruction::Sltiu {
        rd: 31,
        rs1: 31,
        imm: 100,
    };
    assert_encode_decode(&instr, 0x064FBF93);
}

#[test]
fn negative_immediate() {
    let instr = Instruction::Sltiu {
        rd: 5,
        rs1: 10,
        imm: -1,
    };
    assert_encode_decode(&instr, 0xFFF53293);
}

#[test]
fn max_positive_immediate() {
    let instr = Instruction::Sltiu {
        rd: 7,
        rs1: 8,
        imm: 2047,
    };
    assert_encode_decode(&instr, 0x7FF43393);
}

#[test]
fn max_negative_immediate() {
    let instr = Instruction::Sltiu {
        rd: 11,
        rs1: 12,
        imm: -2048,
    };
    assert_encode_decode(&instr, 0x80063593);
}
