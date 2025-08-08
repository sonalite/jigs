use crate::{EncodeError, Instruction};

#[test]
fn unsupported() {
    let inst = Instruction::Unsupported(0x12345678);
    assert_eq!(
        inst.encode(),
        Err(EncodeError::NotImplemented("Unsupported"))
    );
}
