use crate::instruction::Instruction;

#[test]
fn basic() {
    // jal x1, 8
    // rd=1, imm=8, opcode=0x6F
    // imm[20|10:1|11|19:12] = 0b00000000000001000000
    // Encoding: imm[20]=0, imm[10:1]=0000000100, imm[11]=0, imm[19:12]=00000000
    let instruction_word = 0x008000EF; // 0 0000000100 0 00000000 00001 1101111
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Jal { rd, imm } => {
            assert_eq!(rd, 1);
            assert_eq!(imm, 8);
        }
        _ => panic!("Expected Jal instruction"),
    }
}

#[test]
fn zero_register() {
    // jal x0, 0 (commonly used as unconditional jump)
    // rd=0, imm=0, opcode=0x6F
    let instruction_word = 0x0000006F;
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Jal { rd, imm } => {
            assert_eq!(rd, 0);
            assert_eq!(imm, 0);
        }
        _ => panic!("Expected Jal instruction"),
    }
}

#[test]
fn max_register() {
    // jal x31, 16
    // rd=31, imm=16, opcode=0x6F
    let instruction_word = 0x01000FEF; // 0 0000001000 0 00000000 11111 1101111
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Jal { rd, imm } => {
            assert_eq!(rd, 31);
            assert_eq!(imm, 16);
        }
        _ => panic!("Expected Jal instruction"),
    }
}

#[test]
fn negative_offset() {
    // jal x5, -8
    // rd=5, imm=-8, opcode=0x6F
    // -8 in 21 bits = 0x1FFFF8, need to encode properly
    // bit[20]=1, bits[19:12]=11111111, bit[11]=1, bits[10:1]=1111111100
    let instruction_word = 0xFF9FF2EF; // 1 1111111100 1 11111111 00101 1101111
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Jal { rd, imm } => {
            assert_eq!(rd, 5);
            assert_eq!(imm, -8);
        }
        _ => panic!("Expected Jal instruction"),
    }
}

#[test]
fn large_positive_offset() {
    // jal x10, 1048574 (max positive even offset that fits in 21 bits)
    // rd=10, imm=1048574, opcode=0x6F
    // 1048574 = 0xFFFFE
    // bit[20]=0, bits[19:12]=11111111, bit[11]=1, bits[10:1]=1111111111
    let instruction_word = 0x7FFFF56F; // 0 1111111111 1 11111111 01010 1101111
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Jal { rd, imm } => {
            assert_eq!(rd, 10);
            assert_eq!(imm, 1048574);
        }
        _ => panic!("Expected Jal instruction"),
    }
}

#[test]
fn large_negative_offset() {
    // jal x15, -1048576 (min negative offset that fits in 21 bits)
    // rd=15, imm=-1048576, opcode=0x6F
    // -1048576 = 0x100000 in 21 bits
    // bit[20]=1, bits[19:12]=00000000, bit[11]=0, bits[10:1]=0000000000
    let instruction_word = 0x800007EF; // 1 0000000000 0 00000000 01111 1101111
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Jal { rd, imm } => {
            assert_eq!(rd, 15);
            assert_eq!(imm, -1048576);
        }
        _ => panic!("Expected Jal instruction"),
    }
}

#[test]
fn different_registers() {
    // jal x20, 256
    // rd=20, imm=256, opcode=0x6F
    // 256 = 0x100, bits[10:1]=0010000000
    let instruction_word = 0x10000A6F; // 0 0010000000 0 00000000 10100 1101111
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Jal { rd, imm } => {
            assert_eq!(rd, 20);
            assert_eq!(imm, 256);
        }
        _ => panic!("Expected Jal instruction"),
    }
}
