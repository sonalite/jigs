use crate::{EncodeError, Instruction};

#[test]
fn rd_out_of_bounds() {
    let instr = Instruction::Auipc { rd: 32, imm: 100 };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 32)));
}

#[test]
fn rd_max_out_of_bounds() {
    let instr = Instruction::Auipc { rd: 255, imm: 100 };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 255)));
}

#[test]
fn imm_out_of_bounds() {
    let instr = Instruction::Auipc {
        rd: 1,
        imm: 1048576, // 2^20
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", 1048576))
    );
}

#[test]
fn imm_max_out_of_bounds() {
    let instr = Instruction::Auipc {
        rd: 1,
        imm: u32::MAX,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", u32::MAX as i32))
    );
}

#[test]
fn imm_min_valid() {
    let instr = Instruction::Auipc { rd: 1, imm: 0 };
    assert!(instr.encode().is_ok());
}

#[test]
fn imm_max_valid() {
    let instr = Instruction::Auipc {
        rd: 1,
        imm: 1048575, // 2^20 - 1
    };
    assert!(instr.encode().is_ok());
}

#[test]
fn all_max_valid() {
    let instr = Instruction::Auipc {
        rd: 31,
        imm: 1048575,
    };
    assert!(instr.encode().is_ok());
}
