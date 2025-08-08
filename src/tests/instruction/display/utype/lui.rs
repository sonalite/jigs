use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Lui {
        rd: 1,
        imm: 0x12345,
    };
    assert_eq!(format!("{}", instruction), "lui x1, 0x12345");
}

#[test]
fn zero_register() {
    let instruction = Instruction::Lui {
        rd: 0,
        imm: 0xABCDE,
    };
    assert_eq!(format!("{}", instruction), "lui x0, 0xabcde");
}

#[test]
fn max_register() {
    let instruction = Instruction::Lui {
        rd: 31,
        imm: 0x54321,
    };
    assert_eq!(format!("{}", instruction), "lui x31, 0x54321");
}

#[test]
fn zero_immediate() {
    let instruction = Instruction::Lui { rd: 5, imm: 0x0 };
    assert_eq!(format!("{}", instruction), "lui x5, 0x0");
}

#[test]
fn max_immediate() {
    let instruction = Instruction::Lui {
        rd: 10,
        imm: 0xFFFFF,
    };
    assert_eq!(format!("{}", instruction), "lui x10, 0xfffff");
}

#[test]
fn small_immediate() {
    let instruction = Instruction::Lui { rd: 15, imm: 0x1 };
    assert_eq!(format!("{}", instruction), "lui x15, 0x1");
}

#[test]
fn different_values() {
    let instruction = Instruction::Lui {
        rd: 20,
        imm: 0x80000,
    };
    assert_eq!(format!("{}", instruction), "lui x20, 0x80000");
}
