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

// S-type immediate masks and shifts
const IMM_S_11_5_MASK: u32 = 0xFE000000;
const IMM_S_11_5_SHIFT: u32 = 25;
const IMM_S_4_0_MASK: u32 = 0xF80;
const IMM_S_4_0_SHIFT: u32 = 7;

// B-type immediate masks and shifts
// B-type immediate is encoded as: imm[12|10:5]|rs2|rs1|funct3|imm[4:1|11]|opcode
// The immediate represents a signed offset in multiples of 2 bytes
const IMM_B_12_MASK: u32 = 0x80000000; // bit 31 -> imm[12]
const IMM_B_12_SHIFT: u32 = 31;
const IMM_B_11_MASK: u32 = 0x80; // bit 7 -> imm[11]
const IMM_B_11_SHIFT: u32 = 7;
const IMM_B_10_5_MASK: u32 = 0x7E000000; // bits 30:25 -> imm[10:5]
const IMM_B_10_5_SHIFT: u32 = 25;
const IMM_B_4_1_MASK: u32 = 0xF00; // bits 11:8 -> imm[4:1]
const IMM_B_4_1_SHIFT: u32 = 8;

// Opcodes
const REG_OPCODE: u32 = 0x33;
const IMM_OPCODE: u32 = 0x13;
const LOAD_OPCODE: u32 = 0x03;
const STORE_OPCODE: u32 = 0x23;
const BRANCH_OPCODE: u32 = 0x63;

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

// Function codes for Store instructions
const SB_FUNCT3: u8 = 0x0;
const SH_FUNCT3: u8 = 0x1;
const SW_FUNCT3: u8 = 0x2;

