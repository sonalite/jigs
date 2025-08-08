use crate::{Instruction, tests::instruction::assert_encode_decode};

#[test]
fn basic() {
    let instruction = Instruction::Bgeu {
        rs1: 1,
        rs2: 2,
        imm: 8,
    };
    assert_encode_decode(&instruction, 0x0020F463);
}

#[test]
fn zero_registers() {
    let instruction = Instruction::Bgeu {
        rs1: 0,
        rs2: 0,
        imm: 0,
    };
    assert_encode_decode(&instruction, 0x00007063);
}

#[test]
fn max_registers() {
    let instruction = Instruction::Bgeu {
        rs1: 31,
        rs2: 31,
        imm: 16,
    };
    assert_encode_decode(&instruction, 0x01FFF863);
}

#[test]
fn negative_offset() {
    let instruction = Instruction::Bgeu {
        rs1: 5,
        rs2: 6,
        imm: -8,
    };
    assert_encode_decode(&instruction, 0xFE62FCE3);
}

#[test]
fn large_positive_offset() {
    let instruction = Instruction::Bgeu {
        rs1: 10,
        rs2: 11,
        imm: 4094,
    };
    assert_encode_decode(&instruction, 0x7EB57FE3);
}

#[test]
fn large_negative_offset() {
    let instruction = Instruction::Bgeu {
        rs1: 15,
        rs2: 16,
        imm: -4096,
    };
    assert_encode_decode(&instruction, 0x8107F063);
}
