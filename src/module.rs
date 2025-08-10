/// Compiled ARM64 code module containing translated RISC-V instructions
pub struct Module {
    /// Number of instances currently attached to this module
    pub(crate) instance_count: usize,
}

impl Module {
    /// Compile RISC-V code into ARM64 instructions
    ///
    /// # Arguments
    /// * `code` - RISC-V machine code to compile
    ///
    /// # Returns
    /// Compiled module ready for execution
    pub fn compile(code: &[u8]) -> Result<Module, CompileError> {
        // TODO: Implement compilation
        let _ = code;
        Ok(Module { instance_count: 0 })
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        if self.instance_count != 0 {
            panic!(
                "Module dropped with {} attached instances",
                self.instance_count
            );
        }
    }
}

/// Errors that can occur during module compilation
#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    /// The provided code is not valid RISC-V instructions
    InvalidCode,
    /// Compilation is not yet implemented
    NotImplemented,
}
