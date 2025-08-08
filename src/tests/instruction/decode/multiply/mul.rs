use crate::instruction::Instruction;

#[test]
fn basic() {
    // mul x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x0, funct7=0x01, opcode=0x33
    let instruction_word = 0x023100B3; // 0000001 00011 00010 000 00001 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Mul {
            rd: 1,
            rs1: 2,
            rs2: 3
        }
    );
}

#[test]
fn zero_registers() {
    // mul x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x0, funct7=0x01, opcode=0x33
    let instruction_word = 0x02000033; // 0000001 00000 00000 000 00000 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Mul {
            rd: 0,
            rs1: 0,
            rs2: 0
        }
    );
}

#[test]
fn max_registers() {
    // mul x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x0, funct7=0x01, opcode=0x33
    let instruction_word = 0x03FF8FB3; // 0000001 11111 11111 000 11111 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Mul {
            rd: 31,
            rs1: 31,
            rs2: 31
        }
    );
}

#[test]
fn different_registers() {
    // mul x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x0, funct7=0x01, opcode=0x33
    let instruction_word = 0x03478533; // 0000001 10100 01111 000 01010 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Mul {
            rd: 10,
            rs1: 15,
            rs2: 20
        }
    );
}

#[test]
fn wrong_funct7() {
    // mul with wrong funct7 (should be 0x01, using 0x02)
    // rd=1, rs1=2, rs2=3, funct3=0x0, funct7=0x02, opcode=0x33
    let instruction_word = 0x043100B3; // 0000010 00011 00010 000 00001 0110011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
