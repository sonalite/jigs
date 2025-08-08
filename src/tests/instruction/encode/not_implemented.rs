use crate::instruction::{EncodeError, Instruction};

#[test]
fn xor() {
    let inst = Instruction::Xor {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Xor")));
}

#[test]
fn or() {
    let inst = Instruction::Or {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Or")));
}

#[test]
fn srl() {
    let inst = Instruction::Srl {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Srl")));
}

#[test]
fn sra() {
    let inst = Instruction::Sra {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Sra")));
}

#[test]
fn slt() {
    let inst = Instruction::Slt {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Slt")));
}

#[test]
fn sltu() {
    let inst = Instruction::Sltu {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Sltu")));
}

#[test]
fn and() {
    let inst = Instruction::And {
        rd: 1,
        rs1: 2,
        rs2: 3,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("And")));
}

#[test]
fn addi() {
    let inst = Instruction::Addi {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Addi")));
}

#[test]
fn andi() {
    let inst = Instruction::Andi {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Andi")));
}

#[test]
fn ori() {
    let inst = Instruction::Ori {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Ori")));
}

#[test]
fn xori() {
    let inst = Instruction::Xori {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Xori")));
}

#[test]
fn slti() {
    let inst = Instruction::Slti {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Slti")));
}

#[test]
fn sltiu() {
    let inst = Instruction::Sltiu {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Sltiu")));
}

#[test]
fn slli() {
    let inst = Instruction::Slli {
        rd: 1,
        rs1: 2,
        shamt: 5,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Slli")));
}

#[test]
fn srli() {
    let inst = Instruction::Srli {
        rd: 1,
        rs1: 2,
        shamt: 5,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Srli")));
}

#[test]
fn srai() {
    let inst = Instruction::Srai {
        rd: 1,
        rs1: 2,
        shamt: 5,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Srai")));
}

#[test]
fn lb() {
    let inst = Instruction::Lb {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Lb")));
}

#[test]
fn lh() {
    let inst = Instruction::Lh {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Lh")));
}

#[test]
fn lw() {
    let inst = Instruction::Lw {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Lw")));
}

#[test]
fn lbu() {
    let inst = Instruction::Lbu {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Lbu")));
}

#[test]
fn lhu() {
    let inst = Instruction::Lhu {
        rd: 1,
        rs1: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Lhu")));
}

#[test]
fn sb() {
    let inst = Instruction::Sb {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Sb")));
}

#[test]
fn sh() {
    let inst = Instruction::Sh {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Sh")));
}

#[test]
fn sw() {
    let inst = Instruction::Sw {
        rs1: 1,
        rs2: 2,
        imm: 100,
    };
    assert_eq!(inst.encode(), Err(EncodeError::NotImplemented("Sw")));
}

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
