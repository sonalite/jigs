use crate::instruction::{EncodeError, Instruction};

#[test]
fn rd_out_of_bounds() {
    let instr = Instruction::Sll {
        rd: 32,
        rs1: 1,
        rs2: 2,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 32)));
}

#[test]
fn rd_max_out_of_bounds() {
    let instr = Instruction::Sll {
        rd: 255,
        rs1: 1,
        rs2: 2,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 255)));
}

#[test]
fn rs1_out_of_bounds() {
    let instr = Instruction::Sll {
        rd: 1,
        rs1: 32,
        rs2: 2,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rs1", 32)));
}

#[test]
fn rs1_max_out_of_bounds() {
    let instr = Instruction::Sll {
        rd: 1,
        rs1: 255,
        rs2: 2,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidRegister("rs1", 255))
    );
}

#[test]
fn rs2_out_of_bounds() {
    let instr = Instruction::Sll {
        rd: 1,
        rs1: 2,
        rs2: 32,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rs2", 32)));
}

#[test]
fn rs2_max_out_of_bounds() {
    let instr = Instruction::Sll {
        rd: 1,
        rs1: 2,
        rs2: 255,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidRegister("rs2", 255))
    );
}

#[test]
fn all_registers_max_valid() {
    let instr = Instruction::Sll {
        rd: 31,
        rs1: 31,
        rs2: 31,
    };
    assert!(instr.encode().is_ok());
}
