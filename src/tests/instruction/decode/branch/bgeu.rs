use crate::instruction::Instruction;

#[test]
fn basic() {
    // bgeu x1, x2, 8
    // rs1=1, rs2=2, imm=8, funct3=0x7, opcode=0x63
    let instruction_word = 0x0020F463; // 0 000000 0 00010 00001 111 0100 0 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Bgeu {
            rs1: 1,
            rs2: 2,
            imm: 8
        }
    );
}

#[test]
fn zero_registers() {
    // bgeu x0, x0, 0
    // rs1=0, rs2=0, imm=0, funct3=0x7, opcode=0x63
    let instruction_word = 0x00007063; // 0 000000 0 00000 00000 111 0000 0 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Bgeu {
            rs1: 0,
            rs2: 0,
            imm: 0
        }
    );
}

#[test]
fn max_registers() {
    // bgeu x31, x31, 16
    // rs1=31, rs2=31, imm=16, funct3=0x7, opcode=0x63
    let instruction_word = 0x01FFF863; // 0 000000 0 11111 11111 111 1000 0 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Bgeu {
            rs1: 31,
            rs2: 31,
            imm: 16
        }
    );
}

#[test]
fn negative_offset() {
    // bgeu x5, x6, -8
    // rs1=5, rs2=6, imm=-8, funct3=0x7, opcode=0x63
    let instruction_word = 0xFE62FCE3; // 1 111111 0 00110 00101 111 1100 1 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Bgeu {
            rs1: 5,
            rs2: 6,
            imm: -8
        }
    );
}

#[test]
fn large_positive_offset() {
    // bgeu x10, x11, 4094
    // rs1=10, rs2=11, imm=4094, funct3=0x7, opcode=0x63
    let instruction_word = 0x7EB57FE3; // 0 111111 0 01011 01010 111 1111 1 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Bgeu {
            rs1: 10,
            rs2: 11,
            imm: 4094
        }
    );
}

#[test]
fn large_negative_offset() {
    // bgeu x15, x16, -4096
    // rs1=15, rs2=16, imm=-4096, funct3=0x7, opcode=0x63
    let instruction_word = 0x8107F063; // 1 000000 1 10000 01111 111 0000 0 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Bgeu {
            rs1: 15,
            rs2: 16,
            imm: -4096
        }
    );
}
