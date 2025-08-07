use std::fmt;

// Masks for extracting instruction fields
const OPCODE_MASK: u32 = 0x7F;
const RD_MASK: u32 = 0xF80;
const RD_SHIFT: u32 = 7;
const FUNCT3_MASK: u32 = 0x7000;
const FUNCT3_SHIFT: u32 = 12;
const RS1_MASK: u32 = 0xF8000;
const RS1_SHIFT: u32 = 15;
const RS2_MASK: u32 = 0x1F00000;
const RS2_SHIFT: u32 = 20;
const FUNCT7_MASK: u32 = 0xFE000000;
const FUNCT7_SHIFT: u32 = 25;

// Opcodes
const REG_OPCODE: u32 = 0x33;

// Function codes for R-type instructions
const ADDSUB_FUNCT3: u8 = 0x0; // Shared by ADD and SUB
const ADD_FUNCT7: u32 = 0x00;
const SUB_FUNCT7: u32 = 0x20;
const SLL_FUNCT3: u8 = 0x1;
const SLL_FUNCT7: u32 = 0x00;
const XOR_FUNCT3: u8 = 0x4;
const XOR_FUNCT7: u32 = 0x00;
const OR_FUNCT3: u8 = 0x6;
const OR_FUNCT7: u32 = 0x00;
const SRL_SRA_FUNCT3: u8 = 0x5; // Shared by SRL and SRA
const SRL_FUNCT7: u32 = 0x00;
const SRA_FUNCT7: u32 = 0x20;

/// RISC-V instruction representation for 32-bit IM
pub enum Instruction {
    /// Add instruction
    ///
    /// Adds the values in registers `rs1` and `rs2` and stores the result in `rd`.
    /// Performs 32-bit arithmetic addition with overflow wrapping.
    Add { rd: u8, rs1: u8, rs2: u8 },

    /// Sub instruction
    ///
    /// Subtracts the value in register `rs2` from `rs1` and stores the result in `rd`.
    /// Performs 32-bit arithmetic subtraction with overflow wrapping.
    Sub { rd: u8, rs1: u8, rs2: u8 },

    /// Sll instruction
    ///
    /// Shifts the value in register `rs1` left by the shift amount held in the lower 5 bits of register `rs2` and stores the result in `rd`.
    Sll { rd: u8, rs1: u8, rs2: u8 },

    /// Xor instruction
    ///
    /// Performs bitwise XOR between the values in registers `rs1` and `rs2` and stores the result in `rd`.
    Xor { rd: u8, rs1: u8, rs2: u8 },

    /// Or instruction
    ///
    /// Performs bitwise OR between the values in registers `rs1` and `rs2` and stores the result in `rd`.
    Or { rd: u8, rs1: u8, rs2: u8 },

    /// Srl instruction
    ///
    /// Shifts the value in register `rs1` right by the shift amount held in the lower 5 bits of register `rs2` and stores the result in `rd`.
    /// Performs logical right shift (zero-fill).
    Srl { rd: u8, rs1: u8, rs2: u8 },

    /// Sra instruction
    ///
    /// Shifts the value in register `rs1` right by the shift amount held in the lower 5 bits of register `rs2` and stores the result in `rd`.
    /// Performs arithmetic right shift (sign-extend).
    Sra { rd: u8, rs1: u8, rs2: u8 },

    /// Unsupported instruction
    ///
    /// Represents an instruction that is not yet implemented or recognized.
    Unsupported(u32),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add { rd, rs1, rs2 } => {
                write!(f, "add x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                write!(f, "sub x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                write!(f, "sll x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                write!(f, "xor x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Or { rd, rs1, rs2 } => {
                write!(f, "or x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                write!(f, "srl x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                write!(f, "sra x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Unsupported(word) => {
                write!(f, "unsupported: 0x{:08x}", word)
            }
        }
    }
}

impl Instruction {
    /// Decode a 32-bit instruction word into an Instruction
    ///
    /// # Arguments
    ///
    /// * `word` - The 32-bit instruction word to decode
    pub fn decode(word: u32) -> Instruction {
        let opcode = word & OPCODE_MASK;

        match opcode {
            REG_OPCODE => {
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let funct7 = (word & FUNCT7_MASK) >> FUNCT7_SHIFT;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let rs2 = ((word & RS2_MASK) >> RS2_SHIFT) as u8;

                match funct3 {
                    ADDSUB_FUNCT3 => {
                        if funct7 == ADD_FUNCT7 {
                            Instruction::Add { rd, rs1, rs2 }
                        } else if funct7 == SUB_FUNCT7 {
                            Instruction::Sub { rd, rs1, rs2 }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    SLL_FUNCT3 => {
                        if funct7 == SLL_FUNCT7 {
                            Instruction::Sll { rd, rs1, rs2 }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    XOR_FUNCT3 => {
                        if funct7 == XOR_FUNCT7 {
                            Instruction::Xor { rd, rs1, rs2 }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    SRL_SRA_FUNCT3 => {
                        if funct7 == SRL_FUNCT7 {
                            Instruction::Srl { rd, rs1, rs2 }
                        } else if funct7 == SRA_FUNCT7 {
                            Instruction::Sra { rd, rs1, rs2 }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    OR_FUNCT3 => {
                        if funct7 == OR_FUNCT7 {
                            Instruction::Or { rd, rs1, rs2 }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    _ => Instruction::Unsupported(word),
                }
            }
            _ => Instruction::Unsupported(word),
        }
    }
}
