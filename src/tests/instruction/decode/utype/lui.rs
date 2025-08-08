use crate::instruction::Instruction;

#[test]
fn basic() {
    // lui x1, 0x12345
    // rd=1, imm=0x12345, opcode=0x37
    // imm[31:12] = 0x12345, rd=1
    let instruction_word = 0x123450B7; // 0001_0010_0011_0100_0101_00001_0110111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Lui { rd: 1, imm: 74565 });
}

#[test]
fn zero_register() {
    // lui x0, 0xABCDE (loading to x0 is valid but has no effect)
    // rd=0, imm=0xABCDE, opcode=0x37
    let instruction_word = 0xABCDE037; // 1010_1011_1100_1101_1110_00000_0110111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Lui { rd: 0, imm: 703710 });
}

#[test]
fn max_register() {
    // lui x31, 0x54321
    // rd=31, imm=0x54321, opcode=0x37
    let instruction_word = 0x54321FB7; // 0101_0100_0011_0010_0001_11111_0110111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Lui {
            rd: 31,
            imm: 344865
        }
    );
}

#[test]
fn zero_immediate() {
    // lui x5, 0x0
    // rd=5, imm=0x0, opcode=0x37
    let instruction_word = 0x000002B7; // 0000_0000_0000_0000_0000_00101_0110111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Lui { rd: 5, imm: 0 });
}

#[test]
fn max_immediate() {
    // lui x10, 0xFFFFF (max 20-bit value)
    // rd=10, imm=0xFFFFF, opcode=0x37
    let instruction_word = 0xFFFFF537; // 1111_1111_1111_1111_1111_01010_0110111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Lui {
            rd: 10,
            imm: 1048575
        }
    );
}

#[test]
fn small_immediate() {
    // lui x15, 0x1
    // rd=15, imm=0x1, opcode=0x37
    let instruction_word = 0x000017B7; // 0000_0000_0000_0000_0001_01111_0110111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(instruction, Instruction::Lui { rd: 15, imm: 1 });
}

#[test]
fn different_registers() {
    // lui x20, 0x80000 (bit 19 set)
    // rd=20, imm=0x80000, opcode=0x37
    let instruction_word = 0x80000A37; // 1000_0000_0000_0000_0000_10100_0110111
    let instruction = Instruction::decode(instruction_word);
    assert_eq!(
        instruction,
        Instruction::Lui {
            rd: 20,
            imm: 524288
        }
    );
}
