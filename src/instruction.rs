//! RISC-V 32-bit instruction decoder implementation.
//!
//! This module provides decoding and display functionality for RISC-V 32-bit instructions
//! from the RV32IM instruction set (Integer base + Multiplication extension).
//!
//! # Architecture
//!
//! The decoder works by:
//! 1. Extracting the opcode from bits 0-6 of the instruction word
//! 2. Using the opcode to determine the instruction format (R, I, S, B, U, J)
//! 3. Extracting additional fields based on the format
//! 4. Combining opcode, funct3, and funct7 fields to identify the specific instruction
//!
//! # Supported Instructions
//!
//! ## R-Type (Register-to-Register)
//! - Arithmetic: ADD, SUB
//! - Logical: AND, OR, XOR
//! - Shifts: SLL, SRL, SRA
//! - Comparison: SLT, SLTU
//!
//! ## I-Type (Immediate)
//! - Arithmetic: ADDI
//! - Logical: ANDI, ORI, XORI
//! - Shifts: SLLI, SRLI, SRAI
//! - Comparison: SLTI, SLTIU
//! - Loads: LB, LH, LW, LBU, LHU
//! - Jump: JALR
//!
//! ## S-Type (Store)
//! - SB, SH, SW
//!
//! ## B-Type (Branch)
//! - BEQ, BNE, BLT, BGE, BLTU, BGEU
//!
//! ## U-Type (Upper Immediate)
//! - LUI, AUIPC
//!
//! ## J-Type (Jump)
//! - JAL
//!
//! ## System
//! - ECALL, EBREAK
//!
//! ## M Extension (Multiply/Divide)
//! - Multiplication: MUL, MULH, MULHSU, MULHU
//! - Division: DIV, DIVU
//! - Remainder: REM, REMU
//!
//! # Example
//!
//! ```
//! use jigs::Instruction;
//!
//! // Decode an ADD instruction (add x1, x2, x3)
//! let instruction_word = 0x003100B3;
//! let instruction = Instruction::decode(instruction_word);
//!
//! match instruction {
//!     Instruction::Add { rd, rs1, rs2 } => {
//!         assert_eq!(rd, 1);
//!         assert_eq!(rs1, 2);
//!         assert_eq!(rs2, 3);
//!     }
//!     _ => panic!("Expected ADD instruction"),
//! }
//! ```

use std::fmt;

/// Error type for instruction encoding failures.
#[derive(Debug, Clone, PartialEq)]
pub enum EncodeError {
    /// The instruction variant is not yet implemented for encoding
    NotImplemented(&'static str),
}

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

// J-type immediate masks and shifts for JAL
// J-type immediate is encoded as: imm[20|10:1|11|19:12]|rd|opcode
// The immediate represents a signed offset in multiples of 2 bytes
const IMM_J_20_MASK: u32 = 0x80000000; // bit 31 -> imm[20]
const IMM_J_20_SHIFT: u32 = 31;
const IMM_J_19_12_MASK: u32 = 0xFF000; // bits 19:12 -> imm[19:12]
const IMM_J_19_12_SHIFT: u32 = 12;
const IMM_J_11_MASK: u32 = 0x100000; // bit 20 -> imm[11]
const IMM_J_11_SHIFT: u32 = 20;
const IMM_J_10_1_MASK: u32 = 0x7FE00000; // bits 30:21 -> imm[10:1]
const IMM_J_10_1_SHIFT: u32 = 21;

// U-type immediate masks and shifts for LUI and AUIPC
// U-type immediate is encoded as: imm[31:12]|rd|opcode
// The immediate represents the upper 20 bits
const IMM_U_MASK: u32 = 0xFFFFF000; // bits 31:12 -> imm[31:12]
const IMM_U_SHIFT: u32 = 12;

/// RISC-V instruction representation for 32-bit IM
#[derive(Debug, Clone, PartialEq)]
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

    /// Mul instruction
    ///
    /// Multiplies the values in registers `rs1` and `rs2`, storing the lower 32 bits of the result in `rd`.
    /// Part of the M extension.
    Mul { rd: u8, rs1: u8, rs2: u8 },

    /// Mulh instruction
    ///
    /// Multiplies the signed values in registers `rs1` and `rs2`, storing the upper 32 bits of the 64-bit result in `rd`.
    /// Part of the M extension.
    Mulh { rd: u8, rs1: u8, rs2: u8 },

