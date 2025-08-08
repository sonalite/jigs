use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Slli {
        rd: 1,
        rs1: 2,
        shamt: 5,
    };
    assert_encode_decode(&instr, 0x00511093);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Slli {
        rd: 0,
        rs1: 0,
        shamt: 0,
    };
    assert_encode_decode(&instr, 0x00001013);
}

#[test]
fn max_registers() {
    let instr = Instruction::Slli {
        rd: 31,
        rs1: 31,
        shamt: 10,
    };
    assert_encode_decode(&instr, 0x00AF9F93);
}

#[test]
fn max_shift() {
    let instr = Instruction::Slli {
        rd: 5,
        rs1: 10,
        shamt: 31,
    };
    assert_encode_decode(&instr, 0x01F51293);
}

#[test]
fn zero_shift() {
    let instr = Instruction::Slli {
        rd: 7,
        rs1: 8,
        shamt: 0,
    };
    assert_encode_decode(&instr, 0x00041393);
}

#[test]
fn middle_shift() {
    let instr = Instruction::Slli {
        rd: 11,
        rs1: 12,
        shamt: 15,
    };
    assert_encode_decode(&instr, 0x00F61593);
}
