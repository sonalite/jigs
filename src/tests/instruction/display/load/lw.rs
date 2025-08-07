use crate::instruction::Instruction;

#[test]
fn basic() {
    let instr = Instruction::Lw {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", instr), "lw x1, 100(x2)");
}

#[test]
fn zero_registers() {
    let instr = Instruction::Lw {
        rd: 0,
        rs1: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", instr), "lw x0, 0(x0)");
}

#[test]
fn max_registers() {
    let instr = Instruction::Lw {
        rd: 31,
        rs1: 31,
        imm: 0,
    };
    assert_eq!(format!("{}", instr), "lw x31, 0(x31)");
}

#[test]
fn negative_offset() {
    let instr = Instruction::Lw {
        rd: 1,
        rs1: 2,
        imm: -100,
    };
    assert_eq!(format!("{}", instr), "lw x1, -100(x2)");
}
