//! ARM64 instruction encoding for AOT compilation
//!
//! This module provides ARM64 machine code generation helpers and instruction
//! encoding utilities for translating RISC-V instructions to native ARM64.

/// RET instruction (return to link register)
/// Encoding: 1101011_0010_11111_000000_11110_00000
pub const RET: u32 = 0xD65F03C0;
