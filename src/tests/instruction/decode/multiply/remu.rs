use crate::instruction::Instruction;

#[test]
fn basic() {
    // remu x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x7, funct7=0x01, opcode=0x33
    let instruction_word = 0x023170B3; // 0000001 00011 00010 111 00001 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Remu {
            rd: 1,
            rs1: 2,
            rs2: 3
        }
    );
}

#[test]
fn zero_registers() {
    // remu x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x7, funct7=0x01, opcode=0x33
    let instruction_word = 0x02007033; // 0000001 00000 00000 111 00000 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Remu {
            rd: 0,
            rs1: 0,
            rs2: 0
        }
    );
}

#[test]
fn max_registers() {
    // remu x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x7, funct7=0x01, opcode=0x33
    let instruction_word = 0x03FFFFB3; // 0000001 11111 11111 111 11111 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Remu {
            rd: 31,
            rs1: 31,
            rs2: 31
        }
    );
}

#[test]
fn different_registers() {
    // remu x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x7, funct7=0x01, opcode=0x33
    let instruction_word = 0x0347F533; // 0000001 10100 01111 111 01010 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Remu {
            rd: 10,
            rs1: 15,
            rs2: 20
        }
    );
}

#[test]
fn wrong_funct7() {
    // remu with wrong funct7 (should be 0x01, using 0x02)
    // rd=1, rs1=2, rs2=3, funct3=0x7, funct7=0x02, opcode=0x33
    let instruction_word = 0x043170B3; // 0000010 00011 00010 111 00001 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
