use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Srai {
        rd: 1,
        rs1: 2,
        shamt: 5,
    };
    assert_encode_decode(&instr, 0x40515093);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Srai {
        rd: 0,
        rs1: 0,
        shamt: 0,
    };
    assert_encode_decode(&instr, 0x40005013);
}

#[test]
fn max_registers() {
    let instr = Instruction::Srai {
        rd: 31,
        rs1: 31,
        shamt: 10,
    };
    assert_encode_decode(&instr, 0x40AFDF93);
}

#[test]
fn max_shift() {
    let instr = Instruction::Srai {
        rd: 5,
        rs1: 10,
        shamt: 31,
    };
    assert_encode_decode(&instr, 0x41F55293);
}

#[test]
fn zero_shift() {
    let instr = Instruction::Srai {
        rd: 7,
        rs1: 8,
        shamt: 0,
    };
    assert_encode_decode(&instr, 0x40045393);
}

#[test]
fn middle_shift() {
    let instr = Instruction::Srai {
        rd: 11,
        rs1: 12,
        shamt: 15,
    };
    assert_encode_decode(&instr, 0x40F65593);
}
