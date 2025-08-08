use crate::{EncodeError, Instruction};

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
