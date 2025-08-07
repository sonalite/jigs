use crate::instruction::Instruction;

#[test]
fn basic() {
    // sub x1, x2, x3
    // rd=1, rs1=2, rs2=3, funct3=0x0, funct7=0x20, opcode=0x33
    let instruction_word = 0x403100B3; // 0100000 00011 00010 000 00001 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 1);
            assert_eq!(rs1, 2);
            assert_eq!(rs2, 3);
        }
        _ => panic!("Expected Sub instruction"),
    }
}

#[test]
fn zero_registers() {
    // sub x0, x0, x0
    // rd=0, rs1=0, rs2=0, funct3=0x0, funct7=0x20, opcode=0x33
    let instruction_word = 0x40000033; // 0100000 00000 00000 000 00000 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 0);
            assert_eq!(rs1, 0);
            assert_eq!(rs2, 0);
        }
        _ => panic!("Expected Sub instruction"),
    }
}

#[test]
fn max_registers() {
    // sub x31, x31, x31
    // rd=31, rs1=31, rs2=31, funct3=0x0, funct7=0x20, opcode=0x33
    let instruction_word = 0x41FF8FB3; // 0100000 11111 11111 000 11111 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 31);
            assert_eq!(rs1, 31);
            assert_eq!(rs2, 31);
        }
        _ => panic!("Expected Sub instruction"),
    }
}

#[test]
fn different_registers() {
    // sub x10, x15, x20
    // rd=10, rs1=15, rs2=20, funct3=0x0, funct7=0x20, opcode=0x33
    let instruction_word = 0x41478533; // 0100000 10100 01111 000 01010 0110011
    let instruction = Instruction::decode(instruction_word);

    match instruction {
        Instruction::Sub { rd, rs1, rs2 } => {
            assert_eq!(rd, 10);
            assert_eq!(rs1, 15);
            assert_eq!(rs2, 20);
        }
        _ => panic!("Expected Sub instruction"),
    }
}
