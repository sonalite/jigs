use crate::{memory::Memory, module::Module};
use std::ptr;

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
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.detach();
    }
}
