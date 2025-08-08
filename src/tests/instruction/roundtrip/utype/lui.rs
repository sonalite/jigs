use crate::Instruction;
use crate::tests::instruction::assert_encode_decode;

#[test]
fn basic() {
    assert_encode_decode(&Instruction::Lui { rd: 1, imm: 74565 }, 0x123450B7);
}

#[test]
fn zero_register() {
    assert_encode_decode(&Instruction::Lui { rd: 0, imm: 703710 }, 0xABCDE037);
}

#[test]
fn max_register() {
    assert_encode_decode(
        &Instruction::Lui {
            rd: 31,
            imm: 344865,
        },
        0x54321FB7,
    );
}

#[test]
fn zero_immediate() {
    assert_encode_decode(&Instruction::Lui { rd: 5, imm: 0 }, 0x000002B7);
}

#[test]
fn max_immediate() {
    assert_encode_decode(
        &Instruction::Lui {
            rd: 10,
            imm: 1048575,
        },
        0xFFFFF537,
    );
}

#[test]
fn small_immediate() {
    assert_encode_decode(&Instruction::Lui { rd: 15, imm: 1 }, 0x000017B7);
}

#[test]
fn different_registers() {
    assert_encode_decode(
        &Instruction::Lui {
            rd: 20,
            imm: 524288,
        },
        0x80000A37,
    );
}
