use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Sw {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_encode_decode(&instr, 0x0620A223);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Sw {
        rs1: 0,
        rs2: 0,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x00002023);
}

#[test]
fn max_registers() {
    let instr = Instruction::Sw {
        rs1: 31,
        rs2: 31,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x01FFA023);
}

#[test]
fn negative_offset() {
    let instr = Instruction::Sw {
        rs1: 1,
        rs2: 2,
        imm: -100,
    };
    assert_encode_decode(&instr, 0xF820AE23);
}

#[test]
fn max_positive_offset() {
    let instr = Instruction::Sw {
        rs1: 1,
        rs2: 2,
        imm: 2047,
    };
    assert_encode_decode(&instr, 0x7E20AFA3);
}

#[test]
fn max_negative_offset() {
    let instr = Instruction::Sw {
        rs1: 1,
        rs2: 2,
        imm: -2048,
    };
    assert_encode_decode(&instr, 0x8020A023);
}
