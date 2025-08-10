use crate::{memory::Memory, module::Module};
use std::{mem, ptr};

/// Runtime instance for executing compiled RISC-V code
pub struct Instance {
    /// Pointer to the compiled module (null if detached)
    module: *mut Module,
    /// Memory system for this instance (Box for stable pointer)
    memory: Box<Memory>,
}

impl Instance {
    /// Create a new instance with the given memory
    pub fn new(memory: Memory) -> Self {
        Instance {
            module: ptr::null_mut(),
            memory: Box::new(memory),
        }
    }

    /// Attach this instance to a module
    ///
    /// # Safety
    /// The module must outlive this instance unless detached
    pub fn attach(&mut self, module: &mut Module) {
        if !self.module.is_null() {
            self.detach();
        }
        self.module = module as *mut Module;
        unsafe {
            (*self.module).instance_count += 1;
            // Set the module's memory pointer to point to this instance's memory
            *(*self.module).memory_ptr = &mut *self.memory as *mut Memory;
        }
    }

    /// Detach this instance from its module
    pub fn detach(&mut self) {
        if !self.module.is_null() {
            unsafe {
                (*self.module).instance_count -= 1;
                // Clear the module's memory pointer when detaching
                *(*self.module).memory_ptr = ptr::null_mut();
            }
            self.module = ptr::null_mut();
        }
    }

    /// Check if this instance is attached to a module
    pub fn attached(&self) -> bool {
        !self.module.is_null()
    }

    /// Get a reference to this instance's memory
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Get a mutable reference to this instance's memory
    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    /// Call a function in the compiled module
    ///
    /// # Safety
    /// - Instance must be attached to a module
    /// - Function index must be valid
    /// - Module's compiled code must be valid ARM64 instructions
    pub unsafe fn call_function(&mut self, _function_index: usize) -> Result<(), &'static str> {
        unsafe {
            if self.module.is_null() {
                return Err("Instance not attached to module");
            }

            let module = &*self.module;

            // Get the compiled code from the module
            let code = module.code();
            if code.is_empty() {
                return Err("Module has no compiled code");
            }

            // Cast the code buffer to a function pointer
            // The compiled code should start with the function we want to call
            let fn_ptr = code.as_ptr() as *const ();
            let func: extern "C" fn() = mem::transmute(fn_ptr);

            // Call the function
            func();

            Ok(())
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.detach();
    }
}
