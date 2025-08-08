use crate::instruction::Instruction;

#[test]
fn basic() {
    // beq x1, x2, 8
    // rs1=1, rs2=2, imm=8, funct3=0x0, opcode=0x63
    // imm[12|10:5]=0b0000000, rs2=2, rs1=1, funct3=0, imm[4:1|11]=0b01000, opcode=0x63
    let instruction_word = 0x00208463; // 0 000000 0 00010 00001 000 0100 0 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Beq {
            rs1: 1,
            rs2: 2,
            imm: 8
        }
    );
}

#[test]
fn zero_registers() {
    // beq x0, x0, 0
    // rs1=0, rs2=0, imm=0, funct3=0x0, opcode=0x63
    let instruction_word = 0x00000063; // 0 000000 0 00000 00000 000 0000 0 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Beq {
            rs1: 0,
            rs2: 0,
            imm: 0
        }
    );
}

#[test]
fn max_registers() {
    // beq x31, x31, 16
    // rs1=31, rs2=31, imm=16, funct3=0x0, opcode=0x63
    let instruction_word = 0x01FF8863; // 0 000000 0 11111 11111 000 1000 0 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Beq {
            rs1: 31,
            rs2: 31,
            imm: 16
        }
    );
}

#[test]
fn negative_offset() {
    // beq x5, x6, -8
    // rs1=5, rs2=6, imm=-8, funct3=0x0, opcode=0x63
    // -8 in 13 bits = 0x1FF8, split as: bit[12]=1, bits[11]=1, bits[10:5]=111111, bits[4:1]=1100
    let instruction_word = 0xFE628CE3; // 1 111111 0 00110 00101 000 1100 1 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Beq {
            rs1: 5,
            rs2: 6,
            imm: -8
        }
    );
}

#[test]
fn large_positive_offset() {
    // beq x10, x11, 4094 (max positive even offset that fits in 13 bits)
    // rs1=10, rs2=11, imm=4094, funct3=0x0, opcode=0x63
    // 4094 = 0xFFE, split as: bit[12]=0, bit[11]=1, bits[10:5]=111111, bits[4:1]=1111
    let instruction_word = 0x7EB50FE3; // 0 111111 0 01011 01010 000 1111 1 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Beq {
            rs1: 10,
            rs2: 11,
            imm: 4094
        }
    );
}

#[test]
fn large_negative_offset() {
    // beq x15, x16, -4096 (min negative offset that fits in 13 bits)
    // rs1=15, rs2=16, imm=-4096, funct3=0x0, opcode=0x63
    // -4096 = 0x1000 in 13 bits, split as: bit[12]=1, bit[11]=0, bits[10:5]=000000, bits[4:1]=0000
    let instruction_word = 0x81078063; // 1 000000 1 10000 01111 000 0000 0 1100011
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Beq {
            rs1: 15,
            rs2: 16,
            imm: -4096
        }
    );
}
