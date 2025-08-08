use crate::instruction::{EncodeError, Instruction};

#[test]
fn rd_out_of_bounds() {
    let instr = Instruction::Lbu {
        rd: 32,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(
        instr.encode().unwrap_err(),
        EncodeError::InvalidRegister("rd", 32)
    );
}

#[test]
fn rs1_out_of_bounds() {
    let instr = Instruction::Lbu {
        rd: 0,
        rs1: 32,
        imm: 0,
    };
    assert_eq!(
        instr.encode().unwrap_err(),
        EncodeError::InvalidRegister("rs1", 32)
    );
}

#[test]
fn imm_out_of_bounds_positive() {
    let instr = Instruction::Lbu {
        rd: 0,
        rs1: 0,
        imm: 2048,
    };
    assert_eq!(
        instr.encode().unwrap_err(),
        EncodeError::InvalidImmediate("imm", 2048)
    );
}

#[test]
fn imm_out_of_bounds_negative() {
    let instr = Instruction::Lbu {
        rd: 0,
        rs1: 0,
        imm: -2049,
    };
    assert_eq!(
        instr.encode().unwrap_err(),
        EncodeError::InvalidImmediate("imm", -2049)
    );
}
