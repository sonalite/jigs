use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Auipc {
        rd: 1,
        imm: 0x12345,
    };
    assert_eq!(format!("{}", instruction), "auipc x1, 0x12345");
}

#[test]
fn zero_register() {
    let instruction = Instruction::Auipc {
        rd: 0,
        imm: 0xABCDE,
    };
    assert_eq!(format!("{}", instruction), "auipc x0, 0xabcde");
}

#[test]
fn max_register() {
    let instruction = Instruction::Auipc {
        rd: 31,
        imm: 0x54321,
    };
    assert_eq!(format!("{}", instruction), "auipc x31, 0x54321");
}

#[test]
fn zero_immediate() {
    let instruction = Instruction::Auipc { rd: 5, imm: 0x0 };
    assert_eq!(format!("{}", instruction), "auipc x5, 0x0");
}

#[test]
fn max_immediate() {
    let instruction = Instruction::Auipc {
        rd: 10,
        imm: 0xFFFFF,
    };
    assert_eq!(format!("{}", instruction), "auipc x10, 0xfffff");
}

#[test]
fn small_immediate() {
    let instruction = Instruction::Auipc { rd: 15, imm: 0x1 };
    assert_eq!(format!("{}", instruction), "auipc x15, 0x1");
}

#[test]
fn different_registers() {
    let instruction = Instruction::Auipc {
        rd: 20,
        imm: 0x80000,
    };
    assert_eq!(format!("{}", instruction), "auipc x20, 0x80000");
}
