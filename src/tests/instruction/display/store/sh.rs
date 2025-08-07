use crate::instruction::Instruction;

#[test]
fn basic() {
    let instr = Instruction::Sh {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(format!("{}", instr), "sh x2, 100(x1)");
}

#[test]
fn zero_registers() {
    let instr = Instruction::Sh {
        rs1: 0,
        rs2: 0,
        imm: 0,
    };
    assert_eq!(format!("{}", instr), "sh x0, 0(x0)");
}

#[test]
fn max_registers() {
    let instr = Instruction::Sh {
        rs1: 31,
        rs2: 31,
        imm: 0,
    };
    assert_eq!(format!("{}", instr), "sh x31, 0(x31)");
}

#[test]
fn negative_offset() {
    let instr = Instruction::Sh {
        rs1: 1,
        rs2: 2,
        imm: -100,
    };
    assert_eq!(format!("{}", instr), "sh x2, -100(x1)");
}
