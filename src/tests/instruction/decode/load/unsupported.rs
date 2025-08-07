use crate::instruction::Instruction;

#[test]
fn invalid_funct3() {
    // Invalid load instruction with funct3=0x3 (not used) or 0x6 (not used) or 0x7 (not used)
    // Using funct3=0x3
    let word = 0x00003003; // Opcode 0x03 (load), but funct3=0x3 is invalid
    match Instruction::decode(word) {
        Instruction::Unsupported(w) => {
            assert_eq!(w, word);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}

#[test]
fn invalid_funct3_6() {
    // Invalid load instruction with funct3=0x6
    let word = 0x00006003; // Opcode 0x03 (load), but funct3=0x6 is invalid
    match Instruction::decode(word) {
        Instruction::Unsupported(w) => {
            assert_eq!(w, word);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}

#[test]
fn invalid_funct3_7() {
    // Invalid load instruction with funct3=0x7
    let word = 0x00007003; // Opcode 0x03 (load), but funct3=0x7 is invalid
    match Instruction::decode(word) {
        Instruction::Unsupported(w) => {
            assert_eq!(w, word);
        }
        _ => panic!("Expected Unsupported instruction"),
    }
}
