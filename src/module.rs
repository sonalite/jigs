use crate::{compiler::Compiler, instruction::Instruction, memory::Memory};
use std::ptr;

/// Maximum ARM64 code size as a multiple of RISC-V code size
/// ARM64 instructions can require more space for register spilling,
/// immediate loading sequences, and syscall handling
const ARM64_CODE_SIZE_MULTIPLIER: usize = 4;

/// Compiled ARM64 code module containing translated RISC-V instructions
pub struct Module {
    /// Number of instances currently attached to this module
    pub(crate) instance_count: usize,
    /// Pointer to pointer to the attached instance's memory
    /// This is a Box<*mut Memory> so the compiled code can access memory
    /// through this stable pointer, even when the instance changes
    pub(crate) memory_ptr: Box<*mut Memory>,
    /// Buffer containing compiled ARM64 machine code
    code_buffer: *mut u8,
    /// Size of the code buffer in bytes
    code_buffer_size: usize,
    /// Size of the actual compiled code in bytes
    code_size: usize,
}

impl Module {
    /// Create a new Module
    ///
    /// # Arguments
    /// * `max_code_size` - Maximum expected size of RISC-V code (for buffer allocation)
    ///
    /// # Returns
    /// Empty module ready to receive code via set_code()
    pub fn new(max_code_size: usize) -> Result<Module, CompileError> {
        // Calculate ARM64 code buffer size based on RISC-V code size
        let code_buffer_size = max_code_size * ARM64_CODE_SIZE_MULTIPLIER;

        // macOS requires MAP_JIT flag to allocate executable memory on ARM64
        #[cfg(target_os = "macos")]
        let mmap_flags = libc::MAP_PRIVATE | libc::MAP_ANON | libc::MAP_JIT;
        #[cfg(not(target_os = "macos"))]
        let mmap_flags = libc::MAP_PRIVATE | libc::MAP_ANON;

        // Allocate code buffer with read/write permissions initially
        let code_buffer = unsafe {
            let ptr = libc::mmap(
                ptr::null_mut(),
                code_buffer_size,
                libc::PROT_READ | libc::PROT_WRITE,
                mmap_flags,
                -1,
                0,
            );

            if ptr == libc::MAP_FAILED {
                return Err(CompileError::AllocationFailed);
            }

            // mmap returns page-aligned memory, which is always 4-byte aligned
            ptr as *mut u8
        };

        Ok(Module {
            instance_count: 0,
            memory_ptr: Box::new(std::ptr::null_mut()),
            code_buffer,
            code_buffer_size,
            code_size: 0,
        })
    }

    /// Set and compile new RISC-V code for this module
    ///
    /// # Arguments
    /// * `code` - RISC-V machine code to compile
    ///
    /// # Returns
    /// Ok(()) if compilation succeeds
    ///
    /// # Errors
    /// Returns error if instances are attached, code is too large, or compilation fails
    pub fn set_code(&mut self, code: &[u8]) -> Result<(), CompileError> {
        // Check that no instances are attached
        if self.instance_count != 0 {
            return Err(CompileError::InstancesAttached);
        }

        // Check that code size doesn't exceed buffer capacity
        let required_size = code.len() * ARM64_CODE_SIZE_MULTIPLIER;
        if required_size > self.code_buffer_size {
            return Err(CompileError::CodeTooLarge);
        }

        // Decode RISC-V instructions
        let mut instructions = Vec::new();
        for chunk in code.chunks_exact(4) {
            let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let instr = Instruction::decode(word);
            instructions.push(instr);
        }

        // Compile to ARM64
        let mut compiler = Compiler::new();
        let arm64_code = compiler.compile(&instructions);

        // Copy compiled code to the code buffer
        self.code_size = arm64_code.len();
        unsafe {
            ptr::copy_nonoverlapping(arm64_code.as_ptr(), self.code_buffer, self.code_size);

            // Make the code executable
            if libc::mprotect(
                self.code_buffer as *mut libc::c_void,
                self.code_buffer_size,
                libc::PROT_READ | libc::PROT_EXEC,
            ) != 0
            {
                return Err(CompileError::AllocationFailed);
            }
        }

        Ok(())
    }

    /// Get a slice of the compiled ARM64 code
    pub fn code(&self) -> &[u8] {
        if self.code_size == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.code_buffer, self.code_size) }
        }
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

        // Free the code buffer
        unsafe {
            libc::munmap(self.code_buffer as *mut libc::c_void, self.code_buffer_size);
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
    /// Failed to allocate memory for code buffer
    AllocationFailed,
    /// Cannot set code while instances are attached
    InstancesAttached,
    /// Code size exceeds the module's buffer capacity
    CodeTooLarge,
}
