use crate::{EncodeError, Instruction};

#[test]
fn invalid_rd() {
    let instr = Instruction::Jalr {
        rd: 32,
        rs1: 5,
        imm: 8,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 32)));
}

#[test]
fn invalid_rd_too_large() {
    let instr = Instruction::Jalr {
        rd: 255,
        rs1: 5,
        imm: 8,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 255)));
}

#[test]
fn invalid_rs1() {
    let instr = Instruction::Jalr {
        rd: 1,
        rs1: 32,
        imm: 8,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rs1", 32)));
}

#[test]
fn invalid_rs1_too_large() {
    let instr = Instruction::Jalr {
        rd: 1,
        rs1: 255,
        imm: 8,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidRegister("rs1", 255))
    );
}

#[test]
fn immediate_too_large() {
    let instr = Instruction::Jalr {
        rd: 1,
        rs1: 5,
        imm: 2048,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", 2048))
    );
}

#[test]
fn immediate_too_small() {
    let instr = Instruction::Jalr {
        rd: 1,
        rs1: 5,
        imm: -2049,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", -2049))
    );
}

#[test]
fn boundary_values() {
    // Valid max immediate
    let instr = Instruction::Jalr {
        rd: 1,
        rs1: 5,
        imm: 2047,
    };
    assert!(instr.encode().is_ok());

    // Invalid: max + 1
    let instr = Instruction::Jalr {
        rd: 1,
        rs1: 5,
        imm: 2048,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", 2048))
    );

    // Valid min immediate
    let instr = Instruction::Jalr {
        rd: 1,
        rs1: 5,
        imm: -2048,
    };
    assert!(instr.encode().is_ok());

    // Invalid: min - 1
    let instr = Instruction::Jalr {
        rd: 1,
        rs1: 5,
        imm: -2049,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", -2049))
    );
}
