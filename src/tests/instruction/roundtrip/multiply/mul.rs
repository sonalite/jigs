use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instr = Instruction::Mul {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_encode_decode(&instr, 0x023100B3);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Mul {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_encode_decode(&instr, 0x02000033);
}

#[test]
fn max_registers() {
    let instr = Instruction::Mul {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_encode_decode(&instr, 0x03FF8FB3);
}

#[test]
fn different_registers() {
    let instr = Instruction::Mul {
        rd: 10,
        rs1: 15,
        rs2: 20,
    };
    assert_encode_decode(&instr, 0x03478533);
}
