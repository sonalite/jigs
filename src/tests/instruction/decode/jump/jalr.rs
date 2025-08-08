use crate::instruction::Instruction;

#[test]
fn basic() {
    // jalr x1, x2, 8
    // rd=1, rs1=2, imm=8, funct3=0x0, opcode=0x67
    let instruction_word = 0x008100E7; // 000000001000 00010 000 00001 1100111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Jalr {
            rd: 1,
            rs1: 2,
            imm: 8
        }
    );
}

#[test]
fn zero_registers() {
    // jalr x0, x0, 0
    // rd=0, rs1=0, imm=0, funct3=0x0, opcode=0x67
    let instruction_word = 0x00000067;
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Jalr {
            rd: 0,
            rs1: 0,
            imm: 0
        }
    );
}

#[test]
fn max_registers() {
    // jalr x31, x31, 16
    // rd=31, rs1=31, imm=16, funct3=0x0, opcode=0x67
    let instruction_word = 0x010F8FE7; // 000000010000 11111 000 11111 1100111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Jalr {
            rd: 31,
            rs1: 31,
            imm: 16
        }
    );
}

#[test]
fn negative_offset() {
    // jalr x5, x6, -8
    // rd=5, rs1=6, imm=-8, funct3=0x0, opcode=0x67
    // -8 in 12 bits = 0xFF8
    let instruction_word = 0xFF8302E7; // 111111111000 00110 000 00101 1100111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Jalr {
            rd: 5,
            rs1: 6,
            imm: -8
        }
    );
}

#[test]
fn large_positive_offset() {
    // jalr x10, x11, 2047 (max positive offset that fits in 12 bits)
    // rd=10, rs1=11, imm=2047, funct3=0x0, opcode=0x67
    // 2047 = 0x7FF
    let instruction_word = 0x7FF58567; // 011111111111 01011 000 01010 1100111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Jalr {
            rd: 10,
            rs1: 11,
            imm: 2047
        }
    );
}

#[test]
fn large_negative_offset() {
    // jalr x15, x16, -2048 (min negative offset that fits in 12 bits)
    // rd=15, rs1=16, imm=-2048, funct3=0x0, opcode=0x67
    // -2048 = 0x800 in 12 bits
    let instruction_word = 0x800807E7; // 100000000000 10000 000 01111 1100111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Jalr {
            rd: 15,
            rs1: 16,
            imm: -2048
        }
    );
}

#[test]
fn different_registers() {
    // jalr x20, x25, 64
    // rd=20, rs1=25, imm=64, funct3=0x0, opcode=0x67
    let instruction_word = 0x040C8A67; // 000001000000 11001 000 10100 1100111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Jalr {
            rd: 20,
            rs1: 25,
            imm: 64
        }
    );
}

#[test]
fn wrong_funct3() {
    // Invalid jalr with funct3 != 0
    // rd=1, rs1=2, imm=8, funct3=0x1 (wrong), opcode=0x67
    let instruction_word = 0x008110E7; // 000000001000 00010 001 00001 1100111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Unsupported(instruction_word));
}
