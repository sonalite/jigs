use crate::instruction::Instruction;

#[test]
fn test_add_x1_x2_x3() {
    // add x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x0, funct7=0x00, opcode=0x33
    let instruction_word = 0x003100B3; // 0000000 00011 00010 000 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected Add instruction"),
    }
}

#[test]
fn test_add_x31_x0_x15() {
    // add x31, x0, x15
    // rd=31, rs1=0, rs2=15, funct3=0x0, funct7=0x00, opcode=0x33
    let instruction_word = 0x00F00FB3; // 0000000 01111 00000 000 11111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 15);
        }
        _ => panic!("Expected Add instruction"),
    }
}

#[test]
fn test_add_x10_x11_x12() {
    // add x10, x11, x12
    // rd=10, rs1=11, rs2=12, funct3=0x0, funct7=0x00, opcode=0x33
    let instruction_word = 0x00C58533; // 0000000 01100 01011 000 01010 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 11);
            assert_eq!(rs2, 12);
        }
        _ => panic!("Expected Add instruction"),
    }
}
