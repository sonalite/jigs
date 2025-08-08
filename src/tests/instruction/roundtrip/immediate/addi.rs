use crate::Instruction;
use crate::tests::instruction::assert_encode_decode;

#[test]
fn basic() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: 10,
    };
    assert_encode_decode(&instr, 0x00A10093);
}

#[test]
fn zero_registers() {
    let instr = Instruction::Addi {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_encode_decode(&instr, 0x00000013);
}

#[test]
fn max_registers() {
    let instr = Instruction::Addi {
        rd: 31,
        rs1: 31,
        imm: 100,
    };
    assert_encode_decode(&instr, 0x064F8F93);
}

#[test]
fn negative_immediate() {
    let instr = Instruction::Addi {
        rd: 5,
        rs1: 10,
        imm: -1,
    };
    assert_encode_decode(&instr, 0xFFF50293);
}

#[test]
fn max_positive_immediate() {
    let instr = Instruction::Addi {
        rd: 7,
        rs1: 8,
        imm: 2047,
    };
    assert_encode_decode(&instr, 0x7FF40393);
}

#[test]
fn max_negative_immediate() {
    let instr = Instruction::Addi {
        rd: 11,
        rs1: 12,
        imm: -2048,
    };
    assert_encode_decode(&instr, 0x80060593);
}
