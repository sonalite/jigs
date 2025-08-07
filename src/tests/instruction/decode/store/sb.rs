use crate::instruction::Instruction;

#[test]
fn basic() {
    // sb x2, 100(x1)
    // rs1=1, rs2=2, imm=100
    // S-type: imm[11:5]|rs2|rs1|funct3|imm[4:0]|opcode
    // imm=100 (0x064): imm[11:5]=0x03, imm[4:0]=0x04
    let word = 0x06208223;
    match Instruction::decode(word) {
        Instruction::Sb { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Sb instruction"),
    }
}

#[test]
fn zero_registers() {
    // sb x0, 0(x0)
    let word = 0x00000023;
    match Instruction::decode(word) {
        Instruction::Sb { rs1, rs2, imm } => {
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Sb instruction"),
    }
}

#[test]
fn max_registers() {
    // sb x31, 0(x31)
    let word = 0x01FF8023;
    match Instruction::decode(word) {
        Instruction::Sb { rs1, rs2, imm } => {
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Sb instruction"),
    }
}

#[test]
fn negative_offset() {
    // sb x2, -100(x1)
    // imm=-100 (0xF9C): imm[11:5]=0x7C, imm[4:0]=0x1C
    let word = 0xF8208E23;
    match Instruction::decode(word) {
        Instruction::Sb { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, -100);
        }
        _ => panic!("Expected Sb instruction"),
    }
}

#[test]
fn max_positive_offset() {
    // sb x2, 2047(x1)
    // imm=2047 (0x7FF): imm[11:5]=0x3F, imm[4:0]=0x1F
    let word = 0x7E208FA3;
    match Instruction::decode(word) {
        Instruction::Sb { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Sb instruction"),
    }
}

#[test]
fn max_negative_offset() {
    // sb x2, -2048(x1)
    // imm=-2048 (0x800): imm[11:5]=0x40, imm[4:0]=0x00
    let word = 0x80208023;
    match Instruction::decode(word) {
        Instruction::Sb { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Sb instruction"),
    }
}
