use crate::{EncodeError, Instruction};

#[test]
fn invalid_rs1() {
    let instruction = Instruction::Bltu {
        rs1: 32,
        rs2: 0,
        imm: 0,
    };
    assert_eq!(
        instruction.encode(),
        Err(EncodeError::InvalidRegister("rs1", 32))
    );
}

#[test]
fn invalid_rs2() {
    let instruction = Instruction::Bltu {
        rs1: 0,
        rs2: 32,
        imm: 0,
    };
    assert_eq!(
        instruction.encode(),
        Err(EncodeError::InvalidRegister("rs2", 32))
    );
}

#[test]
fn invalid_imm_odd() {
    let instruction = Instruction::Bltu {
        rs1: 0,
        rs2: 0,
        imm: 1,
    };
    assert_eq!(
        instruction.encode(),
        Err(EncodeError::InvalidImmediate("imm", 1))
    );
}

#[test]
fn invalid_imm_too_large() {
    let instruction = Instruction::Bltu {
        rs1: 0,
        rs2: 0,
        imm: 4096,
    };
    assert_eq!(
        instruction.encode(),
        Err(EncodeError::InvalidImmediate("imm", 4096))
    );
}

#[test]
fn invalid_imm_too_small() {
    let instruction = Instruction::Bltu {
        rs1: 0,
        rs2: 0,
        imm: -4098,
    };
    assert_eq!(
        instruction.encode(),
        Err(EncodeError::InvalidImmediate("imm", -4098))
    );
}
