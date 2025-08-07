use crate::instruction::Instruction;

#[test]
fn basic() {
    // sw x2, 100(x1)
    // rs1=1, rs2=2, imm=100
    // S-type: imm[11:5]|rs2|rs1|funct3|imm[4:0]|opcode
    // imm=100 (0x064): imm[11:5]=0x03, imm[4:0]=0x04
    // funct3=0x2 for SW
    let word = 0x0620A223;
    match Instruction::decode(word) {
        Instruction::Sw { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, 100);
        }
        _ => panic!("Expected Sw instruction"),
    }
}

#[test]
fn zero_registers() {
    // sw x0, 0(x0)
    let word = 0x00002023;
    match Instruction::decode(word) {
        Instruction::Sw { rs1, rs2, imm } => {
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Sw instruction"),
    }
}

#[test]
fn max_registers() {
    // sw x31, 0(x31)
    let word = 0x01FFA023;
    match Instruction::decode(word) {
        Instruction::Sw { rs1, rs2, imm } => {
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Sw instruction"),
    }
}

#[test]
fn negative_offset() {
    // sw x2, -100(x1)
    // imm=-100 (0xF9C): imm[11:5]=0x7C, imm[4:0]=0x1C
    let word = 0xF820AE23;
    match Instruction::decode(word) {
        Instruction::Sw { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, -100);
        }
        _ => panic!("Expected Sw instruction"),
    }
}

#[test]
fn max_positive_offset() {
    // sw x2, 2047(x1)
    // imm=2047 (0x7FF): imm[11:5]=0x3F, imm[4:0]=0x1F
    let word = 0x7E20AFA3;
    match Instruction::decode(word) {
        Instruction::Sw { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, 2047);
        }
        _ => panic!("Expected Sw instruction"),
    }
}

#[test]
fn max_negative_offset() {
    // sw x2, -2048(x1)
    // imm=-2048 (0x800): imm[11:5]=0x40, imm[4:0]=0x00
    let word = 0x8020A023;
    match Instruction::decode(word) {
        Instruction::Sw { rs1, rs2, imm } => {
            assert_eq!(rs1, 1);
            assert_eq!(rs2, 2);
            assert_eq!(imm, -2048);
        }
        _ => panic!("Expected Sw instruction"),
    }
}
