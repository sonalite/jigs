use crate::instruction::Instruction;

#[test]
fn basic() {
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
fn zero_register() {
    // add x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x0, funct7=0x00, opcode=0x33
    let instruction_word = 0x00000033; // 0000000 00000 00000 000 00000 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected Add instruction"),
    }
}

#[test]
fn max_registers() {
    // add x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x0, funct7=0x00, opcode=0x33
    let instruction_word = 0x01FF8FB3; // 0000000 11111 11111 000 11111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Add { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected Add instruction"),
    }
}
