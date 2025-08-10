use crate::Instruction;
use crate::compiler::Compiler;

#[test]
fn basic_ret_compilation() {
    let mut compiler = Compiler::new();
    let instructions = vec![Instruction::Jalr {
        rd: 0,
        rs1: 1,
        imm: 0,
    }];

    let code = compiler.compile(&instructions);

    // Should emit a single RET instruction (4 bytes)
    assert_eq!(code.len(), 4);

    // RET instruction encoding: 0xD65F03C0 (little-endian)
    assert_eq!(code, vec![0xC0, 0x03, 0x5F, 0xD6]);
}

#[test]
fn empty_compilation() {
    let mut compiler = Compiler::new();
    let instructions = vec![];

    let code = compiler.compile(&instructions);

    // Should still emit a RET for safety
    assert_eq!(code.len(), 4);
    assert_eq!(code, vec![0xC0, 0x03, 0x5F, 0xD6]);
}

#[test]
fn multiple_instructions() {
    let mut compiler = Compiler::new();
    let instructions = vec![
        Instruction::Add {
            rd: 1,
            rs1: 2,
            rs2: 3,
        },
        Instruction::Sub {
            rd: 4,
            rs1: 5,
            rs2: 6,
        },
    ];

    let code = compiler.compile(&instructions);

    // Should still just emit a RET for now
    assert_eq!(code.len(), 4);
    assert_eq!(code, vec![0xC0, 0x03, 0x5F, 0xD6]);
}
