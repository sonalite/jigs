use crate::{EncodeError, Instruction};

#[test]
fn beq() {
    let inst = Instruction::Beq {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Beq")));
}

#[test]
fn bne() {
    let inst = Instruction::Bne {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Bne")));
}

#[test]
fn blt() {
    let inst = Instruction::Blt {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Blt")));
}

#[test]
fn bge() {
    let inst = Instruction::Bge {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Bge")));
}

#[test]
fn bltu() {
    let inst = Instruction::Bltu {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Bltu")));
}

#[test]
fn bgeu() {
    let inst = Instruction::Bgeu {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Bgeu")));
}

#[test]
fn mul() {
    let inst = Instruction::Mul {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Mul")));
}

#[test]
fn mulh() {
    let inst = Instruction::Mulh {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Mulh")));
}

#[test]
fn mulhsu() {
    let inst = Instruction::Mulhsu {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Mulhsu")));
}

#[test]
fn mulhu() {
    let inst = Instruction::Mulhu {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Mulhu")));
}

#[test]
fn div() {
    let inst = Instruction::Div {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Div")));
}

#[test]
fn divu() {
    let inst = Instruction::Divu {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Divu")));
}

#[test]
fn rem() {
    let inst = Instruction::Rem {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Rem")));
}

#[test]
fn remu() {
    let inst = Instruction::Remu {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Remu")));
}

#[test]
fn jal() {
    let inst = Instruction::Jal { rd: 1, imm: 100 };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Jal")));
}

#[test]
fn jalr() {
    let inst = Instruction::Jalr {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Jalr")));
}

#[test]
fn lui() {
    let inst = Instruction::Lui {
        rd: 1,
        imm: 0x12345,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Lui")));
}

#[test]
fn auipc() {
    let inst = Instruction::Auipc {
        rd: 1,
        imm: 0x12345,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Auipc")));
}

#[test]
fn ecall() {
    let inst = Instruction::Ecall;
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Ecall")));
}

#[test]
fn ebreak() {
    let inst = Instruction::Ebreak;
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Ebreak")));
}

#[test]
fn unsupported() {
    let inst = Instruction::Unsupported(0x12345678);
    assert_eq!(
        inst.encode(),
        Err(EncodeError::NotImplemented("Unsupported"))
    );
}
