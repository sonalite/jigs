use crate::Instruction;
use crate::tests::instruction::assert_encode_decode;

#[test]
fn basic() {
    let instr = Instruction::Sra {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_encode_decode(&instr, 0x403150B3);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Sra {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_encode_decode(&instr, 0x40005033);
}

#[test]
fn max_registers() {
    let instr = Instruction::Sra {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert_encode_decode(&instr, 0x41FFDFB3);
}

#[test]
fn different_registers() {
    let instr = Instruction::Sra {
        rd: 10,
        rs1: 15,
        rs2: 20,
    };
    assert_encode_decode(&instr, 0x4147D533);
}
