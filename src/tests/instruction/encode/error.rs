use crate::{EncodeError, Instruction};
use std::error::Error;

#[test]
fn display_not_implemented() {
    let error = EncodeError::NotImplemented("TestInstruction");
    let display = format!("{}", error);
    assert_eq!(
        display,
        "Encoding not implemented for instruction: TestInstruction"
    );
}

#[test]
fn display_invalid_register() {
    let error = EncodeError::InvalidRegister("rd", 32);
    let display = format!("{}", error);
    assert_eq!(display, "Invalid register value for rd: 32 (must be 0-31)");
}

#[test]
fn display_invalid_immediate() {
    let error = EncodeError::InvalidImmediate("imm", 2048);
    let display = format!("{}", error);
    assert_eq!(display, "Invalid immediate value for imm: 2048");
}

#[test]
fn trait_compatibility() {
    let error = EncodeError::NotImplemented("TestInstruction");
    // Test that we can use it as std::error::Error
    let _error_trait: &dyn Error = &error;
}

#[test]
fn via_instruction() {
    // Test actual error generation via instruction encoding
    let instr = Instruction::Sb {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    match instr.encode() {
        Err(EncodeError::NotImplemented("Sb")) => {
            // Test that we can display the actual error
            let error_display = format!("{}", EncodeError::NotImplemented("Sb"));
            assert_eq!(
                error_display,
                "Encoding not implemented for instruction: Sb"
            );
        }
        _ => panic!("Expected NotImplemented error for Sb instruction"),
    }
}
