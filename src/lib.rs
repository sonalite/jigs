//! Jigs - A high-performance RISC-V runtime for ARM64 systems
//!
//! This library provides RISC-V 32-bit instruction decoding with planned support for:
//! - AOT compilation to native ARM64
//! - Gas-metered execution for controlled resource usage

pub mod instruction;
pub mod memory;

#[cfg(test)]
mod tests;

pub use instruction::{EncodeError, Instruction};
pub use memory::{Memory, PageStore};
