use crate::instruction::Instruction;

#[test]
fn basic() {
    // div x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x4, funct7=0x01, opcode=0x33
    let instruction_word = 0x023140B3; // 0000001 00011 00010 100 00001 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Div {
            rd: 1,
            rs1: 2,
            rs2: 3
        }
    );
}

#[test]
fn zero_registers() {
    // div x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x4, funct7=0x01, opcode=0x33
    let instruction_word = 0x02004033; // 0000001 00000 00000 100 00000 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Div {
            rd: 0,
            rs1: 0,
            rs2: 0
        }
    );
}

#[test]
fn max_registers() {
    // div x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x4, funct7=0x01, opcode=0x33
    let instruction_word = 0x03FFCFB3; // 0000001 11111 11111 100 11111 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Div {
            rd: 31,
            rs1: 31,
            rs2: 31
        }
    );
}

#[test]
fn different_registers() {
    // div x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x4, funct7=0x01, opcode=0x33
    let instruction_word = 0x0347C533; // 0000001 10100 01111 100 01010 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Div {
            rd: 10,
            rs1: 15,
            rs2: 20
        }
    );
}

#[test]
fn wrong_funct7() {
    // div with wrong funct7 (should be 0x01, using 0x02)
    // rd=1, rs1=2, rs2=3, funct3=0x4, funct7=0x02, opcode=0x33
    let instruction_word = 0x043140B3; // 0000010 00011 00010 100 00001 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
