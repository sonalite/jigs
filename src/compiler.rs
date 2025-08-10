//! ARM64 compiler for RISC-V instructions
//!
//! This module provides AOT (Ahead-Of-Time) compilation of RISC-V instructions
//! to native ARM64 machine code.

use crate::{Instruction, arm64};

/// Compiles RISC-V instructions to ARM64 machine code
pub struct Compiler;

impl Compiler {
    /// Creates a new compiler instance
    pub fn new() -> Self {
        Self
    }

    /// Compiles a slice of RISC-V instructions to ARM64
    ///
    /// Currently only emits a single RET instruction regardless of input
    /// Returns the number of bytes written to the buffer
    pub fn compile(&mut self, _instructions: &[Instruction], buffer: &mut [u8]) -> usize {
        // For now, just emit a RET instruction
        let ret_bytes = arm64::RET.to_le_bytes();

        // Ensure buffer has enough space
        if buffer.len() < ret_bytes.len() {
            return 0;
        }

        buffer[..ret_bytes.len()].copy_from_slice(&ret_bytes);
        ret_bytes.len()
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
