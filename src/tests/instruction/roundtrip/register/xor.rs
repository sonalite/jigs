use crate::Instruction;
use crate::tests::instruction::assert_encode_decode;

#[test]
fn basic() {
    let instr = Instruction::Xor {
        rd: 4,
        rs1: 2,
        rs2: 3,
    };
    assert_encode_decode(&instr, 0x00314233);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Xor {
        rd: 0,
        rs1: 0,
        rs2: 0,
    };
    assert_encode_decode(&instr, 0x00004033);
}

#[test]
fn max_registers() {
    let instr = Instruction::Xor {
        rd: 31,
        rs1: 30,
        rs2: 31,
    };
    assert_encode_decode(&instr, 0x01FF4FB3);
}

#[test]
fn different_registers() {
    let instr = Instruction::Xor {
        rd: 13,
        rs1: 11,
        rs2: 10,
    };
    assert_encode_decode(&instr, 0x00A5C6B3);
}