    /// Mulhsu instruction
    ///
    /// Multiplies the signed value in `rs1` by the unsigned value in `rs2`, storing the upper 32 bits of the 64-bit result in `rd`.
    /// Part of the M extension.
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },

    /// Mulhu instruction
    ///
    /// Multiplies the unsigned values in registers `rs1` and `rs2`, storing the upper 32 bits of the 64-bit result in `rd`.
    /// Part of the M extension.
    Mulhu { rd: u8, rs1: u8, rs2: u8 },

    /// Div instruction
    ///
    /// Divides the signed value in register `rs1` by the signed value in register `rs2` and stores the result in `rd`.
    /// Part of the M extension.
    Div { rd: u8, rs1: u8, rs2: u8 },

    /// Divu instruction
    ///
    /// Divides the unsigned value in register `rs1` by the unsigned value in register `rs2` and stores the result in `rd`.
    /// Part of the M extension.
    Divu { rd: u8, rs1: u8, rs2: u8 },

    /// Rem instruction
    ///
    /// Computes the remainder of the signed division of the value in register `rs1` by the value in register `rs2` and stores the result in `rd`.
    /// Part of the M extension.
    Rem { rd: u8, rs1: u8, rs2: u8 },

    /// Remu instruction
    ///
    /// Computes the remainder of the unsigned division of the value in register `rs1` by the value in register `rs2` and stores the result in `rd`.
    /// Part of the M extension.
    Remu { rd: u8, rs1: u8, rs2: u8 },

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

    /// Jal instruction
    ///
    /// Jumps to `pc + imm` and stores the address of the next instruction (`pc + 4`) in register `rd`.
    /// The immediate is a signed offset in bytes (must be even).
    Jal { rd: u8, imm: i32 },

    /// Jalr instruction
    ///
    /// Jumps to `(rs1 + imm) & ~1` and stores the address of the next instruction (`pc + 4`) in register `rd`.
    /// The immediate is a signed 12-bit value.
    Jalr { rd: u8, rs1: u8, imm: i32 },

    /// Lui instruction
    ///
    /// Loads the 20-bit immediate value into the upper 20 bits of register `rd`, zeroing the lower 12 bits.
    /// The immediate is a 20-bit value that will be placed in bits [31:12] of the destination register.
    Lui { rd: u8, imm: u32 },

    /// Auipc instruction
    ///
    /// Adds the 20-bit immediate value (shifted left by 12) to the current PC and stores the result in register `rd`.
    /// The immediate is a 20-bit value that will be placed in bits [31:12] and added to PC.
    Auipc { rd: u8, imm: u32 },

    /// Ecall instruction
    ///
    /// Environment call - used to make a request to the supporting execution environment.
    /// Typically used for system calls in an operating system.
    Ecall,

    /// Ebreak instruction
    ///
    /// Environment breakpoint - used to return control to a debugging environment.
    /// Causes the processor to enter debug mode.
    Ebreak,

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
            Instruction::Mul { rd, rs1, rs2 } => {
                write!(f, "mul x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                write!(f, "mulh x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                write!(f, "mulhsu x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                write!(f, "mulhu x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Div { rd, rs1, rs2 } => {
                write!(f, "div x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Divu { rd, rs1, rs2 } => {
                write!(f, "divu x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Rem { rd, rs1, rs2 } => {
                write!(f, "rem x{}, x{}, x{}", rd, rs1, rs2)
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                write!(f, "remu x{}, x{}, x{}", rd, rs1, rs2)
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
            Instruction::Jal { rd, imm } => {
                write!(f, "jal x{}, {}", rd, imm)
            }
            Instruction::Jalr { rd, rs1, imm } => {
                write!(f, "jalr x{}, {}(x{})", rd, imm, rs1)
            }
            Instruction::Lui { rd, imm } => {
                write!(f, "lui x{}, 0x{:x}", rd, imm)
            }
            Instruction::Auipc { rd, imm } => {
                write!(f, "auipc x{}, 0x{:x}", rd, imm)
            }
            Instruction::Ecall => {
                write!(f, "ecall")
            }
            Instruction::Ebreak => {
                write!(f, "ebreak")
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
            0x33 => {
                // R-type instructions
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let funct7 = (word & FUNCT7_MASK) >> FUNCT7_SHIFT;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let rs2 = ((word & RS2_MASK) >> RS2_SHIFT) as u8;

                match (funct3, funct7) {
                    // Arithmetic operations
                    (0x0, 0x00) => Instruction::Add { rd, rs1, rs2 }, // ADD
                    (0x0, 0x20) => Instruction::Sub { rd, rs1, rs2 }, // SUB

                    // Shift operations
                    (0x1, 0x00) => Instruction::Sll { rd, rs1, rs2 }, // SLL
                    (0x5, 0x00) => Instruction::Srl { rd, rs1, rs2 }, // SRL
                    (0x5, 0x20) => Instruction::Sra { rd, rs1, rs2 }, // SRA

                    // Comparison operations
                    (0x2, 0x00) => Instruction::Slt { rd, rs1, rs2 }, // SLT
                    (0x3, 0x00) => Instruction::Sltu { rd, rs1, rs2 }, // SLTU

                    // Logical operations
                    (0x4, 0x00) => Instruction::Xor { rd, rs1, rs2 }, // XOR
                    (0x6, 0x00) => Instruction::Or { rd, rs1, rs2 },  // OR
                    (0x7, 0x00) => Instruction::And { rd, rs1, rs2 }, // AND

                    // Multiplication operations (M extension)
                    (0x0, 0x01) => Instruction::Mul { rd, rs1, rs2 }, // MUL
                    (0x1, 0x01) => Instruction::Mulh { rd, rs1, rs2 }, // MULH
                    (0x2, 0x01) => Instruction::Mulhsu { rd, rs1, rs2 }, // MULHSU
                    (0x3, 0x01) => Instruction::Mulhu { rd, rs1, rs2 }, // MULHU
                    (0x4, 0x01) => Instruction::Div { rd, rs1, rs2 }, // DIV
                    (0x5, 0x01) => Instruction::Divu { rd, rs1, rs2 }, // DIVU
                    (0x6, 0x01) => Instruction::Rem { rd, rs1, rs2 }, // REM
                    (0x7, 0x01) => Instruction::Remu { rd, rs1, rs2 }, // REMU

                    // Unknown combination
                    _ => Instruction::Unsupported(word),
                }
            }
            0x13 => {
                // I-type immediate instructions
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
                    0x0 => Instruction::Addi { rd, rs1, imm }, // ADDI
                    0x1 => {
                        // SLLI: shift amount in lower 5 bits, upper 7 bits must be 0x00
                        let shamt = (imm_raw & 0x1F) as u8;
                        let upper_bits = (imm_raw >> 5) & 0x7F;
                        if upper_bits == 0x00 {
                            Instruction::Slli { rd, rs1, shamt }
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    0x2 => Instruction::Slti { rd, rs1, imm }, // SLTI
                    0x3 => Instruction::Sltiu { rd, rs1, imm }, // SLTIU
                    0x4 => Instruction::Xori { rd, rs1, imm }, // XORI
                    0x5 => {
                        // SRLI/SRAI: shift amount in lower 5 bits
                        // upper 7 bits: 0x00 for SRLI, 0x20 for SRAI
                        let shamt = (imm_raw & 0x1F) as u8;
                        let upper_bits = (imm_raw >> 5) & 0x7F;
                        if upper_bits == 0x00 {
                            Instruction::Srli { rd, rs1, shamt } // SRLI
                        } else if upper_bits == 0x20 {
                            Instruction::Srai { rd, rs1, shamt } // SRAI
                        } else {
                            Instruction::Unsupported(word)
                        }
                    }
                    0x6 => Instruction::Ori { rd, rs1, imm }, // ORI
                    0x7 => Instruction::Andi { rd, rs1, imm }, // ANDI
                    _ => unreachable!("funct3 is masked to 3 bits, so it's always 0-7"),
                }
            }
            0x03 => {
                // Load instructions
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
                    0x0 => Instruction::Lb { rd, rs1, imm },  // LB
                    0x1 => Instruction::Lh { rd, rs1, imm },  // LH
                    0x2 => Instruction::Lw { rd, rs1, imm },  // LW
                    0x4 => Instruction::Lbu { rd, rs1, imm }, // LBU
                    0x5 => Instruction::Lhu { rd, rs1, imm }, // LHU
                    _ => Instruction::Unsupported(word),
                }
            }
            0x23 => {
                // Store instructions
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
                    0x0 => Instruction::Sb { rs1, rs2, imm }, // SB
                    0x1 => Instruction::Sh { rs1, rs2, imm }, // SH
                    0x2 => Instruction::Sw { rs1, rs2, imm }, // SW
                    _ => Instruction::Unsupported(word),
                }
            }
            0x63 => {
                // Branch instructions
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
                    0x0 => Instruction::Beq { rs1, rs2, imm },  // BEQ
                    0x1 => Instruction::Bne { rs1, rs2, imm },  // BNE
                    0x4 => Instruction::Blt { rs1, rs2, imm },  // BLT
                    0x5 => Instruction::Bge { rs1, rs2, imm },  // BGE
                    0x6 => Instruction::Bltu { rs1, rs2, imm }, // BLTU
                    0x7 => Instruction::Bgeu { rs1, rs2, imm }, // BGEU
                    _ => Instruction::Unsupported(word),
                }
            }
            0x6F => {
                // JAL (Jump and Link)
                // JAL is J-type instruction
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;

                // J-type immediate reconstruction
                // The immediate is encoded in a scrambled format:
                // inst[31] = imm[20], inst[30:21] = imm[10:1], inst[20] = imm[11], inst[19:12] = imm[19:12]
                let bit_20 = (word & IMM_J_20_MASK) >> IMM_J_20_SHIFT;
                let bits_19_12 = (word & IMM_J_19_12_MASK) >> IMM_J_19_12_SHIFT;
                let bit_11 = (word & IMM_J_11_MASK) >> IMM_J_11_SHIFT;
                let bits_10_1 = (word & IMM_J_10_1_MASK) >> IMM_J_10_1_SHIFT;

                // Reconstruct the immediate value (21 bits, with bit 0 always 0)
                let imm_raw =
                    (bit_20 << 20) | (bits_19_12 << 12) | (bit_11 << 11) | (bits_10_1 << 1);

                // Sign-extend from 21 bits to 32 bits
                let imm = if imm_raw & 0x100000 != 0 {
                    // Sign bit is set, sign-extend
                    (imm_raw | 0xFFE00000) as i32
                } else {
                    imm_raw as i32
                };

                Instruction::Jal { rd, imm }
            }
            0x67 => {
                // JALR (Jump and Link Register)
                // JALR is I-type instruction with funct3 = 0
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

                if funct3 == 0x0 {
                    // JALR uses funct3 = 0x0
                    Instruction::Jalr { rd, rs1, imm }
                } else {
                    Instruction::Unsupported(word)
                }
            }
            0x37 => {
                // LUI (Load Upper Immediate)
                // LUI is U-type instruction
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;

                // U-type immediate is already in the correct position (bits 31:12)
                // We just need to extract it
                let imm = (word & IMM_U_MASK) >> IMM_U_SHIFT;

                Instruction::Lui { rd, imm }
            }
            0x17 => {
                // AUIPC (Add Upper Immediate to PC)
                // AUIPC is U-type instruction
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;

                // U-type immediate is already in the correct position (bits 31:12)
                // We just need to extract it
                let imm = (word & IMM_U_MASK) >> IMM_U_SHIFT;

                Instruction::Auipc { rd, imm }
            }
            0x73 => {
                // System instructions
                // System instructions - check the immediate field to determine which one
                // For ECALL and EBREAK, funct3 must be 0 and rs1, rd must be 0
                let funct3 = (((word & FUNCT3_MASK) >> FUNCT3_SHIFT) & 0x7) as u8;
                let rd = ((word & RD_MASK) >> RD_SHIFT) as u8;
                let rs1 = ((word & RS1_MASK) >> RS1_SHIFT) as u8;
                let imm = (word & IMM_I_MASK) >> IMM_I_SHIFT;

                if funct3 == 0 && rd == 0 && rs1 == 0 {
                    match imm {
                        0x000 => Instruction::Ecall,  // ECALL
                        0x001 => Instruction::Ebreak, // EBREAK
                        _ => Instruction::Unsupported(word),
                    }
                } else {
                    Instruction::Unsupported(word)
                }
            }
            _ => Instruction::Unsupported(word),
        }
    }

    /// Encode an instruction into a 32-bit instruction word
    ///
    /// # Returns
    ///
    /// Returns the encoded 32-bit instruction word on success, or an `EncodeError` if the
    /// instruction cannot be encoded.
    ///
    /// # Errors
    ///
    /// Returns `EncodeError::NotImplemented` for instruction variants that have not yet been
    /// implemented for encoding.
    pub fn encode(&self) -> Result<u32, EncodeError> {
        match self {
            Instruction::Add { rd, rs1, rs2 } => {
                Ok(encode_r_type(0x33, *rd, 0x0, *rs1, *rs2, 0x00))
            }
            Instruction::Sub { .. } => Err(EncodeError::NotImplemented("Sub")),
            Instruction::Sll { .. } => Err(EncodeError::NotImplemented("Sll")),
            Instruction::Xor { .. } => Err(EncodeError::NotImplemented("Xor")),
            Instruction::Or { .. } => Err(EncodeError::NotImplemented("Or")),
            Instruction::Srl { .. } => Err(EncodeError::NotImplemented("Srl")),
            Instruction::Sra { .. } => Err(EncodeError::NotImplemented("Sra")),
            Instruction::Slt { .. } => Err(EncodeError::NotImplemented("Slt")),
            Instruction::Sltu { .. } => Err(EncodeError::NotImplemented("Sltu")),
            Instruction::And { .. } => Err(EncodeError::NotImplemented("And")),
            Instruction::Addi { .. } => Err(EncodeError::NotImplemented("Addi")),
            Instruction::Andi { .. } => Err(EncodeError::NotImplemented("Andi")),
            Instruction::Ori { .. } => Err(EncodeError::NotImplemented("Ori")),
            Instruction::Xori { .. } => Err(EncodeError::NotImplemented("Xori")),
            Instruction::Slti { .. } => Err(EncodeError::NotImplemented("Slti")),
            Instruction::Sltiu { .. } => Err(EncodeError::NotImplemented("Sltiu")),
            Instruction::Slli { .. } => Err(EncodeError::NotImplemented("Slli")),
            Instruction::Srli { .. } => Err(EncodeError::NotImplemented("Srli")),
            Instruction::Srai { .. } => Err(EncodeError::NotImplemented("Srai")),
            Instruction::Lb { .. } => Err(EncodeError::NotImplemented("Lb")),
            Instruction::Lh { .. } => Err(EncodeError::NotImplemented("Lh")),
            Instruction::Lw { .. } => Err(EncodeError::NotImplemented("Lw")),
            Instruction::Lbu { .. } => Err(EncodeError::NotImplemented("Lbu")),
            Instruction::Lhu { .. } => Err(EncodeError::NotImplemented("Lhu")),
            Instruction::Sb { .. } => Err(EncodeError::NotImplemented("Sb")),
            Instruction::Sh { .. } => Err(EncodeError::NotImplemented("Sh")),
            Instruction::Sw { .. } => Err(EncodeError::NotImplemented("Sw")),
            Instruction::Beq { .. } => Err(EncodeError::NotImplemented("Beq")),
            Instruction::Bne { .. } => Err(EncodeError::NotImplemented("Bne")),
            Instruction::Blt { .. } => Err(EncodeError::NotImplemented("Blt")),
            Instruction::Bge { .. } => Err(EncodeError::NotImplemented("Bge")),
            Instruction::Bltu { .. } => Err(EncodeError::NotImplemented("Bltu")),
            Instruction::Bgeu { .. } => Err(EncodeError::NotImplemented("Bgeu")),
            Instruction::Mul { .. } => Err(EncodeError::NotImplemented("Mul")),
            Instruction::Mulh { .. } => Err(EncodeError::NotImplemented("Mulh")),
            Instruction::Mulhsu { .. } => Err(EncodeError::NotImplemented("Mulhsu")),
            Instruction::Mulhu { .. } => Err(EncodeError::NotImplemented("Mulhu")),
            Instruction::Div { .. } => Err(EncodeError::NotImplemented("Div")),
            Instruction::Divu { .. } => Err(EncodeError::NotImplemented("Divu")),
            Instruction::Rem { .. } => Err(EncodeError::NotImplemented("Rem")),
            Instruction::Remu { .. } => Err(EncodeError::NotImplemented("Remu")),
            Instruction::Jal { .. } => Err(EncodeError::NotImplemented("Jal")),
            Instruction::Jalr { .. } => Err(EncodeError::NotImplemented("Jalr")),
            Instruction::Lui { .. } => Err(EncodeError::NotImplemented("Lui")),
            Instruction::Auipc { .. } => Err(EncodeError::NotImplemented("Auipc")),
            Instruction::Ecall => Err(EncodeError::NotImplemented("Ecall")),
            Instruction::Ebreak => Err(EncodeError::NotImplemented("Ebreak")),
            Instruction::Unsupported(_) => Err(EncodeError::NotImplemented("Unsupported")),
        }
    }
}

/// Encode an R-type instruction
fn encode_r_type(opcode: u32, rd: u8, funct3: u32, rs1: u8, rs2: u8, funct7: u32) -> u32 {
    opcode
        | ((rd as u32) << RD_SHIFT)
        | (funct3 << FUNCT3_SHIFT)
        | ((rs1 as u32) << RS1_SHIFT)
        | ((rs2 as u32) << RS2_SHIFT)
        | (funct7 << FUNCT7_SHIFT)
}