// Function codes for Branch instructions
const BEQ_FUNCT3: u8 = 0x0;
const BNE_FUNCT3: u8 = 0x1;
const BLT_FUNCT3: u8 = 0x4;
const BGE_FUNCT3: u8 = 0x5;
const BLTU_FUNCT3: u8 = 0x6;
const BGEU_FUNCT3: u8 = 0x7;

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

    /// Sb instruction
    ///
    /// Stores the least significant byte from register `rs2` to memory at address `rs1 + imm`.
    Sb { rs1: u8, rs2: u8, imm: i32 },

    /// Sh instruction
    ///
    /// Stores the least significant halfword (16 bits) from register `rs2` to memory at address `rs1 + imm`.
    Sh { rs1: u8, rs2: u8, imm: i32 },

    /// Sw instruction
    ///
    /// Stores a word (32 bits) from register `rs2` to memory at address `rs1 + imm`.
    Sw { rs1: u8, rs2: u8, imm: i32 },

    /// Beq instruction
    ///
    /// Branches to `pc + imm` if the values in registers `rs1` and `rs2` are equal.
    /// The immediate is a signed offset in bytes.
    Beq { rs1: u8, rs2: u8, imm: i32 },

    /// Bne instruction
    ///
    /// Branches to `pc + imm` if the values in registers `rs1` and `rs2` are not equal.
    /// The immediate is a signed offset in bytes.
    Bne { rs1: u8, rs2: u8, imm: i32 },

    /// Blt instruction
    ///
    /// Branches to `pc + imm` if the signed value in register `rs1` is less than the signed value in register `rs2`.
    /// The immediate is a signed offset in bytes.
    Blt { rs1: u8, rs2: u8, imm: i32 },

    /// Bge instruction
    ///
    /// Branches to `pc + imm` if the signed value in register `rs1` is greater than or equal to the signed value in register `rs2`.
    /// The immediate is a signed offset in bytes.
    Bge { rs1: u8, rs2: u8, imm: i32 },

    /// Bltu instruction
    ///
    /// Branches to `pc + imm` if the unsigned value in register `rs1` is less than the unsigned value in register `rs2`.
    /// The immediate is a signed offset in bytes.
    Bltu { rs1: u8, rs2: u8, imm: i32 },

    /// Bgeu instruction
    ///
    /// Branches to `pc + imm` if the unsigned value in register `rs1` is greater than or equal to the unsigned value in register `rs2`.
    /// The immediate is a signed offset in bytes.
    Bgeu { rs1: u8, rs2: u8, imm: i32 },

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
            Instruction::Sb { rs1, rs2, imm } => {
                write!(f, "sb x{}, {}(x{})", rs2, imm, rs1)
            }
            Instruction::Sh { rs1, rs2, imm } => {
                write!(f, "sh x{}, {}(x{})", rs2, imm, rs1)
            }
            Instruction::Sw { rs1, rs2, imm } => {
                write!(f, "sw x{}, {}(x{})", rs2, imm, rs1)
            }
            Instruction::Beq { rs1, rs2, imm } => {
                write!(f, "beq x{}, x{}, {}", rs1, rs2, imm)
            }
            Instruction::Bne { rs1, rs2, imm } => {
                write!(f, "bne x{}, x{}, {}", rs1, rs2, imm)
            }
            Instruction::Blt { rs1, rs2, imm } => {
                write!(f, "blt x{}, x{}, {}", rs1, rs2, imm)
            }
            Instruction::Bge { rs1, rs2, imm } => {
                write!(f, "bge x{}, x{}, {}", rs1, rs2, imm)
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                write!(f, "bltu x{}, x{}, {}", rs1, rs2, imm)
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                write!(f, "bgeu x{}, x{}, {}", rs1, rs2, imm)
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
            STORE_OPCODE => {
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let rs2 = ((word & RS2_MASK) >> RS2_SHIFT) as u8;
                // S-type immediate is split into two parts
                let imm_11_5 = (word & IMM_S_11_5_MASK) >> IMM_S_11_5_SHIFT;
                let imm_4_0 = (word & IMM_S_4_0_MASK) >> IMM_S_4_0_SHIFT;
                let imm_raw = (imm_11_5 << 5) | imm_4_0;
                // Sign-extend the 12-bit immediate
                let imm = if imm_raw & 0x800 != 0 {
                    // Sign bit is set, sign-extend
                    (imm_raw | 0xFFFFF000) as i32
                } else {
                    imm_raw as i32
                };

                match funct3 {
                    SB_FUNCT3 => Instruction::Sb { rs1, rs2, imm },
                    SH_FUNCT3 => Instruction::Sh { rs1, rs2, imm },
                    SW_FUNCT3 => Instruction::Sw { rs1, rs2, imm },
                    _ => Instruction::Unsupported(word),
                }
            }
            BRANCH_OPCODE => {
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let rs2 = ((word & RS2_MASK) >> RS2_SHIFT) as u8;

                // B-type immediate reconstruction
                // The immediate is encoded in a scrambled format:
                // inst[31] = imm[12], inst[30:25] = imm[10:5], inst[11:8] = imm[4:1], inst[7] = imm[11]
                let bit_12 = (word & IMM_B_12_MASK) >> IMM_B_12_SHIFT;
                let bit_11 = (word & IMM_B_11_MASK) >> IMM_B_11_SHIFT;
                let bits_10_5 = (word & IMM_B_10_5_MASK) >> IMM_B_10_5_SHIFT;
                let bits_4_1 = (word & IMM_B_4_1_MASK) >> IMM_B_4_1_SHIFT;

                // Reconstruct the immediate value (13 bits, with bit 0 always 0)
                let imm_raw = (bit_12 << 12) | (bit_11 << 11) | (bits_10_5 << 5) | (bits_4_1 << 1);

                // Sign-extend from 13 bits to 32 bits
                let imm = if imm_raw & 0x1000 != 0 {
                    // Sign bit is set, sign-extend
                    (imm_raw | 0xFFFFE000) as i32
                } else {
                    imm_raw as i32
                };

                match funct3 {
                    BEQ_FUNCT3 => Instruction::Beq { rs1, rs2, imm },
                    BNE_FUNCT3 => Instruction::Bne { rs1, rs2, imm },
                    BLT_FUNCT3 => Instruction::Blt { rs1, rs2, imm },
                    BGE_FUNCT3 => Instruction::Bge { rs1, rs2, imm },
                    BLTU_FUNCT3 => Instruction::Bltu { rs1, rs2, imm },
                    BGEU_FUNCT3 => Instruction::Bgeu { rs1, rs2, imm },
                    _ => Instruction::Unsupported(word),
                }
            }
            _ => Instruction::Unsupported(word),
        }
    }
}
