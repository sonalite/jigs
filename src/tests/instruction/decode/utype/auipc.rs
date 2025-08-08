use crate::instruction::Instruction;

#[test]
fn basic() {
    // auipc x1, 0x12345
    // rd=1, imm=0x12345, opcode=0x17
    // imm[31:12] = 0x12345, rd=1
    let instruction_word = 0x12345097; // 0001_0010_0011_0100_0101_00001_0010111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Auipc { rd: 1, imm: 74565 });
}

#[test]
fn zero_register() {
    // auipc x0, 0xABCDE (loading to x0 is valid but has no effect)
    // rd=0, imm=0xABCDE, opcode=0x17
    let instruction_word = 0xABCDE017; // 1010_1011_1100_1101_1110_00000_0010111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Auipc { rd: 0, imm: 703710 });
}

#[test]
fn max_register() {
    // auipc x31, 0x54321
    // rd=31, imm=0x54321, opcode=0x17
    let instruction_word = 0x54321F97; // 0101_0100_0011_0010_0001_11111_0010111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Auipc {
            rd: 31,
            imm: 344865
        }
    );
}

#[test]
fn zero_immediate() {
    // auipc x5, 0x0
    // rd=5, imm=0x0, opcode=0x17
    let instruction_word = 0x00000297; // 0000_0000_0000_0000_0000_00101_0010111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Auipc { rd: 5, imm: 0 });
}

#[test]
fn max_immediate() {
    // auipc x10, 0xFFFFF (max 20-bit value)
    // rd=10, imm=0xFFFFF, opcode=0x17
    let instruction_word = 0xFFFFF517; // 1111_1111_1111_1111_1111_01010_0010111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Auipc {
            rd: 10,
            imm: 1048575
        }
    );
}

#[test]
fn small_immediate() {
    // auipc x15, 0x1
    // rd=15, imm=0x1, opcode=0x17
    let instruction_word = 0x00001797; // 0000_0000_0000_0000_0001_01111_0010111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Auipc { rd: 15, imm: 1 });
}

#[test]
fn different_registers() {
    // auipc x20, 0x80000 (bit 19 set)
    // rd=20, imm=0x80000, opcode=0x17
    let instruction_word = 0x80000A17; // 1000_0000_0000_0000_0000_10100_0010111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Auipc {
            rd: 20,
            imm: 524288
        }
    );
}
