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

// I-type immediate mask and shift
const IMM_I_MASK: u32 = 0xFFF00000;
const IMM_I_SHIFT: u32 = 20;

// Opcodes
const REG_OPCODE: u32 = 0x33;
const IMM_OPCODE: u32 = 0x13;
const LOAD_OPCODE: u32 = 0x03;

// Function codes for I-type instructions
const ADDI_FUNCT3: u8 = 0x0;
const SLLI_FUNCT3: u8 = 0x1;
const SLTI_FUNCT3: u8 = 0x2;
const SLTIU_FUNCT3: u8 = 0x3;
const XORI_FUNCT3: u8 = 0x4;
const SRLI_SRAI_FUNCT3: u8 = 0x5; // Shared by SRLI and SRAI
const ORI_FUNCT3: u8 = 0x6;
const ANDI_FUNCT3: u8 = 0x7;

// Function codes for Load instructions
const LB_FUNCT3: u8 = 0x0;
const LH_FUNCT3: u8 = 0x1;
const LW_FUNCT3: u8 = 0x2;
const LBU_FUNCT3: u8 = 0x4;
const LHU_FUNCT3: u8 = 0x5;

// Function codes for R-type instructions
const ADD_SUB_FUNCT3: u8 = 0x0; // Shared by ADD and SUB
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
const SLT_FUNCT3: u8 = 0x2;
const SLT_FUNCT7: u32 = 0x00;
const SLTU_FUNCT3: u8 = 0x3;
const SLTU_FUNCT7: u32 = 0x00;
const AND_FUNCT3: u8 = 0x7;
const AND_FUNCT7: u32 = 0x00;

