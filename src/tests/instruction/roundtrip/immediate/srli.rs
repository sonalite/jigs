use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Srli {
        rd: 1,
        rs1: 2,
        shamt: 5,
    };
    assert_encode_decode(&instr, 0x00515093);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Srli {
        rd: 0,
        rs1: 0,
        shamt: 0,
    };
    assert_encode_decode(&instr, 0x00005013);
}

#[test]
fn max_registers() {
    let instr = Instruction::Srli {
        rd: 31,
        rs1: 31,
        shamt: 10,
    };
    assert_encode_decode(&instr, 0x00AFDF93);
}

#[test]
fn max_shift() {
    let instr = Instruction::Srli {
        rd: 5,
        rs1: 10,
        shamt: 31,
    };
    assert_encode_decode(&instr, 0x01F55293);
}

#[test]
fn zero_shift() {
    let instr = Instruction::Srli {
        rd: 7,
        rs1: 8,
        shamt: 0,
    };
    assert_encode_decode(&instr, 0x00045393);
}

#[test]
fn middle_shift() {
    let instr = Instruction::Srli {
        rd: 11,
        rs1: 12,
        shamt: 15,
    };
    assert_encode_decode(&instr, 0x00F65593);
}
