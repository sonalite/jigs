use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Sltiu {
        rd: 1,
        rs1: 2,
        imm: 10,
    };
    assert_eq!(format!("{}", instruction), "sltiu x1, x2, 10");
}

#[test]
fn zero_registers() {
    let instruction = Instruction::Sltiu {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", instruction), "sltiu x0, x0, 0");
}

#[test]
fn max_registers() {
    let instruction = Instruction::Sltiu {
        rd: 31,
        rs1: 31,
        imm: 100,
    };
    assert_eq!(format!("{}", instruction), "sltiu x31, x31, 100");
}

#[test]
fn different_registers() {
    let instruction = Instruction::Sltiu {
        rd: 5,
        rs1: 10,
        imm: 255,
    };
    assert_eq!(format!("{}", instruction), "sltiu x5, x10, 255");
}

#[test]
fn max_positive_immediate() {
    let instruction = Instruction::Sltiu {
        rd: 7,
        rs1: 8,
        imm: 2047, // 0x7FF - max positive 12-bit signed
    };
    assert_eq!(format!("{}", instruction), "sltiu x7, x8, 2047");
}

#[test]
fn negative_immediate() {
    let instruction = Instruction::Sltiu {
        rd: 11,
        rs1: 12,
        imm: -1,
    };
    assert_eq!(format!("{}", instruction), "sltiu x11, x12, -1");
}

#[test]
fn min_negative_immediate() {
    let instruction = Instruction::Sltiu {
        rd: 15,
        rs1: 16,
        imm: -2048, // 0x800 - min negative 12-bit signed
    };
    assert_eq!(format!("{}", instruction), "sltiu x15, x16, -2048");
}

#[test]
fn seqz_pseudo_instruction() {
    let instruction = Instruction::Sltiu {
        rd: 1,
        rs1: 2,
        imm: 1,
    };
    assert_eq!(format!("{}", instruction), "sltiu x1, x2, 1");
}