/// RISC-V instruction representation for 32-bit IM
#[derive(Debug)]
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

    /// Slt instruction
    ///
    /// Sets `rd` to 1 if the signed value in register `rs1` is less than the signed value in register `rs2`, otherwise sets `rd` to 0.
    Slt { rd: u8, rs1: u8, rs2: u8 },

    /// Sltu instruction
    ///
    /// Sets `rd` to 1 if the unsigned value in register `rs1` is less than the unsigned value in register `rs2`, otherwise sets `rd` to 0.
    Sltu { rd: u8, rs1: u8, rs2: u8 },

    /// And instruction
    ///
    /// Performs bitwise AND between the values in registers `rs1` and `rs2` and stores the result in `rd`.
    And { rd: u8, rs1: u8, rs2: u8 },

    /// Addi instruction
    ///
    /// Adds the sign-extended 12-bit immediate to the value in register `rs1` and stores the result in `rd`.
    /// Performs 32-bit arithmetic addition with overflow wrapping.
    Addi { rd: u8, rs1: u8, imm: i32 },

    /// Slti instruction
    ///
    /// Sets `rd` to 1 if the signed value in register `rs1` is less than the sign-extended 12-bit immediate, otherwise sets `rd` to 0.
    Slti { rd: u8, rs1: u8, imm: i32 },

    /// Sltiu instruction
    ///
    /// Sets `rd` to 1 if the unsigned value in register `rs1` is less than the sign-extended 12-bit immediate (compared as unsigned), otherwise sets `rd` to 0.
    Sltiu { rd: u8, rs1: u8, imm: i32 },

    /// Xori instruction
    ///
    /// Performs bitwise XOR between the value in register `rs1` and the sign-extended 12-bit immediate, stores the result in `rd`.
    Xori { rd: u8, rs1: u8, imm: i32 },

    /// Ori instruction
    ///
    /// Performs bitwise OR between the value in register `rs1` and the sign-extended 12-bit immediate, stores the result in `rd`.
    Ori { rd: u8, rs1: u8, imm: i32 },

    /// Andi instruction
    ///
    /// Performs bitwise AND between the value in register `rs1` and the sign-extended 12-bit immediate, stores the result in `rd`.
    Andi { rd: u8, rs1: u8, imm: i32 },

    /// Slli instruction
    ///
    /// Shifts the value in register `rs1` left by the immediate shift amount (lower 5 bits) and stores the result in `rd`.
    Slli { rd: u8, rs1: u8, shamt: u8 },

    /// Srli instruction
    ///
    /// Shifts the value in register `rs1` right by the immediate shift amount (lower 5 bits) and stores the result in `rd`.
    /// Performs logical right shift (zero-fill).
    Srli { rd: u8, rs1: u8, shamt: u8 },

    /// Srai instruction
    ///
    /// Shifts the value in register `rs1` right by the immediate shift amount (lower 5 bits) and stores the result in `rd`.
    /// Performs arithmetic right shift (sign-extend).
    Srai { rd: u8, rs1: u8, shamt: u8 },

    /// Lb instruction
    ///
    /// Loads a byte from memory at address `rs1 + imm` and sign-extends it to 32 bits, storing the result in `rd`.
    Lb { rd: u8, rs1: u8, imm: i32 },

    /// Lh instruction
    ///
    /// Loads a halfword (16 bits) from memory at address `rs1 + imm` and sign-extends it to 32 bits, storing the result in `rd`.
    Lh { rd: u8, rs1: u8, imm: i32 },

    /// Lw instruction
    ///
    /// Loads a word (32 bits) from memory at address `rs1 + imm`, storing the result in `rd`.
    Lw { rd: u8, rs1: u8, imm: i32 },

    /// Lbu instruction
    ///
    /// Loads a byte from memory at address `rs1 + imm` and zero-extends it to 32 bits, storing the result in `rd`.
    Lbu { rd: u8, rs1: u8, imm: i32 },

    /// Lhu instruction
    ///
    /// Loads a halfword (16 bits) from memory at address `rs1 + imm` and zero-extends it to 32 bits, storing the result in `rd`.
    Lhu { rd: u8, rs1: u8, imm: i32 },

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
            Instruction::Slt { rd, rs1, rs2 } => {
                write!(f, "slt x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                write!(f, "sltu x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::And { rd, rs1, rs2 } => {
                write!(f, "and x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Addi { rd, rs1, imm } => {
                write!(f, "addi x{}, x{}, {}", rd, rs1, imm)
            }
            Instruction::Slti { rd, rs1, imm } => {
                write!(f, "slti x{}, x{}, {}", rd, rs1, imm)
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                write!(f, "sltiu x{}, x{}, {}", rd, rs1, imm)
            }
            Instruction::Xori { rd, rs1, imm } => {
                write!(f, "xori x{}, x{}, {}", rd, rs1, imm)
            }
            Instruction::Ori { rd, rs1, imm } => {
                write!(f, "ori x{}, x{}, {}", rd, rs1, imm)
            }
            Instruction::Andi { rd, rs1, imm } => {
                write!(f, "andi x{}, x{}, {}", rd, rs1, imm)
            }
            Instruction::Slli { rd, rs1, shamt } => {
                write!(f, "slli x{}, x{}, {}", rd, rs1, shamt)
            }
            Instruction::Srli { rd, rs1, shamt } => {
                write!(f, "srli x{}, x{}, {}", rd, rs1, shamt)
            }
            Instruction::Srai { rd, rs1, shamt } => {
                write!(f, "srai x{}, x{}, {}", rd, rs1, shamt)
            }
            Instruction::Lb { rd, rs1, imm } => {
                write!(f, "lb x{}, {}(x{})", rd, imm, rs1)
            }
            Instruction::Lh { rd, rs1, imm } => {
                write!(f, "lh x{}, {}(x{})", rd, imm, rs1)
            }
            Instruction::Lw { rd, rs1, imm } => {
                write!(f, "lw x{}, {}(x{})", rd, imm, rs1)
            }
            Instruction::Lbu { rd, rs1, imm } => {
                write!(f, "lbu x{}, {}(x{})", rd, imm, rs1)
            }
            Instruction::Lhu { rd, rs1, imm } => {
                write!(f, "lhu x{}, {}(x{})", rd, imm, rs1)
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
                    ADD_SUB_FUNCT3 => {
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
                    SLT_FUNCT3 => {
                        if funct7 == SLT_FUNCT7 {
                            Instruction::Slt { rd, rs1, rs2 }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    SLTU_FUNCT3 => {
                        if funct7 == SLTU_FUNCT7 {
                            Instruction::Sltu { rd, rs1, rs2 }
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
                    AND_FUNCT3 => {
                        if funct7 == AND_FUNCT7 {
                            Instruction::And { rd, rs1, rs2 }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    _ => {
                        // This case is unreachable since funct3 is masked to 3 bits (0-7)
                        // and all values are handled above
                        unreachable!()
                    }
                }
            }
            IMM_OPCODE => {
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                // Sign-extend the 12-bit immediate
                let imm_raw = (word & IMM_I_MASK) >> IMM_I_SHIFT;
                let imm = if imm_raw & 0x800 != 0 {
                    // Sign bit is set, sign-extend
                    (imm_raw | 0xFFFFF000) as i32
                } else {
                    imm_raw as i32
                };

                match funct3 {
                    ADDI_FUNCT3 => Instruction::Addi { rd, rs1, imm },
                    SLLI_FUNCT3 => {
                        // For SLLI, the immediate encodes the shift amount in lower 5 bits
                        // and the upper 7 bits must be 0x00
                        let shamt = (imm_raw & 0x1F) as u8;
                        let upper_bits = (imm_raw >> 5) & 0x7F;
                        if upper_bits == 0x00 {
                            Instruction::Slli { rd, rs1, shamt }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    SLTI_FUNCT3 => Instruction::Slti { rd, rs1, imm },
                    SLTIU_FUNCT3 => Instruction::Sltiu { rd, rs1, imm },
                    XORI_FUNCT3 => Instruction::Xori { rd, rs1, imm },
                    SRLI_SRAI_FUNCT3 => {
                        // For SRLI/SRAI, the immediate encodes the shift amount in lower 5 bits
                        // and the upper 7 bits determine which instruction (0x00 for SRLI, 0x20 for SRAI)
                        let shamt = (imm_raw & 0x1F) as u8;
                        let upper_bits = (imm_raw >> 5) & 0x7F;
                        if upper_bits == 0x00 {
                            Instruction::Srli { rd, rs1, shamt }
                        } else if upper_bits == 0x20 {
                            Instruction::Srai { rd, rs1, shamt }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    ORI_FUNCT3 => Instruction::Ori { rd, rs1, imm },
                    ANDI_FUNCT3 => Instruction::Andi { rd, rs1, imm },
                    _ => {
                        // This case is unreachable since funct3 is masked to 3 bits (0-7)
                        // and all values 0-7 are handled above
                        unreachable!()
                    }
                }
            }
            LOAD_OPCODE => {
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                // Sign-extend the 12-bit immediate
                let imm_raw = (word & IMM_I_MASK) >> IMM_I_SHIFT;
                let imm = if imm_raw & 0x800 != 0 {
                    // Sign bit is set, sign-extend
                    (imm_raw | 0xFFFFF000) as i32
                } else {
                    imm_raw as i32
                };

                match funct3 {
                    LB_FUNCT3 => Instruction::Lb { rd, rs1, imm },
                    LH_FUNCT3 => Instruction::Lh { rd, rs1, imm },
                    LW_FUNCT3 => Instruction::Lw { rd, rs1, imm },
                    LBU_FUNCT3 => Instruction::Lbu { rd, rs1, imm },
                    LHU_FUNCT3 => Instruction::Lhu { rd, rs1, imm },
                    _ => Instruction::Unsupported(word),
                }
            }
            _ => Instruction::Unsupported(word),
        }
    }
}
