use crate::instruction::Instruction;

#[test]
fn basic() {
    // sh x2, 100(x1)
    // rs1=1, rs2=2, imm=100
    // S-type: imm[11:5]|rs2|rs1|funct3|imm[4:0]|opcode
    // imm=100 (0x064): imm[11:5]=0x03, imm[4:0]=0x04
    // funct3=0x1 for SH
    let word = 0x06209223;
    let instruction = Instruction::decode(word);
    assert_eq!(
        instruction,
        Instruction::Sh {
            rs1: 1,
            rs2: 2,
            imm: 100
        }
    );
}

#[test]
fn zero_registers() {
    // sh x0, 0(x0)
    let word = 0x00001023;
    let instruction = Instruction::decode(word);
    assert_eq!(
        instruction,
        Instruction::Sh {
            rs1: 0,
            rs2: 0,
            imm: 0
        }
    );
}

#[test]
fn max_registers() {
    // sh x31, 0(x31)
    let word = 0x01FF9023;
    let instruction = Instruction::decode(word);
    assert_eq!(
        instruction,
        Instruction::Sh {
            rs1: 31,
            rs2: 31,
            imm: 0
        }
    );
}

#[test]
fn negative_offset() {
    // sh x2, -100(x1)
    // imm=-100 (0xF9C): imm[11:5]=0x7C, imm[4:0]=0x1C
    let word = 0xF8209E23;
    let instruction = Instruction::decode(word);
    assert_eq!(
        instruction,
        Instruction::Sh {
            rs1: 1,
            rs2: 2,
            imm: -100
        }
    );
}

#[test]
fn max_positive_offset() {
    // sh x2, 2047(x1)
    // imm=2047 (0x7FF): imm[11:5]=0x3F, imm[4:0]=0x1F
    let word = 0x7E209FA3;
    let instruction = Instruction::decode(word);
    assert_eq!(
        instruction,
        Instruction::Sh {
            rs1: 1,
            rs2: 2,
            imm: 2047
        }
    );
}

#[test]
fn max_negative_offset() {
    // sh x2, -2048(x1)
    // imm=-2048 (0x800): imm[11:5]=0x40, imm[4:0]=0x00
    let word = 0x80209023;
    let instruction = Instruction::decode(word);
    assert_eq!(
        instruction,
        Instruction::Sh {
            rs1: 1,
            rs2: 2,
            imm: -2048
        }
    );
}
