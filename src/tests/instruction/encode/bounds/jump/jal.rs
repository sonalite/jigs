use crate::{EncodeError, Instruction};

#[test]
fn invalid_register() {
    let instr = Instruction::Jal { rd: 32, imm: 8 };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 32)));
}

#[test]
fn invalid_register_too_large() {
    let instr = Instruction::Jal { rd: 255, imm: 8 };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidRegister("rd", 255)));
}

#[test]
fn odd_offset() {
    let instr = Instruction::Jal { rd: 1, imm: 7 };
    assert_eq!(instr.encode(), Err(EncodeError::InvalidImmediate("imm", 7)));
}

#[test]
fn odd_negative_offset() {
    let instr = Instruction::Jal { rd: 1, imm: -7 };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", -7))
    );
}

#[test]
fn offset_too_large() {
    let instr = Instruction::Jal {
        rd: 1,
        imm: 1048576,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", 1048576))
    );
}

#[test]
fn offset_too_small() {
    let instr = Instruction::Jal {
        rd: 1,
        imm: -1048578,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", -1048578))
    );
}

#[test]
fn boundary_values() {
    // Valid max even offset
    let instr = Instruction::Jal {
        rd: 1,
        imm: 1048574,
    };
    assert!(instr.encode().is_ok());

    // Invalid: max + 2
    let instr = Instruction::Jal {
        rd: 1,
        imm: 1048576,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", 1048576))
    );

    // Valid min even offset
    let instr = Instruction::Jal {
        rd: 1,
        imm: -1048576,
    };
    assert!(instr.encode().is_ok());

    // Invalid: min - 2
    let instr = Instruction::Jal {
        rd: 1,
        imm: -1048578,
    };
    assert_eq!(
        instr.encode(),
        Err(EncodeError::InvalidImmediate("imm", -1048578))
    );
}
