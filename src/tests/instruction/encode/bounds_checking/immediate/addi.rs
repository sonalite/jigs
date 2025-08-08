use crate::instruction::{EncodeError, Instruction};

#[test]
fn rd_out_of_bounds() {
    let instr = Instruction::Addi {
        rd: 32,
        rs1: 1,
        imm: 100,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 32)));
}

#[test]
fn rd_max_out_of_bounds() {
    let instr = Instruction::Addi {
        rd: 255,
        rs1: 1,
        imm: 100,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 255)));
}

#[test]
fn rs1_out_of_bounds() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 32,
        imm: 100,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rs1", 32)));
}

#[test]
fn rs1_max_out_of_bounds() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 255,
        imm: 100,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidRegister("rs1", 255))
    );
}

#[test]
fn imm_negative_out_of_bounds() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: -2049,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", -2049))
    );
}

#[test]
fn imm_positive_out_of_bounds() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: 2048,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", 2048))
    );
}

#[test]
fn imm_max_negative_out_of_bounds() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: i32::MIN,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", i32::MIN))
    );
}

#[test]
fn imm_max_positive_out_of_bounds() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: i32::MAX,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", i32::MAX))
    );
}

#[test]
fn imm_min_valid() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: -2048,
    };
    assert!(instr.encode().is_ok());
}

#[test]
fn imm_max_valid() {
    let instr = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: 2047,
    };
    assert!(instr.encode().is_ok());
}

#[test]
fn all_max_valid() {
    let instr = Instruction::Addi {
        rd: 31,
        rs1: 31,
        imm: 2047,
    };
    assert!(instr.encode().is_ok());
}
