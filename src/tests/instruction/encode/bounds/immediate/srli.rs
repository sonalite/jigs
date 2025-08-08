use crate::{EncodeError, Instruction};

#[test]
fn rd_out_of_bounds() {
    let instr = Instruction::Srli {
        rd: 32,
        rs1: 1,
        shamt: 5,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 32)));
}

#[test]
fn rd_max_out_of_bounds() {
    let instr = Instruction::Srli {
        rd: 255,
        rs1: 1,
        shamt: 5,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 255)));
}

#[test]
fn rs1_out_of_bounds() {
    let instr = Instruction::Srli {
        rd: 1,
        rs1: 32,
        shamt: 5,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rs1", 32)));
}

#[test]
fn rs1_max_out_of_bounds() {
    let instr = Instruction::Srli {
        rd: 1,
        rs1: 255,
        shamt: 5,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidRegister("rs1", 255))
    );
}

#[test]
fn shamt_out_of_bounds() {
    let instr = Instruction::Srli {
        rd: 1,
        rs1: 2,
        shamt: 32,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("shamt", 32))
    );
}

#[test]
fn shamt_large() {
    let instr = Instruction::Srli {
        rd: 1,
        rs1: 2,
        shamt: 100,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("shamt", 100))
    );
}

#[test]
fn shamt_max() {
    let instr = Instruction::Srli {
        rd: 1,
        rs1: 2,
        shamt: 255,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("shamt", 255))
    );
}

#[test]
fn shamt_valid_min() {
    let instr = Instruction::Srli {
        rd: 1,
        rs1: 2,
        shamt: 0,
    };
    assert!(instr.encode().is_ok());
}

#[test]
fn shamt_valid_max() {
    let instr = Instruction::Srli {
        rd: 1,
        rs1: 2,
        shamt: 31,
    };
    assert!(instr.encode().is_ok());
}

#[test]
fn all_max_valid() {
    let instr = Instruction::Srli {
        rd: 31,
        rs1: 31,
        shamt: 31,
    };
    assert!(instr.encode().is_ok());
}
