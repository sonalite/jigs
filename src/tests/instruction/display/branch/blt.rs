use crate::instruction::Instruction;

#[test]
fn basic() {
    let instruction = Instruction::Blt {
        rs1: 1,
        rs2: 2,
        imm: 8,
    };
    assert_eq!(format!("{}", instruction), "blt x1, x2, 8");
}

#[test]
fn zero_registers() {
    let instruction = Instruction::Blt {
        rs1: 0,
        rs2: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", instruction), "blt x0, x0, 0");
}

#[test]
fn max_registers() {
    let instruction = Instruction::Blt {
        rs1: 31,
        rs2: 31,
        imm: 16,
    };
    assert_eq!(format!("{}", instruction), "blt x31, x31, 16");
}

#[test]
fn negative_offset() {
    let instruction = Instruction::Blt {
        rs1: 5,
        rs2: 6,
        imm: -8,
    };
    assert_eq!(format!("{}", instruction), "blt x5, x6, -8");
}

#[test]
fn large_positive_offset() {
    let instruction = Instruction::Blt {
        rs1: 10,
        rs2: 11,
        imm: 4094,
    };
    assert_eq!(format!("{}", instruction), "blt x10, x11, 4094");
}

#[test]
fn large_negative_offset() {
    let instruction = Instruction::Blt {
        rs1: 15,
        rs2: 16,
        imm: -4096,
    };
    assert_eq!(format!("{}", instruction), "blt x15, x16, -4096");
}
