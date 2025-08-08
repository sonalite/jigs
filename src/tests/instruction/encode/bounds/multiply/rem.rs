use crate::{EncodeError, Instruction};

#[test]
fn invalid_rd() {
    let instr = Instruction::Rem {
        rd: 32,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 32)));
}

#[test]
fn invalid_rs1() {
    let instr = Instruction::Rem {
        rd: 1,
        rs1: 255,
        rs2: 3,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidRegister("rs1", 255))
    );
}

#[test]
fn invalid_rs2() {
    let instr = Instruction::Rem {
        rd: 1,
        rs1: 2,
        rs2: 100,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidRegister("rs2", 100))
    );
}
